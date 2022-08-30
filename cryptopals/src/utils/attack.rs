use crate::{get_hamming_distance, hex_string_to_u8_vec, BoxedError};
use lazy_static::lazy_static;
use std::collections::{BTreeMap, HashMap};

type PlainText = String;
type CipherText = String;
type Score = f64;
// ref
// LETTER_FREQ_TABLE
// : https://pi.math.cornell.edu/~mec/2003-2004/cryptography/subs/frequencies
// : https://en.wikipedia.org/wiki/Letter_frequency (' ' == 12.5)

lazy_static! {
    pub static ref LETTER_FREQ_TABLE: HashMap<char, f64> = {
        let mut m = HashMap::<char, f64>::new();
        m.insert(' ', 12.5);
        m.insert('E', 12.02);
        m.insert('T', 9.10);
        m.insert('A', 8.12);
        m.insert('O', 7.68);
        m.insert('I', 7.31);
        m.insert('N', 6.95);
        m.insert('S', 6.28);
        m.insert('R', 6.02);
        m.insert('H', 5.92);
        m.insert('D', 4.32);
        m.insert('L', 3.98);
        m.insert('U', 2.88);
        m.insert('C', 2.71);
        m.insert('M', 2.61);
        m.insert('F', 2.30);
        m.insert('Y', 2.11);
        m.insert('W', 2.09);
        m.insert('G', 2.03);
        m.insert('P', 1.82);
        m.insert('B', 1.49);
        m.insert('V', 1.11);
        m.insert('K', 0.69);
        m.insert('X', 0.17);
        m.insert('Q', 0.11);
        m.insert('J', 0.10);
        m.insert('Z', 0.07);

        m
    };
}

fn is_alphabet(c: char) -> bool {
    if c >= 'A' && c <= 'Z' {
        true
    } else if c >= 'a' && c <= 'z' {
        true
    } else {
        false
    }
}

fn is_space(c: char) -> bool {
    if c == ' ' {
        true
    } else {
        false
    }
}

pub fn single_char_key_attack(ct: Vec<u8>) -> Result<(PlainText, Score), BoxedError> {
    let mut score_table = HashMap::<u8, f64>::new();

    // finding key based on the score calculated by the frequency attack
    for key_candidate in 0..255 {
        let mut score: f64 = 0.0;

        for byte in ct.iter() {
            let c = (byte ^ key_candidate) as char;

            if is_alphabet(c) {
                let upper_c = c
                    .to_uppercase()
                    .next()
                    .ok_or("expect the upper case char")?;

                let letter_score = LETTER_FREQ_TABLE
                    .get(&upper_c)
                    .ok_or(format!("{} is not in the table", c))?;

                score += letter_score;
            } else if is_space(c) {
                let letter_score = LETTER_FREQ_TABLE
                    .get(&c)
                    .ok_or(format!("{} is not in the table", c))?;

                score += letter_score;
            }
        }

        score_table.insert(key_candidate, score);
    }

    let key = score_table
        .iter()
        .max_by(|a, b| a.1.total_cmp(&b.1))
        .map(|(k, v)| (k, v))
        .ok_or("error")?;

    println!("[single_char_key_attack] key: {:?}", key.0.clone() as char);

    let pt: Vec<u8> = ct.iter().map(|ct_byte| ct_byte ^ key.0).collect();

    let pt: String = match String::from_utf8(pt) {
        Ok(pt) => pt,
        Err(err) => {
            println!(
                "Failed to convert u8 to string, from_utf8(), err: {:?}",
                err
            );
            String::default()
        }
    };

    Ok((pt, key.1.to_owned()))
}

pub fn break_arbitrary_size_repeating_key_xor_cipher(
    min: u64,
    max: u64,
    ct: Vec<u8>,
) -> Result<Vec<u8>, BoxedError> {
    type KeySize = u64;
    type Score = f64;

    let mut hamming_distances: HashMap<KeySize, Score> = HashMap::default();

    for key_size in min..=max {
        let tmp = ct.clone();

        let mut score: f64 = 0.0;

        let range = key_size as usize;

        let key_size_chunks = vec![
            tmp[range * 0..range * 1].as_ref(),
            tmp[range * 1..range * 2].as_ref(),
            tmp[range * 2..range * 3].as_ref(),
            tmp[range * 3..range * 4].as_ref(),
            tmp[range * 4..range * 5].as_ref(),
            tmp[range * 5..range * 6].as_ref(),
        ];

        score += get_hamming_distance(key_size_chunks[0], key_size_chunks[1])? as f64;
        // score += get_hamming_distance(chunks[2], chunks[3])? as f64;
        // score += get_hamming_distance(chunks[4], chunks[5])? as f64;

        hamming_distances.insert(key_size, score / key_size as f64);
    }

    let key_size = hamming_distances
        .iter()
        .min_by(|a, b| {
            a.1.partial_cmp(&b.1)
                .ok_or("expect to compare values")
                .unwrap()
        })
        .map(|(k, _v)| k)
        .ok_or("expect to get key_size(candidate)")?;

    println!("key_size: {:?}", key_size);

    //

    panic!()
    // Ok(vec![])
}
