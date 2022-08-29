use std::time::{Duration, Instant};

use crate::{alist::AList, benchmarkable::Benchmarkable, sllist::SLList};

pub fn benchmark_vec() {
    benchmark::<Vec<i32>>();
}

pub fn benchmark_alist() {
    benchmark::<AList<i32>>();
}

pub fn benchmark_sllist() {
    benchmark::<SLList<i32>>();
}

/* 
 Benchmarks the "add" and "get" methods of the Benchmarkable type, formatting the
 results in tables like this:

 Timing table for Vec::push
           N     time (s)        # ops  microsec/op
---------------------------------------------------
        1000     0.000003         1000     0.003356
        2000     0.000003         2000     0.001613
        4000     0.000006         4000     0.001470
        8000     0.000011         8000     0.001320
       16000     0.000021        16000     0.001316
       32000     0.000059        32000     0.001850
       64000     0.000104        64000     0.001619
      128000     0.000240       128000     0.001871

Timing table for Vec::last
           N     time (s)        # ops  microsec/op
---------------------------------------------------
        1000     0.000000        10000     0.000002
        2000     0.000000        10000     0.000002
        4000     0.000000        10000     0.000002
        8000     0.000000        10000     0.000002
       16000     0.000000        10000     0.000002
       32000     0.000000        10000     0.000003
       64000     0.000000        10000     0.000003
      128000     0.000000        10000     0.000002

 */
fn benchmark<T: Benchmarkable>() {
    // some example code
    // you should delete this code and write actual benchmarking code
    let mut list = T::new();
    let start = Instant::now();
    for _ in 0..10000 {
        list.add(42);
    }
    let add_elapsed = start.elapsed();
    println!("Took {:?} to run 10000 {}::{} calls", add_elapsed, T::ALG_NAME, T::ADD_NAME);

    unimplemented!(); // TODO: Fill in this function
}

struct Timing {
    n: u64,
    elapsed: Duration,
    op_count: u64,
}

impl Timing {
    fn new(n: u64, elapsed: Duration, op_count: u64) -> Self {
        Timing {
            n,
            elapsed,
            op_count,
        }
    }
}

fn print_timing_table(alg_name: &str, method_name: &str, timings: &Vec<Timing>) {
    println!("Timing table for {}::{}", alg_name, method_name);
    println!(
        "{:>12} {:>12} {:>12} {:>12}",
        "N", "time (s)", "# ops", "microsec/op"
    );
    println!("{}", "-".repeat(4 * 12 + 3));
    for timing in timings {
        let time = timing.elapsed.as_secs_f64();
        let time_per_op = time * 1e6 / timing.op_count as f64;
        println!(
            "{:>12} {:>12.6} {:>12} {:>12.6}",
            timing.n, time, timing.op_count, time_per_op
        );
    }
    println!();
}
