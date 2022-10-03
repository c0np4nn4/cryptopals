use rand;
use utils::{
    crypto::aes::{
        detect_ecb, detect_ecb_probability, encrypt_cbc, encrypt_ecb,
        generate_random_16_bytes, get_data, get_random_readable_data,
    },
    padding::{append_rand_bytes, prepend_rand_bytes},
};

#[test]
fn chal_11() {
    // let key = generate_random_16_bytes();

    // let mut data = get_data();

    // prepend_rand_bytes(&mut data, 5, 10);

    // append_rand_bytes(&mut data, 5, 10);

    // let rand: u8 = rand::random::<u8>() % 2;

    // let res;
    // if rand == 1 {
    //     println!("mode: CBC");

    //     let iv = generate_random_16_bytes();

    //     res = encrypt_cbc(key.to_vec(), &mut data, iv);
    // } else {
    //     println!("mode: ECB");
    //     res = encrypt_ecb(key.to_vec(), &mut data);
    // }

    // let prob_ecb = detect_ecb_probability(res);

    // if prob_ecb > 0.0 {
    //     println!("detect ECB mode");
    // } else {
    //     println!("detect CBC mode");
    // }

    for _ in 0..16 {
        foo();
        println!("");
    }
}

fn foo() {
    let key = generate_random_16_bytes();

    let mut data = get_data();

    prepend_rand_bytes(&mut data, 5, 10);

    append_rand_bytes(&mut data, 5, 10);

    let rand: u8 = rand::random::<u8>() % 2;

    let res;
    if rand == 1 {
        println!("Encrypt in CBC(Cipher Block Chaining) mode");

        let iv = generate_random_16_bytes();

        res = encrypt_cbc(key.to_vec(), &mut data, iv);
    } else {
        println!("Encrypt in ECB(Electronic Code Block) mode");
        res = encrypt_ecb(key.to_vec(), &mut data);
    }

    let prob_ecb = detect_ecb_probability(res);

    if prob_ecb > 0.0 {
        println!("Detect     ECB(Electronic Code Block) mode");
    } else {
        println!("Detect     CBC(Cipher Block Chaining) mode");
    }
}
