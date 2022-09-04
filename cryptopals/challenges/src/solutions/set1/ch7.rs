use std::fs;

use utils::{
    crypto::{mode::Mode, AES},
    types,
};

#[test]
fn chal_7() {
    let ct: String = fs::read_to_string("../../data/7.txt")
        .unwrap()
        .split_ascii_whitespace()
        .collect();

    let key: String = "YELLOW SUBMARINE".to_string();

    // let res =
    // aes_decrypt(ct.as_bytes().to_vec(), key.as_bytes().to_vec(), MOO::ECB)
    //     .unwrap();

    // let aes = AES::new(ct.as_bytes().to_vec(), None, Mode::ECB);

    // let res = aes.decrypt(key.as_bytes().try_into().unwrap());

    // println!("res: {:?}", String::from_utf8(res));

    // println!("res: {:?}", res);
    // panic!()
}
