mod alist;
mod benchmarkable;
mod benchmarks;
#[cfg(test)]
mod randomized_test;
mod sllist;

use clap::Parser;
use std::str::FromStr;

use crate::benchmarks::{benchmark_alist, benchmark_sllist, benchmark_vec};

#[derive(Debug)]
enum Algorithm {
    Vec,
    AList,
    SLList,
}

impl FromStr for Algorithm {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Vec" => Ok(Algorithm::Vec),
            "AList" => Ok(Algorithm::AList),
            "SLList" => Ok(Algorithm::SLList),
            _ => Err("Not a valid algorithm"),
        }
    }
}

#[derive(Parser, Debug)]
/// Benchmark various list algorithms
struct Args {
    /// which algorithm to use. Can be Vec, AList, or SLList.
    algorithm: Vec<Algorithm>,
}

fn main() {
    let args = Args::parse();
    for alg in args.algorithm {
        match alg {
            Algorithm::Vec => benchmark_vec(),
            Algorithm::AList => benchmark_alist(),
            Algorithm::SLList => benchmark_sllist(),
        }
    }
}
