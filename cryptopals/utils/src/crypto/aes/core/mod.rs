pub(crate) mod decrypt;
pub(crate) mod encrypt;
pub(crate) mod key;
pub mod state;

#[cfg(test)]
mod test;

use self::{key::RoundKey, state::State};
use crate::xor::fixed_xor;
use super::CryptoError;

const BLOCK_SIZE: usize = 16;

pub struct AES {
    pub key: [u8; 16],
}

impl AES {
    pub fn new(key: [u8; 16]) -> AES {
        AES { key }
    }

    fn add_round_key(&self, prev_state: State, round_key: RoundKey) -> Result<State, CryptoError> {
        let next_state = fixed_xor(prev_state.to_vec(), round_key.to_vec())?;

        Ok(state::from_vec(next_state)?)
    }
}
