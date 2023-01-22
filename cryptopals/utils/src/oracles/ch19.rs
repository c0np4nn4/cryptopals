use crate::{
    crypto::aes::{encrypt_ctr, get_random_aes_key, BLOCK_SIZE},
    padding::pkcs7::pkcs7,
};

use super::{Key, OracleError};

pub struct Oracle19 {
    key: Key,
    nonce: [u8; 8],
}

impl Oracle19 {
    pub fn new() -> Self {
        let key = get_random_aes_key();
        let nonce = [0u8; 8];

        Oracle19 { key, nonce }
    }

    pub fn encrypt(&self, data: Vec<u8>) -> Result<Vec<u8>, OracleError> {
        let mut pt = data.clone();

        if data.len() % 16 != 0 {
            pt = pkcs7(&data, BLOCK_SIZE)?;
        }

        encrypt_ctr(self.key.to_vec(), pt, self.nonce.clone())
    }
}
