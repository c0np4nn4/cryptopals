use crate::crypto::{aes::core::{AES, state::{State, get_columns}}, CryptoError};

const MIX_COLUMNS_MATRIX: [u8; 16] = [
    0x02, 0x03, 0x01, 0x01, //
    0x01, 0x02, 0x03, 0x01, //
    0x01, 0x01, 0x02, 0x03, //
    0x03, 0x01, 0x01, 0x02, //
];

impl AES {
    pub(crate) fn mix_columns(&self, prev_state: State) -> Result<State, CryptoError> {
        let state_columns = get_columns(prev_state);

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

            res[0x0 + row_idx] = tmp_column[0];
            res[0x4 + row_idx] = tmp_column[1];
            res[0x8 + row_idx] = tmp_column[2];
            res[0xc + row_idx] = tmp_column[3];
        }

        Ok(res)
    }

    pub fn xtime(&self, x: u8, byte: u8) -> Result<u8, CryptoError> {
        let res = match x {
            // enc
            1 => self.xtime_1(byte)?,
            2 => self.xtime_2(byte)?,
            3 => self.xtime_3(byte)?,
            // dec
            9 => self.xtime_9(byte)?,
            0xB => self.xtime_b(byte)?,
            0xD => self.xtime_d(byte)?,
            0xE => self.xtime_e(byte)?,
            //
            _ => {
                return Err(format!("Not implemented yet").into());
            }
        };

        Ok(res)
    }

    #[inline]
    fn xtime_1(&self, byte: u8) -> Result<u8, CryptoError> {
        Ok(byte)
    }

    #[inline]
    fn xtime_2(&self, byte: u8) -> Result<u8, CryptoError> {
        let mut res = byte;

        if res & 0x80 == 0x80 {
            res = 0x1b ^ (res << 1);
        } else {
            res = res << 1;
        }

        Ok(res)
    }

    #[inline]
    fn xtime_3(&self, byte: u8) -> Result<u8, CryptoError> {
        let res = self.addition(self.xtime_2(byte)?, self.xtime_1(byte)?)?;

        Ok(res)
    }

    #[inline]
    fn xtime_4(&self, byte: u8) -> Result<u8, CryptoError> {
        let val = self.xtime_2(byte)?;

        let res = self.xtime_2(val)?;

        Ok(res)
    }


    pub fn addition(&self, l: u8, r: u8) -> Result<u8, CryptoError> {
        Ok(l ^ r)
    }

    #[inline]
    fn xtime_8(&self, byte:u8) -> Result<u8, CryptoError> {
        let val_1 = self.xtime_2(byte)?;

        let val_2 = self.xtime_2(val_1)?;

        let val_3 = self.xtime_2(val_2)?;

        Ok(val_3)
    }

    #[inline]
    fn xtime_9(&self, byte: u8) -> Result<u8, CryptoError> {
        let val_1 = self.xtime_8(byte)?;

        let res = self.addition(val_1, byte)?;

        Ok(res)
    }

    #[inline]
    fn xtime_b(&self, byte: u8) -> Result<u8, CryptoError> {
        let val_1 = self.xtime_8(byte)?;

        let val_2 = self.xtime_2(byte)?;

        let val_3 = self.addition(val_1, val_2)?;

        let res = self.addition(val_3, byte)?;

        Ok(res)
    }

    #[inline]
    fn xtime_d(&self, byte: u8) -> Result<u8, CryptoError> {
        let val_1 = self.xtime_8(byte)?;

        let val_2 = self.xtime_4(byte)?;

        let val_3 = self.addition(val_1, val_2)?;

        let res = self.addition(val_3, byte)?;

        Ok(res)
    }

    #[inline]
    fn xtime_e(&self, byte: u8) -> Result<u8, CryptoError> {
        let val_1 = self.xtime_8(byte)?;

        let val_2 = self.xtime_4(byte)?;

        let val_3 = self.addition(val_1, val_2)?;

        let res = self.addition(val_3, self.xtime_2(byte)?)?;

        Ok(res)
    }
}
