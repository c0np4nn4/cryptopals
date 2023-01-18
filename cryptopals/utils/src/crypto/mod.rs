use std::error::Error;

pub mod aes;
pub mod mt19937;

pub mod stream_cipher;

pub type CryptoError= Box<dyn Error>;

pub trait PRNG {
    fn extract(&mut self) -> u32; 
}
