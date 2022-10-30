use utils::{
    base64::{self, base64_dec},
    crypto::aes::{decrypt_cbc, into_aes_blocks, BLOCK_SIZE},
    oracles::{into_block, Block, Oracle17, OracleResultStatus},
    xor::fixed_xor,
};

#[test]
fn chal_17() {
    let oracle = Oracle17::new();

    let ct = oracle.ciphertext.clone();
    let iv = oracle.iv.clone();

    // match oracle.verify_padding(ct.clone(), iv).unwrap().status {
    //     OracleResultStatus::OK => {}
    //     OracleResultStatus::ERR => panic!("error has been occurred"),
    // }

    {
        //     let key = oracle.key;
        //     let pt = decrypt_cbc(key.to_vec(), ct[0..16].to_vec(), iv).unwrap();

        //     println!("pt: {:02x?}", pt);
        //     println!("pt: {:?}", String::from_utf8(pt));

        // let block_1 = ct[0..16].to_vec();

        // let pt_1 = recover_plaintext(oracle, block_1, iv);

        // println!("[!] pt: {:?}", pt_1);

        let mut init_block = vec![iv];

        let mut blocks = into_aes_blocks(ct).unwrap();

        init_block.append(&mut blocks);

        let blocks: Vec<Block> = init_block;

        let mut pt = Vec::<u8>::new();

        for idx in 0..blocks.len() - 2 {
            let mut tmp = recover_plaintext(&oracle, blocks[idx + 1].to_vec(), blocks[idx]);
            pt.append(&mut tmp);
        }

        println!("res: {:02x?}", pt);
        println!("res: {:?}", String::from_utf8(pt.clone()).unwrap());
        println!(
            "res: {:?}",
            String::from_utf8(base64_dec(String::from_utf8(pt).unwrap()).unwrap())
        );
    }
}

fn recover_plaintext(oracle: &Oracle17, block: Vec<u8>, origin_iv: [u8; 16]) -> Vec<u8> {
    let mut zeroing_iv = [0u8; 16];

    for ziv_idx in (0..16).rev() {
        // println!("\n[111] block: {:02x?}", block.clone());

        // println!("[111] zeroing_iv: {:02x?}", zeroing_iv);

        for b in 0u8..=255u8 {
            zeroing_iv[ziv_idx] = b;

            match oracle
                .verify_padding(block.clone(), zeroing_iv)
                .unwrap()
                .status
            {
                OracleResultStatus::OK => {
                    // println!("[555] ziv_idx: {:?}", ziv_idx);

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
                OracleResultStatus::ERR => {
                    // println!("OKAY Let's adjust the zeroing byte..\n");
                    // println!("b: {:02x?}", b);
                }
            };
        }
    }

    // println!("zerong_iv: {:02x?}", zeroing_iv);

    let dec_result = fixed_xor(zeroing_iv.to_vec(), vec![0x10u8; BLOCK_SIZE]).unwrap();
    // println!("dec_resul: {:02x?}", dec_result);

    let pt = fixed_xor(dec_result, origin_iv.to_vec()).unwrap();
    // println!("pt:        {:02x?}", pt);
    // println!("pt:        {:?}", String::from_utf8(pt.clone()));

    pt
}
