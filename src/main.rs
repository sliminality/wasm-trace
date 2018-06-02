extern crate wasm_trace;

use std::env;
use wasm_trace::module::WasmModule;

fn main() {
    let path = env::args().nth(1).expect("USAGE: cargo run module.wasm");
    match WasmModule::from_file(path) {
        Ok(mut module) => {
            module.instrument_module();
            if let Err(e) =  WasmModule::to_file("output.wasm", module) {
                panic!(e);
            } else {
                println!("Modified wasm module -> output.wasm");
            }           
        }
        Err(e) => {
            panic!(e);
        }
    }
}
