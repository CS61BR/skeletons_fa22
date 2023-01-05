use std::fmt::Display;

use crate::{percolation::Percolatable, random::Random};

pub struct PercolationStats {
    pub counts: Vec<usize>, // of length width * height
    // counts[i] represents how many perculated after exactly i tiles opened
    mean: f64,            // mean of (open sites)/(width * height) to percolate
    stddev: f64,          // stddev of ratio
    confidence_low: f64,  // 95% threshhold
    confidence_high: f64, // 95% threshhold
}

impl Display for PercolationStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "mean: {:.4}, stddev: {:.4}, 95% confidence interval: ({:.4}, {:.4})",
            self.mean, self.stddev, self.confidence_low, self.confidence_high
        )
    }
}

pub fn calculate_stats<P: Percolatable>(
    width: usize,
    height: usize,
    trials: usize,
    random: &mut Random,
) -> PercolationStats {
    unimplemented!() // TODO: fill in this method
}
