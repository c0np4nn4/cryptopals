use std::fs;
use utils::{
    base64::base64_dec,
    crypto::aes::{decrypt_ctr, BLOCK_SIZE},
    padding::pkcs7::{pkcs7, trim_pkcs7},
};

#[test]
fn chal_18() {
    let key: Vec<u8> = "YELLOW SUBMARINE".as_bytes().to_vec();
    let nonce = [0u8; 8];

    let data = fs::read_to_string("../../data/18.txt")
        .unwrap()
        .trim()
        .to_string();

    let mut data = base64_dec(data).unwrap();

    pkcs7(&mut data, BLOCK_SIZE);

    let res = decrypt_ctr(key, data.clone(), nonce).unwrap();

    trim_pkcs7(&mut data, BLOCK_SIZE).unwrap();

    // let res = data;

    println!("res (hex): {:02x?}", res);
    println!("res (str): {:?}", String::from_utf8_lossy(&res));
}
