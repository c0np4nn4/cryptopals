use crate::BoxedError;

pub struct State {
    pub state: [u8; 16],
}

impl State {
    pub fn default() -> State {
        State { state: [0; 16] }
    }

    pub fn new(state: [u8; 16]) -> State {
        State { state }
    }

    pub fn get_columns(&self) -> Vec<Vec<u8>> {
        let mut res = vec![];

        for idx in 0..4 {
            let mut tmp = vec![];

            tmp.push(self.state[idx * 4 + 0]);
            tmp.push(self.state[idx * 4 + 1]);
            tmp.push(self.state[idx * 4 + 2]);
            tmp.push(self.state[idx * 4 + 3]);

            res.push(tmp);
        }

        res
    }

    // pub fn get_rows(&self) -> Vec<[u8; 4]> {
    //     let mut res = Vec::<[u8; 4]>::new();

    //     let mut tmp = [0u8; 4];

    //     for idx in 0..4 {
    //         tmp[0] = self.state[0x0 + idx];
    //         tmp[1] = self.state[0x4 + idx];
    //         tmp[2] = self.state[0x8 + idx];
    //         tmp[3] = self.state[0xc + idx];

    //         res.push(tmp);
    //     }

    //     println!("[!!] rows: {:#?}", res);

    //     res
    // }

    pub fn from_vec(state: Vec<u8>) -> Result<State, BoxedError> {
        let mut res: [u8; 16] = [0; 16];

        for idx in 0..16 {
            res[idx] = state[idx].clone();
        }

        Ok(State { state: res })
    }

    pub fn from_row_vec(rows: Vec<Vec<u8>>) -> Result<State, BoxedError> {
        if rows.len() != 4 {
            return Err(format!("Failed to parse Vec<u8> to state").into());
        } else {
            let mut res: [u8; 16] = [0; 16];

            for i in 0..4 {
                for j in 0..4 {
                    res[i * 4 + j] = rows[i][j];
                }
            }

            Ok(State { state: res })
        }
    }

    pub fn into_vec(&self) -> Result<Vec<u8>, BoxedError> {
        let res: Vec<u8> = self.state.into_iter().collect();

        Ok(res)
    }
}
