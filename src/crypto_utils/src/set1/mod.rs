mod base64;

#[cfg(test)]
mod tests;

pub use base64::*;

pub type BoxedError = Box<dyn std::error::Error + Send + Sync>;
