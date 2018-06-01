/// An example that returns number of times `entered_func()` and 
/// `exited_func()` are invoked

static mut ENTERED_FUNC_COUNT: usize = 0;
static mut EXITED_FUNC_COUNT: usize = 0;

pub fn main() {
}

#[no_mangle]
pub fn do_work(x: i32) -> i32 {
    x - 1
}

#[no_mangle]
pub fn do_more_work(x: i32) -> i32 {
    x - 2
}


#[no_mangle]
pub fn interface_return_entered_func_count() -> usize {
    let res1 = do_work(10);
    let res2 = do_more_work(res1);
    // to make sure res2 is GENed & not marked as dead code
    println!("{}", res2);
    return_entered_func_count()
}

#[no_mangle]
pub fn interface_return_exited_func_count() -> usize {
    let res1 = do_work(10);
    let res2 = do_more_work(res1);
    // to make sure res2 is GENed & not marked as dead code
    // println!("{}", res2);        
    return_exited_func_count()
}

#[no_mangle]
pub fn return_entered_func_count() -> usize {
    unsafe {
        ENTERED_FUNC_COUNT
    }
}

#[no_mangle]
pub fn return_exited_func_count() -> usize {
    unsafe {
        EXITED_FUNC_COUNT
    }
}


#[no_mangle]
pub fn entered_func() {
  unsafe {
    ENTERED_FUNC_COUNT += 1;
  }
}

#[no_mangle]
pub fn exited_func() {
  unsafe {
    EXITED_FUNC_COUNT += 1;
  }
}

