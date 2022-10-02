use aes::{
    cipher::{
        generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit,
    },
    Aes128,
};
use std::{convert::TryInto, fs};
use utils::{base64::base64_dec, padding::pkcs7::pkcs7, xor::fixed_xor};

#[test]
fn chal_10() {
    const BLOCKSIZE: usize = 16;
    const KEY: &'static str = "YELLOW SUBMARINE";

    let data: String = fs::read_to_string("../../data/10.txt")
        .unwrap()
        .split_ascii_whitespace()
        .collect();

    let mut data = base64_dec(data).unwrap();

    pkcs7(&mut data, BLOCKSIZE);

    let key = GenericArray::from_slice(KEY.as_bytes());

    let cipher = Aes128::new(&key);

    // decryption
    let mut last_block: [u8; 16] =
        data[data.len() - 16..data.len()].try_into().unwrap();

    let mut res = Vec::new();

    for idx in (0..=data.len() - 32).rev().step_by(16) {
        let mut block = GenericArray::from_slice(&last_block).to_owned();

        cipher.decrypt_block(&mut block);

        // println!("idx: {:?}", idx);
        let previous_block: [u8; 16] = data[idx..idx + 16].try_into().unwrap();

        let block = fixed_xor(block.to_vec(), previous_block.to_vec()).unwrap();

        let mut tmp: Vec<u8> = block;

        tmp.append(&mut res);

        res = tmp;

        last_block = previous_block;
    }

    let res: String = res.into_iter().map(|byte| byte as char).collect();
    println!("res: {:?}", res);

    // encryption
    // let mut previous_block = [0u8; 16];

    // let mut res = Vec::new();

    // for idx in (0..data.len()).step_by(16) {
    //     let block: [u8; 16] = data[idx..idx + 16].try_into().unwrap();
    //     println!("\n[target] block: {:02x?}", block);

    //     // for cbc
    //     let block: [u8; 16] =
    //         fixed_xor(block.to_vec(), previous_block.to_vec())
    //             .unwrap()
    //             .try_into()
    //             .unwrap();
    //     println!("previous block: {:02x?}", previous_block);
    //     println!("[xor] block:    {:02x?}", block);

    //     let mut block = GenericArray::from_slice(&block).clone();

    //     // cipher.encrypt_block(&mut block);
    //     cipher.encrypt_block(&mut block);

    //     println!("[res] block:    {:02x?}", block);
    //     res.append(&mut block.to_vec());

    //     previous_block = block.try_into().unwrap();
    // }

    // let res: String = res.into_iter().map(|byte| byte as char).collect();

    // println!("res: {:?}", res);
}
