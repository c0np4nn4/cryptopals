use std::{collections::HashMap, fs};

use utils::{base64::base64_dec, crypto::aes::encrypt_ecb_with_unknown_key};

#[test]
fn chal_12() {
    let data: String = fs::read_to_string("../../data/12.txt")
        .unwrap()
        .split_ascii_whitespace()
        .collect();

    let data = base64_dec(data).unwrap();

    // 1. finding the block size
    let mut data_clone = data.clone();

    let target = encrypt_ecb_with_unknown_key(data_clone).unwrap();

    let mut block_size: usize = 0;

    let mut buffer: Vec<u8> = vec![];

    for size in 1..128 {
        let mut data_clone = data.clone();

        buffer.push(0x41);

        let mut prefix = buffer.clone();

        prefix.append(&mut data_clone);

        let comparison = encrypt_ecb_with_unknown_key(prefix).unwrap()[buffer.len()..].to_vec();

        if target.eq(&comparison) {
            block_size = size;
            break;
        }
    }

    println!("[+] block size: {:?}", block_size);

    // 2. detecting the ECB mode
    let mut chunk: Vec<u8> = vec![0x41u8; block_size * 2];

    let res = encrypt_ecb_with_unknown_key(chunk).unwrap();

    if res[0..block_size].eq(&res[block_size..block_size * 2]) {
        println!("[+] Detect the ECB mode");
    } else {
        panic!();
    }

    // 3. feed n-short block to the function,
    let mut rainbow_table = HashMap::<Vec<u8>, u8>::new();

    let buffer: Vec<u8> = vec![0x41; 7];

    for byte in 0..=255 {
        let mut buffer_clone = buffer.clone();

        buffer_clone.push(byte);

        let res = encrypt_ecb_with_unknown_key(buffer_clone).unwrap();

        rainbow_table.insert(res, byte);
    }

    let mut result: Vec<u8> = Vec::new();

    for idx in 0..data.len() {
        let mut buffer_clone = buffer.clone();

        buffer_clone.push(data[idx]);

        let res = encrypt_ecb_with_unknown_key(buffer_clone).unwrap();

        if let Some(byte) = rainbow_table.get(&res) {
            result.push(*byte);
        }
    }

    println!("[+] result: {:?}", String::from_utf8(result).unwrap());
}
