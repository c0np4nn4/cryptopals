use rand::Rng;

use crate::crypto::aes::{self, encrypt_ecb, get_random_aes_key, get_random_data};

use super::OracleError;

pub struct Oracle14 {
    key: [u8; 16],
    random_prefix: Vec<u8>,
}

impl Oracle14 {
    pub fn new() -> Self {
        let key: [u8; 16] = get_random_aes_key();

        let prefix_size = rand::thread_rng().gen_range(0..32);

        let random_prefix = get_random_data(prefix_size);

        Oracle14 { key, random_prefix }
    }

    pub fn encrypt(&mut self, mut input_data: Vec<u8>) -> Result<Vec<u8>, OracleError> {
        // prefix + input + target
        let mut msg = aes::get_dummy_data();

        // let mut msg = "Hello World!".as_bytes().to_vec();

        let mut prefix = self.random_prefix.clone();

        prefix.append(&mut input_data);
        prefix.append(&mut msg);

        let mut data = prefix;

        encrypt_ecb(self.key.to_vec(), &mut data)
    }
}
