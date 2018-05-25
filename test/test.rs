static mut BUFFER: [i32; 10] = [-1; 10];

#[no_mangle]
pub extern "C" fn call_multiple(x: i32) -> i32 {
    let result = double(x) + negate(5) + 1;
    unsafe {
        BUFFER[x as usize % 10] = result;
    }
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
pub fn id(x: i32) -> i32 {
    return double(x) + negate(x);
}

#[no_mangle]
pub unsafe fn expose() -> [i32; 10] {
    return BUFFER.clone();
}
