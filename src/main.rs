extern crate wasm_trace;

use std::env;
use wasm_trace::module::WasmModule;

fn main() {
    let path = env::args().nth(1).expect("USAGE: cargo run module.wasm");
    match WasmModule::from_file(path) {
        Ok(mut module) => {
            println!("\n-------------------\nTypes\n-------------------");
            for (i, t) in module.types().iter().enumerate() {
                println!("{} {:?}", i, t);
            }

            println!("\n------------------\nOrignal Functions\n-------------------");
            for f in module.functions() {
                println!("{}", f);
            }

            module.add_prelude_instruction(Instruction::GetLocal(0));
            module.add_epilogue_instruction(Instruction::GetLocal(0));

            println!("\n------------------\nModified Functions\n-------------------");
            for f in module.functions() {
                println!("{}", f);
            }

            if let Err(e) = WasmModule::to_file("a.wasm", module) {
                panic!(e);
            }
        }
        Err(e) => {
            panic!(e);
        }
    }
}
