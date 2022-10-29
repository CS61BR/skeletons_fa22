#![allow(dead_code)] // allow things in this module to go unused
use std::num::Wrapping;

/// small random implementation,
/// source: https://stackoverflow.com/questions/521295/seeding-the-random-number-generator-in-javascript
pub struct Random {
    a: Wrapping<u32>,
    b: Wrapping<u32>,
    c: Wrapping<u32>,
    d: Wrapping<u32>,
}

impl Random {
    /// Constructs a new Random using the cyrb123 hash function
    pub fn new(seed: &str) -> Random {
        let mut h1 = Wrapping(1779033703);
        let mut h2 = Wrapping(3144134277);
        let mut h3 = Wrapping(1013904242);
        let mut h4 = Wrapping(2773480762);
        for b in seed.as_bytes() {
            let k = Wrapping(u32::from(*b));
            h1 = h2 ^ ((h1 ^ k) * Wrapping(597399067));
            h2 = h3 ^ ((h2 ^ k) * Wrapping(2869860233));
            h3 = h4 ^ ((h3 ^ k) * Wrapping(951274213));
            h4 = h1 ^ ((h4 ^ k) * Wrapping(2716044179));
        }
        h1 = h3 ^ ((h1 >> 18) * Wrapping(597399067));
        h2 = h4 ^ ((h2 >> 22) * Wrapping(2869860233));
        h3 = h1 ^ ((h3 >> 17) * Wrapping(951274213));
        h4 = h2 ^ ((h4 >> 19) * Wrapping(2716044179));
        Random {
            a: h1 ^ h2 ^ h3 ^ h4,
            b: h1 ^ h2,
            c: h1 ^ h3,
            d: h1 ^ h4,
        }
    }

    /// returns the next u32 according to the sfc32 algorithm
    pub fn next(&mut self) -> u32 {
        let mut t = self.a + self.b;
        self.a = self.b ^ self.b >> 9;
        self.b = self.c + (self.c << 3);
        self.c = self.c << 21 | self.c >> 11;
        self.d += 1;
        t += self.d;
        self.c += t;
        t.0
    }

    /// returns a usize in the range [0, n)
    pub fn next_below(&mut self, n: usize) -> usize {
        self.next() as usize % n
    }

    /// returns an f64 in the range [0, 1)
    pub fn next_f64(&mut self) -> f64 {
        let n: f64 = self.next().into();
        n / 4294967296.0 // 2**32
    }
}
