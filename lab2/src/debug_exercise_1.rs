// Exercise for learning how the debug, breakpoint, and step-into features work.

pub fn main() {
    let top = 10;
    let bot = 2;
    let result = rounded_division(top, bot);
    println!("round({top} / {bot}) = {result}");

    let top = 9;
    let bot = 4;
    let result = rounded_division(top, bot);
    println!("round({top} / {bot}) = {result}");

    let top = 3;
    let bot = 4;
    let result = rounded_division(top, bot);
    println!("round({top} / {bot}) = {result}");
}

// This method is bugged! Step through it in rust-gdb to figure out why.
// Optionally, you can fix the bug
fn rounded_division(top: i32, bot: i32) -> i32 {
    let quotient = top / bot;
    let result = (f64::from(quotient)).round();
    result as i32
}
