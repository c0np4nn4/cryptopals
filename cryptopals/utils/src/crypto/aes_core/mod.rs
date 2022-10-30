mod decrypt;
mod encrypt;
mod key;
pub mod state;
#[cfg(test)]
mod test;

use self::{key::RoundKey, state::State};
use crate::{padding::pkcs7::pkcs7, xor::fixed_xor, BoxedError};

const BLOCK_SIZE: usize = 16;

pub struct AES {
    pub key: [u8; 16],
}

impl AES {
    pub fn new(key: [u8; 16]) -> AES {
        AES { key }
    }

    fn add_round_key(&self, prev_state: State, round_key: State) -> Result<State, BoxedError> {
        let next_state = fixed_xor(prev_state.to_vec(), round_key.to_vec())?;

        Ok(state::from_vec(next_state)?)
    }

    /// encryption for `only one` block
    pub fn encrypt(&self, data: Vec<u8>) -> Result<Vec<u8>, BoxedError> {
        let mut res: Vec<u8> = vec![];

        if data.len() > BLOCK_SIZE {
            return Err(format!("cannot encrypt data, len: {:?} > 16", data.len()).into());
        }

        let mut data = data;

        pkcs7(&mut data, BLOCK_SIZE);

        let mut state = state::from_vec(data)?;

        let ext_key: [RoundKey; 11] = self.key_expansion()?;

        // round
        // round 0
        state = self.add_round_key(state, ext_key[0])?;

        // round 1 ~ 9
        for key_idx in 1..10 {
            // byte sub
            state = self.sub_bytes(state);

            // shift rows
            state = self.shift_rows(state);

            // mix columns
            state = self.mix_columns(state)?;

            // add round key
            state = self.add_round_key(state, ext_key[key_idx])?;
        }

        // round 10
        // byte sub
        state = self.sub_bytes(state);

        // shift rows
        state = self.shift_rows(state);

        // add round key
        state = self.add_round_key(state, ext_key[10])?;

        res.append(&mut state.to_vec());

        Ok(res)
    }

    // pub fn decrypt(&self, key: [u8; 16]) -> Vec<u8> {
    //     vec![]
    // }
}
