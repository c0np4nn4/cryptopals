use std::fs;

use rand::Rng;
use utils::oracles::Oracle17;

#[test]
fn chal_17() {
    let oracle = Oracle17::new();

    // let data = fs::read_to_string("../../data/17.txt").unwrap();

    // let data: Vec<String> = data
    //     .split("\n")
    //     .into_iter()
    //     .map(|s| s.to_string())
    //     .collect();

    // println!("data: {:#?}", data);

    // let random_idx = rand::thread_rng().gen_range(0..data.len());

    // println!("[!] idx:  {:?}", random_idx);
    // println!("[!] data: {:?}", data[random_idx]);

    // let input_data = data[random_idx].clone().as_bytes().to_vec();

    let input_data = "\
        YELLOW SUBMARINE\
        YELLOW SUBMARINE\
        \
        "
    .as_bytes()
    .to_vec();

    let (ct, iv) = oracle.encrypt(input_data);

    println!("ct: {:02x?}", ct);
    println!("iv: {:02x?}", iv);

    let _ = oracle.decrypt(ct, [0u8; 16]);
}
