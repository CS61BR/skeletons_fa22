fn is_leap_year(year: i32) -> bool {
    unimplemented!("is_leap_year") // Optional TODO: Fill in this method
}

fn check_leap_year(year: i32) {
    if is_leap_year(year) {
        println!("{year} is a leap year.");
    } else {
        println!("{year} is not a leap year.");
    }
}

fn main() {
    println!("Checking the year 2000, which should be a leap year:");
    check_leap_year(2000);
    println!("Checking the year 1700, which should be not a leap year:");
    check_leap_year(1700);
}
