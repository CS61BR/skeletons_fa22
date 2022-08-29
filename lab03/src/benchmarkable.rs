use crate::{alist::AList, sllist::SLList};

pub trait Benchmarkable {
    const ALG_NAME: &'static str;
    const ADD_NAME: &'static str;
    const GET_NAME: &'static str;
    fn new() -> Self;
    fn add(&mut self, value: i32);
    fn get(&self) -> Option<&i32>;
}

impl Benchmarkable for Vec<i32> {
    const ALG_NAME: &'static str = "Vec";
    const ADD_NAME: &'static str = "push";
    const GET_NAME: &'static str = "last";
    fn new() -> Self {
        Self::new()
    }

    fn add(&mut self, value: i32) {
        self.push(value)
    }

    fn get(&self) -> Option<&i32> {
        self.last()
    }
}

impl Benchmarkable for AList<i32> {
    const ALG_NAME: &'static str = "AList";
    const ADD_NAME: &'static str = "add_last";
    const GET_NAME: &'static str = "get_last";
    fn new() -> Self {
        Self::new()
    }

    fn add(&mut self, value: i32) {
        self.add_last(value)
    }

    fn get(&self) -> Option<&i32> {
        self.get_last()
    }
}

impl Benchmarkable for SLList<i32> {
    const ALG_NAME: &'static str = "SLList";
    const ADD_NAME: &'static str = "add_first";
    const GET_NAME: &'static str = "get_last";
    fn new() -> Self {
        Self::new()
    }

    fn add(&mut self, value: i32) {
        self.add_first(value)
    }

    fn get(&self) -> Option<&i32> {
        self.get_last()
    }
}
