use std::fs;

use rand::Rng;

use crate::{
    crypto::aes::{decrypt_cbc, encrypt_cbc, get_random_aes_iv, get_random_aes_key, BLOCK_SIZE},
    padding::pkcs7::verify_pkcs7,
};

use super::{Ciphertext, Key, OracleError, Plaintext, IV};

pub struct Oracle17 {
    key: Key,
    // pub key: Key,
    pub iv: IV,
    pub ciphertext: Ciphertext,
}

#[derive(Debug)]
pub enum OracleResultStatus {
    OK,
    ERR,
}

pub struct OracleResult {
    pub status: OracleResultStatus,
}

impl Oracle17 {
    pub fn new() -> Result<Self, OracleError> {
        let key = get_random_aes_key();

        let iv = get_random_aes_iv();

        let data = fs::read_to_string("../../data/17.txt").unwrap();

        let data: Vec<String> = data
            .split("\n")
            .into_iter()
            .map(|s| s.to_string())
            .collect();

        let random_idx = rand::thread_rng().gen_range(0..data.len() - 1);

        let mut flag = data[random_idx].clone().as_bytes().to_vec();

        // let mut flag = "YELLOW SUBMARINE".as_bytes().to_vec();

        let ciphertext = encrypt_cbc(key.to_vec(), flag, iv)?;

        let oracle = Oracle17 {
            key,
            iv,
            ciphertext,
        };

        Ok(oracle)
    }

    pub fn verify_padding(
        &self,
        input_data: Ciphertext,
        iv: IV,
    ) -> Result<OracleResult, OracleError> {
        let mut maybe_pt: Plaintext = decrypt_cbc(self.key.to_vec(), input_data, iv)?;

        match verify_pkcs7(&mut maybe_pt, BLOCK_SIZE) {
            true => {
                // println!("[222] maybe_pt: {:02x?}", maybe_pt);
                return Ok(OracleResult {
                    status: OracleResultStatus::OK,
                });
            }
            false => {
                return Ok(OracleResult {
                    status: OracleResultStatus::ERR,
                });
            }
        };
    }

    pub fn solve(&self, answer: Vec<u8>) -> bool {
        let pt = decrypt_cbc(self.key.to_vec(), self.ciphertext.clone(), self.iv).unwrap();

        if pt == answer {
            return true;
        } else {
            return false;
        }
    }
}
