use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use crate::Map61B;

const STARTING_BUCKETS: usize = 10;
const RESIZE_FACTOR: usize = 4; // newlen = r * (# elements)
const MAX_LOAD_FACTOR: f64 = 0.75;
const MIN_LOAD_FACTOR: f64 = 0.05; // only applicable when buckets.len() > STARTING_BUCKETS

pub struct MyHashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    num_elements: usize,
}

/*
The following helper functions may be useful. Feel free to delete or modify them.
*/

fn hash_index<K: Hash>(key: &K, len: usize) -> usize {
    let mut h = DefaultHasher::new();
    key.hash(&mut h);
    (h.finish() % (len as u64)) as usize
}

fn create_buckets<T>(num_buckets: usize) -> Vec<Vec<T>> {
    (0..num_buckets).map(|_| Vec::new()).collect()
}

fn loading<K, V>(h: &MyHashMap<K, V>) -> f64 {
    h.num_elements as f64 / h.buckets.len() as f64
}

/*
TODO: implement Map61B for MyHashMap.
*/
