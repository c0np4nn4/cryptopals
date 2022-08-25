use crate::{hex_string_to_u8_vec, string_to_u8_vec, BoxedError};

pub struct Base64Table {
    pub table: [char; 64],
}
// const TABLE: [char; 64] =

impl Base64Table {
    fn new() -> Base64Table {
        Base64Table {
            table: [
                'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', //
                'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', //
                'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', //
                'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', //
                'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', //
                'o', 'p', 'q', 'r', 's', 't', 'u', 'v', //
                'w', 'x', 'y', 'z', '0', '1', '2', '3', //
                '4', '5', '6', '7', '8', '9', '+', '/',
            ],
        }
    }

    fn get_index(&self, data: &char) -> Result<u8, BoxedError> {
        if data.clone() == '=' {
            return Ok(0);
        }

        let idx: u8 = match self
            .table
            .iter()
            .position(|&c| c == data.clone())
            .ok_or(format!("Can not find char: {} in array", data))
        {
            Ok(i) => i.try_into()?,
            Err(err) => {
                return Err(format!("error: {:?}", err).into());
            }
        };

        Ok(idx)
    }
}

fn u8_to_u6(byte: u8) -> u8 {
    byte & 0b_0011_1111
}

fn second_word(byte: u8) -> u8 {
    byte & 0b_0000_1111
}

fn padding_0_byte(data: Vec<u8>) -> Result<String, BoxedError> {
    let base64_table = Base64Table::new();

    let mut base64_chunk: [u8; 4] = [0; 4];

    let mut res: String = String::default();

    for idx in (0..data.len()).step_by(3) {
        base64_chunk[0] = u8_to_u6(data[idx + 0] >> 2);
        base64_chunk[1] = u8_to_u6((data[idx + 0] << 4) | second_word(data[idx + 1] >> 4));
        base64_chunk[2] = u8_to_u6((data[idx + 1] << 2) | second_word(data[idx + 2] >> 6));
        base64_chunk[3] = u8_to_u6(data[idx + 2]);

        res = format!(
            "{}{}{}{}{}",
            res,
            base64_table.table[base64_chunk[0] as usize],
            base64_table.table[base64_chunk[1] as usize],
            base64_table.table[base64_chunk[2] as usize],
            base64_table.table[base64_chunk[3] as usize],
        );
    }

    Ok(res)
}

fn padding_1_byte(mut data: Vec<u8>) -> Result<String, BoxedError> {
    let base64_table = Base64Table::new();

    data.push(0 as u8);

    let mut base64_chunk: [u8; 4] = [0; 4];

    let mut res: String = String::default();

    for idx in (0..data.len()).step_by(3) {
        base64_chunk[0] = u8_to_u6(data[idx + 0] >> 2);
        base64_chunk[1] = u8_to_u6((data[idx + 0] << 4) | second_word(data[idx + 1] >> 4));
        base64_chunk[2] = u8_to_u6((data[idx + 1] << 2) | second_word(data[idx + 2] >> 6));
        base64_chunk[3] = u8_to_u6(data[idx + 2]);

        res = format!(
            "{}{}{}{}{}",
            res,
            base64_table.table[base64_chunk[0] as usize],
            base64_table.table[base64_chunk[1] as usize],
            base64_table.table[base64_chunk[2] as usize],
            base64_table.table[base64_chunk[3] as usize],
        );
    }

    res.pop();
    res = format!("{}=", res);

    Ok(res)
}

fn padding_2_byte(mut data: Vec<u8>) -> Result<String, BoxedError> {
    let base64_table = Base64Table::new();

    data.push(0 as u8);
    data.push(0 as u8);

    let mut base64_chunk: [u8; 4] = [0; 4];

    let mut res: String = String::default();

    for idx in (0..data.len()).step_by(3) {
        base64_chunk[0] = u8_to_u6(data[idx + 0] >> 2);
        base64_chunk[1] = u8_to_u6((data[idx + 0] << 4) | second_word(data[idx + 1] >> 4));
        base64_chunk[2] = u8_to_u6((data[idx + 1] << 2) | second_word(data[idx + 2] >> 6));
        base64_chunk[3] = u8_to_u6(data[idx + 2]);

        res = format!(
            "{}{}{}{}{}",
            res,
            base64_table.table[base64_chunk[0] as usize],
            base64_table.table[base64_chunk[1] as usize],
            base64_table.table[base64_chunk[2] as usize],
            base64_table.table[base64_chunk[3] as usize],
        );
    }

    res.pop();
    res.pop();
    res = format!("{}=", res);
    res = format!("{}=", res);

    Ok(res)
}

pub fn base64_enc(data: String) -> Result<String, BoxedError> {
    let data = hex_string_to_u8_vec(data)?;

    let res = match data.len() % 3 {
        0 => padding_0_byte(data)?,

        1 => padding_2_byte(data)?,

        2 => padding_1_byte(data)?,

        _ => {
            return Err(format!("Unexpected result from data.len() % 3").into());
        }
    };

    Ok(res)
}

pub fn base64_dec(data: String) -> Result<Vec<u8>, BoxedError> {
    let base64_table = Base64Table::new();

    let data: Vec<u8> = string_to_u8_vec(data)?;

    let mut data_decrypted: Vec<u8> = Vec::new();

    for idx in (0..data.len()).step_by(4) {
        let word = &data[idx + 0..idx + 4];

        let mut word_dec: Vec<u8> = Vec::default();

        word_dec.push(base64_table.get_index(&(word[0] as char))?);
        word_dec.push(base64_table.get_index(&(word[1] as char))?);
        word_dec.push(base64_table.get_index(&(word[2] as char))?);
        word_dec.push(base64_table.get_index(&(word[3] as char))?);

        // println!("{:?}", word_dec);

        data_decrypted.push((word_dec[0] << 2) | (word_dec[1] >> 4));
        data_decrypted.push((word_dec[1] << 4) | (word_dec[2] >> 2));
        data_decrypted.push((word_dec[2] << 6) | (word_dec[3] >> 0));
    }

    let maybe_pad_slice = &data[data.len() - 2..data.len()];
    if maybe_pad_slice[0] == '=' as u8 {
        data_decrypted.pop();
        data_decrypted.pop();
    } else if maybe_pad_slice[1] == '=' as u8 {
        data_decrypted.pop();
    }

    Ok(data_decrypted)
}
