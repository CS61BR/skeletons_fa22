use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

use crate::Map61B;

/*
Implement the Map61B trait for the standard library types
HashMap and BTreeMap. Very straightforward since all the methods
in Map61B are named the same way they are in the standard library.
*/

impl<K: Eq + Hash, V> Map61B for HashMap<K, V> {
    type Key = K;
    type Value = V;

    fn new() -> Self {
        Self::new()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn clear(&mut self) {
        self.clear()
    }

    fn contains_key(&self, key: &Self::Key) -> bool {
        self.contains_key(key)
    }

    fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value> {
        self.insert(key, value)
    }

    fn get(&self, key: &Self::Key) -> Option<&Self::Value> {
        self.get(key)
    }

    fn get_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Value> {
        self.get_mut(key)
    }

    fn remove(&mut self, key: &Self::Key) -> Option<Self::Value> {
        self.remove(key)
    }
}

impl<K: Ord, V> Map61B for BTreeMap<K, V> {
    type Key = K;
    type Value = V;

    fn new() -> Self {
        Self::new()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn clear(&mut self) {
        self.clear()
    }

    fn contains_key(&self, key: &Self::Key) -> bool {
        self.contains_key(key)
    }

    fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value> {
        self.insert(key, value)
    }

    fn get(&self, key: &Self::Key) -> Option<&Self::Value> {
        self.get(key)
    }

    fn get_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Value> {
        self.get_mut(key)
    }

    fn remove(&mut self, key: &Self::Key) -> Option<Self::Value> {
        self.remove(key)
    }
}
