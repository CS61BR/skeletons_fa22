use std::{collections::HashMap, io::BufRead};

use crate::timeseries::TimeSeries;

pub struct NGramMap {
    // add any struct fields you need
}

impl NGramMap {
    pub fn new(words_file: &str, counts_file: &str) -> Result<Self, std::io::Error> {
        todo!()
    }

    /// Returns total the number of times word appeared in the range [start_year, end_year]
    /// If word does not appear in the data, returns 0
    pub fn total_count(&self, word: &str, start_year: usize, end_year: usize) -> f64 {
        todo!()
    }

    /// Returns the history of the word, divided by the total counts for each year
    /// If word does not appear in the data, returns an empty TimeSeries
    pub fn weight_history(&self, word: &str, start_year: usize, end_year: usize) -> TimeSeries {
        todo!()
    }

    /// Returns the sums of the weighted histories of the given words
    pub fn summed_weight_history(
        &self,
        words: &Vec<&str>,
        start_year: usize,
        end_year: usize,
    ) -> TimeSeries {
        todo!()
    }
}
