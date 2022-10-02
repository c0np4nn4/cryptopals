use crate::BoxedError;

pub fn decode_hex(mut data: String) -> Result<Vec<u8>, BoxedError> {
    if data.is_empty() {
        return Err(format!("data should be exist").into());
    }

    if data.len() % 2 == 1 {
        let last_char = data
            .chars()
            .nth(data.len() - 1)
            .ok_or("Cannot get the last char from data")?;

        data = data[..data.len() - 1].to_string();

        data = format!("{}0{}", data, last_char);
    }

    let mut res = Vec::<u8>::new();

    for w in (0..data.len()).step_by(2) {
        let b = u8::from_str_radix(&data[w..w + 2], 16)?;
        res.push(b);
    }

    Ok(res)
}

// pub fn string_to_u8_vec(data: String) -> Result<Vec<u8>, BoxedError> {
//     if data.is_empty() {
//         return Err(format!("data should be exist").into());
//     }

//     let res: Vec<u8> =
//         data.chars().into_iter().map(|byte| byte as u8).collect();

//     Ok(res)
// }

pub fn u8_vec_to_hex_string(data: Vec<u8>) -> Result<String, BoxedError> {
    if data.is_empty() {
        return Err(format!("data should be exist").into());
    }

    let res: String = data
        .iter()
        .map(|byte| format!("{:x}{:x}", byte / 16, byte % 16))
        .collect();

    Ok(res)
}
