mod mix_columns;
mod shift_rows;
mod sub_bytes;

pub use mix_columns::*;
pub use shift_rows::*;
pub use sub_bytes::*;

use super::{key::RoundKey, state, AES, BLOCK_SIZE};
use crate::{padding::pkcs7::pkcs7, BoxedError};

impl AES {
    pub fn encrypt(&self, data: Vec<u8>) -> Result<Vec<u8>, BoxedError> {
        let mut res: Vec<u8> = vec![];

        if data.len() > BLOCK_SIZE {
            return Err(format!("cannot encrypt data, len: {:?} > 16", data.len()).into());
        }

        let mut data = data;

        if data.len() < 16 {
            pkcs7(&mut data, BLOCK_SIZE);
        }

        let mut state = state::from_vec(data)?;

        let ext_key: [RoundKey; 11] = self.key_expansion()?;

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
}
