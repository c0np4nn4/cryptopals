use aes::Block;

use crate::{crypto::aes::BLOCK_SIZE, oracles::Oracle16, padding::pkcs7::pkcs7};

pub fn bit_flipping_attack(
    // ct: &mut Vec<u8>,
    // pt: Vec<u8>,
    oracle: &Oracle16,
    target_data: &str,
    _delimiter: char, //
) -> Vec<u8> {
    assert_eq!(target_data.len() <= 16, true);

    // assert_eq!(delimiter, delimiter);

    let mut pad_size = 0;
    if oracle.prefix.len() % 16 != 0 {
        pad_size = BLOCK_SIZE - (oracle.prefix.len() % BLOCK_SIZE);
    }

    let mut dummy = Vec::<u8>::new();

    for _ in 0..pad_size + BLOCK_SIZE {
        dummy.push(0x11);
    }

    for _ in pad_size + BLOCK_SIZE..pad_size + BLOCK_SIZE * 2 {
        dummy.push(0x00);
    }

    // println!("dummy_size: {:?}", dummy.len());

    let mut ct = oracle.encrypt(dummy);

    // {
    //     println!("\n[!] ciphertext: (len:{})", ct.len());
    //     for i in (0..ct.len()).step_by(BLOCK_SIZE) {
    //         let mut tmp: [u8; BLOCK_SIZE] = [0u8; BLOCK_SIZE];
    //         for j in 0..BLOCK_SIZE {
    //             tmp[j] = ct[i + j];
    //         }

    //         println!("[!] block[{}]: {:02x?}", i / BLOCK_SIZE, tmp);
    //     }
    // }

    // {
    //     let mut tmp_pt = oracle.decrypt(ct.clone());

    //     pkcs7(&mut tmp_pt, BLOCK_SIZE);

    //     println!("\n[!] plaintext: (len: {})", tmp_pt.len());
    //     for i in (0..tmp_pt.len()).step_by(BLOCK_SIZE) {
    //         let mut tmp: [u8; BLOCK_SIZE] = [0u8; BLOCK_SIZE];
    //         for j in 0..BLOCK_SIZE {
    //             tmp[j] = tmp_pt[i + j];
    //         }

    //         println!("[!] block[{}]: {:02x?}", i / BLOCK_SIZE, tmp);
    //     }
    // }

    let target_idx = oracle.prefix.len() / 16;
    // println!("\n[!] target_idx: {:#?}", target_idx);

    let mut target_decrypted_block: [u8; BLOCK_SIZE] = [0u8; BLOCK_SIZE];
    for idx in 0..BLOCK_SIZE {
        target_decrypted_block[idx] = ct[idx + (target_idx * BLOCK_SIZE)];
    }

    // println!(
    //     "\n\t[!] target_decrypted_block: {:02x?}\n",
    //     target_decrypted_block
    // );

    target_data
        .as_bytes()
        .to_vec()
        .iter()
        .enumerate()
        .for_each(|(idx, b)| {
            ct[target_idx * BLOCK_SIZE + idx] = b ^ target_decrypted_block[idx];
        });

    // {
    //     println!("\n[!] after ciphertext: (len: {})", ct.len());
    //     for i in (0..ct.len()).step_by(BLOCK_SIZE) {
    //         let mut tmp: [u8; BLOCK_SIZE] = [0u8; BLOCK_SIZE];
    //         for j in 0..BLOCK_SIZE {
    //             tmp[j] = ct[i + j];
    //         }

    //         println!("[!] block[{}]: {:02x?}", i / BLOCK_SIZE, tmp);
    //     }
    // }

    // {
    //     let mut tmp_pt = oracle.decrypt(ct.clone());

    //     pkcs7(&mut tmp_pt, BLOCK_SIZE);

    //     println!("\n[!] plaintext: (len: {})", tmp_pt.len());
    //     for i in (0..tmp_pt.len()).step_by(BLOCK_SIZE) {
    //         let mut tmp: [u8; BLOCK_SIZE] = [0u8; BLOCK_SIZE];
    //         for j in 0..BLOCK_SIZE {
    //             tmp[j] = tmp_pt[i + j];
    //         }

    //         println!("[!] block[{}]: {:02x?}", i / BLOCK_SIZE, tmp);
    //     }
    // }

    ct
}
