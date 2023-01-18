use aes::{
    cipher::{generic_array::GenericArray, BlockEncrypt, KeyInit},
    Aes128,
};

use crate::crypto::aes::{core::AES, get_random_aes_key};



#[test]
fn test_aes_encrypt() {
    let data: [u8; 16] = "Hello World! LOL".to_string().as_bytes().try_into().unwrap();
        
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
    let data: [u8; 16] = [205, 31, 176, 149, 14, 70, 178, 123, 187, 150, 193, 191, 116, 27, 247, 4];

    let key: [u8; 16] = [
        0x2b, 0x7e, 0x15, 0x16, //
        0x28, 0xae, 0xd2, 0xa6, //
        0xab, 0xf7, 0x15, 0x88, //
        0x09, 0xcf, 0x4f, 0x3c,
    ];

    let aes = AES::new(key);

    let res = aes.decrypt(data.to_vec()).unwrap();

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

    println!("[!] res: {:?}", String::from_utf8_lossy(&res));
    println!("[!] cmp: {:?}", String::from_utf8_lossy(&comp));

    assert_eq!(
        res,
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

#[test]
fn check_sub_bytes() {
    let aes = AES::new(get_random_aes_key());

    let state: [u8; 16] = [0, 8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

    let tmp = aes.sub_bytes(state).unwrap();
    let res = aes.inv_sub_bytes(tmp).unwrap();

    assert_eq!(state, res);
}

#[test]
fn check_shift_rows() {
    let aes = AES::new(get_random_aes_key());

    let state: [u8; 16] = [0, 8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

    let tmp = aes.shift_rows(state).unwrap();
    let res = aes.inv_shift_rows(tmp).unwrap();

    assert_eq!(state, res);
}

#[test]
fn check_mix_columns() {
    let aes = AES::new(get_random_aes_key());

    let state: [u8; 16] = [0, 8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

    let tmp = aes.mix_columns(state).unwrap();
    let res = aes.inv_mix_columns(tmp).unwrap();

    assert_eq!(state, res);
}

#[test]
fn enc_and_dec() {
    let state: [u8; 16] = "Hello World! LOL".to_string().as_bytes().try_into().unwrap();

    let key: [u8; 16] = [
        0x2b, 0x7e, 0x15, 0x16, //
        0x28, 0xae, 0xd2, 0xa6, //
        0xab, 0xf7, 0x15, 0x88, //
        0x09, 0xcf, 0x4f, 0x3c,
    ];

    let aes = AES::new(key);

    println!("state: {:?}", state);

    let ct = aes.encrypt(state.to_vec()).unwrap();
    let pt = aes.decrypt(ct.clone()).unwrap();

    println!("msg: {:?}", std::str::from_utf8(&state));
    println!("ct: {:?}", String::from_utf8_lossy(&ct));
    println!("pt: {:?}", String::from_utf8_lossy(&pt));
}
