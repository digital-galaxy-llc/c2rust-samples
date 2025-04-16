#[no_mangle] pub extern "C" fn foo() -> i32 {
    return 0;
}

#[no_mangle] pub extern "C" fn main() -> i32 {
    foo();
    return 0;
}