mod tests;

fn is_leap_year(year: i32) -> bool {
    unimplemented!("is_leap_year") // TODO: YOUR CODE HERE
}

fn check_leap_year(year: i32) {
    if is_leap_year(year) {
        println!("{year} is a leap year.");
    } else {
        println!("{year} is not a leap year.");
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        println!("Please enter command line arguments.");
        println!("e.g. year 2000");
    } else {
        for arg in args {
            match arg.parse() {
                Ok(n) => check_leap_year(n),
                Err(_) => println!("{arg} is not a valid number."),
            }
        }
    }
}
