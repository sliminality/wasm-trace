//! WebAssembly module and components.

use std::path::Path;
use std::fmt;
use std::iter;
use std::{u32, i32};
use std::collections::HashMap;
use parity_wasm::elements::*;
use itertools::Itertools;

use either::Either;
use tracer::{EntryKind, EXPOSE_TRACER, EXPOSE_TRACER_LEN, LOG_CALL};

static VOID_VALUE_PLACEHOLDER: i32 = i32::MAX;

#[derive(Debug)]
/// Wrapper around the parity-wasm `Module` struct, with convenience functions.
pub struct WasmModule {
    module: Module,
    function_names: HashMap<usize, String>,
}

impl WasmModule {
    /// Deserializes a `.wasm` file to a module.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let module = deserialize_file(path)?;
        let mut result = WasmModule {
            module,
            function_names: HashMap::new(),
        };

        result.function_names = result.exported_function_names();

        Ok(result)
    }

    /// Serializes a module to a file.
    pub fn to_file<P: AsRef<Path>>(path: P, wasm_module: WasmModule) -> Result<(), Error> {
        serialize_to_file(path, wasm_module.module)
    }

    /// Iterates over the module's imports.
    pub fn imports(&self) -> impl Iterator<Item = &ImportEntry> {
        self.module
            .import_section()
            .map_or(Either::Left(iter::empty()),
                    |section| Either::Right(section.entries().iter()))
    }

    /// Counts number of imported functions.
    /// Use this instead of `self.imports().size_hint()`!
    pub fn imported_functions_count(&self) -> usize {
        self.module.import_count(ImportCountType::Function)
    }

    /// Iterates over the imported functions within the function index space of the module.
    /// Imported functions do not have bodies.
    pub fn imported_functions(&self) -> impl Iterator<Item = WasmFunction> {
        self.imports()
            .filter_map(move |import| if let External::Function(tyid) = import.external() {
                     // NOTE: Unlike with Internal::Function(id),
                     // the field of External::Function(_) is an index into
                     // the type section.
                     let name = import.field();
                     let ty = self.get_type(*tyid)
                         .expect(format!("Couldn't get type {} for imported function {}",
                                         tyid,
                                         name)
                                         .as_str());

                     // `i` is an index into the import section, but not all imports are functions,
                     // so we can't use `i` directly as an index into the function index space.
                     // Here, we return a tuple containing the information we need, and construct
                     // the WasmFunction in the next step of the iterator.
                     Some((ty, name))
                 } else {
                     None
                 })
            .enumerate()
            .map(|(i, (ty, name))| WasmFunction {
                 // id is the index in the function index space.
                 // An imported function's id is its order in the import section.
                 id: i,
                 ty,
                 name: Some(name),
                 body: None,
                 source: SourceSection::Import,
             })
    }

    /// Iterates over the function index space of the module.
    /// According to the [WebAssembly design docs](https://github.com/sunfishcode/wasm-reference-manual/blob/master/WebAssembly.md):
    /// > The function index space begins with an index for each imported
    /// > function, in the order the imports appear in the Import Section,
    /// > if present, followed by an index for each function in the Function Section,
    /// > if present, in the order of that section.
    pub fn functions(&self) -> impl Iterator<Item = WasmFunction> {
        let function_count = self.module.functions_space();
        if function_count == 0 {
            return Either::Left(iter::empty::<WasmFunction>());
        }

        let imported_count = self.imported_functions_count();
        let own_count = self.own_functions_count();
        assert_eq!(function_count, imported_count + own_count);

        let own_functions = self.function_types()
            .zip(self.function_bodies())
            .enumerate()
            .map(move |(i, (ty, body))| {
                // Functions from the module function section appear
                // after imported functions, in the index space.
                let id = imported_count + i;
                let name = self.get_function_name(id);
                WasmFunction {
                    id,
                    ty,
                    name,
                    body: Some(body),
                    source: SourceSection::Function,
                }
            });

        let imported_functions = self.imported_functions();
        Either::Right(imported_functions.chain(own_functions))
    }

    /// Instruments a module by adding a prologue and epilogue to each exported function.
    pub fn instrument_module(&mut self) -> Result<(), Error> {
        let logger = self.function_names
            .iter()
            .find(|(_, name)| *name == LOG_CALL)
            .map(|(&id, _)| id);

        if logger.is_none() {
            return Err(Error::Other("Could not find tracing functions in module exports"));
        }

        let mut working = CodeSection::with_bodies(self.function_bodies().to_vec());
        self.add_tracing_instructions(logger.unwrap(), &mut working)?;

        // Replace the module code section with the instrumented bodies.
        if let Some(current_section) = self.module.code_section_mut() {
            *current_section = working;
        } else {
            return Err(Error::Other("Could not replace code section with instrumented version"));
        }

        return Ok(());
    }

    fn add_tracing_instructions(&self,
                                logger_id: usize,
                                working: &mut CodeSection)
                                -> Result<(), Error> {
        let imports_count = self.imported_functions_count();
        let to_instrument = working
            .bodies_mut()
            .iter_mut()
            .zip(self.functions().skip(imports_count))
            .enumerate()
            .filter_map(|(i, (mut_body, func))| {
                let id = i + imports_count;
                // Only instrument exported functions for now.
                match self.function_names.get(&id) {
                    None => None,
                    Some(name) if name == EXPOSE_TRACER || name == EXPOSE_TRACER_LEN ||
                                  name == LOG_CALL => None,
                    _ => {
                        let return_ty = match func.ty {
                            Type::Function(ty) => ty.return_type(),
                        };
                        Some((id, return_ty, mut_body))
                    }
                }
            });

        for (id, return_ty, mut_body) in to_instrument {
            self.instrument_function(logger_id, id, return_ty, mut_body);
        }

        Ok(())
    }

    fn instrument_function(&self,
                           logger_id: usize,
                           id: usize,
                           return_ty: Option<ValueType>,
                           mut_body: &mut FuncBody) {
        let call_logger = Instruction::Call(logger_id as u32);

        // Record that a function call occurred, and the id of the callee.
        let prologue = vec![Instruction::I32Const(EntryKind::FunctionCall as i32),
                            Instruction::I32Const(id as i32),
                            call_logger.clone()];

        // Record returning from the function.
        let mut epilogue = match return_ty {
            // If the function has a return type, we need to capture the returned value from
            // the top of the stack.
            Some(ty) => {
                // Create a new local to store the return value.
                let return_local = Local::new(1, ty);
                let return_local_id: u32 = mut_body.locals().iter().map(|loc| loc.count()).sum();
                mut_body.locals_mut().push(return_local);

                // Capture the top of the stack into our local and return that.
                vec![Instruction::TeeLocal(return_local_id),
                     Instruction::I32Const(EntryKind::FunctionReturnValue as i32),
                     Instruction::GetLocal(return_local_id),
                     call_logger.clone()]
            }
            // If the function has no return value, we simply record that the return
            // is void, and use a placeholder value for the data.
            None => {
                vec![Instruction::I32Const(EntryKind::FunctionReturnVoid as i32),
                     Instruction::I32Const(VOID_VALUE_PLACEHOLDER),
                     call_logger.clone()]
            }
        };

        let mut instrumented = prologue;

        // Iterate over all instructions, using a moving window to check if the
        // next instruction is `return`.
        // If so, append the epilogue onto the instrumented body, along with
        // the current instruction.
        for (curr, next) in mut_body.code().elements().into_iter().tuple_windows() {
            instrumented.push(curr.clone());
            if let Instruction::Return = next {
                instrumented.append(&mut epilogue.clone());
            }
        }

        // Since we iterated over tuple windows but only pushed the first element of
        // each pair, we missed the last instruction, which is always `end` according
        // to the spec.
        // Since `end` implicitly returns, we want to add the epilogue there as well.
        match instrumented.last() {
            // Is the end reachable? If not, there will be nothing on the stack,
            // so `tee_local` will throw an error.
            Some(Instruction::Unreachable) => {}
            Some(_) => {
                instrumented.append(&mut epilogue);
            }
            _ => {}
        };

        // Add the final instruction.
        instrumented.push(Instruction::End);

        // Update the working copy of the function body with the new instructions.
        let ref mut insts = mut_body.code_mut().elements_mut();
        **insts = instrumented;
    }

    /// Prints the index in the function index space, type signature, and instruction
    /// list for each function in this module.
    pub fn print_functions(&self) {
        for f in self.functions() {
            println!("{}", f);
        }
    }

    fn exported_function_names(&self) -> HashMap<usize, String> {
        let mut names = HashMap::new();
        for export in self.exports() {
            match export.internal() {
                Internal::Function(id) => {
                    // NOTE(slim): `id` is an index into the function index space,
                    // not the types section or the function section.
                    let name = export.field().to_owned();
                    names.insert(*id as usize, name);
                }
                // Skip over exports that aren't functions.
                _ => {}
            }
        }
        names
    }

    /// Function name for index of exported function in function index space.
    pub fn get_function_name(&self, id: usize) -> Option<&str> {
        self.function_names.get(&id).map(String::as_str)
    }

    /// Iterates over the type of each function in the function section of the module.
    pub fn function_types(&self) -> impl Iterator<Item = &Type> {
        self.function_type_refs()
            .iter()
            .map(move |&func| {
                     self.get_type(func.type_ref())
                         .expect("Invalid module: could not get type for function")
                 })
    }

    /// Counts the functions the in the module's function section 
    /// (doesn't include imported functions).
    pub fn own_functions_count(&self) -> usize {
        self.module
            .function_section()
            .map_or(0, |sec| sec.entries().len())
    }

    /// Entries in the module's export section.
    pub fn exports(&self) -> &[ExportEntry] {
        self.module
            .export_section()
            .map_or(&[], ExportSection::entries)
    }

    /// Types in the module's type section.
    pub fn types(&self) -> &[Type] {
        self.module
            .type_section()
            .map_or(&[], TypeSection::types)
    }

    /// Type for index in type index space.
    pub fn get_type(&self, tyid: u32) -> Option<&Type> {
        self.module
            .type_section()
            .and_then(|sec| sec.types().get(tyid as usize))
    }

    /// Entries in the module's function section.
    /// Entries in this section are type indices that represent the type signature of the module's functions.
    fn function_type_refs(&self) -> &[Func] {
        self.module
            .function_section()
            .map_or(&[], FunctionSection::entries)
    }

    /// Bodies of the module's functions from the code section.
    pub fn function_bodies(&self) -> &[FuncBody] {
        self.module
            .code_section()
            .map_or(&[], CodeSection::bodies)
    }
}

