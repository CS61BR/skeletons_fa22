# Lab 7

## Background

Take a look through `ullmap.rs`: this is an implementation of a unordered-linked-list map. The structure of `bstmap.rs` will be very similar to `ullmap.rs`.

## Differences from the Java version

 - the methods to implement are a bit different. See the [Writing Code](#writing-code) section.
 - the "speed tests" have all been combined into `benchmark.rs`
 - no additional asymptotics problems (just do the Java ones)

## Writing Code

You will be creating a `BSTMap<K, V>` type, which should go in a file named `bstmap.rs`. The `BSTMap` type should implement the `Map61B` trait as following:
```
impl<K: Ord, V> Map61B for BSTMap<K, V> { ... }
```
You are required to implement the following methods:
 - `new`
 - `len`
 - `clear`
 - `contains_key`
 - `insert`
 - `get`
 - `get_mut`

The following methods are optional. If you don't implement them, just replace their bodies with `unimplemented!()`:
 - `remove`
 - `into_iter` (from the `IntoIterator` trait). If you don't plan on implementing this, you can use `std::vec::IntoIter<(K, V)>` as a placeholder for the trait's `IntoIter` type.



## Testing Code

To test the required methods, run
```
cargo test --lib map
```
To test the optional methods as well, run
```
cargo test --lib
```

## Running Code

Once you have a working implementation of `BSTMap`, it's time to run some benchmarks!
```
cargo run --release
```

## Cleanup and Submission

make sure no errors from `cargo clippy`, and run `cargo fmt`

no submission this is just for fun
