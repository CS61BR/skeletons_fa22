use std::{
    collections::BTreeMap,
    ops::{AddAssign, DivAssign},
};

#[derive(Clone, Debug)]
pub struct TimeSeries {
    pub data: BTreeMap<usize, f64>,
}

impl TimeSeries {
    /// Creates a new, empty TimeSeries
    pub fn new() -> Self {
        todo!()
    }

    /// Removes all keys that are not in the range [start_year, end_year]
    pub fn restrict(&mut self, start_year: usize, end_year: usize) {
        todo!()
    }
}

impl AddAssign<&TimeSeries> for TimeSeries {
    /// Adds two TimeSeries together, keeping all keys that appear in either TimeSeries
    /// assumes a default value of 0 if one of the two TimeSeries does not contain a key
    fn add_assign(&mut self, rhs: &TimeSeries) {
        todo!()
    }
}

impl DivAssign<&TimeSeries> for TimeSeries {
    /// Divides self by rhs, only keeping keys that appear in both TimeSeries
    /// Assume rhs does not contain any zeroes
    fn div_assign(&mut self, rhs: &TimeSeries) {
        todo!()
    }
}
