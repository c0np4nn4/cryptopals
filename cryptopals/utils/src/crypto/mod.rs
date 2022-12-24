use std::error::Error;

pub mod aes;
pub mod aes_core;
pub mod mt19937;

pub type CryptoError= Box<dyn Error>;
