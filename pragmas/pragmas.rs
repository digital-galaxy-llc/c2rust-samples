static mut I: i32 = 0;

#[no_mangle]
pub extern "C" fn main() -> i32 {
    let mut a = 1;
    
    // Pragma: inmain
    
    unsafe {
        // For loop with local i (shadowing the global I)
        let mut i = 0;
        while i < 3 {
            // Pragma: infor
            a += i;
            i += 1;
        }
    }
    
    return a;
}