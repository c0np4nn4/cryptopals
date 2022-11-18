use utils::crypto::mt19937::MT19937;

#[test]
fn chal_21() {
    let seed: u64 = 5489;

    let rng = MT19937::new(seed);
    println!("rand: {:?}", rng.state);

    let rrng_32 = mersenne_twister::MT19937::new_unseeded();
    println!("rand 32: {:?}", rrng_32);

    // let rrng_64 = mersenne_twister::MT19937_64::new_unseeded();
    // println!("rand 64: {:?}", rrng_64);
}