#[derive(Debug, PartialEq)]
/// WebAssembly function.
pub struct WasmFunction<'a> {
    id: usize,
    ty: &'a Type,
    name: Option<&'a str>,
    body: Option<&'a FuncBody>,
    source: SourceSection,
}

/// Instructions a function body.
impl<'a> WasmFunction<'a> {
    pub fn instructions(&self) -> impl Iterator<Item = &Instruction> {
        self.body
            .map_or(Either::Left(iter::empty()),
                    |body| Either::Right(body.code().elements().iter()))
    }
}

impl<'a> Eq for WasmFunction<'a> {}

impl<'a> fmt::Display for WasmFunction<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name_part = self.name
            .map_or(format!("#{}", self.id),
                    |name| format!("#{} {}", self.id, name));

        let ty_part = match self.ty {
            Type::Function(fn_ty) => {
                let params = fn_ty
                    .params()
                    .iter()
                    .map(|x| format!("{} ", x))
                    .collect::<String>();
                let ret = fn_ty
                    .return_type()
                    .map_or("()".to_owned(), |x| format!("{}", x));
                format!("{}-> {}", params, ret)
            }
        };

        let instructions = self.instructions()
            .map(|inst| format!("\t{:?}\n", inst))
            .collect::<String>();

        write!(f,
               "{:?} {} : {}\n{}",
               self.source,
               name_part,
               ty_part,
               instructions)
    }
}

