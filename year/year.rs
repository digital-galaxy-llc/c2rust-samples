use std::io::{self, Write};

static NUM: [&str; 10] = ["", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine"];

static FOR_TEN: [&str; 10] = ["", "", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety"];

static AF_TEN: [&str; 10] = ["Ten", "Eleven", "Twelve", "Thirteen", "Fourteen", "Fifteen", "Sixteen", "Seventeen", "Eighteen", "Nineteen"];

#[no_mangle]
pub extern "C" fn convert(thousands: i32, hundreds: i32, tens: i32, ones: i32) {
    print!("\nThe year in words is:\n");
    
    print!("{} thousand", NUM[thousands as usize]);

    if hundreds != 0 {
        print!(" {} hundred", NUM[hundreds as usize]);
    }

    if tens != 1 {
        print!(" {} {}", FOR_TEN[tens as usize], NUM[ones as usize]);
    } else {
        print!(" {}", AF_TEN[ones as usize]); 
    }
}

#[no_mangle]
pub extern "C" fn main() -> i32 {
    let mut year = 0;
    let mut n1000;
    let mut n100;
    let mut n10; 
    let mut n1;

    print!("\nEnter the year (4 digits): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    year = input.trim().parse().unwrap();

    if year > 9999 || year < 1000 {
        println!("\nError !! The year must contain 4 digits.");
        std::process::exit(1);
    }

    n1000 = year / 1000;
    n100 = (year % 1000) / 100;
    n10 = (year % 100) / 10;
    n1 = (year % 10) % 10;

    convert(n1000, n100, n10, n1);

    0
}