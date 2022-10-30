use crate::{
    crypto::aes::{into_aes_blocks, BLOCK_SIZE},
    oracles::{into_block, Block, Oracle17, OracleResultStatus, IV},
    xor::fixed_xor,
};

pub fn padding_oracle_attack(oracle: &Oracle17, ct: Vec<u8>, iv: IV) -> Vec<u8> {
    let mut init_block = vec![iv];

    let mut blocks = into_aes_blocks(ct).unwrap();

    init_block.append(&mut blocks);

    let blocks: Vec<Block> = init_block;

    let mut pt = Vec::<u8>::new();

    for idx in 0..blocks.len() - 1 {
        let mut tmp = recover_plaintext(&oracle, blocks[idx + 1].to_vec(), blocks[idx]);
        pt.append(&mut tmp);
    }

    pt
}

fn recover_plaintext(oracle: &Oracle17, block: Vec<u8>, origin_iv: [u8; 16]) -> Vec<u8> {
    let mut zeroing_iv = [0u8; 16];

    for ziv_idx in (0..16).rev() {
        for b in 0u8..=255u8 {
            zeroing_iv[ziv_idx] = b;

            match oracle
                .verify_padding(block.clone(), zeroing_iv)
                .unwrap()
                .status
            {
                OracleResultStatus::OK => {
                    let ziv = zeroing_iv.to_vec();

                    let ziv = fixed_xor(ziv, vec![(16 - ziv_idx) as u8; BLOCK_SIZE]).unwrap();

                    let mut ziv =
                        fixed_xor(ziv, vec![(16 - ziv_idx + 1) as u8; BLOCK_SIZE]).unwrap();

                    if ziv_idx == 0 {
                        break;
                    }

                    for i in 0..(ziv_idx - 1) {
                        ziv[i] = 0x00;
                    }

                    zeroing_iv = into_block(ziv).unwrap();

                    break;
                }
                OracleResultStatus::ERR => {}
            };
        }
    }

    let dec_result = fixed_xor(zeroing_iv.to_vec(), vec![0x10u8; BLOCK_SIZE]).unwrap();

    let pt = fixed_xor(dec_result, origin_iv.to_vec()).unwrap();

    pt
}
