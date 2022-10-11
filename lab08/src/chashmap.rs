use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use crate::Map61B;

const STARTING_BUCKETS: usize = 10;
const RESIZE_FACTOR: usize = 4; // newlen = r * (# elements)
const MAX_LOAD_FACTOR: f64 = 0.75;
const MIN_LOAD_FACTOR: f64 = 0.05; // only applicable when buckets.len() > STARTING_BUCKETS

// A linked-list based hashmap
// Rather than using a Vec for each bucket, all buckets are
// backed my linked list nodes in the "chains" vec
// Each "index" stored in hashmap is actually an index+1
// 0 represents "no next element"
pub struct CHashMap<K, V> {
    heads: Vec<usize>,          // indices of chains
    chains: Vec<(K, V, usize)>, // key, value, next index of chains
}

fn hash_index<K: Hash>(key: &K, len: usize) -> usize {
    let mut h = DefaultHasher::new();
    key.hash(&mut h);
    (h.finish() % (len as u64)) as usize
}

impl<K: Hash + Eq, V> CHashMap<K, V> {
    fn find_index(&self, hash_index: usize, key: &K) -> Option<usize> {
        let mut i = self.heads[hash_index];
        while i != 0 {
            if self.chains[i - 1].0 == *key {
                // see comment above struct
                return Some(i - 1);
            }
            i = self.chains[i - 1].2;
        }
        None
    }

    // returns None if target_index is the head of a list
    fn find_preceding(&self, hash_index: usize, target_index: usize) -> Option<usize> {
        if self.heads[hash_index] == target_index + 1 {
            return None;
        }
        let mut r = self.heads[hash_index] - 1;
        while self.chains[r].2 != target_index + 1 {
            r = self.chains[r].2 - 1;
        }
        Some(r)
    }

    fn loading(&self) -> f64 {
        self.chains.len() as f64 / self.heads.len() as f64
    }

    fn resize(&mut self, new_len: usize) {
        self.heads.clear();
        self.heads.resize(new_len, 0);
        for i in 0..self.chains.len() {
            let h = hash_index(&self.chains[i].0, new_len);
            self.chains[i].2 = self.heads[h];
            self.heads[h] = i + 1; // see comment above struct
        }
    }
}

impl<K, V> IntoIterator for CHashMap<K, V> {
    type Item = (K, V);
    type IntoIter = std::vec::IntoIter<(K, V)>;

    fn into_iter(self) -> Self::IntoIter {
        self.chains
            .into_iter()
            .map(|(k, v, _)| (k, v))
            .collect::<Vec<(K, V)>>()
            .into_iter()
    }
}

impl<K: Hash + Eq, V> Map61B for CHashMap<K, V> {
    type Key = K;
    type Value = V;

    fn new() -> Self {
        Self {
            heads: vec![0; STARTING_BUCKETS],
            chains: Vec::new(),
        }
    }

    fn len(&self) -> usize {
        self.chains.len()
    }

    fn clear(&mut self) {
        self.heads.clear();
        self.heads.resize(STARTING_BUCKETS, 0);
        self.chains.clear();
    }

    fn contains_key(&self, key: &Self::Key) -> bool {
        self.get(key).is_some()
    }

    fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value> {
        let h = hash_index(&key, self.heads.len());
        if let Some(i) = self.find_index(h, &key) {
            return Some(std::mem::replace(&mut self.chains[i].1, value));
        }
        self.chains.push((key, value, self.heads[h]));
        self.heads[h] = self.chains.len(); // not - 1, see comment above struct
        if self.loading() > MAX_LOAD_FACTOR {
            self.resize(self.chains.len() * RESIZE_FACTOR);
        }
        None
    }

    fn get(&self, key: &Self::Key) -> Option<&Self::Value> {
        let h = hash_index(&key, self.heads.len());
        let i = self.find_index(h, key)?;
        Some(&self.chains[i].1)
    }

    fn get_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Value> {
        let h = hash_index(&key, self.heads.len());
        let i = self.find_index(h, key)?;
        Some(&mut self.chains[i].1)
    }

    fn remove(&mut self, key: &Self::Key) -> Option<Self::Value> {
        let hash_rm = hash_index(&key, self.heads.len());
        let rm_index = self.find_index(hash_rm, key)?;
        let hash_end = hash_index(&self.chains.last().unwrap().0, self.heads.len());

        let pre_rm = self.find_preceding(hash_rm, rm_index);
        let pre_end = self.find_preceding(hash_end, self.chains.len() - 1);
        // if pre-rm and pre-end are different, pre-rm will point to the link after rm, and pre-end will point to rm
        // if they are the same, point to the link after rm
        match pre_end {
            Some(i) => self.chains[i].2 = rm_index + 1,
            None => self.heads[hash_end] = rm_index + 1,
        }
        match pre_rm {
            Some(i) => self.chains[i].2 = self.chains[rm_index].2,
            None => self.heads[hash_rm] = self.chains[rm_index].2,
        }

        let res = self.chains.swap_remove(rm_index).1;
        if self.loading() < MIN_LOAD_FACTOR && self.heads.len() != STARTING_BUCKETS {
            self.resize(STARTING_BUCKETS.max(self.chains.len() * RESIZE_FACTOR));
        }
        Some(res)
    }
}
