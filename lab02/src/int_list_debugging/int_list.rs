use std::fmt::Display;

// similar to "A Bad Stack" in "Learn Rust With Entirely Too Many Linked Lists"
pub enum IntList {
    Empty,
    More(i32, Box<IntList>),
}

impl IntList {
    /// creates an empty IntList
    pub fn empty() -> IntList {
        IntList::Empty
    }

    /// creates an IntList from a vector of i32s
    pub fn from(mut v: Vec<i32>) -> IntList {
        if v.is_empty() {
            IntList::Empty
        } else {
            let first_int = v.remove(0);
            IntList::More(first_int, Box::new(IntList::from(v)))
        }
    }

    /// computes the size of the list using recursion
    pub fn size(&self) -> usize {
        match self {
            IntList::Empty => 0,
            IntList::More(_, next) => 1 + next.size(),
        }
    }

    /// computes the size of the list without recursion
    pub fn size_iterative(mut s: &Self) -> usize {
        let mut total = 0;
        while let IntList::More(_, next) = s {
            total += 1;
            s = next;
        }
        total
    }

    /// returns the ith item of this IntList
    /// (this really should return an Option, but for simplicity we panic instead)
    pub fn get(&self, i: usize) -> i32 {
        match self {
            IntList::Empty => panic!("index out of bounds"),
            IntList::More(v, next) => {
                if i == 0 {
                    *v
                } else {
                    next.get(i - 1)
                }
            }
        }
    }
}

impl Display for IntList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut c = self;
        while let IntList::More(v, next) = c {
            write!(f, "{} -> ", v)?;
            c = next;
        }
        write!(f, "Empty]")
    }
}