/// The module section in which the function originates.
#[derive(Debug, PartialEq, Eq)]
pub enum SourceSection {
    Import,
    Function,
}

#[cfg(test)]
mod test {
    use parity_wasm::elements::*;
    use super::{WasmModule, WasmFunction, EntryKind};

    #[test]
    fn list_functions() {
        let file = "./tests/function-names.wasm";
        let module = WasmModule::from_file(file).unwrap();
        let functions = module.functions().collect::<Vec<WasmFunction>>();
        let expected = 
              map!{ 0 => Some("_Z3addii"), 1 => Some("_Z4add1i"), 2 => Some("_Z5halved"), 3 => Some("_Z7doubleri") };
        for (id, name) in expected.into_iter() {
            assert_eq!(name, functions[id].name);
        }
    }

    #[test]
    fn list_functions_with_some_imports() {
        let file = "./tests/imports.wasm";
        let module = WasmModule::from_file(file).unwrap();
        let functions = module.functions().collect::<Vec<WasmFunction>>();
        let expected = [Some("printf"), Some("_Z2hiv")];
        for (id, &name) in expected.into_iter().enumerate() {
            assert_eq!(name, functions[id].name);
        }
    }

    #[test]
    fn list_functions_with_many_imports() {
        let file = "./tests/more-imports.wasm";
        let module = WasmModule::from_file(file).unwrap();
        let mut names = module.functions().map(|f| f.name).enumerate();
        let num_imported_functions = module.imported_functions_count();

        let expected = ["_Z12entered_funcNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE",
                        "_Z11exited_funcNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE"];

        for name in expected.iter() {
            // Check that the function with the given name exists...
            let func = names.find(|&(_, n)| n == Some(name));
            assert_eq!(func.is_some(), true);
            // ...and has an index after the imports.
            assert_eq!(func.unwrap().0 > num_imported_functions, true);
        }
    }

