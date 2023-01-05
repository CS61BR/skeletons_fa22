use crate::wqu::WeightedQuickUnion;

pub struct Percolation {
    // add any struct fields you need
}

pub trait Percolatable {
    fn new(width: usize, height: usize) -> Self;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn open(&mut self, row: usize, col: usize);
    fn is_open(&self, row: usize, col: usize) -> bool;
    fn is_full(&self, row: usize, col: usize) -> bool;
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

    fn open(&mut self, row: usize, col: usize) {
        unimplemented!() // TODO: fill in this method
    }

    fn is_open(&self, row: usize, col: usize) -> bool {
        unimplemented!() // TODO: fill in this method
    }

    fn is_full(&self, row: usize, col: usize) -> bool {
        unimplemented!() // TODO: fill in this method
    }

    fn number_of_open_sites(&self) -> usize {
        unimplemented!() // TODO: fill in this method
    }

    fn percolates(&self) -> bool {
        unimplemented!() // TODO: fill in this method
    }
}
