use super::{AES, BLOCK_SIZE, state, key::RoundKey};
use crate::crypto::CryptoError;

pub mod inv_shift_rows;
pub mod inv_sub_bytes;
pub mod inv_mix_columns;

impl AES {
    pub fn decrypt(&self, ct: Vec<u8>) -> Result<Vec<u8>, CryptoError> {

        if ct.len() != BLOCK_SIZE {
            return Err(format!("cannot decrypt data, len: {:?} != 16", ct.len()).into());
        }

        let mut state = state::from_vec(ct)?;

        // key expansion
        let ext_key: [RoundKey; 11] = self.key_expansion()?;

        // round 0
        state = self.add_round_key(state, ext_key[10])?;

        // round 1 ~ 9
        for key_idx in (1..10).rev() {
            // inv shift rows 
            state = self.inv_shift_rows(state)?;

            // inv sub bytes 
            state = self.inv_sub_bytes(state)?;

            // add round key
            state = self.add_round_key(state, ext_key[key_idx])?;

            // inv mix columns
            state = self.inv_mix_columns(state)?;
        }

        // round 10
        // sub bytes
        state = self.inv_shift_rows(state)?;

        // shift rows
        state = self.inv_sub_bytes(state)?;

        // add round key
        state = self.add_round_key(state, ext_key[0])?;

        Ok(state.to_vec())
    }
}
