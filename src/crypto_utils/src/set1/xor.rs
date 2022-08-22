use crate::{hex_string_to_u8_vec, BoxedError};

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
