#![allow(dead_code)] // allow things in this module to go unused
pub struct WeightedQuickUnion {
    parent: Vec<usize>, // parent of index (itself if root)
    size: Vec<usize>,   // number of elements in subtree rooted at index
    count: usize,       // number of components
}

impl WeightedQuickUnion {
    pub fn new(n: usize) -> Self {
        let parent = (0..n).collect();
        let size = vec![1; n];
        Self {
            parent,
            size,
            count: n,
        }
    }

    /// Returns the number of sets
    pub fn count(&self) -> usize {
        self.count
    }

    /// Returns the canonical element of the set containing element `p`
    pub fn find(&self, mut p: usize) -> Result<usize, &'static str> {
        if p >= self.parent.len() {
            return Err("index out of bounds");
        }
        while p != self.parent[p] {
            p = self.parent[p];
        }
        Ok(p)
    }

    /// Returns whether two elements are in the same set
    pub fn connected(&self, p: usize, q: usize) -> Result<bool, &'static str> {
        Ok(self.find(p)? == self.find(q)?)
    }

    /// Merges the set containing element `p` with the set containing element `q`
    pub fn union(&mut self, p: usize, q: usize) -> Result<(), &'static str> {
        let mut root_p = self.find(p)?;
        let mut root_q = self.find(q)?;
        if root_p == root_q {
            return Ok(());
        }
        if self.size[root_p] > self.size[root_q] {
            (root_p, root_q) = (root_q, root_p);
        }
        self.parent[root_p] = root_q;
        self.size[root_q] += self.size[root_p];
        self.count -= 1;
        Ok(())
    }
}
