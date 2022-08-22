// small random implementations
// source: https://stackoverflow.com/questions/521295/seeding-the-random-number-generator-in-javascript

pub struct Random {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
}

impl Random {
    // cyrb123 hash function
    pub fn new(seed: &str) -> Random {
        let mut h1 = 1779033703;
        let mut h2 = 3144134277;
        let mut h3 = 1013904242;
        let mut h4 = 2773480762;
        for b in seed.as_bytes() {
            let k: u32 = (*b).into();
            h1 = h2 ^ ((h1 ^ k) * 597399067);
            h2 = h3 ^ ((h2 ^ k) * 2869860233);
            h3 = h4 ^ ((h3 ^ k) * 951274213);
            h4 = h1 ^ ((h4 ^ k) * 2716044179);
        }
        h1 = h3 ^ ((h1 >> 18) * 597399067);
        h2 = h4 ^ ((h2 >> 22) * 2869860233);
        h3 = h1 ^ ((h3 >> 17) * 951274213);
        h4 = h2 ^ ((h4 >> 19) * 2716044179);
        Random {
            a: h1 ^ h2 ^ h3 ^ h4,
            b: h1 ^ h2,
            c: h1 ^ h3,
            d: h1 ^ h4,
        }
    }

    // sfc32 algorithm
    pub fn next(&mut self) -> u32 {
        let mut t = self.a + self.b;
        self.a = self.b ^ self.b >> 9;
        self.b = self.c + (self.c << 3);
        self.c = self.c << 21 | self.c >> 11;
        self.d += 1;
        t += self.d;
        self.c += t;
        t
    }

    pub fn next_f64(&mut self) -> f64 {
        let n: f64 = self.next().into();
        n / 4294967296.0 // 2**32
    }
}
