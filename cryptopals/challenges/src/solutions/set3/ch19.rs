use std::fs;

use utils::{base64::base64_dec, oracles::Oracle19};

#[test]
fn chal_19() {
    let oracle = Oracle19::new();

    let data = fs::read_to_string("../../data/19.txt")
        .unwrap()
        .trim()
        .to_string();

    let data: Vec<String> = data
        .split("\n")
        .into_iter()
        .map(|s| s.to_string())
        .collect();

    let data: Vec<Vec<u8>> = data.into_iter().map(|d| base64_dec(d).unwrap()).collect();

    let mut ct: Vec<Vec<u8>> = vec![];

    for d in data {
        let c = oracle.encrypt(d);
        ct.push(c);
    }

    // TODO

    // Keystream: [u8; 16]
    // CT ^ KEYSTREAM = PT
    // PT must be the `readable` string
    // scoring the `maybe_PT` based on the frequency table for each byte of the stream
    // ! Recover the KEYSTREAM
    // ! Get the PlainText
}
