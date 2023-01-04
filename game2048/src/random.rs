#![allow(dead_code)] // allow things in this module to go unused
use std::num::Wrapping;

/// small random implementation
/// sfc64 algorithm with 256-bit random state
pub struct Random {
    a: Wrapping<u64>,
    b: Wrapping<u64>,
    c: Wrapping<u64>,
    d: Wrapping<u64>,
}

impl Random {
    /// Constructs a new Random by hashing the string
    pub fn new(seed: &str) -> Random {
        let fib = Wrapping::<u64>(11400714819323198485); // 2^64 / phi
        let mut ar = [
            Wrapping(3141592653589793238), // random numbers to initialize
            Wrapping(4626433832795028841), // I chose digits of pi
            Wrapping(9716939937510582097),
            Wrapping(4944592307816406286),
        ];
        let mut c = 0;
        // not a good hash, but preserves most of the entropy
        for b in seed.as_bytes() {
            ar[c] = (ar[c] + Wrapping(u64::from(*b))) * fib;
            ar[c] = (ar[c] << 25) | (ar[c] >> 39);
            c = (c + 1) & 3;
        }
        Random {
            a: ar[0],
            b: ar[1],
            c: ar[2],
            d: ar[3],
        }
    }

    /// returns the next u64 according to the sfc64 algorithm
    pub fn next(&mut self) -> u64 {
        let t = self.a + self.b + self.d;
        self.a = self.b ^ (self.b >> 11);
        self.b = self.c + (self.c << 3);
        self.c = ((self.c << 24) | (self.c >> 40)) + t;
        self.d += 1;
        t.0
    }

    /// returns a usize uniformly in the range [0, n).
    pub fn next_below(&mut self, n: usize) -> usize {
        if n < 2 {
            return 0;
        }
        let mut mask: usize = !0;
        mask >>= (n - 1).leading_zeros();
        // sampling with rejection makes sure result is unbiased
        loop {
            let mut x = self.next() as usize;
            x &= mask;
            if x < n {
                break x;
            }
        }
    }

    /// returns an f64 uniformly in the range [0, 1)
    pub fn next_f64(&mut self) -> f64 {
        // f64 has 53 bits of precision, so shifting by 11 makes sure
        // all values in range have the same precision
        (self.next() >> 11) as f64 / ((u64::MAX >> 11) + 1) as f64
    }
}
