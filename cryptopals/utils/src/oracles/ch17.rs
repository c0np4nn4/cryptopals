use crate::{
    crypto::aes::{decrypt_cbc, encrypt_cbc, get_random_aes_iv, get_random_aes_key, BLOCK_SIZE},
    padding::pkcs7::trim_pkcs7,
};

use super::{Ciphertext, Key, OracleError, Plaintext, IV};

pub struct Oracle17 {
    key: Key,
    iv: IV,
}

impl Oracle17 {
    pub fn new() -> Self {
        let key = get_random_aes_key();

        let iv = get_random_aes_iv();

        Oracle17 { key, iv }
    }

    pub fn encrypt(&self, mut input_data: Plaintext) -> (Ciphertext, IV) {
        (
            encrypt_cbc(self.key.to_vec(), &mut input_data, self.iv),
            //
            self.iv.clone(),
        )
    }

    pub fn decrypt(&self, input_data: Ciphertext, iv: IV) -> Result<(), OracleError> {
        let mut maybe_pt: Plaintext = decrypt_cbc(self.key.to_vec(), input_data, self.iv)?;

        // println!("maybe_pt: {:02x?}", maybe_pt);
        println!("maybe_pt: {:?}", String::from_utf8(maybe_pt.clone()));

        trim_pkcs7(&mut maybe_pt, BLOCK_SIZE)?;

        Ok(())
    }
}
