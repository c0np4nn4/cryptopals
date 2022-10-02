use std::convert::TryInto;

pub fn pkcs7(data: &mut Vec<u8>, block_size: usize) {
    let pad = block_size - (data.len() % block_size);

    if pad < 16 {
        for _ in 0..pad {
            data.push(pad.try_into().expect("pad cannot be converted into u8"));
        }
    }
}
