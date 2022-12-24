use super::{key::RoundKey, state, AES, BLOCK_SIZE};
use crate::{padding::pkcs7::pkcs7, BoxedError};

pub mod mix_columns;
pub mod shift_rows;
pub mod sub_bytes;

impl AES {
    pub fn encrypt(&self, pt: Vec<u8>) -> Result<Vec<u8>, BoxedError> {
        if pt.len() > BLOCK_SIZE {
            return Err(format!("cannot encrypt data, len: {:?} > 16", pt.len()).into());
        }

        let mut pt = pt;

        // padding
        if pt.len() < 16 {
            pkcs7(&mut pt, BLOCK_SIZE);
        }

        let mut state = state::from_vec(pt)?;

        // key expansion
        let ext_key: [RoundKey; 11] = self.key_expansion()?;

        // round 0
        state = self.add_round_key(state, ext_key[0])?;

        // round 1 ~ 9
        for key_idx in 1..10 {
            // sub bytes
            state = self.sub_bytes(state);

            // shift rows
            state = self.shift_rows(state);

            // mix columns
            state = self.mix_columns(state)?;

            // add round key
            state = self.add_round_key(state, ext_key[key_idx])?;
        }

        // round 10
        // sub bytes
        state = self.sub_bytes(state);

        // shift rows
        state = self.shift_rows(state);

        // add round key
        state = self.add_round_key(state, ext_key[10])?;

        Ok(state.to_vec())
    }
}
