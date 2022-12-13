const W: u32 = 32;
const N: usize = 624;
const M: usize = 397;
const R: u32 = 31;

// =-=-= Twist =-=-=
//
const A: u32 = 0x9908_b0df;

// =-=-= Extract =-=-=
//
const U: u32 = 11;
const D: u32 = 0xffff_ffff;
//
const S: u32 = 7;
const B: u32 = 0x9d2c_5680;
//
const T: u32 = 15;
const C: u32 = 0xefc6_0000;
//
const L: u32 = 18;
//
const F: u32 = 1812433253;

const MASK: u32 = 0xffff_ffff;
const LOWER_MASK: u32 = (1 << R) - 1;
const UPPER_MASK: u32 = (!LOWER_MASK) & MASK;

pub struct MT19937 {
    pub state: [u32; N],
    pub idx: usize,
}

impl MT19937 {
    pub fn new() -> Self {
        let mut state:[u32; N] = [0; N];

        for i in 0..N {
            state[i] = 0;
        }

        let idx: usize = 0;

        MT19937 {
            state,
            idx
        }
    }

    pub fn init(&mut self, seed: u32) {
        self.state[0] = seed;

        self.idx = N;

        for i in 1..N {
            let q: u64 = (self.state[i - 1] ^ (self.state[i - 1] >> (W - 2))) as u64;

            let q: u64 = F as u64 * q;

            let q: u64 = q + i as u64;

            let q: u32 = (q as u32) & MASK;

            self.state[i] = q;
        }
    }

    fn twist(&mut self) {
        for i in 0..N {
            let x = (self.state[i] & UPPER_MASK) 
                        | (self.state[(i + 1) % N] & LOWER_MASK);

            let mut xA = x >> 1;

            if x % 2 == 1 {
                xA = xA ^ A;
            }
            self.state[i] = self.state[(i + M) % N] ^ xA;
        }
        self.idx = 0;
    }

    pub fn extract(&mut self) -> u32 {
        if self.idx >= N {
            if self.idx > N {
                // println!("ERROR");
                self.init(5489);
            }
            self.twist();
        }

        let mut y = self.state[self.idx];

        y = y ^ ((y >> U) & D);
        y = y ^ ((y << S) & B);
        y = y ^ ((y << T) & C);
        y = y ^ (y >> L);

        self.idx = self.idx + 1;

        return y & MASK;
    }
}
