use std::fs;

use crate::base64_dec;

#[test]
fn test_break_repeating_key_xor() {
    let ct_candidates = fs::read_to_string("./src/challenge-data/6.txt")
        .unwrap()
        .split_ascii_whitespace()
        .collect();

    let res = base64_dec(ct_candidates).unwrap();
    println!("{:?}", res);
}
