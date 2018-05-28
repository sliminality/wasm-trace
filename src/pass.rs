/// Interfacing with WASM.

use std::path::Path;
use std::fmt;
use std::iter;
use std::collections::HashMap;
use parity_wasm::elements::*;

use util::Either;

#[derive(Debug)]
pub struct WasmModule {
    module: Module,
    function_names: HashMap<usize, String>,
}

impl WasmModule {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let module = deserialize_file(path)?;
        let mut result = WasmModule::new(module);
        let function_names = result.exported_function_names();
        result.function_names = function_names;
        Ok(result)
    }

    fn new(module: Module) -> Self {
        WasmModule {
            module,
            function_names: HashMap::new(),
        }
    }

    pub fn functions(&self) -> impl Iterator<Item = WasmFunction> {
        let function_count = self.module.functions_space();
        if function_count == 0 {
            return Either::A(iter::empty::<WasmFunction>());
        }

        let bodies = self.function_bodies();
        let types = self.function_types();
        assert_eq!(function_count, bodies.size_hint().0);
        assert_eq!(function_count, types.size_hint().0);

        let functions = types
            .zip(bodies)
            .enumerate()
            .map(move |(id, (ty, body))| {
                     let name = self.get_function_name(id);
                     let is_export = name.is_some(); // TODO(slim): Make this more robust later.
                     WasmFunction::new(id, ty, name, body, is_export)
                 });

        Either::B(functions)
    }

    pub fn print_functions(&self) {
        for f in self.functions() {
            println!("{}", f);
        }
    }

    fn exported_function_names(&self) -> HashMap<usize, String> {
        let mut names = HashMap::new();
        for export in self.exports() {
            if let Internal::Function(id) = export.internal() {
                let name = export.field().to_owned();
                names.insert(*id as usize, name);
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

    pub fn exports(&self) -> impl Iterator<Item = &ExportEntry> {
        self.module
            .export_section()
            .map_or(Either::A(iter::empty()),
                    |sec| Either::B(sec.entries().iter()))
    }

    pub fn types(&self) -> impl Iterator<Item = &Type> {
        self.module
            .type_section()
            .map_or(Either::A(iter::empty::<&Type>()),
                    |sec| Either::B(sec.types().iter()))
    }

    pub fn get_type(&self, tyid: u32) -> Option<&Type> {
        self.module
            .type_section()
            .and_then(|sec| sec.types().get(tyid as usize))
    }

    fn function_type_refs(&self) -> impl Iterator<Item = &Func> {
        self.module
            .function_section()
            .map_or(Either::A(iter::empty::<&Func>()),
                    |sec| Either::B(sec.entries().iter()))
    }

    pub fn function_bodies(&self) -> impl Iterator<Item = &FuncBody> {
        self.module
            .code_section()
            .map_or(Either::A(iter::empty::<&FuncBody>()),
                    |sec| Either::B(sec.bodies().iter()))
    }
}

#[derive(Debug)]
pub struct WasmFunction<'a> {
    id: usize,
    ty: &'a Type,
    name: Option<&'a str>,
    body: &'a FuncBody,
    is_export: bool,
}

impl<'a> WasmFunction<'a> {
    pub fn new(id: usize,
               ty: &'a Type,
               name: Option<&'a str>,
               body: &'a FuncBody,
               is_export: bool)
               -> Self {
        WasmFunction {
            id,
            ty,
            name,
            body,
            is_export,
        }
    }

    pub fn instructions(&self) -> impl Iterator<Item = &Instruction> {
        self.body.code().elements().iter()
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

        write!(f, "{} : {}\n{}", name_part, ty_part, instructions)
    }
}

#[cfg(test)]
mod test {
    use parity_wasm::elements::*;
    use super::WasmModule;

    static FILE: &str = "./test/function-names.wasm";

    #[test]
    fn list_functions() {
        let module = WasmModule::from_file(FILE).unwrap();
        let functions = module.function_names;
        let expected =
            map!{ 0 => "_Z3addii", 1 => "_Z4add1i", 2 => "_Z5halved", 3 => "_Z7doubleri" };
        assert_eq!(functions, expected);
    }

    #[test]
    fn count_functions() {
        let module = parity_wasm::deserialize_file(FILE).unwrap();
        assert_eq!(module.functions_space(), 4);
    }

    #[test]
    fn list_instructions() {
        let module = WasmModule::from_file(FILE).unwrap();
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
