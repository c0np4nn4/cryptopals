pub mod attack;
pub mod base64;
pub mod crypto;
pub mod hamming_distance;
pub mod padding;
pub mod types;
pub mod xor;

use std::error::Error;

pub type BoxedError = Box<dyn Error>;
