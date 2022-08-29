# Lab 3

## Background

Resources:
 - Our AList is a simple Vec implementation; for a more advanced Vec implementation, check out [the Rustonomicon](https://doc.rust-lang.org/nomicon/vec/vec.html)

In this lab, we will be doing some microbenchmarking. In compiled languages like Rust, C, C++, and Go, microbenchmarks should be viewed with suspicion, because the compiler can sometimes optimize our calls out completely. Furthermore, inlining behavior can have a huge impact on performance, and every program will have different inlining behavior.


 ## Writing Code

For the microbenchmarking part of the lab, you will be writing `benchmark` in `benchmarks.rs`. You will then fix the resize strategy in `alist.rs`.

For the testing part of the lab, you will be writing tests in `randomized_test/mod.rs`.

## Running Code

To run the benchmarks, run `cargo run --release Vec AList SLList`. The `--release` is very important; this compiles the code in "release" mode, which is much faster and more consistent.

## Testing Code

To run the tests, run `cargo test`.

## Cleanup and Submission

make sure no errors from `cargo clippy`, and run `cargo fmt`

no submission this is just for fun
