use std::fs;

use utils::{
    attack::{
        break_arbitrary_size_repeating_key_xor_cipher, ArbSizedKeyAttackResult,
    },
    base64::base64_dec,
    ham_dist::get_hamming_distance,
};

#[test]
fn test_break_repeating_key_xor() {
    let ct_candidates: String = fs::read_to_string("../../data/6.txt")
        .unwrap()
        .split_ascii_whitespace()
        .collect();

    let res = base64_dec(ct_candidates).unwrap();

    let ArbSizedKeyAttackResult { pt, key } =
        break_arbitrary_size_repeating_key_xor_cipher(2, 40, res).unwrap();

    println!("plaintext: {:?}", pt);

    println!("key: {:?}", key);
}

#[test]
fn test_hamming_distance() {
    let l = "this is a test".to_string().as_bytes().to_vec();
    let r = "wokka wokka!!!".to_string().as_bytes().to_vec();

    let res = get_hamming_distance(&l, &r).unwrap();
    println!("{:?}", res);
}