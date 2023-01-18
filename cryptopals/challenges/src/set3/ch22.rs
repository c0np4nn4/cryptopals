use std::time::{SystemTime, Duration, UNIX_EPOCH};
use std::thread::sleep;
use rand::prelude::*;
use utils::crypto::PRNG;
use utils::crypto::mt19937::core::MT19937;

fn sleep_for_a_while() {
    // let time = rand::thread_rng().gen_range(40..1000);
    // let time = rand::thread_rng().gen_range(5..30);
    let time = rand::thread_rng().gen_range(1..3);
    println!("[-] sleep: {:?}s", time);
    sleep(Duration::from_secs(time));
}

#[test]
fn chal_22() {
    let mut my_rng = MT19937::new();

    sleep_for_a_while();

    let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as u32;
    my_rng.init(seed);
    println!("[-] seed: {:?}\n", seed);

    sleep_for_a_while();

    let random_number = my_rng.extract();
    println!("[-] random_number: {:?}\n", random_number);

    // attack
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as u32;
    // for t in (now - 1010)..now{
    for t in (now - 40)..now{
        let mut rng2 = MT19937::new();

        rng2.init(t);

        let candidate = rng2.extract();

        if candidate == random_number {
            println!("[+] recovered seed: {:?}", t);
            break;
        }
    }
}
