mod core;
mod cbc;
mod ctr;
mod ecb;

pub use cbc::*;
pub use ctr::*;
pub use ecb::*;

use rand::Rng;

use super::CryptoError;

pub const BLOCK_SIZE: usize = 16;

pub fn into_aes_blocks(data: Vec<u8>) -> Result<Vec<[u8; 16]>, CryptoError> {
    if data.len() % 16 != 0 {
        return Err(format!(
            "AES block size should be '16', found data size: {}",
            data.len()
        )
        .into());
    } else {
        let blocks: Vec<[u8; BLOCK_SIZE]> = data
            .iter()
            .enumerate()
            .step_by(BLOCK_SIZE)
            .map(|(i, _)| {
                let mut tmp = [0u8; 16];
                for j in 0..BLOCK_SIZE {
                    tmp[j] = data[i + j];
                }

                tmp
            })
            .collect();

        Ok(blocks)
    }
}
pub fn from_u64_little_endian(data: u64) -> Result<[u8; 8], CryptoError> {
    let mut res = [0u8; 8];

    let mut a = data;

    for i in 0..8 {
        res[i] = a as u8 & 0xff;

        a >>= 8;
    }

    Ok(res)
}

pub fn from_u64(data: u64) -> Result<[u8; 8], CryptoError> {
    let mut res = [0u8; 8];

    let mut a = data;

    for i in (0..8).rev() {
        res[i] = a as u8 & 0xff;

        a >>= 8;
    }

    Ok(res)
}

pub fn get_random_data(len: u32) -> Vec<u8> {
    let mut data: Vec<u8> = Vec::new();

    for _ in 0..len {
        data.push(rand::random());
    }

    data
}

pub fn get_random_readable_data(len: u32) -> Vec<u8> {
    let mut data: Vec<u8> = Vec::new();

    for _ in 0..len {
        data.push(rand::thread_rng().gen_range(33..=125));
    }

    data
}
fn generate_random_16_bytes() -> [u8; 16] {
    let res: [u8; 16] = {
        let mut res: Vec<u8> = Vec::new();
        for _ in 0..16 {
            res.push(rand::random());
        }

        res.try_into().unwrap()
    };

    res
}

pub fn get_random_aes_key() -> [u8; 16] {
    generate_random_16_bytes()
}

pub fn get_random_aes_iv() -> [u8; 16] {
    generate_random_16_bytes()
}

pub fn get_dummy_data() -> Vec<u8> {
    "\
    This is a dummy data.This is a dummy data.\
    This is a dummy data.This is a dummy data.\
    This is a dummy data.This is a dummy data.\
    This is a dummy data.This is a dummy data.\
    This is a dummy data.This is a dummy data.\
    This is a dummy data.This is a dummy data."
        .as_bytes()
        .to_vec()
}
