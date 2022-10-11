use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use crate::Map61B;

const STARTING_BUCKETS: usize = 10;
const RESIZE_FACTOR: usize = 4; // newlen = r * (# elements)
const MAX_LOAD_FACTOR: f64 = 0.75;
const MIN_LOAD_FACTOR: f64 = 0.05;

enum Bucket<K, V> {
    Full(K, V),
    Removed,
    Empty,
}

// open-addressed, linear-probing hashmap
pub struct OHashMap<K, V> {
    buckets: Vec<Bucket<K, V>>,
    num_elements: usize,
    num_removed: usize,
}

fn hash_index<K: Hash>(key: &K, len: usize) -> usize {
    let mut h = DefaultHasher::new();
    key.hash(&mut h);
    (h.finish() % (len as u64)) as usize
}

fn create_buckets<K, V>(num_buckets: usize) -> Vec<Bucket<K, V>> {
    (0..num_buckets).map(|_| Bucket::Empty).collect()
}

impl<K: Hash + Eq, V> OHashMap<K, V> {
    fn find_index(&self, key: &K) -> usize {
        let mut i = hash_index(key, self.buckets.len());
        loop {
            match &self.buckets[i] {
                Bucket::Full(k, _) => {
                    if k == key {
                        break i;
                    }
                }
                Bucket::Removed => {}
                Bucket::Empty => break i,
            }
            i = (i + 1) % self.buckets.len();
        }
    }

    fn resize(&mut self, new_len: usize) {
        let old_buckets = std::mem::replace(&mut self.buckets, create_buckets(new_len));
        for b in old_buckets {
            if let Bucket::Full(k, v) = b {
                let i = self.find_index(&k);
                self.buckets[i] = Bucket::Full(k, v);
            }
        }
        self.num_removed = 0;
    }
}

impl<K, V> IntoIterator for OHashMap<K, V> {
    type Item = (K, V);
    type IntoIter = std::vec::IntoIter<(K, V)>;

    fn into_iter(self) -> Self::IntoIter {
        let mut storage = Vec::with_capacity(self.num_elements);
        for b in self.buckets {
            if let Bucket::Full(k, v) = b {
                storage.push((k, v));
            }
        }
        storage.into_iter()
    }
}

impl<K: Hash + Eq, V> Map61B for OHashMap<K, V> {
    type Key = K;
    type Value = V;

    fn new() -> Self {
        Self {
            buckets: create_buckets(STARTING_BUCKETS),
            num_elements: 0,
            num_removed: 0,
        }
    }

    fn len(&self) -> usize {
        self.num_elements
    }

    fn clear(&mut self) {
        self.buckets = create_buckets(STARTING_BUCKETS);
        self.num_elements = 0;
        self.num_removed = 0;
    }

    fn contains_key(&self, key: &Self::Key) -> bool {
        self.get(key).is_some()
    }

    fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value> {
        let i = self.find_index(&key);
        if let Bucket::Full(_, ref mut v) = self.buckets[i] {
            return Some(std::mem::replace(v, value));
        }
        self.buckets[i] = Bucket::Full(key, value);
        self.num_elements += 1;
        let loading = (self.num_elements + self.num_removed) as f64 / self.buckets.len() as f64;
        if loading > MAX_LOAD_FACTOR {
            self.resize(self.num_elements * RESIZE_FACTOR);
        }
        None
    }

    fn get(&self, key: &Self::Key) -> Option<&Self::Value> {
        let i = self.find_index(key);
        match self.buckets[i] {
            Bucket::Full(_, ref v) => Some(v),
            _ => None,
        }
    }

    fn get_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Value> {
        let i = self.find_index(key);
        match self.buckets[i] {
            Bucket::Full(_, ref mut v) => Some(v),
            _ => None,
        }
    }

    fn remove(&mut self, key: &Self::Key) -> Option<Self::Value> {
        let i = self.find_index(key);
        if let Bucket::Full(_, _) = self.buckets[i] {
            let prev = std::mem::replace(&mut self.buckets[i], Bucket::Removed);
            self.num_elements -= 1;
            self.num_removed += 1;
            let loading = self.num_elements as f64 / self.buckets.len() as f64;
            if loading < MIN_LOAD_FACTOR {
                self.resize(STARTING_BUCKETS.max(self.num_elements * RESIZE_FACTOR));
            }
            if let Bucket::Full(_, v) = prev {
                return Some(v);
            }
        }
        None
    }
}
