use crate::{
    crypto::{mode::Mode, AES},
    types,
};

#[test]
fn test_aes_encrypt_ecb() {
    let msg = {
        let msg = String::from("Hello, World!");

        msg.as_bytes().to_vec()
    };

    let key = {
        let key = String::from("YELLOW SUBMARINE");

        key.as_bytes().try_into().unwrap()
    };

    let aes = AES::new(None, Mode::ECB);

    let res = aes.encrypt(msg, key).unwrap();

    println!("res: {:?}", types::u8_vec_to_hex_string(res));
}
