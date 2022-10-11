# Lab 8

## Background

You might feel some déjà vu after doing lab 7; we're doing the same thing, but with hashmaps! The different hashmap implementations are explained on the [hashmap implementations page](./hashmap_impls.md).

## Differences from the Java version

The Java version has an emphasis on polymorphism, and testing different bucket types. However, the bucket choice ends up being inconsequential, because they're all being used in the same way; the exotic bucket types just end up being worse `ArrayList`s. 

In the Rust version, you'll be exploring three unique implementations of a hashmap (two external chaining and one open addressing). Each implementation has different performance characteristics, so the design choices actually matter.

Other minor differences

 - the methods to implement are a bit different. See the [Writing Code](#writing-code) section.
 - the "speed tests" have all been combined into `benchmark.rs`

## Writing Code

Read through the [hashmap implementations page](./hashmap_impls.md). You will be implementing the `MyHashMap<K, V>` type, which is in `myhashmap.rs`. The `MyHashMap` type should implement the `Map61B` trait as following:
```
impl<K: Hash + Eq, V> Map61B for MyHashMap<K, V> { ... }
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

Once you have a working implementation of `MyHashMap`, it's time to run some benchmarks!
```
cargo run --release
```
Experiment with different loading and factors: can you improve the performance of `MyHashMap`? What about the other hashmaps implementations?

## Cleanup and Submission

make sure no errors from `cargo clippy`, and run `cargo fmt`

no submission this is just for fun
