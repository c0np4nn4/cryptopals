use super::{encrypt::SUB_BYTES_TABLE, AES};
use crate::{xor::fixed_xor_word, BoxedError};

pub type RoundKey = [u8; 16];

const RCON: [[u8; 4]; 10] = [
    [0x01, 0x00, 0x00, 0x00],
    [0x02, 0x00, 0x00, 0x00],
    [0x04, 0x00, 0x00, 0x00],
    [0x08, 0x00, 0x00, 0x00],
    [0x10, 0x00, 0x00, 0x00],
    [0x20, 0x00, 0x00, 0x00],
    [0x40, 0x00, 0x00, 0x00],
    [0x80, 0x00, 0x00, 0x00],
    [0x1B, 0x00, 0x00, 0x00],
    [0x36, 0x00, 0x00, 0x00],
];

impl AES {
    pub fn key_expansion(&self) -> Result<[RoundKey; 11], BoxedError> {
        //
        let key = self.key;

        let mut ext_key = [[0u8; 16]; 11];

        // 0, 1, 2, 3
        ext_key[0] = key;

        // 4, 5, 6, 7, ..., 44
        for key_idx in 1..11 {
            for word_idx in 0..4 {
                let mut prev_word: [u8; 4] = [0u8; 4];

                if word_idx == 0 {
                    prev_word[0] = ext_key[key_idx - 1][12];
                    prev_word[1] = ext_key[key_idx - 1][13];
                    prev_word[2] = ext_key[key_idx - 1][14];
                    prev_word[3] = ext_key[key_idx - 1][15];

                    prev_word = rot_word_1_right(prev_word)?;

                    prev_word = sub_word(prev_word)?;

                    prev_word = fixed_xor_word(prev_word, RCON[key_idx - 1])?;

                    let mut tmp = [0u8; 4];
                    tmp[0] = ext_key[key_idx - 1][0];
                    tmp[1] = ext_key[key_idx - 1][1];
                    tmp[2] = ext_key[key_idx - 1][2];
                    tmp[3] = ext_key[key_idx - 1][3];

                    let tmp = fixed_xor_word(prev_word, tmp)?;

                    ext_key[key_idx][0] = tmp[0];
                    ext_key[key_idx][1] = tmp[1];
                    ext_key[key_idx][2] = tmp[2];
                    ext_key[key_idx][3] = tmp[3];
                } else {
                    prev_word[0] = ext_key[key_idx][4 * (word_idx - 1) + 0];
                    prev_word[1] = ext_key[key_idx][4 * (word_idx - 1) + 1];
                    prev_word[2] = ext_key[key_idx][4 * (word_idx - 1) + 2];
                    prev_word[3] = ext_key[key_idx][4 * (word_idx - 1) + 3];

                    let mut tmp = [0u8; 4];
                    tmp[0] = ext_key[key_idx - 1][4 * word_idx + 0];
                    tmp[1] = ext_key[key_idx - 1][4 * word_idx + 1];
                    tmp[2] = ext_key[key_idx - 1][4 * word_idx + 2];
                    tmp[3] = ext_key[key_idx - 1][4 * word_idx + 3];

                    let tmp = fixed_xor_word(prev_word, tmp)?;

                    ext_key[key_idx][4 * word_idx + 0] = tmp[0];
                    ext_key[key_idx][4 * word_idx + 1] = tmp[1];
                    ext_key[key_idx][4 * word_idx + 2] = tmp[2];
                    ext_key[key_idx][4 * word_idx + 3] = tmp[3];
                }
            }
        }

        Ok(ext_key)
    }
}

fn sub_word(word: [u8; 4]) -> Result<[u8; 4], BoxedError> {
    let mut res_word = [0u8; 4];

    for idx in 0..4 {
        res_word[idx] = SUB_BYTES_TABLE[word[idx] as usize];
    }

    Ok(res_word)
}

fn rot_word_1_right(word: [u8; 4]) -> Result<[u8; 4], BoxedError> {
    let mut res = [0u8; 4];

    res[0] = word[1].clone();
    res[1] = word[2].clone();
    res[2] = word[3].clone();
    res[3] = word[0].clone();

    Ok(res)
}
