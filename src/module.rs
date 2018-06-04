/// Interfacing with WASM.

use std::path::Path;
use std::fmt;
use std::iter;
use std::collections::HashMap;
use parity_wasm::elements::*;

use either::Either;
use tracer::{EntryKind, EXPOSE_TRACER, EXPOSE_TRACER_LEN, LOG_CALL};

#[derive(Debug)]
pub struct WasmModule {
    module: Module,
    function_names: HashMap<usize, String>,
}

impl WasmModule {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let module = deserialize_file(path)?;
        let mut result = WasmModule::new(module);
        result.function_names = result.exported_function_names();

        Ok(result)
    }

    pub fn to_file<P: AsRef<Path>>(path: P, wasm_module: WasmModule) -> Result<(), Error> {
        serialize_to_file(path, wasm_module.module)
    }

    fn new(module: Module) -> Self {
        WasmModule {
            module,
            function_names: HashMap::new(),
        }
    }

    pub fn imports(&self) -> impl Iterator<Item = &ImportEntry> {
        self.module
            .import_section()
            .map_or(Either::Left(iter::empty()),
                    |section| Either::Right(section.entries().iter()))
    }

    /// Safe function for counting the number of imported functions.
    /// Use this instead of `self.imports().size_hint()`!
    pub fn imported_functions_count(&self) -> usize {
        self.module.import_count(ImportCountType::Function)
    }

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
    /// According to the [WebAssembly design docs](https://github.com/sunfishcode/
    /// wasm-reference-manual/blob/master/WebAssembly.md):
    ///
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

    /// Instrument a .wasm module by adding a prologue and epilogue to each function.
    /// For now, only add tracing calls to exported functions.
    pub fn instrument_module(&mut self) {
        let mut log_call = 0;

        for (key, value) in self.function_names.clone().into_iter() {
            if value == LOG_CALL {
                log_call = key;
            }
        }

        self.add_tracing_instructions(log_call);
    }

    fn add_tracing_instructions(&mut self, log_call: usize) {
        let imports_count = self.imported_functions_count();
        if let Some(section) = self.module.code_section_mut() {
            for (i, body) in section.bodies_mut().iter_mut().enumerate() {
                let id = i + imports_count;
                // TODO: Refactor this when we switch to using the names section.
                match self.function_names.get(&id) {
                    None => {
                        // Only instrument exported functions for now.
                        continue;
                    }
                    Some(name) if name == EXPOSE_TRACER || name == EXPOSE_TRACER_LEN ||
                                  name == LOG_CALL => {
                        // Don't instrument functions that are part of tracing, or we'll get an infinite loop.
                        continue;
                    }
                    _ => {
                        let prologue = vec![Instruction::I32Const(EntryKind::FunctionCall as i32),
                                            Instruction::I32Const(id as i32),
                                            Instruction::Call(log_call as u32)];

                        let void_return_value = -1;
                        let epilogue = vec![Instruction::I32Const(EntryKind::FunctionReturnVoid as
                                                                  i32),
                                            Instruction::I32Const(void_return_value as i32),
                                            Instruction::Call(log_call as u32)];

                        // If the function has a return value, the penultimate instruction will be a
                        // `get_local($return_value_index)`.
                        // let insts = body.code().elements();
                        // // Ensure a penultimate instruction exists.
                        // if insts.len() >= 2 {
                        //     let penultimate_index = insts.len() - 2;
                        //     if let Some(Instruction::GetLocal(ret_val_id)) =
                        //         insts.get(penultimate_index) {}
                        // }

                        WasmModule::instrument_function(body, prologue, epilogue);
                    }
                }
            }
        }
    }

    fn instrument_function(body: &mut FuncBody,
                           prologue: Vec<Instruction>,
                           epilogue: Vec<Instruction>) {
        let insts = body.code_mut().elements_mut();

        for (i, inst) in prologue.into_iter().enumerate() {
            insts.insert(i, inst);
        }

        // needs to put Instruction::Call right before Instruction::End
        // as the second to last instruction
        // putting it at size - 1 puts at the pos of the last element & pushes  the last element back
        // e.g. to insert 3 second to last in [0, 1, 2] we put it at index 2 (size - 1)
        for inst in epilogue.into_iter() {
            let i = insts.len() - 1;
            insts.insert(i, inst);
        }
    }

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

    pub fn get_function_name(&self, id: usize) -> Option<&str> {
        self.function_names.get(&id).map(String::as_str)
    }

    pub fn function_types(&self) -> impl Iterator<Item = &Type> {
        self.function_type_refs()
            .iter()
            .map(move |&func| {
                     self.get_type(func.type_ref())
                         .expect("Invalid module: could not get type for function")
                 })
    }

    pub fn own_functions_count(&self) -> usize {
        self.module
            .function_section()
            .map_or(0, |sec| sec.entries().len())
    }

    pub fn exports(&self) -> &[ExportEntry] {
        self.module
            .export_section()
            .map_or(&[], ExportSection::entries)
    }

    pub fn types(&self) -> &[Type] {
        self.module
            .type_section()
            .map_or(&[], TypeSection::types)
    }

    pub fn get_type(&self, tyid: u32) -> Option<&Type> {
        self.module
            .type_section()
            .and_then(|sec| sec.types().get(tyid as usize))
    }

    fn function_type_refs(&self) -> &[Func] {
        self.module
            .function_section()
            .map_or(&[], FunctionSection::entries)
    }

    pub fn function_bodies(&self) -> &[FuncBody] {
        self.module
            .code_section()
            .map_or(&[], CodeSection::bodies)
    }
}

