use aes::{
    cipher::{generic_array::GenericArray, BlockEncrypt, KeyInit},
    Aes128,
};

use crate::crypto::aes_core::AES;

#[test]
fn test_aes_encrypt() {
    let data: [u8; 16] = [
        0x32, 0x43, 0xf6, 0xa8, //
        0x88, 0x5a, 0x30, 0x8d, //
        0x31, 0x31, 0x98, 0xa2, //
        0xe0, 0x37, 0x07, 0x34,
    ];

    let key: [u8; 16] = [
        0x2b, 0x7e, 0x15, 0x16, //
        0x28, 0xae, 0xd2, 0xa6, //
        0xab, 0xf7, 0x15, 0x88, //
        0x09, 0xcf, 0x4f, 0x3c,
    ];

    let aes = AES::new(key);

    let res = aes.encrypt(data.to_vec()).unwrap();

    let cmp = {
        let key = GenericArray::from_slice(&key);

        let cipher = Aes128::new(&key);

        let mut res = Vec::new();

        for idx in (0..data.len()).step_by(16) {
            let block: [u8; 16] = data[idx..idx + 16].try_into().unwrap();

            let mut block = GenericArray::from_slice(&block).clone();

            cipher.encrypt_block(&mut block);

            res.append(&mut block.to_vec());
        }

        res
    };

    println!("[!] res: {:?}", res);
    println!("[!] cmp: {:?}", cmp);

    assert_eq!(res, cmp);
}

#[test]
fn test_aes_decrypt() {
    let data: [u8; 16] = [
        0x32, 0x43, 0xf6, 0xa8, //
        0x88, 0x5a, 0x30, 0x8d, //
        0x31, 0x31, 0x98, 0xa2, //
        0xe0, 0x37, 0x07, 0x34,
    ];

    let key: [u8; 16] = [
        0x2b, 0x7e, 0x15, 0x16, //
        0x28, 0xae, 0xd2, 0xa6, //
        0xab, 0xf7, 0x15, 0x88, //
        0x09, 0xcf, 0x4f, 0x3c,
    ];

    let aes = AES::new(key);

    let res = aes.encrypt(data.to_vec()).unwrap();

    let comp = {
        let key = GenericArray::from_slice(&key);

        let cipher = Aes128::new(&key);

        let mut res = Vec::new();

        for idx in (0..data.len()).step_by(16) {
            let block: [u8; 16] = data[idx..idx + 16].try_into().unwrap();

            let mut block = GenericArray::from_slice(&block).clone();

            cipher.encrypt_block(&mut block);

            res.append(&mut block.to_vec());
        }

        res
    };

    println!("[!] res: {:?}", res);
    println!("[!] cmp: {:?}", comp);

    assert_eq!(
        res,
        // [
        //     // 0x39, 0x02, 0xdc, 0x19, //
        //     // 0x25, 0xdc, 0x11, 0x6a, //
        //     // 0x84, 0x09, 0x85, 0x0b, //
        //     // 0x1d, 0xfb, 0x97, 0x32
        //     0x39, 0x25, 0x84, 0x1d, //
        //     0x02, 0xdc, 0x09, 0xfb, //
        //     0xdc, 0x11, 0x85, 0x97, //
        //     0x19, 0x6a, 0x0b, 0x32
        // ]
        comp
    );
}

#[test]
fn key_expansion() {
    let key: [u8; 16] = [
        0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f,
        0x3c,
    ];

    let aes = AES::new(key);

    let ext_key = aes.key_expansion().unwrap();

    assert_eq!(ext_key[10][12], 0xb6);
    assert_eq!(ext_key[10][13], 0x63);
    assert_eq!(ext_key[10][14], 0x0c);
    assert_eq!(ext_key[10][15], 0xa6);
}
