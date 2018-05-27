/// Interfacing with WASM.

use std::path::Path;
use std::collections::HashMap;
use parity_wasm::elements::*;

#[derive(Debug)]
pub struct WasmModule {
    module: Module,
    function_names: HashMap<u32, String>,
}

impl WasmModule {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let module = deserialize_file(path)?;
        let mut function_names = HashMap::new();

        if let Some(exports) = module.export_section() {
            for export in exports.entries() {
                if let Internal::Function(i) = export.internal() {
                    let name = export.field();
                    function_names.insert(*i, name.to_owned());
                }
            }
        }

        Ok(WasmModule {
               module,
               function_names,
           })
    }

    pub fn print_instructions(&self) {
        let bodies = self.module.code_section().unwrap().bodies();

        for (i, body) in bodies.iter().enumerate() {
            println!("Function index: {}", i);
            let instructions = body.code().elements();
            for instruction in instructions.iter() {
                println!("\t{:?}", instruction);
            }
        }
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
        let expected = map!{0 => "_Z3addii", 3 => "_Z7doubleri", 2 => "_Z5halved", 1 => "_Z4add1i"};
        assert_eq!(functions, expected);
    }

    #[test]
    fn count_functions() {
        let module = parity_wasm::deserialize_file(FILE).unwrap();
        assert_eq!(module.functions_space(), 4);
    }

    #[test]
    fn list_instructions() {
        let module = parity_wasm::deserialize_file(FILE).unwrap();
        let expected = [vec![Instruction::GetLocal(1),
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

        let bodies = module.code_section().unwrap().bodies();

        for (i, body) in bodies.iter().enumerate() {
            println!("Function {}", i);
            let instructions = body.code().elements().iter().zip(expected[i].iter());
            for (actual, exp) in instructions {
                assert_eq!(actual, exp);
            }
        }
    }
}
