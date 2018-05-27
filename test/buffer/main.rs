/// An example of writing to a buffer and accessing it from JavaScript.

const RUST_ARRAY_SIZE: usize = 10;
static mut RUST_ARRAY: [i32; RUST_ARRAY_SIZE] = [-1; RUST_ARRAY_SIZE];

#[no_mangle]
pub extern "C" fn write(i: usize, x: i32) {
    unsafe {
        for j in 0..10 {
            RUST_ARRAY[j] = -1;
        }
        // Write `x` at the ith index.
        RUST_ARRAY[i] = x;
    }
}

#[no_mangle]
pub unsafe fn expose() -> *const i32 {
    return &RUST_ARRAY as *const i32;
}

#[no_mangle]
pub fn main(i: usize, x: i32) -> *const i32 {
    write(i, x);
    unsafe { expose() }
}
