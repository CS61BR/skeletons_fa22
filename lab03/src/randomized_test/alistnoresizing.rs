/*
This version does no re-allocation, making the implementation much simpler.

Note that we are restricting T to Default + Copy so we can initialize an array.
Popular types that are Default + Copy include i32, u32, usize, f64, etc
*/

use std::mem;

pub struct AListNR<T> {
    items: Box<[T]>,
    len: usize,
}

#[allow(dead_code)] // so clippy doesn't complain about unused functions
impl<T: Default + Copy> AListNR<T> {
    pub fn new() -> Self {
        Self {
            items: Box::from([T::default(); 1000]),
            len: 0,
        }
    }

    pub fn add_last(&mut self, t: T) {
        self.items[self.len] = t;
        self.len += 1;
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            Some(&self.items[index])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.len {
            Some(&mut self.items[index])
        } else {
            None
        }
    }

    pub fn get_last(&self) -> Option<&T> {
        if self.len > 0 {
            Some(&self.items[self.len - 1])
        } else {
            None
        }
    }

    pub fn get_last_mut(&mut self) -> Option<&mut T> {
        if self.len > 0 {
            Some(&mut self.items[self.len - 1])
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn remove_last(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        Some(mem::take(&mut self.items[self.len]))
    }
}
