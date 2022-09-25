use crate::wqu::WeightedQuickUnion;

pub struct Percolation {
    width: usize,
    height: usize,
    full: WeightedQuickUnion,
    percolates: WeightedQuickUnion,
    open: Vec<bool>,
    open_count: usize,
    can_percolate: bool,
}

pub trait Percolatable {
    fn new(width: usize, height: usize) -> Self;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn open(&mut self, row: usize, col: usize) -> Result<(), &'static str>;
    fn is_open(&self, row: usize, col: usize) -> Result<bool, &'static str>;
    fn is_full(&self, row: usize, col: usize) -> Result<bool, &'static str>;
    fn number_of_open_sites(&self) -> usize;
    fn percolates(&self) -> bool;
}

impl Percolatable for Percolation {
    fn new(width: usize, height: usize) -> Self {
        unimplemented!() // TODO: fill in this method
    }

    fn width(&self) -> usize {
        unimplemented!() // TODO: fill in this method
    }

    fn height(&self) -> usize {
        unimplemented!() // TODO: fill in this method
    }

    fn open(&mut self, row: usize, col: usize) -> Result<(), &'static str> {
        unimplemented!() // TODO: fill in this method
    }

    fn is_open(&self, row: usize, col: usize) -> Result<bool, &'static str> {
        unimplemented!() // TODO: fill in this method
    }

    fn is_full(&self, row: usize, col: usize) -> Result<bool, &'static str> {
        unimplemented!() // TODO: fill in this method
    }

    fn number_of_open_sites(&self) -> usize {
        unimplemented!() // TODO: fill in this method
    }

    fn percolates(&self) -> bool {
        unimplemented!() // TODO: fill in this method
    }
}
