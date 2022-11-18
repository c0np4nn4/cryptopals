const W: u64 = 32;
const N: usize = 624;
//
const F: u64 = 1812433253;
//
const M: usize = 397;
const R: u64 = 31;
//
const A: u64 = 0x9908_b0df;
//
const D: u64 = 0xffff_ffff;
const B: u64 = 0x9d2c_5680;
const C: u64 = 0xefc6_0000;
//
const U: u64 = 11;
const S: u64 = 7;
const T: u64 = 15;
const L: u64 = 18;

pub struct MT19937 {
    pub state: [u64; N],
    pub cnt: usize,
}

impl MT19937 {
    pub fn new(seed: u64) -> Self {
        let mut state = [0u64; N];

        state[0] = seed;

        for i in 1..N {
            state[i] = (F * (state[i - 1] ^ (state[i - 1] >> (W - 2))) + 1) & ((1 << W) - 1);
        }

        let mut instance = MT19937 { state, cnt: 0usize };

        instance.twist();

        instance
    }

    fn twist(&mut self) {
        //
        for i in 1..N {
            let lower_mask = (1 << R) - 1;

            let upper_mask = (!lower_mask) & ((1 << W) - 1);

            let tmp = (self.state[i] & upper_mask) + (self.state[(i + 1) % N] & lower_mask);

            let mut tmp_a = tmp >> 1;

            if tmp % 2 == 1 {
                tmp_a = tmp_a ^ A;
            }
            self.state[i] = self.state[(i + M) % N] ^ tmp_a;
        }
    }

    pub fn temper(&mut self) -> u64 {
        if self.cnt == N {
            self.twist();
        }

        let mut y = self.state[self.cnt];

        y ^= (y >> U) & D;

        y ^= (y << S) & B;

        y ^= (y << T) & C;

        y = y ^ (y >> L);

        self.cnt += 1;

        return y & ((1 << W) - 1);
    }
}
