// use crate::BoxedError;

// use super::types::State;

// pub fn get_columns(state: State) -> Vec<Vec<u8>> {
//     let mut res = vec![];

//     for idx in 0..4 {
//         let mut tmp = vec![];

//         tmp.push(state[idx * 4 + 0]);
//         tmp.push(state[idx * 4 + 1]);
//         tmp.push(state[idx * 4 + 2]);
//         tmp.push(state[idx * 4 + 3]);

//         res.push(tmp);
//     }

//     res
// }

// pub fn from_vec(state: Vec<u8>) -> Result<State, BoxedError> {
//     if !state.len().eq(&16) {
//         return Err(
//             format!("The length of the input vector should be 16").into()
//         );
//     }

//     let mut res: [u8; 16] = [0; 16];

//     for idx in 0..16 {
//         res[idx] = state[idx].clone();
//     }

//     Ok(res)
// }
