use std::fs;

use rand::Rng;
use utils::{
    attack::{break_arbitrary_size_repeating_key_xor_cipher, ArbSizedKeyAttackResult},
    base64::base64_dec,
    oracles::Oracle19,
    xor::fixed_xor,
};

#[test]
fn chal_20() {
    // same oracle with ch19
    let oracle = Oracle19::new();

    // ------------------------------------------------------------------------------------
    let data = fs::read_to_string("../../data/20.txt")
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
    // ------------------------------------------------------------------------------------

    // cryptanalysis starts
    let mut min_len = std::usize::MAX;

    for c in ct.clone() {
        if min_len > c.len() {
            min_len = c.len();
        }
    }

    for c in ct.iter_mut() {
        // unsafe
        c.truncate(min_len);
    }

    println!("min_len: {:?}", min_len);

    // extracting a key stream using the dummy plaintexts
    let key_stream: Vec<u8> = {
        let mut dummy_ct: Vec<u8> = vec![];

        for _ in 0..4096 {
            let mut a = oracle.encrypt(get_dummy_pt(min_len));
            dummy_ct.append(&mut a);
        }

        let ArbSizedKeyAttackResult { pt: _pt, key } =
            break_arbitrary_size_repeating_key_xor_cipher(1, min_len as u64, dummy_ct).unwrap();

        println!("key len: {:?}", key.len());

        key
    };

    for c in ct.clone() {
        let k = key_stream.clone();

        let pt = fixed_xor(c, k).unwrap();

        println!("pt: {:?}", String::from_utf8_lossy(&pt));
    }
}

fn get_dummy_pt(len: usize) -> Vec<u8> {
    let mut character: Vec<u8> = vec![];

    for c in ('a' as u8)..('z' as u8) {
        character.push(c);
    }

    for c in ('A' as u8)..('Z' as u8) {
        character.push(c);
    }

    character.push(' ' as u8);

    let mut res: Vec<u8> = vec![];

    for _ in 0..len {
        let idx = rand::thread_rng().gen_range(0..character.len());

        let c = character[idx];

        res.push(c);
    }

    res
}
