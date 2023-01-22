use crate::{
    padding::pkcs7::pkcs7, 
    xor::fixed_xor, 
    crypto::CryptoError
};

use super::{into_aes_blocks, BLOCK_SIZE, core::{AES, key::from_vec}};

pub fn encrypt_cbc(key: Vec<u8>, data: Vec<u8>, iv: [u8; BLOCK_SIZE]) -> Result<Vec<u8>, CryptoError> {
    let padded_data = pkcs7(&data, BLOCK_SIZE)?;

    let cipher = AES::new(from_vec(key)?);

    let mut previous_block = iv;

    let mut res = Vec::new();

    for idx in (0..padded_data.len()).step_by(BLOCK_SIZE) {
        let block: Vec<u8> = padded_data[idx..idx + BLOCK_SIZE].to_vec();

        let block: Vec<u8> = fixed_xor(block.to_vec(), previous_block.to_vec())?;

        let block: Vec<u8> = cipher.encrypt(block)?;

        res.append(&mut block.to_vec());

        previous_block = block.try_into().unwrap();
    }

    Ok(res)
}

pub fn decrypt_cbc(
    key: Vec<u8>,
    data: Vec<u8>,
    iv: [u8; BLOCK_SIZE],
) -> Result<Vec<u8>, CryptoError> {


    let cipher = AES::new(from_vec(key)?);

    let blocks = into_aes_blocks(data)?;

    let mut res = Vec::<u8>::new();

    // blocks[0]
    {
        // let mut target_block = GenericArray::from(blocks.clone()[0]);

        // cipher.decrypt_block(&mut target_block);

        let target_block = blocks.clone()[0].to_vec();

        let target_block = cipher.decrypt(target_block)?;

        let mut res_block = fixed_xor(target_block, iv.to_vec())?;

        res.append(&mut res_block);
    }

    // blocks[1..]
    for i in 1..blocks.len() {
        let target_block = blocks.clone()[i].to_vec();

        let target_block = cipher.decrypt(target_block)?;

        let mut res_block = fixed_xor(target_block.to_vec(), blocks[i - 1].clone().to_vec())?;

        res.append(&mut res_block);
    }

    Ok(res)
}
