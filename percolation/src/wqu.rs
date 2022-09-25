pub struct WeightedQuickUnion {
    parent: Vec<usize>, // parent of index (itself if root)
    size: Vec<usize>,   // number of elelements in subtree rooted at index
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

    pub fn count(&self) -> usize {
        self.count
    }

    fn root(&self, mut p: usize) -> usize {
        while p != self.parent[p] {
            p = self.parent[p];
        }
        p
    }

    pub fn connected(&self, p: usize, q: usize) -> Result<bool, &'static str> {
        if p >= self.parent.len() || q >= self.parent.len() {
            return Err("index out of bounds");
        }
        Ok(self.root(p) == self.root(q))
    }

    pub fn union(&mut self, p: usize, q: usize) -> Result<(), &'static str> {
        if p >= self.parent.len() || q >= self.parent.len() {
            return Err("index out of bounds");
        }

        let mut root_p = self.root(p);
        let mut root_q = self.root(q);
        if root_p == root_q {
            return Ok(());
        }
        if self.size[root_p] > self.size[root_p] {
            (root_p, root_q) = (root_q, root_p);
        }
        self.parent[root_p] = root_q;
        self.size[root_q] += self.size[root_p];
        self.count -= 1;
        Ok(())
    }
}
