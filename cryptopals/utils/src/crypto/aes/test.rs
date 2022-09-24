use crate::{
    crypto::{mode::Mode, AES},
    types,
};

#[test]
fn test_aes_encrypt_ecb() {
    // let msg = {
    //     let msg = String::from("Hello, World!");

    //     msg.as_bytes().to_vec()
    // };

    let data: [u8; 16] = [
        0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d, 0x31, 0x31, 0x98, 0xa2,
        0xe0, 0x37, 0x07, 0x34,
    ];

    // let key = {
    //     let key = String::from("YELLOW SUBMARINE");

    //     key.as_bytes().try_into().unwrap()
    // };

    let key: [u8; 16] = [
        0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88,
        0x09, 0xcf, 0x4f, 0x3c,
    ];

    let aes = AES::new(None, key, Mode::ECB);

    let res = aes.encrypt(data.to_vec()).unwrap();

    println!("res: {:?}", types::u8_vec_to_hex_string(res));
}

#[test]
fn key_expansion() {
    let key: [u8; 16] = [
        0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88,
        0x09, 0xcf, 0x4f, 0x3c,
    ];

    let aes = AES::new(None, key, Mode::ECB);

    let ext_key = aes.key_expansion().unwrap();

    assert_eq!(ext_key[10][12], 0xb6);
    assert_eq!(ext_key[10][13], 0x63);
    assert_eq!(ext_key[10][14], 0x0c);
    assert_eq!(ext_key[10][15], 0xa6);
}
