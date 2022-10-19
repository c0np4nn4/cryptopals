use std::fs;

use utils::{
    base64::base64_dec,
    crypto::aes::{decrypt_cbc, BLOCK_SIZE},
    padding::pkcs7::pkcs7,
};

#[test]
fn chal_10() {
    const KEY: &'static str = "YELLOW SUBMARINE";

    let data: String = fs::read_to_string("../../data/10.txt")
        .unwrap()
        .split_ascii_whitespace()
        .collect();

    let mut data = base64_dec(data).unwrap();

    pkcs7(&mut data, BLOCK_SIZE);

    let res = decrypt_cbc(KEY.as_bytes().to_vec(), data, [0u8; BLOCK_SIZE]);

    let res: String = res.into_iter().map(|byte| byte as char).collect();
    println!("res: {:?}", res);
}
