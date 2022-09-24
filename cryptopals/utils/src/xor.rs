use crate::{types, BoxedError};

pub fn fixed_xor(l: Vec<u8>, r: Vec<u8>) -> Result<Vec<u8>, BoxedError> {
    // let l = casting::hex_string_to_u8_vec(l)?;

    // let r = casting::hex_string_to_u8_vec(r)?;

    if l.len() != r.len() {
        return Err(format!(
            "length is different, {} and {}",
            l.len(),
            r.len()
        )
        .into());
    }

    let mut res = Vec::<u8>::new();

    for idx in 0..l.len() {
        res.push(l[idx] ^ r[idx]);
    }

    Ok(res)
}

pub fn fixed_xor_word(l: [u8; 4], r: [u8; 4]) -> Result<[u8; 4], BoxedError> {
    let mut res = [0u8; 4];

    for idx in 0..l.len() {
        res[idx] = l[idx] ^ r[idx];
    }

    Ok(res)
}

pub fn repeating_key_xor(
    pt: String,
    key: String,
) -> Result<String, BoxedError> {
    let pt = types::string_to_u8_vec(pt)?;

    let key = types::string_to_u8_vec(key)?;

    let mut expanded_key = Vec::<u8>::new();

    for idx in 0..pt.len() {
        expanded_key.push(key[idx % key.len()]);
    }

    let res: Vec<u8> = pt
        .iter()
        .zip(expanded_key.iter())
        .map(|(l, r)| l ^ r)
        .collect();

    let res: String = types::u8_vec_to_hex_string(res)?;

    Ok(res)
}
