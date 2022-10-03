use std::{collections::HashMap, fs};

use utils::crypto::aes::detect_ecb;

#[test]
fn chal_8() {
    let data: Vec<Vec<u8>> = fs::read_to_string("../../data/8.txt")
        .unwrap()
        .split_ascii_whitespace()
        .map(|ct| ct.as_bytes().to_vec())
        .collect();

    let mut result: HashMap<usize, Vec<(usize, usize)>> = HashMap::default();

    data.iter().enumerate().for_each(|(idx, data)| {
        if let Some(res) = detect_ecb(data.clone()) {
            result.insert(idx, res);
        }
    });

    println!("res: {:?}", result);
}
