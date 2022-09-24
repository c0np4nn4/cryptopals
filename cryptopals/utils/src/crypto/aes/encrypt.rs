use crate::BoxedError;

use super::{state::State, AES, SUB_BYTES_TABLE};

const MIX_COLUMNS_MATRIX: [u8; 16] = [
    0x02, 0x03, 0x01, 0x01, //
    0x01, 0x02, 0x03, 0x01, //
    0x01, 0x01, 0x02, 0x03, //
    0x03, 0x01, 0x01, 0x02, //
];

impl AES {
    pub(crate) fn sub_bytes(&self, prev_state: State) -> State {
        let mut next_state = State::default();

        for idx in 0..16 {
            next_state.state[idx] =
                SUB_BYTES_TABLE[prev_state.state[idx] as usize];
        }

        next_state
    }

    pub(crate) fn shift_rows(&self, prev_state: State) -> State {
        let mut next_state = State::default();

        // col 1
        next_state.state[0x0] = prev_state.state[0x0];
        next_state.state[0x1] = prev_state.state[0x5];
        next_state.state[0x2] = prev_state.state[0xa];
        next_state.state[0x3] = prev_state.state[0xf];

        // col 2
        next_state.state[0x4] = prev_state.state[0x4];
        next_state.state[0x5] = prev_state.state[0x9];
        next_state.state[0x6] = prev_state.state[0xe];
        next_state.state[0x7] = prev_state.state[0x3];

        // col 3
        next_state.state[0x8] = prev_state.state[0x8];
        next_state.state[0x9] = prev_state.state[0xd];
        next_state.state[0xa] = prev_state.state[0x2];
        next_state.state[0xb] = prev_state.state[0x7];

        // col 4
        next_state.state[0xc] = prev_state.state[0xc];
        next_state.state[0xd] = prev_state.state[0x1];
        next_state.state[0xe] = prev_state.state[0x6];
        next_state.state[0xf] = prev_state.state[0xb];

        next_state
    }

    pub(crate) fn mix_columns(
        &self,
        prev_state: State,
    ) -> Result<State, BoxedError> {
        let state_columns = prev_state.get_columns();

        let mut res: State = State::default();

        for row_idx in 0..4 {
            let mut tmp_column: Vec<u8> = Vec::default();

            for col_idx in 0..4 {
                let term_1 = self.xtime(
                    MIX_COLUMNS_MATRIX[row_idx * 4 + 0],
                    state_columns[col_idx][0],
                )?;

                let term_2 = self.xtime(
                    MIX_COLUMNS_MATRIX[row_idx * 4 + 1],
                    state_columns[col_idx][1],
                )?;

                let term_3 = self.xtime(
                    MIX_COLUMNS_MATRIX[row_idx * 4 + 2],
                    state_columns[col_idx][2],
                )?;

                let term_4 = self.xtime(
                    MIX_COLUMNS_MATRIX[row_idx * 4 + 3],
                    state_columns[col_idx][3],
                )?;

                let tmp = self.addition(term_1, term_2)?;
                let tmp = self.addition(tmp, term_3)?;
                let tmp = self.addition(tmp, term_4)?;

                tmp_column.push(tmp);
            }

            // res.state[0x0 + 4 * row_idx] = tmp_column[0];
            // res.state[0x1 + 4 * row_idx] = tmp_column[1];
            // res.state[0x2 + 4 * row_idx] = tmp_column[2];
            // res.state[0x3 + 4 * row_idx] = tmp_column[3];

            res.state[0x0 + row_idx] = tmp_column[0];
            res.state[0x4 + row_idx] = tmp_column[1];
            res.state[0x8 + row_idx] = tmp_column[2];
            res.state[0xc + row_idx] = tmp_column[3];
        }

        // Ok(State::from_vec(rows)?)
        Ok(res)
    }
}
