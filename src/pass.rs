/// Interfacing with WASM.
#[cfg(test)]
mod test {
    use parity_wasm::elements::*;

    #[test]
    fn count_functions() {
        let module = parity_wasm::deserialize_file("./test/simple/function-names.wasm").unwrap();
        assert_eq!(module.functions_space(), 4);
    }

    #[test]
    fn list_instructions() {
        let module = parity_wasm::deserialize_file("./test/simple/function-names.wasm").unwrap();
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
