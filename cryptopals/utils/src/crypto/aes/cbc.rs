use aes::{
    cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit},
    Aes128,
};

use crate::{
    padding::pkcs7::{pkcs7, trim_pkcs7},
    xor::fixed_xor,
};

use super::BLOCK_SIZE;

pub fn encrypt_cbc(key: Vec<u8>, data: &mut Vec<u8>, iv: [u8; BLOCK_SIZE]) -> Vec<u8> {
    pkcs7(data, BLOCK_SIZE);

    let key = GenericArray::from_slice(&key);

    let cipher = Aes128::new(&key);

    let mut previous_block = iv;

    let mut res = Vec::new();

    for idx in (0..data.len()).step_by(BLOCK_SIZE) {
        let block: [u8; BLOCK_SIZE] = data[idx..idx + BLOCK_SIZE].try_into().unwrap();

        let block: [u8; BLOCK_SIZE] = fixed_xor(block.to_vec(), previous_block.to_vec())
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

pub fn decrypt_cbc(key: Vec<u8>, data: Vec<u8>, iv: [u8; BLOCK_SIZE]) -> Vec<u8> {
    let key = GenericArray::from_slice(&key);

    let cipher = Aes128::new(&key);

    let mut last_block: [u8; BLOCK_SIZE] = data[data.len() - BLOCK_SIZE..data.len()]
        .try_into()
        .unwrap();

    let mut res = Vec::new();

    for idx in (0..=data.len() - BLOCK_SIZE * 2).rev().step_by(BLOCK_SIZE) {
        let mut block = GenericArray::from_slice(&last_block).to_owned();

        cipher.decrypt_block(&mut block);

        let previous_block: [u8; BLOCK_SIZE] = data[idx..idx + BLOCK_SIZE].try_into().unwrap();

        let block = fixed_xor(block.to_vec(), previous_block.to_vec()).unwrap();

        let mut tmp: Vec<u8> = block.clone();

        tmp.append(&mut res);

        res = tmp;

        last_block = previous_block;
    }

    let last_block: [u8; BLOCK_SIZE] = data[0..BLOCK_SIZE].try_into().unwrap();

    let mut block = GenericArray::from_slice(&last_block).to_owned();

    cipher.decrypt_block(&mut block);

    let block = fixed_xor(block.to_vec(), iv.to_vec()).unwrap();

    let mut tmp: Vec<u8> = block.clone();

    tmp.append(&mut res);

    res = tmp;

    trim_pkcs7(&mut res, BLOCK_SIZE);

    res
}
