use utils::crypto::mt19937::core::MT19937;

#[test]
fn chal_21() {
    let seed: u32 = 5489;

    let mut my_rng = MT19937::new();
    my_rng.init(seed);
    println!("my rng: {:?}", my_rng.state);

    let rrng_32 = mersenne_twister::MT19937::new_unseeded();
    println!("\n\n\nrand 32: {:?}", rrng_32);
}
