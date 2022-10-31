use crate::{
    crypto::aes::{encrypt_ctr, get_random_aes_key, BLOCK_SIZE},
    padding::pkcs7::pkcs7,
};

use super::Key;

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

    pub fn encrypt(&self, data: Vec<u8>) -> Vec<u8> {
        let mut data = data;

        pkcs7(&mut data, BLOCK_SIZE);

        encrypt_ctr(self.key.to_vec(), data, self.nonce.clone()).unwrap()
    }
}
