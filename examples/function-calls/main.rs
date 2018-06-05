#[macro_use]
extern crate wasm_trace;

use wasm_trace::tracer::Tracer;

tracer_dependencies!();
tracer_bootstrap!();

#[no_mangle]
pub extern "C" fn do_stuff(x: i32) -> i32 {
    println!("{}", double(x) + double(x));
    println!("{}", factorial(x as u32));
    let result = double(x) + negate(5) + 1;
    void();
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

#[no_mangle]
pub fn void() {
    println!("No return value here!");
}

#[no_mangle]
pub fn factorial(n: u32) -> u32 {
    if n == 1 || n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

pub fn main() {
    println!("{}", do_stuff(10));
}
