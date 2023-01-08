use crate::crypto::{aes_core::{state::State, AES}, CryptoError};

impl AES {
    pub(crate) fn shift_rows(&self, prev_state: State) -> Result<State, CryptoError> {
        let mut next_state = State::default();

        // col 1
        next_state[0x0] = prev_state[0x0];
        next_state[0x1] = prev_state[0x5];
        next_state[0x2] = prev_state[0xa];
        next_state[0x3] = prev_state[0xf];

        // col 2
        next_state[0x4] = prev_state[0x4];
        next_state[0x5] = prev_state[0x9];
        next_state[0x6] = prev_state[0xe];
        next_state[0x7] = prev_state[0x3];

        // col 3
        next_state[0x8] = prev_state[0x8];
        next_state[0x9] = prev_state[0xd];
        next_state[0xa] = prev_state[0x2];
        next_state[0xb] = prev_state[0x7];

        // col 4
        next_state[0xc] = prev_state[0xc];
        next_state[0xd] = prev_state[0x1];
        next_state[0xe] = prev_state[0x6];
        next_state[0xf] = prev_state[0xb];

        Ok(next_state)
    }
}
