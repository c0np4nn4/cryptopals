mod base64;
mod xor;

#[cfg(test)]
mod tests;

pub use base64::*;
pub use xor::*;

pub type BoxedError = Box<dyn std::error::Error + Send + Sync>;

pub fn hex_string_to_u8_vec(mut data: String) -> Result<Vec<u8>, BoxedError> {
    if data.is_empty() {
        return Err(format!("data should be exist").into());
    }

    if data.len() % 2 == 1 {
        data = format!("0{}", data);
    }

    let mut res = Vec::<u8>::new();

    for w in (0..data.len()).step_by(2) {
        let b = u8::from_str_radix(&data[w..w + 2], 16)?;
        res.push(b);
    }

    Ok(res)
}
