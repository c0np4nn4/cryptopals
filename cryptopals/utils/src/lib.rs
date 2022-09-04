pub mod attack;
pub mod base64;
pub mod crypto;
pub mod hamming_distance;
pub mod types;
pub mod xor;

pub type BoxedError = Box<dyn std::error::Error>;
