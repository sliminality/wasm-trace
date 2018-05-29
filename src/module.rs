/// Interfacing with WASM.

use std::path::Path;
use std::fmt;
use std::iter;
use std::collections::HashMap;
use parity_wasm::elements::*;

use either::Either;

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
    pub fn count_imported_functions(&self) -> usize {
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

        let imported_functions = self.imported_functions();
        let imported_count = self.count_imported_functions();
        let own_count = self.count_own_functions();
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

        Either::Right(imported_functions.chain(own_functions))
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
            .map(Func::type_ref)
            .map(move |tyid| {
                     self.get_type(tyid)
                         .expect("Invalid module: could not get type for function")
                 })
    }

    pub fn count_own_functions(&self) -> usize {
        self.module
            .function_section()
            .map_or(0, |sec| sec.entries().len())
    }

    pub fn exports(&self) -> impl Iterator<Item = &ExportEntry> {
        self.module
            .export_section()
            .map_or(Either::Left(iter::empty()),
                    |sec| Either::Right(sec.entries().iter()))
    }

    pub fn types(&self) -> impl Iterator<Item = &Type> {
        self.module
            .type_section()
            .map_or(Either::Left(iter::empty::<&Type>()),
                    |sec| Either::Right(sec.types().iter()))
    }

    pub fn get_type(&self, tyid: u32) -> Option<&Type> {
        self.module
            .type_section()
            .and_then(|sec| sec.types().get(tyid as usize))
    }

    fn function_type_refs(&self) -> impl Iterator<Item = &Func> {
        self.module
            .function_section()
            .map_or(Either::Left(iter::empty::<&Func>()),
                    |sec| Either::Right(sec.entries().iter()))
    }

    pub fn function_bodies(&self) -> impl Iterator<Item = &FuncBody> {
        self.module
            .code_section()
            .map_or(Either::Left(iter::empty::<&FuncBody>()),
                    |sec| Either::Right(sec.bodies().iter()))
    }
}

#[derive(Debug)]
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
#[derive(Debug)]
pub enum SourceSection {
    Import,
    Function,
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::path::Path;
    use parity_wasm::elements::*;
    use super::{WasmModule, WasmFunction};

    fn assert_functions<P: AsRef<Path>>(path: P, expected: &HashMap<usize, String>) {
        let module = WasmModule::from_file(path).unwrap();
        let functions = module.function_names;
        assert_eq!(&functions, expected);
    }

    #[test]
    fn list_functions() {
        let file = "./test/function-names.wasm";
        let expected =
            map!{ 0 => "_Z3addii", 1 => "_Z4add1i", 2 => "_Z5halved", 3 => "_Z7doubleri" };
        assert_functions(file, &expected);
    }

    #[test]
    fn list_functions_with_imports() {
        let file = "./test/imports.wasm";
        let expected = map!{ 1 => "_Z2hiv" };
        assert_functions(file, &expected);
    }

    #[test]
    fn count_functions() {
        let files = [("./test/function-names.wasm", 4),
                     ("./test/imports.wasm", 2),
                     ("./test/more-imports.wasm", 32)];
        for (file, num_functions) in files.iter() {
            let module = WasmModule::from_file(file).unwrap();
            let expected = *num_functions as usize;
            assert_eq!(module.functions().collect::<Vec<WasmFunction>>().len(),
                       expected);
            assert_eq!(module.module.functions_space(), expected);
            assert_eq!(module.count_own_functions() + module.count_imported_functions(),
                       expected);
        }
    }

    #[test]
    fn list_instructions() {
        let file = "./test/function-names.wasm";
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
}
