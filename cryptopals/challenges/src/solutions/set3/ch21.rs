use utils::rng::{gen_rand, seed_rand, MTRand};

#[test]
fn chal_21() {
    let seed: u64 = 0x1337;

    let mut mt_rand: MTRand = seed_rand(seed);

    let rand = gen_rand(&mut mt_rand);

    println!("rand: {:?}", rand);
}
