use crate::crypto::aes::BLOCK_SIZE;
use std::error::Error;

mod ch14;
mod ch16;
mod ch17;

pub use ch14::*;
pub use ch16::*;
pub use ch17::*;

pub type Plaintext = Vec<u8>;

pub type Ciphertext = Vec<u8>;

pub type IV = [u8; BLOCK_SIZE];

pub type Key = [u8; BLOCK_SIZE];

pub type Block = [u8; BLOCK_SIZE];

pub type OracleError = Box<dyn Error>;

pub fn into_block(v: Vec<u8>) -> Result<Block, OracleError> {
    if v.len() != 16 {
        return Err(format!("block len should be 16").into());
    }

    let mut res: [u8; BLOCK_SIZE] = [0u8; BLOCK_SIZE];

    for i in 0..16 {
        res[i] = v[i];
    }

    Ok(res)
}
