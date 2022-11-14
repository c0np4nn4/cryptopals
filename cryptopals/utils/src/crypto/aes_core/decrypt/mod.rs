use super::AES;
use crate::BoxedError;

impl AES {
    pub fn decrypt(&self, data: Vec<u8>) -> Result<Vec<u8>, BoxedError> {
        Ok(vec![])
    }
}
