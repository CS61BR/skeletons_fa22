fn next_number(n: u64) -> u64 {
    unimplemented!() // TODO: Fill in this method
}

fn main() {
    let mut n = 5;
    print!("{} ", n);

    // Some starter code to test
    while n != 1 {
        n = next_number(n);
        print!("{} ", n);
    }
    println!();
}
