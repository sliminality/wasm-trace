#[macro_use]
extern crate wasm_trace;

use wasm_trace::tracer::Tracer;

tracer_dependencies!();
tracer_bootstrap!();

#[no_mangle]
pub extern "C" fn double_subtract5_add1(x: i32) -> i32 {
    println!("{}", double(x) + double(x));
    let result = double(x) + negate(5) + 1;
    return result;
}

#[no_mangle]
pub fn double(x: i32) -> i32 {
    return x * 2;
}

#[no_mangle]
pub fn negate(x: i32) -> i32 {
    return -1 * x;
}

pub fn main() {
    println!("{}", double_subtract5_add1(10));
}
