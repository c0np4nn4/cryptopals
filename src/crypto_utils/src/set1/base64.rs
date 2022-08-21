use crate::BoxedError;

const TABLE: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];

fn u8_to_u6(byte: u8) -> u8 {
    byte & 0b_0011_1111
}

fn first_word(byte: u8) -> u8 {
    byte & 0b_1111_0000
}

fn second_word(byte: u8) -> u8 {
    byte & 0b_0000_1111
}

fn base64_enc_padding_0_byte(data: Vec<u8>) -> Result<String, BoxedError> {
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
            TABLE[base64_chunk[0] as usize],
            TABLE[base64_chunk[1] as usize],
            TABLE[base64_chunk[2] as usize],
            TABLE[base64_chunk[3] as usize],
        );
    }

    Ok(res)
}

fn base64_enc_padding_1_byte(mut data: Vec<u8>) -> Result<String, BoxedError> {
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
            TABLE[base64_chunk[0] as usize],
            TABLE[base64_chunk[1] as usize],
            TABLE[base64_chunk[2] as usize],
            TABLE[base64_chunk[3] as usize],
        );
    }

    res.pop();
    res = format!("{}=", res);

    Ok(res)
}

fn base64_enc_padding_2_byte(mut data: Vec<u8>) -> Result<String, BoxedError> {
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
            TABLE[base64_chunk[0] as usize],
            TABLE[base64_chunk[1] as usize],
            TABLE[base64_chunk[2] as usize],
            TABLE[base64_chunk[3] as usize],
        );
    }

    res.pop();
    res.pop();
    res = format!("{}=", res);
    res = format!("{}=", res);

    Ok(res)
}

fn hex_string_to_u8_vec(mut data: String) -> Result<Vec<u8>, BoxedError> {
    if data.is_empty() {
        return Err(format!("data should be exist").into());
    }

    if data.len() % 2 == 1 {
        println!("odd number");
        data = format!("{}0", data);
        println!("after adding pre_pad data: {:?}", data);
    }

    let mut res = Vec::<u8>::new();

    for w in (0..data.len()).step_by(2) {
        println!("from_Str: {:?}", &data[w..w + 2]);
        let b = u8::from_str_radix(&data[w..w + 2], 16)?;
        res.push(b);
    }

    Ok(res)
}

pub fn base64_enc(data: String) -> Result<String, BoxedError> {
    let data = hex_string_to_u8_vec(data)?;

    let res = match data.len() % 3 {
        0 => base64_enc_padding_0_byte(data)?,

        1 => base64_enc_padding_2_byte(data)?,

        2 => base64_enc_padding_1_byte(data)?,

        _ => {
            return Err(format!("Unexpected result from data.len() % 3").into());
        }
    };

    Ok(res)
}

pub fn hex_string_to_base64(data: String) -> Result<String, BoxedError> {
    let res = base64_enc(data)?;

    Ok(res)
}