#[derive(Debug, PartialEq)]
pub struct WasmFunction<'a> {
    id: usize,
    ty: &'a Type,
    name: Option<&'a str>,
    body: Option<&'a FuncBody>,
    source: SourceSection,
}

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
    use super::{WasmModule, WasmFunction};

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
    fn insert_tracing_instructions() {
        // all exported functions in this .wasm
        let file = "./tests/function-names.wasm";
        let mut module = WasmModule::from_file(file).unwrap();
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

        // TODO: Clean this up to make it clearer.
        let mock_log_call: u32 = 999;
        let return_value: i32 = -1;
        let after_insertion = vec![vec![Instruction::I32Const(0),
                                        Instruction::I32Const(0),
                                        Instruction::Call(mock_log_call),

                                        Instruction::GetLocal(1),
                                        Instruction::GetLocal(0),
                                        Instruction::I32Add,

                                        Instruction::I32Const(1),
                                        Instruction::I32Const(return_value),
                                        Instruction::Call(mock_log_call),
                                        Instruction::End],
                                   vec![Instruction::I32Const(0),
                                        Instruction::I32Const(1),
                                        Instruction::Call(mock_log_call),

                                        Instruction::GetLocal(0),
                                        Instruction::GetLocal(0),
                                        Instruction::Call(0),
                                        Instruction::GetLocal(0),
                                        Instruction::I32Add,

                                        Instruction::I32Const(1),
                                        Instruction::I32Const(return_value),
                                        Instruction::Call(mock_log_call),
                                        Instruction::End],
                                   vec![Instruction::I32Const(0),
                                        Instruction::I32Const(2),
                                        Instruction::Call(mock_log_call),

                                        Instruction::GetLocal(0),
                                        Instruction::F64Const(4602678819172646912),
                                        Instruction::F64Mul,

                                        Instruction::I32Const(1),
                                        Instruction::I32Const(return_value),
                                        Instruction::Call(mock_log_call),
                                        Instruction::End],
                                   vec![Instruction::I32Const(0),
                                        Instruction::I32Const(3),
                                        Instruction::Call(mock_log_call),

                                        Instruction::GetLocal(0),
                                        Instruction::I32Const(1),
                                        Instruction::I32Shl,

                                        Instruction::I32Const(1),
                                        Instruction::I32Const(return_value),
                                        Instruction::Call(mock_log_call),
                                        Instruction::End]];

        for (i, f) in module.functions().enumerate() {
            for (j, inst) in f.instructions().enumerate() {
                assert_eq!(*inst, before_insertion[i][j]);
            }
        }

        module.add_tracing_instructions(mock_log_call as usize);

        for (i, f) in module.functions().enumerate() {
            for (j, inst) in f.instructions().enumerate() {
                assert_eq!(*inst, after_insertion[i][j]);
            }
        }

    }


}
