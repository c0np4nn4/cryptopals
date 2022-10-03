use std::convert::TryInto;
use std::ops::{AddAssign, DivAssign};

use aes::cipher::{generic_array::GenericArray, KeyInit};
use aes::cipher::{BlockDecrypt, BlockEncrypt};
use aes::Aes128;

use crate::padding::pkcs7::pkcs7;

use super::BLOCK_SIZE;

pub fn encrypt_ecb(key: Vec<u8>, data: &mut Vec<u8>) -> Vec<u8> {
    pkcs7(data, BLOCK_SIZE);

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
}

pub fn decrypt_ecb(key: Vec<u8>, data: Vec<u8>) -> Vec<u8> {
    let key = GenericArray::from_slice(&key);

    let cipher = Aes128::new(&key);

    let mut res = Vec::new();

    for idx in (0..data.len()).step_by(16) {
        let block: [u8; 16] = data[idx..idx + 16].try_into().unwrap();

        let mut block = GenericArray::from_slice(&block).clone();

        cipher.decrypt_block(&mut block);

        res.append(&mut block.to_vec());
    }

    res
}

// pub fn detect_ecb(data: Vec<Vec<u8>>) -> Vec<Vec<(usize, usize)>> {
//     let mut result_table: Vec<Vec<(usize, usize)>> = vec![vec![]];

//     for candidate_idx in 0..data.len() {
//         let candidate_data = data[candidate_idx].clone();

//         let mut result_at_candidate_idx: Vec<(usize, usize)> = vec![];
//         for i in (0..candidate_data.len()).step_by(16) {
//             for j in (i + 16..candidate_data.len()).step_by(16) {
//                 let chunk_1: [u8; 16] =
//                     candidate_data[i..i + 16].try_into().unwrap();

//                 let chunk_2: [u8; 16] =
//                     candidate_data[j..j + 16].try_into().unwrap();

//                 if chunk_1.eq(&chunk_2) {
//                     result_at_candidate_idx.push((i, j));
//                 }
//             }
//         }
//         result_table.push(result_at_candidate_idx);
//     }

//     result_table
// }

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

    for i in (0..data.len()).step_by(16) {
        for j in (i + 16..data.len()).step_by(16) {
            let chunk_1: [u8; 16] = data[i..i + 16].try_into().unwrap();

            let chunk_2: [u8; 16] = data[j..j + 16].try_into().unwrap();

            // // TODO
            // // compare the byte-sequence, not the block itself
            // // calculate the similarity between the two chunks

            let mut similarity: f32 = 0.0;
            for k in 0..12 {
                if chunk_1[k..k + 4] == chunk_2[k..k + 4] {
                    similarity += 1.0;
                }
            }
            similarity.div_assign(16.0);

            probability.add_assign(similarity);
        }
    }

    probability
}
