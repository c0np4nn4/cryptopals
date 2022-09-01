pub mod attack;
pub mod base64;
pub mod casting;
pub mod crypto;
pub mod ham_dist;
pub mod xor;

pub type BoxedError = Box<dyn std::error::Error + Send + Sync>;
