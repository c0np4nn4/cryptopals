use crate::BoxedError;

pub enum MOO {
    ECB,
}
pub fn aes_decrypt(
    ct: Vec<u8>,
    key: Vec<u8>,
    mode: MOO,
) -> Result<Vec<u8>, BoxedError> {
    Ok(vec![])
}
