use crate::{
    crypto::aes_core::{
        state::{get_columns, State},
        AES,
    },
    BoxedError,
};

const INV_MIX_COLUMNS_MATRIX: [u8; 16] = [
    0x0E, 0x0B, 0x0D, 0x09,
    0x09, 0x0E, 0x0B, 0x0D,
    0x0D, 0x09, 0x0E, 0x0B,
    0x0B, 0x0D, 0x09, 0x0E,
];

impl AES {
    pub(crate) fn inv_mix_columns(&self, prev_state: State) -> Result<State, BoxedError> {
        let state_columns = get_columns(prev_state);

        let mut res: State = State::default();

        for row_idx in 0..4 {
            let mut tmp_column: Vec<u8> = Vec::default();

            for col_idx in 0..4 {
                let term_1 = self.xtime(
                    INV_MIX_COLUMNS_MATRIX[row_idx * 4 + 0],
                    state_columns[col_idx][0],
                )?;

                let term_2 = self.xtime(
                    INV_MIX_COLUMNS_MATRIX[row_idx * 4 + 1],
                    state_columns[col_idx][1],
                )?;

                let term_3 = self.xtime(
                    INV_MIX_COLUMNS_MATRIX[row_idx * 4 + 2],
                    state_columns[col_idx][2],
                )?;

                let term_4 = self.xtime(
                    INV_MIX_COLUMNS_MATRIX[row_idx * 4 + 3],
                    state_columns[col_idx][3],
                )?;

                let tmp = self.addition(term_1, term_2)?;
                let tmp = self.addition(tmp, term_3)?;
                let tmp = self.addition(tmp, term_4)?;

                tmp_column.push(tmp);
            }

            res[0x0 + row_idx] = tmp_column[0];
            res[0x4 + row_idx] = tmp_column[1];
            res[0x8 + row_idx] = tmp_column[2];
            res[0xc + row_idx] = tmp_column[3];
        }

        Ok(res)
    }

}
