mod base64;
mod xor;

#[cfg(test)]
mod tests;

pub use base64::*;
pub use xor::*;

pub type BoxedError = Box<dyn std::error::Error + Send + Sync>;