    #[test]
    fn count_functions() {
        let files = [("./tests/function-names.wasm", 4),
                     ("./tests/imports.wasm", 2),
                     ("./tests/more-imports.wasm", 32)];
        for (file, num_functions) in files.iter() {
            let module = WasmModule::from_file(file).unwrap();
            let expected = *num_functions as usize;
            assert_eq!(module.functions().collect::<Vec<WasmFunction>>().len(),
                       expected);
            assert_eq!(module.module.functions_space(), expected);
            assert_eq!(module.own_functions_count() + module.imported_functions_count(),
                       expected);
        }
    }

    #[test]
    /// Check whether we are correctly indexing functions to recover caller/callee names.
    fn track_callee() {
        let file = "./tests/caller-callee-imports.wasm";
        let module = WasmModule::from_file(file).unwrap();

        // Find caller.
        let caller = module
            .functions()
            .find(|f| f.name.map_or(false, |name| name.contains("caller")));
        assert_eq!(caller.is_some(), true, "caller exists");

        // Find instruction where caller calls the callee.
        let caller = caller.unwrap();
        let callee_id = caller
            .instructions()
            .filter_map(|inst| if let Instruction::Call(callee) = inst {
                            Some(callee)
                        } else {
                            None
                        })
            .nth(0);
        assert_eq!(callee_id.is_some(), true, "callee id exists");

        let callee_id = callee_id.unwrap();
        let callee = module.functions().nth(*callee_id as usize);
        assert_eq!(callee.is_some(), true, "callee exists");

        let callee = callee.unwrap();
        let callee_name = callee.name;
        assert_eq!(callee_name.is_some(), true, "callee name exists");
        assert_eq!(callee_name.unwrap().contains("callee"),
                   true,
                   "callee_id is correct");
    }

    #[test]
    fn list_instructions() {
        let file = "./tests/function-names.wasm";
        let module = WasmModule::from_file(file).unwrap();
        let expected = vec![vec![Instruction::GetLocal(1),
                                 Instruction::GetLocal(0),
                                 Instruction::I32Add,
                                 Instruction::End],
                            vec![Instruction::GetLocal(0),
                                 Instruction::GetLocal(0),
                                 Instruction::Call(0),
                                 Instruction::GetLocal(0),
                                 Instruction::I32Add,
                                 Instruction::End],
                            vec![Instruction::GetLocal(0),
                                 Instruction::F64Const(4602678819172646912),
                                 Instruction::F64Mul,
                                 Instruction::End],
                            vec![Instruction::GetLocal(0),
                                 Instruction::I32Const(1),
                                 Instruction::I32Shl,
                                 Instruction::End]];

        for (i, f) in module.functions().enumerate() {
            for (j, inst) in f.instructions().enumerate() {
                assert_eq!(*inst, expected[i][j]);
            }
        }
    }

