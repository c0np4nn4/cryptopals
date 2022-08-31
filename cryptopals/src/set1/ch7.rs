use std::fs;

use crate::{aes_decrypt, MOO};

#[test]
fn test_aes_in_ecb_mode() {
    let ct: String = fs::read_to_string("./src/challenge-data/7.txt")
        .unwrap()
        .split_ascii_whitespace()
        .collect();

    let key: String = "YELLOW SUBMARINE".to_string();

    let res =
        aes_decrypt(ct.as_bytes().to_vec(), key.as_bytes().to_vec(), MOO::ECB)
            .unwrap();

    println!("res: {:?}", res);
    panic!()
}
