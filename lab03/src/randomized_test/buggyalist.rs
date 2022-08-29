/* A slice-based list. This is meant to be a homegrown version of Vec,
but in practice is much worse than Vec.

Much like the real Vec, the main challenge is resizing when out of capacity.

Reasons why this is a terrible implementation:
 - the real Vec implementation uses Unique<T> and an Allocator, which gives
   much finer control over memory details. However, being Grug-brained
   developers, we don't want to deal with more unsafe than we have to,
   so will stick with boxed slices instead.
 - Because we're using Box<[T]>, we need to fill the unused spaces with
   something. To avoid unsound behavior, we need to restrict the usage to
   T: Default.
 - In order to allocate a Box<[T]>, we need to use a lot of unsafe functions.
   This has the potential to crash our programs, exspecially in the presence
   of ZSTs and other corner cases.
*/

use std::mem;

pub struct BAList<T> {
    items: Box<[T]>,
    len: usize,
}

#[allow(dead_code)] // so clippy doesn't complain about unused functions
impl<T: Default> BAList<T> {
    pub fn new() -> Self {
        Self {
            items: Box::from([]),
            len: 0,
        }
    }

    fn resize(&mut self, capacity: usize) {
        let mut new_items = allocate_slice(capacity);
        for i in 0..self.items.len() {
            mem::swap(&mut new_items[i], &mut self.items[i]);
        }
        self.items = new_items;
    }

    pub fn add_last(&mut self, t: T) {
        if self.len >= self.items.len() {
            self.resize(self.len * 2 + 1);
        }

        self.items[self.len] = t;
        self.len += 1;
    }

    pub fn get(&self, ind: usize) -> Option<&T> {
        self.items.get(ind)
    }

    pub fn get_mut(&mut self, ind: usize) -> Option<&mut T> {
        self.items.get_mut(ind)
    }

    pub fn get_last(&self) -> Option<&T> {
        match self.len {
            0 => None,
            _ => Some(&self.items[self.len - 1]),
        }
    }

    pub fn get_last_mut(&mut self) -> Option<&mut T> {
        match self.len {
            0 => None,
            _ => Some(&mut self.items[self.len - 1]),
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn remove_last(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        if self.len < self.items.len() / 4 && self.len > 4 {
            self.resize(self.len / 2);
        }
        self.len -= 1;
        Some(mem::take(&mut self.items[self.len]))
    }
}

/* Allocates a fixed-length Box<[T]> */
fn allocate_slice<T: Default>(n: usize) -> Box<[T]> {
    unsafe {
        let layout = std::alloc::Layout::array::<T>(n).unwrap();
        let ptr = std::alloc::alloc(layout) as *mut T;
        for i in 0..n {
            std::ptr::write(ptr.add(i), T::default());
        }
        let slice = core::slice::from_raw_parts_mut(ptr, n);
        Box::from_raw(slice)
    }
}
