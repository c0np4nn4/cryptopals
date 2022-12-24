use std::collections::HashMap;
use utils::oracles::Oracle14;

fn get_prefix_size(oracle: &mut Oracle14) -> usize {
    let mut dummy_chunk = "AAAAAAAAAAAAAAAA".as_bytes().to_vec();

    let mut start_idx = 0;
    loop {
        dummy_chunk.push('A' as u8);

        let tmp = oracle.encrypt(dummy_chunk.clone()).unwrap();

        let mut chunk_1 = [0u8; 16];
        let mut chunk_2 = [0u8; 16];

        for idx in (0..tmp.len() - 16).step_by(16) {
            for i in 0..16 {
                chunk_1[i] = tmp[idx + i];
                chunk_2[i] = tmp[idx + i + 16];
            }

            if chunk_1 == chunk_2 {
                start_idx = idx;
                break;
            }
        }
        if chunk_1 == chunk_2 {
            break;
        }
    }

    let prefix_len = start_idx + 32 - dummy_chunk.len();

    prefix_len
}

// data == dummy + 15 controlled bytes
fn make_rainbow_table(oracle: &mut Oracle14, data: Vec<u8>, idx: usize) -> HashMap<Vec<u8>, u8> {
    let mut rainbow_table = HashMap::<Vec<u8>, u8>::new();

    for b in 0..=255 {
        let mut rainbow_tmp = data.clone();
        rainbow_tmp.push(b);

        let data = rainbow_tmp;

        let tmp = oracle.encrypt(data).unwrap();

        let mut chunk = [0u8; 16];
        for i in 0..16 {
            chunk[i] = tmp[idx + i];
        }

        // println!("chunk 222 {:?}", chunk);

        rainbow_table.insert(chunk.to_vec(), b);
    }

    rainbow_table
}

#[test]
fn chal_14() {
    let mut oracle = Oracle14::new();

    let ciphertext = oracle.encrypt(vec![]).unwrap();

    let prefix_len = get_prefix_size(&mut oracle);

    let dummy_len = 16 - (prefix_len % 16);

    let data_len = ciphertext.len() - prefix_len;

    let mut dummy = vec![];
    for _ in 0..dummy_len {
        dummy.push('A' as u8);
    }

    println!("[+] ciphertext (len: {:?})", ciphertext.len(),);

    println!("1. prefix len: {:?}", prefix_len);

    println!("2. dummy len: {:?}", dummy_len);

    println!("3. data len: {:?}", data_len);

    let mut plaintext = vec![];

    let mut data_queue = vec!['A' as u8; 15];

    for block_idx in (0..ciphertext.len() - 16).step_by(16) {
        let mut pad;

        for pad_size in 1..=16 {
            pad = vec![];

            let pad_size = 16 - pad_size;

            for _ in 0..pad_size {
                pad.push('A' as u8);
            }

            let mut dummy_clone = dummy.clone();
            dummy_clone.append(&mut pad);

            let res = oracle.encrypt(dummy_clone).unwrap();

            let mut dummy_clone = dummy.clone();
            dummy_clone.append(&mut data_queue.clone());

            // println!("111 data_queue: {:?}", data_queue);
            let rainbow_table =
                make_rainbow_table(&mut oracle, dummy_clone, prefix_len + dummy_len);

            let mut chunk = [0u8; 16];
            for i in 0..16 {
                chunk[i] = res[prefix_len + dummy_len + block_idx + i];
            }

            let recovered_byte = match rainbow_table.get(&chunk.to_vec()) {
                Some(b) => b,
                None => {
                    // println!("recovered text: {:?}", String::from_utf8_lossy(&plaintext));
                    // println!("ciphertext len: {:?}", ciphertext.len());
                    // println!("idx: {:?}", block_idx);
                    // println!("data_queue: {:?}", data_queue);
                    // println!("data_queue: {:?}", String::from_utf8_lossy(&data_queue));
                    // println!("data_queue len: {:?}", data_queue.len());
                    // println!("plaintext len: {:?}", plaintext.len());
                    // println!("data len: {:?}", data_len);
                    // println!("chunk: {:?}", chunk.to_vec());

                    plaintext.append(&mut data_queue);
                    break;
                    // panic!();
                }
            };

            let decrypted_byte = data_queue.remove(0);

            plaintext.push(decrypted_byte);

            data_queue.push(recovered_byte.clone());
        }
    }

    for _ in 0..15 {
        plaintext.remove(0);
    }
    println!("plaintext: {:?}", String::from_utf8(plaintext));
}
