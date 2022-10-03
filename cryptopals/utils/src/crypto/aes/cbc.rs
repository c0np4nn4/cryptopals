use aes::{
    cipher::{
        generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit,
    },
    Aes128,
};

use crate::{padding::pkcs7::pkcs7, xor::fixed_xor};

use super::BLOCK_SIZE;

pub fn encrypt_cbc(key: Vec<u8>, data: &mut Vec<u8>, iv: [u8; 16]) -> Vec<u8> {
    pkcs7(data, BLOCK_SIZE);

    let key = GenericArray::from_slice(&key);

    let cipher = Aes128::new(&key);

    let mut previous_block = iv;

    let mut res = Vec::new();

    for idx in (0..data.len()).step_by(16) {
        let block: [u8; 16] = data[idx..idx + 16].try_into().unwrap();

        let block: [u8; 16] =
            fixed_xor(block.to_vec(), previous_block.to_vec())
                .unwrap()
                .try_into()
                .unwrap();

        let mut block = GenericArray::from_slice(&block).clone();

        cipher.encrypt_block(&mut block);

        res.append(&mut block.to_vec());

        previous_block = block.try_into().unwrap();
    }

    res
}

pub fn decrypt_cbc(key: Vec<u8>, data: Vec<u8>) -> Vec<u8> {
    let key = GenericArray::from_slice(&key);

    let cipher = Aes128::new(&key);

    let mut last_block: [u8; 16] =
        data[data.len() - 16..data.len()].try_into().unwrap();

    let mut res = Vec::new();

    for idx in (0..=data.len() - 32).rev().step_by(16) {
        let mut block = GenericArray::from_slice(&last_block).to_owned();

        cipher.decrypt_block(&mut block);

        let previous_block: [u8; 16] = data[idx..idx + 16].try_into().unwrap();

        let block = fixed_xor(block.to_vec(), previous_block.to_vec()).unwrap();

        let mut tmp: Vec<u8> = block;

        tmp.append(&mut res);

        res = tmp;

        last_block = previous_block;
    }

    res
}
