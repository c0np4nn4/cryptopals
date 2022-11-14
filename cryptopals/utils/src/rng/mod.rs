// ref: https://github.com/ESultanik/mtwister/blob/master/mtwister.c
// wiki: https://en.wikipedia.org/wiki/Mersenne_Twister#Algorithmic_detail

const UPPER_MASK: u64 = 0x8000_0000;
const LOWER_MASK: u64 = 0x7fff_ffff;
const TEMPERING_MASK_B: u64 = 0x9d2c_5680;
const TEMPERING_MASK_C: u64 = 0xefc6_0000;

#[derive(Debug)]
pub struct MTRand {
    pub mt: [u64; 624],
    pub idx: usize,
}
impl MTRand {
    fn new() -> Self {
        let mt = [0u64; 624];
        let idx = 0usize;
        MTRand { mt, idx }
    }
}

fn m_seed_rand(rand: &mut MTRand, seed: u64) {
    rand.mt[0] = seed & 0xffff_ffff;

    for idx in 1..624 {
        rand.mt[idx] = (6069 * rand.mt[idx - 1]) & 0xffff_ffff;
    }

    rand.idx = 624;
}

pub fn seed_rand(seed: u64) -> MTRand {
    let mut rand = MTRand::new();

    m_seed_rand(&mut rand, seed);

    return rand;
}

fn gen_rand_long(rand: &mut MTRand) -> u64 {
    let mut y: u64 = 0;

    static mag: [u64; 2] = [0u64, 0x9908_b0df];

    if rand.idx >= 624 {
        let kk = 0usize;

        if rand.idx >= 624 + 1 {
            m_seed_rand(rand, 4357);
        }

        for i in 0..(624 - 397) {
            y = (rand.mt[i] & UPPER_MASK) | (rand.mt[i + 1] & LOWER_MASK);

            rand.mt[i] = rand.mt[i + 397] ^ (y >> 1) ^ mag[(y & 0x1) as usize];
        }

        for i in (624 - 397)..(624 - 1) {
            y = (rand.mt[i] & UPPER_MASK) | (rand.mt[i + 1] & LOWER_MASK);

            // println!("(624 - 397 == {}) + 624 - 397: {:?}", i, i + (624 - 397));
            rand.mt[i] = rand.mt[i - 227] ^ (y >> 1) ^ mag[(y & 0x1) as usize];
        }

        y = (rand.mt[624 - 1] & UPPER_MASK) | (rand.mt[0] & LOWER_MASK);

        rand.mt[624 - 1] = rand.mt[397 - 1] ^ (y >> 1) ^ mag[(y & 0x1) as usize];

        rand.idx = 0;
    }

    rand.idx += 1;
    y = rand.mt[rand.idx];

    y ^= y >> 11;

    y ^= (y << 7) & TEMPERING_MASK_B;

    y ^= (y << 15) & TEMPERING_MASK_C;

    y ^= y >> 18;

    return y;
}

pub fn gen_rand(rand: &mut MTRand) -> u64 {
    return gen_rand_long(rand);
}
