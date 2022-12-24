use super::into_aes_blocks;
use crate::{crypto::{aes::from_u64_little_endian, CryptoError, aes_core::{key::from_vec, AES}}, xor::fixed_xor};

pub fn decrypt_ctr(key: Vec<u8>, data: Vec<u8>, nonce: [u8; 8]) -> Result<Vec<u8>, CryptoError> {
    encrypt_ctr(key, data, nonce)
}

pub fn encrypt_ctr(key: Vec<u8>, data: Vec<u8>, nonce: [u8; 8]) -> Result<Vec<u8>, CryptoError> {
    let mut res = Vec::<u8>::new();

    let cipher = AES::new(from_vec(key)?);

    let blocks = into_aes_blocks(data)?;

    for counter in 0..blocks.len() {
        // let ctr = from_u64(counter as u64)?;
        let ctr = from_u64_little_endian(counter as u64)?;

        let mut nonce_vec = nonce.to_vec();

        let mut ctr_vec = ctr.to_vec();

        nonce_vec.append(&mut ctr_vec);

        let ctr: [u8; 16] = nonce_vec[0..16].try_into().unwrap();

        let ctr = cipher.encrypt(ctr.to_vec())?;

        res.append(&mut fixed_xor(ctr, blocks[counter].to_vec())?);
    }

    Ok(res)
}
