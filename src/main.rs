#![feature(extern_prelude)]

#[macro_use]
mod macros;
mod util;
mod pass;

use std::env;
use pass::WasmModule;

extern crate parity_wasm;

fn main() {
    let path = env::args().nth(1).expect("USAGE: cargo run module.wasm");
    if let Ok(module) = WasmModule::from_file(path) {
        module.print_functions();
    }
}
