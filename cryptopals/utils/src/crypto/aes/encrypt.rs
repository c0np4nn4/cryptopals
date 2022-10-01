// use crate::BoxedError;

// use super::{state::get_columns, types::State, AES, SUB_BYTES_TABLE};

// const MIX_COLUMNS_MATRIX: [u8; 16] = [
//     0x02, 0x03, 0x01, 0x01, //
//     0x01, 0x02, 0x03, 0x01, //
//     0x01, 0x01, 0x02, 0x03, //
//     0x03, 0x01, 0x01, 0x02, //
// ];

// impl AES {
//     pub(crate) fn sub_bytes(&self, prev_state: State) -> State {
//         let mut next_state = State::default();

//         for idx in 0..16 {
//             next_state[idx] = SUB_BYTES_TABLE[prev_state[idx] as usize];
//         }

//         next_state
//     }

//     pub(crate) fn shift_rows(&self, prev_state: State) -> State {
//         let mut next_state = State::default();

//         // col 1
//         next_state[0x0] = prev_state[0x0];
//         next_state[0x1] = prev_state[0x5];
//         next_state[0x2] = prev_state[0xa];
//         next_state[0x3] = prev_state[0xf];

//         // col 2
//         next_state[0x4] = prev_state[0x4];
//         next_state[0x5] = prev_state[0x9];
//         next_state[0x6] = prev_state[0xe];
//         next_state[0x7] = prev_state[0x3];

//         // col 3
//         next_state[0x8] = prev_state[0x8];
//         next_state[0x9] = prev_state[0xd];
//         next_state[0xa] = prev_state[0x2];
//         next_state[0xb] = prev_state[0x7];

//         // col 4
//         next_state[0xc] = prev_state[0xc];
//         next_state[0xd] = prev_state[0x1];
//         next_state[0xe] = prev_state[0x6];
//         next_state[0xf] = prev_state[0xb];

//         next_state
//     }

//     pub(crate) fn mix_columns(
//         &self,
//         prev_state: State,
//     ) -> Result<State, BoxedError> {
//         let state_columns = get_columns(prev_state);

//         let mut res: State = State::default();

//         for row_idx in 0..4 {
//             let mut tmp_column: Vec<u8> = Vec::default();

//             for col_idx in 0..4 {
//                 let term_1 = self.xtime(
//                     MIX_COLUMNS_MATRIX[row_idx * 4 + 0],
//                     state_columns[col_idx][0],
//                 )?;

//                 let term_2 = self.xtime(
//                     MIX_COLUMNS_MATRIX[row_idx * 4 + 1],
//                     state_columns[col_idx][1],
//                 )?;

//                 let term_3 = self.xtime(
//                     MIX_COLUMNS_MATRIX[row_idx * 4 + 2],
//                     state_columns[col_idx][2],
//                 )?;

//                 let term_4 = self.xtime(
//                     MIX_COLUMNS_MATRIX[row_idx * 4 + 3],
//                     state_columns[col_idx][3],
//                 )?;

//                 let tmp = self.addition(term_1, term_2)?;
//                 let tmp = self.addition(tmp, term_3)?;
//                 let tmp = self.addition(tmp, term_4)?;

//                 tmp_column.push(tmp);
//             }

//             res[0x0 + row_idx] = tmp_column[0];
//             res[0x4 + row_idx] = tmp_column[1];
//             res[0x8 + row_idx] = tmp_column[2];
//             res[0xc + row_idx] = tmp_column[3];
//         }

//         Ok(res)
//     }
// }
