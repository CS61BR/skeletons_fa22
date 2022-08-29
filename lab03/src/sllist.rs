use std::mem;

// a list node
struct SLNode<T> {
    item: T,
    next: Option<Box<SLNode<T>>>,
}

/* A singly linked list. This singly-linked list is great for its constant-time
add_first and get_first operations.

Note that Option<Box<_>> was chosen instead of Box<Option<_>>, because the former
can be compiled into a nullable pointer.

Several of the implementations are ugly; linked lists are just hard to make
work with ownership rules.
*/
pub struct SLList<T> {
    head: Option<Box<SLNode<T>>>,
    len: usize,
}

#[allow(dead_code)] // so clippy doesn't complain about unused functions
impl<T> SLList<T> {
    pub fn new() -> SLList<T> {
        SLList { head: None, len: 0 }
    }

    // adds an element to the beginning of the list
    pub fn add_first(&mut self, t: T) {
        let first = Some(Box::from(SLNode {
            item: t,
            next: None,
        }));
        // this mem::replace is necessary because we want to own old, but in order for
        // the SLList to remain safe, old's location needs get something.
        let old = mem::replace(&mut self.head, first);
        // unwrap is ok here since we just placed a Some()
        self.head.as_mut().unwrap().as_mut().next = old;
        self.len += 1;
    }

    // gets a reference to the first item, returning None if the list is empty
    pub fn get_first(&self) -> Option<&T> {
        match &self.head {
            Some(n) => Some(&n.item),
            None => None,
        }
    }

    // gets a mutable reference to the first item, returning None if the list is empty
    pub fn get_first_mut(&mut self) -> Option<&mut T> {
        match &mut self.head {
            Some(n) => Some(&mut n.item),
            None => None,
        }
    }

    // adds an element to the end of the list
    pub fn add_last(&mut self, t: T) {
        let mut end = &mut self.head;
        while let Some(n) = end {
            end = &mut n.as_mut().next
        }
        *end = Some(Box::from(SLNode {
            item: t,
            next: None,
        }));
        self.len += 1;
    }

    // gets a reference to the last item, returning None if the list is empty
    pub fn get_last(&self) -> Option<&T> {
        if let Some(node) = &self.head {
            let mut last = node;
            while let Some(next) = &last.next {
                last = next;
            }
            Some(&last.item)
        } else {
            None
        }
    }

    // gets a mutable reference to the last item, returning None if the list is empty
    pub fn get_last_mut(&mut self) -> Option<&mut T> {
        if let Some(node) = &mut self.head {
            let mut last = node;
            while let Some(next) = &mut last.next {
                last = next;
            }
            Some(&mut last.item)
        } else {
            None
        }
    }

    // returns the length of the list
    pub fn len(&self) -> usize {
        self.len
    }
}
