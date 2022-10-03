use rand::Rng;

pub mod pkcs7;

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
