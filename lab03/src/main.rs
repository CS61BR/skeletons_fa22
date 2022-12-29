mod alist;
mod benchmarkable;
mod benchmarks;
#[cfg(test)]
mod randomized_test;
mod sllist;

use alist::AList;
use benchmarks::benchmark;
use clap::Parser;
use sllist::SLList;
use std::str::FromStr;

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
            Algorithm::Vec => benchmark::<Vec<i32>>(),
            Algorithm::AList => benchmark::<AList<i32>>(),
            Algorithm::SLList => benchmark::<SLList<i32>>(),
        }
    }
}
