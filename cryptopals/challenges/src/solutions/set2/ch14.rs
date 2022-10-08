use aes::{
    cipher::{generic_array::GenericArray, Key, KeyInit},
    Aes128,
};
use utils::crypto::aes::{decrypt_ecb, encrypt_ecb, get_random_aes_key};

struct Oracle {
    key: [u8; 16],
    cipher: Aes128,
}

impl Oracle {
    fn new() -> Self {
        let key: [u8; 16] = get_random_aes_key();

        let cipher = Aes128::new(&GenericArray::from_slice(&key));

        Oracle { key, cipher }
    }

    // input_data == my_prefix + plaintext
    fn encrypt(&self, mut input_data: Vec<u8>) -> Vec<u8> {
        // let random_prefix = make_random_prefix();
        // prepare the data, (random_prefix + input_data)
        encrypt_ecb(self.key.to_vec(), &mut input_data)
    }

    // input_data == my_prefix + ciphertext
    fn decrypt(&self, input_data: Vec<u8>) {}
}

#[test]
fn chal_14() {
    // 1. create an oracle
    let oracle = Oracle::new();

    // 2. prepare the ciphertext
    let plaintext = "example".as_bytes().to_vec();
    let ciphertext = oracle.encrypt(plaintext.clone());

    // 3. break it
    // loop {}
}
