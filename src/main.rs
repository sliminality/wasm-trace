#![feature(extern_prelude)]

#[macro_use]
mod macros;
mod either;
mod module;

use std::env;
use module::WasmModule;

extern crate parity_wasm;

fn main() {
    let path = env::args().nth(1).expect("USAGE: cargo run module.wasm");
    match WasmModule::from_file(path) {
        Ok(module) => {
            module.print_functions();
        }
        Err(e) => {
            panic!(e);
        }
    }
}
