use aes::{
    cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit},
    Aes128,
};

use crate::{padding::pkcs7::pkcs7, xor::fixed_xor, BoxedError};

use super::{into_aes_blocks, BLOCK_SIZE};

pub fn encrypt_cbc(key: Vec<u8>, data: &mut Vec<u8>, iv: [u8; BLOCK_SIZE]) -> Vec<u8> {
    pkcs7(data, BLOCK_SIZE);

    let key = GenericArray::from_slice(&key);

    let cipher = Aes128::new(&key);

    let mut previous_block = iv;

    let mut res = Vec::new();

    for idx in (0..data.len()).step_by(BLOCK_SIZE) {
        let block: [u8; BLOCK_SIZE] = data[idx..idx + BLOCK_SIZE].try_into().unwrap();

        let block: [u8; BLOCK_SIZE] = fixed_xor(block.to_vec(), previous_block.to_vec())
            .unwrap()
            .try_into()
            .unwrap();

        let mut block = GenericArray::from_slice(&block).clone();

        cipher.encrypt_block(&mut block);

        res.append(&mut block.to_vec());

        previous_block = block.try_into().unwrap();
    }

    res
}

pub fn decrypt_cbc(
    key: Vec<u8>,
    data: Vec<u8>,
    iv: [u8; BLOCK_SIZE],
) -> Result<Vec<u8>, BoxedError> {
    let key = GenericArray::from_slice(&key);

    let cipher = Aes128::new(&key);

    let blocks = into_aes_blocks(data)?;

    let mut res = Vec::<u8>::new();

    // blocks[0]
    {
        let mut target_block = GenericArray::from(blocks.clone()[0]);

        cipher.decrypt_block(&mut target_block);

        let mut res_block = fixed_xor(target_block.to_vec(), iv.to_vec())?;

        res.append(&mut res_block);
    }

    // blocks[1..]
    for i in 1..blocks.len() {
        let mut target_block = GenericArray::from(blocks.clone()[i]);

        cipher.decrypt_block(&mut target_block);

        let mut res_block = fixed_xor(target_block.to_vec(), blocks[i - 1].clone().to_vec())?;

        res.append(&mut res_block);
    }

    Ok(res)
}
