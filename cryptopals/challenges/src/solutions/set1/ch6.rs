use std::fs;

use utils::{
    attack::{
        break_arbitrary_size_repeating_key_xor_cipher, ArbSizedKeyAttackResult,
    },
    base64::base64_dec,
};

#[test]
fn chal_6() {
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
