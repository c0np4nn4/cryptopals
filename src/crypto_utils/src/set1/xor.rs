use crate::BoxedError;

pub fn fixed_xor(l: Vec<u8>, r: Vec<u8>) -> Result<String, BoxedError> {
    if l.len() != r.len() {
        return Err(format!("length is different, {} and {}", l.len(), r.len()).into());
    }

    Ok(String::default())
}
