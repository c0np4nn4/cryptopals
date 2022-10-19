use crate::crypto::aes::{self, decrypt_cbc, encrypt_cbc, get_random_aes_key, BLOCK_SIZE};

pub struct Oracle16 {
    key: [u8; 16],
    prefix: String,
    postfix: String,
}

impl Oracle16 {
    pub fn new() -> Self {
        let key: [u8; 16] = get_random_aes_key();

        let prefix = "comment1=cooking%20MCs;userdata=".to_string();

        let postfix = ";comment2=%20like%20a%20pound%20of%20bacon".to_string();

        Oracle16 {
            key,
            prefix,
            postfix,
        }
    }

    pub fn encrypt(&self, mut input_data: Vec<u8>, iv: [u8; BLOCK_SIZE]) -> Vec<u8> {
        // if input_data.contains(";admin=true;".as_bytes().to_vec()) {
        //     panic!()
        // }
        // check
        {
            let data_tmp = input_data.clone();
            let data_tmp = String::from_utf8_lossy(&data_tmp);

            let data_tmp = data_tmp
                .split(";")
                .into_iter()
                .filter(|value| value.clone().eq(&"admin=true".to_string()))
                .next()
                .is_some();
        }

        let mut prefix = self.prefix.clone().as_bytes().to_vec();
        let mut postfix = self.postfix.clone().as_bytes().to_vec();

        let mut data = Vec::<u8>::new();

        data.append(&mut prefix);

        data.append(&mut input_data);

        data.append(&mut postfix);

        encrypt_cbc(self.key.to_vec(), &mut data, iv)
    }

    pub fn decrypt(&self, input_data: Vec<u8>, iv: [u8; BLOCK_SIZE]) -> Vec<u8> {
        decrypt_cbc(self.key.to_vec(), input_data, iv)
    }
}
