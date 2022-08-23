use crate::{hex_string_to_u8_vec, string_to_u8_vec, u8_vec_to_hex_string, BoxedError};

pub fn fixed_xor(l: String, r: String) -> Result<String, BoxedError> {
    let l = hex_string_to_u8_vec(l)?;

    let r = hex_string_to_u8_vec(r)?;

    if l.len() != r.len() {
        return Err(format!("length is different, {} and {}", l.len(), r.len()).into());
    }

    let mut res = Vec::<u8>::new();

    for idx in 0..l.len() {
        res.push(l[idx] ^ r[idx]);
    }

    Ok(String::from_utf8(res)?)
}

pub fn repeating_key_xor(pt: String, key: String) -> Result<String, BoxedError> {
    let pt = string_to_u8_vec(pt)?;

    let key = string_to_u8_vec(key)?;

    let mut expanded_key = Vec::<u8>::new();

    for idx in 0..pt.len() {
        expanded_key.push(key[idx % key.len()]);
    }

    let res: Vec<u8> = pt
        .iter()
        .zip(expanded_key.iter())
        .map(|(l, r)| l ^ r)
        .collect();

    let res: String = u8_vec_to_hex_string(res)?;

    Ok(res)
}
