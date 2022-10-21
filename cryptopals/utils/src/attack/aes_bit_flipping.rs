use aes::Block;

use crate::{crypto::aes::BLOCK_SIZE, padding::pkcs7::pkcs7};

pub fn bit_flipping_attack(
    ct: &mut Vec<u8>,
    pt: Vec<u8>,
    data: &str,
    // delimiter: &str,
    delimiter: char, //
) {
    assert_eq!(data.len() <= 16, true);

    assert_eq!(delimiter, delimiter);

    // println!("ct: {:?}", ct.len());
    // println!("pt: {:?}", pt.len());

    let mut pt = pt;
    pkcs7(&mut pt, BLOCK_SIZE);

    // seperate the ct into blocks
    let blocks: Vec<[u8; BLOCK_SIZE]> = pt
        .iter()
        .enumerate()
        .step_by(BLOCK_SIZE)
        .map(|(i, _)| {
            let mut tmp: [u8; BLOCK_SIZE] = [0u8; BLOCK_SIZE];
            for j in 0..BLOCK_SIZE {
                tmp[j] = pt[i + j];
            }

            tmp
        })
        .collect();

    // println!("block len: {:?}", blocks.len());

    {
        // the `delimiter` must be existed in block[0..dlmt_i]
        let dlmt_i = BLOCK_SIZE - data.len();

        let mut block_num: i32 = -1;

        for block in blocks.iter() {
            for idx in 0..dlmt_i {
                if block[idx] == delimiter as u8 {
                    println!(
                        "[!] delimiter found:\n\tblock: {:02x?}\n\tidx: {:?}",
                        block, idx
                    );

                    block_num = idx as i32;
                }
            }
        }

        if block_num <= 0 {
            println!("hi");
        }
    }
}
