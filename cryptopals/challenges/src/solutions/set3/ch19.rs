use rand::Rng;
use std::fs;
use utils::{
    attack::{single_char_key_attack, SingleCharKeyAttackResult},
    base64::base64_dec,
    crypto::aes::BLOCK_SIZE,
    oracles::Oracle19,
    padding::pkcs7::trim_pkcs7,
    xor::fixed_xor,
};

#[test]
fn chal_19() {
    let oracle = Oracle19::new();

    // ------------------------------------------------------------------------------------
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
    // ------------------------------------------------------------------------------------

    // cryptanalysis starts
    let mut max_len = 0usize;

    for c in ct.clone() {
        if max_len < c.len() {
            max_len = c.len();
        }
    }

    // get the key_stream first
    let key_stream = {
        let mut dummy_ct: Vec<Vec<u8>> = vec![];

        for _ in 0..2048 {
            dummy_ct.push(oracle.encrypt(get_dummy_pt(max_len)));
        }

        get_key_stream(dummy_ct, max_len)
        // get_key_stream(ct.clone(), max_len)
    };

    let mut pt: Vec<String> = vec!["".to_string(); ct.len()];

    for ct_idx in 0..ct.len() {
        let c = ct[ct_idx].clone();

        for i in (0..c.len()).step_by(16) {
            let key_chunk = key_stream[i..i + 16].to_vec();

            let ct_chunk = c[i..i + 16].to_vec();

            let mut res = fixed_xor(key_chunk, ct_chunk).unwrap();

            trim_pkcs7(&mut res, BLOCK_SIZE).unwrap();

            pt[ct_idx] = format!("{}{}", pt[ct_idx], String::from_utf8(res).unwrap());
        }
    }

    println!("pt: {:#?}", pt);
}

fn get_key_stream(ct: Vec<Vec<u8>>, len: usize) -> Vec<u8> {
    let mut key_stream = Vec::<u8>::new();

    for col in 0..len {
        let mut col_bit_stream = Vec::<u8>::new();

        for ct_idx in 0..ct.len() {
            if ct[ct_idx].len() > col {
                col_bit_stream.push(ct[ct_idx][col]);
            }
        }

        let SingleCharKeyAttackResult { key, score, .. } =
            single_char_key_attack(col_bit_stream).unwrap();

        key_stream.push(key as u8);
    }

    key_stream
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
