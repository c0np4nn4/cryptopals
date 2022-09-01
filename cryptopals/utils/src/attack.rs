use lazy_static::lazy_static;
use std::{char, collections::HashMap};

use crate::ham_dist;
use crate::BoxedError;

// ref
// LETTER_FREQ_TABLE
// : https://pi.math.cornell.edu/~mec/2003-2004/cryptography/subs/frequencies
// : https://en.wikipedia.org/wiki/Letter_frequency (' ' == 12.5)

pub struct SingleCharKeyAttackResult {
    pub key: char,
    pub pt: String,
    pub score: f64,
}

pub struct ArbSizedKeyAttackResult {
    pub key: String,
    pub pt: String,
    // pub score: f64,
}

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

pub fn single_char_key_attack(
    ct: Vec<u8>,
) -> Result<SingleCharKeyAttackResult, BoxedError> {
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

    let result = score_table
        .iter()
        .max_by(|a, b| a.1.total_cmp(&b.1))
        .map(|(k, v)| (k, v))
        .ok_or("error")?;

    let (key, score) = (result.0.to_owned() as char, result.1.to_owned());

    println!("[single_char_key_attack] key: {:?}", key.clone() as char);

    let pt: Vec<u8> = ct.iter().map(|ct_byte| ct_byte ^ key as u8).collect();

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

    Ok(SingleCharKeyAttackResult { key, pt, score })
}

pub fn break_arbitrary_size_repeating_key_xor_cipher(
    min: u64,
    max: u64,
    ct: Vec<u8>,
) -> Result<ArbSizedKeyAttackResult, BoxedError> {
    type Score = f64;

    let mut hamming_distances: HashMap<_, Score> = HashMap::default();

    for key_size in min..=max {
        let ct_clone = ct.clone();

        let mut score: f64 = 0.0;

        let key_size = key_size as usize;

        for idx in 0..(ct_clone.len() / key_size) - 1 {
            score += ham_dist::get_hamming_distance(
                &ct_clone[(idx + 0) * key_size..(idx + 1) * key_size],
                &ct_clone[(idx + 1) * key_size..(idx + 2) * key_size],
            )? as f64;
        }

        let score =
            score / (ct_clone.len() / key_size) as f64 / key_size as f64;

        hamming_distances.insert(key_size, score);
    }

    let key_size: usize = hamming_distances
        .iter()
        .min_by(|a, b| {
            a.1.partial_cmp(&b.1)
                .ok_or("expect to compare values")
                .unwrap()
        })
        .map(|(k, _v)| k)
        .ok_or("expect to get key_size(candidate)")?
        .to_owned();

    let mut chunks: HashMap<usize, Vec<u8>> = HashMap::<usize, Vec<u8>>::new();

    for idx in 0..key_size {
        chunks.insert(idx, vec![]);
    }

    for (idx, byte) in ct.iter().enumerate() {
        let index = &idx.rem_euclid(key_size);

        let chunk = chunks
            .get_mut(index)
            .ok_or(format!("expect chunk[{}]", index))?;

        chunk.push(byte.clone());
    }

    let mut pt_chunks: HashMap<usize, String> = HashMap::<usize, String>::new();
    let mut rec_key: String = String::default();

    for idx in 0..key_size {
        let ct = chunks.get(&idx).ok_or("expect to get ciphertext chunk")?;

        let SingleCharKeyAttackResult { pt, key, .. } =
            single_char_key_attack(ct.clone())?;

        pt_chunks.insert(idx, pt);
        rec_key.push(key);
    }

    let mut pt: String = String::default();
    for idx in 0..ct.len() {
        let chunk_idx = &idx.rem_euclid(key_size);

        let pt_chunk = pt_chunks
            .get(chunk_idx)
            .ok_or("expect to get plaintext chunk")?
            .as_bytes()
            .to_vec();

        pt.push(pt_chunk[idx / key_size] as char);
    }

    Ok(ArbSizedKeyAttackResult { pt, key: rec_key })
}
