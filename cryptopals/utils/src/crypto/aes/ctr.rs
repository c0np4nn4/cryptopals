use super::{from_u64, into_aes_blocks};
use crate::{crypto::aes::from_u64_little_endian, xor::fixed_xor, BoxedError};
use aes::{
    cipher::{generic_array::GenericArray, BlockEncrypt, KeyInit},
    Aes128,
};

pub fn decrypt_ctr(key: Vec<u8>, data: Vec<u8>, nonce: [u8; 8]) -> Result<Vec<u8>, BoxedError> {
    encrypt_ctr(key, data, nonce)
}

pub fn encrypt_ctr(key: Vec<u8>, data: Vec<u8>, nonce: [u8; 8]) -> Result<Vec<u8>, BoxedError> {
    let mut res = Vec::<u8>::new();

    let key = GenericArray::from_slice(&key);

    let cipher = Aes128::new(&key);

    let blocks = into_aes_blocks(data)?;

    for counter in 0..blocks.len() {
        // let ctr = from_u64(counter as u64)?;
        let ctr = from_u64_little_endian(counter as u64)?;

        let mut nonce_vec = nonce.to_vec();

        let mut ctr_vec = ctr.to_vec();

        nonce_vec.append(&mut ctr_vec);

        let ctr: [u8; 16] = nonce_vec[0..16].try_into().unwrap();

        let mut ctr = GenericArray::from_slice(&ctr).clone();

        cipher.encrypt_block(&mut ctr);

        res.append(&mut fixed_xor(ctr.to_vec(), blocks[counter].to_vec())?);
    }

    Ok(res)
}
