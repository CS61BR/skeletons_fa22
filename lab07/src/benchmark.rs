use std::{
    collections::{BTreeMap, HashMap},
    io::{self, Write},
    time::Instant,
};

use lab07::{bstmap::BSTMap, ullmap::ULLMap, Map61B};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    println!("Welcome to the insertion benchmark!");
    println!("The algorithm choices are as follows:");
    println!("  u: ULLMap");
    println!("  b: BSTMap");
    println!("  t: std::collections::BTreeMap");
    println!("  h: std::collections::HashMap");
    println!("Enter benchmarks in the format \"ordered/random choices num\". For example, to run the random insertion benchmark on all four algorithms with 100 items, enter");
    println!("random ubth 100");
    loop {
        println!();
        print!("Enter benchmark, or q to quit: ");
        stdout.flush()?;
        let mut buffer = String::new();
        stdin.read_line(&mut buffer)?;
        if buffer.contains('q') {
            return Ok(());
        }
        let mut tokens = buffer.split_whitespace();
        let bench = tokens.next();
        let alg = tokens.next();
        let num = tokens.next();
        let num = if let Some(num) = num {
            num.parse::<usize>().ok()
        } else {
            None
        };
        match (bench, alg, num) {
            (Some("ordered"), Some(s), Some(n)) => all_ordered(s, n),
            (Some("random"), Some(s), Some(n)) => all_random(s, n),
            _ => println!("Sorry, I didn't understand that."),
        }
    }
}

fn all_ordered(choices: &str, n: usize) {
    for c in choices.chars() {
        match c {
            'u' => bench_ordered::<ULLMap<usize, usize>>("ULLMap  ", n),
            'b' => bench_ordered::<BSTMap<usize, usize>>("BSTMap  ", n),
            't' => bench_ordered::<BTreeMap<usize, usize>>("BTreeMap", n),
            'h' => bench_ordered::<HashMap<usize, usize>>("HashMap ", n),
            _ => {}
        }
    }
}

fn all_random(choices: &str, n: usize) {
    for c in choices.chars() {
        match c {
            'u' => bench_random::<ULLMap<usize, usize>>("ULLMap  ", n),
            'b' => bench_random::<BSTMap<usize, usize>>("BSTMap  ", n),
            't' => bench_random::<BTreeMap<usize, usize>>("BTreeMap", n),
            'h' => bench_random::<HashMap<usize, usize>>("HashMap ", n),
            _ => {}
        }
    }
}

fn bench_ordered<T: Map61B<Key = usize, Value = usize>>(name: &str, n: usize) {
    let mut map = T::new();
    let start = Instant::now();
    for i in 0..n {
        map.insert(i, i);
    }
    println!(
        "{} time taken: {:>12.6} seconds",
        name,
        start.elapsed().as_secs_f64()
    );
}

fn bench_random<T: Map61B<Key = usize, Value = usize>>(name: &str, n: usize) {
    let mut map = T::new();
    let rand_values = (0..n).map(|_| rand::random()).collect::<Vec<usize>>();
    let start = Instant::now();
    for v in rand_values {
        map.insert(v, v);
    }
    println!(
        "{} time taken: {:>12.6} seconds",
        name,
        start.elapsed().as_secs_f64()
    );
}
