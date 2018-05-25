/// An example of writing to a buffer and accessing it from JavaScript.

const BUFFER_SIZE: usize = 10;
static mut BUFFER: [i32; BUFFER_SIZE] = [-1; BUFFER_SIZE];

#[no_mangle]
pub extern "C" fn write(i: usize, x: i32) {
    unsafe {
        for j in 0..10 {
            if j % 2 == 0 {
                BUFFER[j] = -1 * (j as i32 + 1);
            } else {
                BUFFER[j] = j as i32 + 1;
            }
        }

        // Write `x` at the ith index.
        BUFFER[i] = x;
    }
}

#[no_mangle]
pub unsafe fn expose() -> *const i32 {
    return &BUFFER as *const i32;
}

#[no_mangle]
pub fn main(i: usize, x: i32) -> *const i32 {
    write(i, x);
    unsafe { expose() }
}
