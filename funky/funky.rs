#[no_mangle]
pub extern "C" fn foo() -> i8 {
    return '1' as i8;
}

#[no_mangle]
pub extern "C" fn maxout_in(paste: i32, matrix: *mut *mut i8) -> i32 {
    unsafe {
        let o: i8 = foo();
        return ((*(*matrix.offset(1)).offset(2) as i32) * 5) - paste;
    }
}

#[no_mangle]
pub extern "C" fn main() -> i32 {
    let multi: *const i8 = "a multi\0".as_ptr() as *const i8;
    0
}