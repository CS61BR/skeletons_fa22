use crate::Map61B;

struct ULLNode<K, V> {
    key: K,
    value: V,
    next: Option<Box<ULLNode<K, V>>>,
}

pub struct ULLMap<K, V> {
    root: Option<Box<ULLNode<K, V>>>,
    size: usize,
}

impl<K, V> IntoIterator for ULLMap<K, V> {
    type Item = (K, V);
    type IntoIter = std::vec::IntoIter<(K, V)>;

    fn into_iter(self) -> Self::IntoIter {
        let mut node = self.root;
        let mut storage = Vec::with_capacity(self.size);
        while let Some(n) = node {
            let ULLNode { key, value, next } = *n;
            storage.push((key, value));
            node = next;
        }
        storage.into_iter()
    }
}

impl<K: Eq, V> Map61B for ULLMap<K, V> {
    type Key = K;
    type Value = V;

    fn new() -> Self {
        Self {
            root: None,
            size: 0,
        }
    }

    fn len(&self) -> usize {
        self.size
    }

    fn clear(&mut self) {
        self.size = 0;
        self.root = None;
    }

    fn contains_key(&self, key: &Self::Key) -> bool {
        self.get(key).is_some()
    }

    fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value> {
        let mut node = &mut self.root;
        while let Some(n) = node {
            if n.key == key {
                let prev = std::mem::replace(&mut n.value, value);
                return Some(prev);
            }
            node = &mut n.next;
        }
        *node = Some(Box::from(ULLNode {
            key,
            value,
            next: None,
        }));
        self.size += 1;
        None
    }

    fn get(&self, key: &Self::Key) -> Option<&Self::Value> {
        let mut node = &self.root;
        while let Some(n) = node {
            if n.key == *key {
                return Some(&n.value);
            }
            node = &n.next;
        }
        None
    }

    fn get_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Value> {
        let mut node = &mut self.root;
        while let Some(n) = node {
            if n.key == *key {
                return Some(&mut n.value);
            }
            node = &mut n.next;
        }
        None
    }

    fn remove(&mut self, key: &Self::Key) -> Option<Self::Value> {
        let mut node = &mut self.root;
        loop {
            if let Some(n) = node {
                if n.key == *key {
                    let removed = node.take();
                    let ULLNode {
                        key: _,
                        value,
                        next,
                    } = *removed.unwrap();
                    *node = next;
                    self.size -= 1;
                    return Some(value);
                }
            } else {
                return None;
            }
            if let Some(n) = node {
                node = &mut n.next;
            }
        }
    }
}
