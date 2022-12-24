// ref: https://nayak.io/posts/mersenne_twister/
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

pub fn untemper(y: u32) -> u32 {
    // y3
    let y = y ^ (y >> L);

    // y2
    let y = y ^ ((y << T) & C);

    // y1
    let mask = 0x7f; // (bin) 0111_1111_1111_1111

    let b = B & (mask << 7);
    let tmp1 = y ^ ((y << S) & b);

    let b = B & (mask << 14);
    let tmp2 = tmp1 ^ ((tmp1 << S) & b);

    let b = B & (mask << 21);
    let tmp3 = tmp2 ^ ((tmp2 << S) & b);

    let b = B & (mask << 28);
    let tmp4 = tmp3 ^ ((tmp3 << S) & b);

    let y = tmp4;

    // y
    let y = y ^ (y >> U);
    let y = y ^ (y >> U);
    let y = y ^ (y >> U);

    y
}
