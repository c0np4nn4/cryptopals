use std::convert::TryInto;
use std::fs;

use aes::cipher::BlockDecrypt;
use aes::cipher::{generic_array::GenericArray, KeyInit};
use aes::Aes128;
use utils::base64::base64_dec;
use utils::types::u8_vec_to_hex_string;
// use utils::crypto;

#[test]
fn chal_7() {
    let key_str = String::from("YELLOW SUBMARINE");

    let key = GenericArray::from_slice(key_str.as_bytes());

    let cipher = Aes128::new(&key);

    let data: String = fs::read_to_string("../../data/7.txt")
        .unwrap()
        .split_ascii_whitespace()
        .collect();

    let data = base64_dec(data).unwrap();

    let mut res = Vec::new();

    for idx in (0..data.len()).step_by(16) {
        let block: [u8; 16] = data[idx..idx + 16].try_into().unwrap();

        let mut block = GenericArray::from_slice(&block).clone();
        println!("\nblock: {:02x?}", block);

        cipher.decrypt_block(&mut block);
        println!("block: {:02x?}", block);

        res.append(&mut block.to_vec());
    }

    let res: String = res.into_iter().map(|byte| byte as char).collect();

    println!("res: {:?}", res);
}
