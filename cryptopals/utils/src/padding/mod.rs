pub mod pkcs7;

use rand::Rng;
use std::error::Error;

pub type PaddingError = Box<dyn Error>;


pub fn prepend_rand_bytes(data: &mut Vec<u8>, min: usize, max: usize) {
    // let pre_count: usize = rand::random::<usize>() % len;
    let pre_count = rand::thread_rng().gen_range(min..max);

    let mut prefix: Vec<u8> = vec![];

    for _ in 0..pre_count {
        prefix.push(rand::random());
    }

    prefix.append(data);

    *data = prefix;
}

pub fn append_rand_bytes(data: &mut Vec<u8>, min: usize, max: usize) {
    // let post_count: usize = rand::random::<usize>() % len;
    let post_count = rand::thread_rng().gen_range(min..max);

    let mut postfix: Vec<u8> = vec![];

    for _ in 0..post_count {
        postfix.push(rand::random());
    }

    data.append(&mut postfix);
}

pub fn append_byte(data: &mut Vec<u8>, pad: u8, block_size: usize) {
    let mut postfix: Vec<u8> = vec![];

    for _ in 0..(block_size - (data.len() % block_size)) {
        postfix.push(pad);
    }

    data.append(&mut postfix);
}
