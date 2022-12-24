use std::convert::TryInto;

use super::PaddingError;

pub fn pkcs7(data: &mut Vec<u8>, block_size: usize) {
    let pad = block_size - (data.len() % block_size);

    if pad <= 16 {
        for _ in 0..pad {
            data.push(pad.try_into().expect("pad cannot be converted into u8"));
        }
    }
}

pub fn verify_pkcs7(data: &mut Vec<u8>, block_size: usize) -> bool {
    let last_block: Vec<u8> = data[data.len() - block_size..data.len()]
        .try_into()
        .unwrap();

    // let maybe_pad = last_block[block_size - 1];
    let maybe_pad = last_block.last().unwrap().to_owned();
    // println!("maybe_pad: {:?}", maybe_pad);

    if maybe_pad > 0x10 || maybe_pad < 0x01 {
        return false;
    }

    let mut count = 0;

    for idx in last_block.len() - maybe_pad as usize..block_size {
        if last_block[idx] == maybe_pad {
            count += 1;
        }
    }

    if count == maybe_pad {
        true
    } else {
        false
    }
}

pub fn trim_pkcs7(data: &mut Vec<u8>, block_size: usize) -> Result<(), PaddingError> {
    if verify_pkcs7(data, block_size) {
        let last_block: Vec<u8> = data[data.len() - block_size..data.len()]
            .try_into()
            .unwrap();

        let count = last_block[block_size - 1];

        for _ in 0..count {
            match data.last() {
                Some(d) => {
                    if *d != count {
                        return Err(
                            format!("invalid padding, expect: {}, found: {}", count, d).into()
                        );
                    }
                }
                None => {
                    return Err(format!("block is empty").into());
                }
            }
            data.pop();
        }
    }

    Ok(())
}
