use std::fs;
use utils::base64::base64_dec;
use utils::crypto::aes::decrypt_ecb;

#[test]
fn chal_7() {
    let key = String::from("YELLOW SUBMARINE");

    let data: String = fs::read_to_string("../../data/7.txt")
        .unwrap()
        .split_ascii_whitespace()
        .collect();

    let data = base64_dec(data).unwrap();

    let res = decrypt_ecb(key.as_bytes().to_vec(), data).unwrap();

    let res: String = res.into_iter().map(|byte| byte as char).collect();

    println!("res: {:?}", res);
}