    #[test]
    fn add_tracing_instructions() {
        let file = "./tests/function-names.wasm";
        let module = WasmModule::from_file(file).unwrap();
        let before_insertion = vec![vec![Instruction::GetLocal(1),
                                         Instruction::GetLocal(0),
                                         Instruction::I32Add,
                                         Instruction::End],
                                    vec![Instruction::GetLocal(0),
                                         Instruction::GetLocal(0),
                                         Instruction::Call(0),
                                         Instruction::GetLocal(0),
                                         Instruction::I32Add,
                                         Instruction::End],
                                    vec![Instruction::GetLocal(0),
                                         Instruction::F64Const(4602678819172646912),
                                         Instruction::F64Mul,
                                         Instruction::End],
                                    vec![Instruction::GetLocal(0),
                                         Instruction::I32Const(1),
                                         Instruction::I32Shl,
                                         Instruction::End]];

        for (i, f) in module.functions().enumerate() {
            for (j, inst) in f.instructions().enumerate() {
                assert_eq!(*inst, before_insertion[i][j]);
            }
        }

        // TODO: Clean this up to make it clearer.
        let mock_log_call: u32 = 999;
        let after_insertion =
            vec![vec![Instruction::I32Const(EntryKind::FunctionCall as i32),
                      Instruction::I32Const(0),
                      Instruction::Call(mock_log_call),

                      Instruction::GetLocal(1),
                      Instruction::GetLocal(0),
                      Instruction::I32Add,

                      Instruction::TeeLocal(0),
                      Instruction::I32Const(EntryKind::FunctionReturnValue as i32),
                      Instruction::GetLocal(0),
                      Instruction::Call(mock_log_call),
                      Instruction::End],

                 vec![Instruction::I32Const(EntryKind::FunctionCall as i32),
                      Instruction::I32Const(1),
                      Instruction::Call(mock_log_call),

                      Instruction::GetLocal(0),
                      Instruction::GetLocal(0),
                      Instruction::Call(0),
                      Instruction::GetLocal(0),
                      Instruction::I32Add,

                      Instruction::TeeLocal(0),
                      Instruction::I32Const(EntryKind::FunctionReturnValue as i32),
                      Instruction::GetLocal(0),
                      Instruction::Call(mock_log_call),
                      Instruction::End],

                 vec![Instruction::I32Const(EntryKind::FunctionCall as i32),
                      Instruction::I32Const(2),
                      Instruction::Call(mock_log_call),

                      Instruction::GetLocal(0),
                      Instruction::F64Const(4602678819172646912),
                      Instruction::F64Mul,

                      Instruction::TeeLocal(0),
                      Instruction::I32Const(EntryKind::FunctionReturnValue as i32),
                      Instruction::GetLocal(0),
                      Instruction::Call(mock_log_call),
                      Instruction::End],

                 vec![Instruction::I32Const(EntryKind::FunctionCall as i32),
                      Instruction::I32Const(3),
                      Instruction::Call(mock_log_call),

                      Instruction::GetLocal(0),
                      Instruction::I32Const(1),
                      Instruction::I32Shl,

                      Instruction::TeeLocal(0),
                      Instruction::I32Const(EntryKind::FunctionReturnValue as i32),
                      Instruction::GetLocal(0),
                      Instruction::Call(mock_log_call),
                      Instruction::End]];

        let mut working = CodeSection::with_bodies(module.function_bodies().to_vec());
        module
            .add_tracing_instructions(mock_log_call as usize, &mut working)
            .unwrap();

        for (i, f) in working.bodies().iter().enumerate() {
            for (j, inst) in f.code().elements().iter().enumerate() {
                assert_eq!(*inst, after_insertion[i][j]);
            }
        }

    }


}
