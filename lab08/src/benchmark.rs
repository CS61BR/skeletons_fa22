use std::{
    collections::{BTreeMap, HashMap},
    io::{self, Write},
    time::Instant,
};

use lab08::{chashmap::CHashMap, myhashmap::MyHashMap, ohashmap::OHashMap, ullmap::ULLMap, Map61B};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    println!("Welcome to the insertion benchmark!");
    println!("The algorithm choices are as follows:");
    println!("  u: ULLMap");
    println!("  m: MyHashMap");
    println!("  c: CHashMap");
    println!("  o: OHashMap");
    println!("  t: std::collections::BTreeMap");
    println!("  h: std::collections::HashMap");
    println!("Enter benchmarks in the format \"ordered/random choices num\". For example, to run the random insertion benchmark on all six algorithms with 100 items, enter");
    println!("random umcoth 100");
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
        let choices = tokens.next();
        let num = tokens.next().and_then(|s| s.parse().ok());
        match (bench, choices, num) {
            (Some("ordered"), Some(choices), Some(n)) => bench_all(choices, n, true),
            (Some("random"), Some(choices), Some(n)) => bench_all(choices, n, false),
            _ => println!("Sorry, I didn't understand that."),
        }
    }
}

fn bench_all(choices: &str, n: usize, ordered: bool) {
    for c in choices.chars() {
        match c {
            'u' => bench_single::<ULLMap<usize, usize>>("ULLMap   ", n, ordered),
            'm' => bench_single::<MyHashMap<usize, usize>>("MyHashMap", n, ordered),
            'c' => bench_single::<CHashMap<usize, usize>>("CHashMap ", n, ordered),
            'o' => bench_single::<OHashMap<usize, usize>>("OHashMap ", n, ordered),
            't' => bench_single::<BTreeMap<usize, usize>>("BTreeMap ", n, ordered),
            'h' => bench_single::<HashMap<usize, usize>>("HashMap  ", n, ordered),
            _ => {}
        }
    }
}

fn bench_single<T: Map61B<Key = usize, Value = usize>>(name: &str, n: usize, ordered: bool) {
    let mut map = T::new();
    let values: Vec<usize> = if ordered {
        (0..n).collect()
    } else {
        (0..n).map(|_| rand::random()).collect()
    };
    let start = Instant::now();
    for v in values {
        map.insert(v, v);
    }
    println!(
        "{} time taken: {:>12.6} seconds",
        name,
        start.elapsed().as_secs_f64()
    );
}
