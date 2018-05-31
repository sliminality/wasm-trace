extern crate wasm_trace;

use std::env;
use wasm_trace::module::WasmModule;

fn main() {
    let path = env::args().nth(1).expect("USAGE: cargo run module.wasm");
    match WasmModule::from_file(path) {
        Ok(module) => {
            println!("\n-------------------\nTypes\n-------------------");
            for (i, t) in module.types().iter().enumerate() {
                println!("{} {:?}", i, t);
            }

            println!("\n------------------\nFunctions\n-------------------");
            for f in module.functions() {
                println!("{}", f);
            }
        }
        Err(e) => {
            panic!(e);
        }
    }
}
