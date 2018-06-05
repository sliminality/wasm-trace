extern crate wasm_trace;

use std::env;
use wasm_trace::module::WasmModule;

fn main() {
    let path = env::args().nth(1).expect("USAGE: cargo run module.wasm");
    match WasmModule::from_file(path) {
        Ok(mut module) => {
            if let Err(e) = module.instrument_module() {
                panic!("Error instrumenting module: {}", e);
            }
            if let Err(e) = WasmModule::to_file("output.wasm", module) {
                panic!("Error writing instrumented module: {}", e);
            }
            println!("Modified wasm module -> output.wasm");
        }
        Err(e) => {
            panic!("Error initializing module: {}", e);
        }
    }
}
