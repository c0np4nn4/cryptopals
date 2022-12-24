use std::convert::TryInto;

use lazy_static::lazy_static;

use crate::crypto::CryptoError;
use crate::crypto::aes_core::AES;
use crate::crypto::aes_core::key::from_vec;
use crate::padding::pkcs7::pkcs7;
use crate::xor::fixed_xor;

use super::BLOCK_SIZE;

pub fn encrypt_ecb(key: Vec<u8>, data: &mut Vec<u8>) -> Result<Vec<u8>, CryptoError> {
    pkcs7(data, BLOCK_SIZE);

    let cipher = AES::new(from_vec(key)?);

    let mut res = Vec::new();

    for idx in (0..data.len()).step_by(16) {
        let block = data[idx..idx + 16].to_vec();

        let block = cipher.encrypt(block)?;

        res.append(&mut block.to_vec());
    }

    Ok(res)
}

pub fn decrypt_ecb(key: Vec<u8>, data: Vec<u8>) -> Result<Vec<u8>, CryptoError> {
    let cipher = AES::new(from_vec(key)?);

    let mut res = Vec::new();

    for idx in (0..data.len()).step_by(16) {
        let block = data[idx..idx + 16].to_vec();

        let block = cipher.decrypt(block)?;

        res.append(&mut block.to_vec());
    }

    Ok(res)
}

pub fn detect_ecb(data: Vec<u8>) -> Option<Vec<(usize, usize)>> {
    let mut clues: Vec<(usize, usize)> = vec![];

    for i in (0..data.len()).step_by(16) {
        for j in (i + 16..data.len()).step_by(16) {
            let chunk_1: [u8; 16] = data[i..i + 16].try_into().unwrap();

            let chunk_2: [u8; 16] = data[j..j + 16].try_into().unwrap();

            if chunk_1.eq(&chunk_2) {
                clues.push((i, j));
            }
        }
    }

    if clues.is_empty() {
        None
    } else {
        Some(clues)
    }
}

pub fn detect_ecb_probability(data: Vec<u8>) -> f32 {
    let mut probability: f32 = 0.0;

    for i in (0..data.len() - 16).step_by(16) {
        let chunk_1: [u8; 16] = data[i..i + 16].try_into().unwrap();

        let chunk_2: [u8; 16] = data[i + 16..i + 32].try_into().unwrap();

        // // TODO
        // // compare the byte-sequence, not the block itself
        // // calculate the similarity between the two chunks

        let mut similarity: f32 = 0.0;
        for k in 0..12 {
            let word_1: Vec<u8> = chunk_1[k..k + 4].to_vec();
            let word_2: Vec<u8> = chunk_2[k..k + 4].to_vec();

            let res = fixed_xor(word_1, word_2).unwrap();

            let _ = res.into_iter().map(|b| {
                if b == 0x00 {
                    similarity += 1.0;
                }
            });

            // if chunk_1[k..k + 4] == chunk_2[k..k + 4] {
            //     similarity += 1.0;
            // }
        }
        // similarity.div_assign(16.0);

        probability += similarity;
    }

    println!("prob: {:#?}", probability);
    probability
}

lazy_static! {
    pub static ref RANDOM_KEY: [u8; 16] = {
        use crate::crypto::aes;

        let key = aes::get_random_aes_key();

        key
    };
}

pub fn encrypt_ecb_with_unknown_key(data: &mut Vec<u8>) -> Result<Vec<u8>, CryptoError> {
    pkcs7(data, BLOCK_SIZE);

    let key = RANDOM_KEY.clone();

    let res = encrypt_ecb(key.to_vec(), data)?;

    Ok(res)
}
