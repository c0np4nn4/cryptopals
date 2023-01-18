use std::time::{UNIX_EPOCH, SystemTime};

use utils::crypto::{mt19937::core::MT19937, stream_cipher, aes::get_random_readable_data};
use rand::Rng;

// const SEED: u32 = 0xc0de_cafe;
const SEED: u32 = 0x0000_cafe;

#[test]
fn chal_24() {


    // 1. encryption
    let mut rng_1 = MT19937::new();
    rng_1.init(SEED);
    let pt = "Hello World!".as_bytes().to_vec();
    let ct = stream_cipher::encrypt(&mut rng_1, pt);

    // 2. decryppion
    let mut rng_2 = MT19937::new();
    rng_2.init(SEED);
    let pt = stream_cipher::decrypt(&mut rng_2, ct);

    println!("[+] pt: {:?}", String::from_utf8_lossy(&pt));

    // 3. encrypt a known plaintext with random prefix
    let rand_num = rand::thread_rng().gen_range(2..128);
    let mut rand_prefix = get_random_readable_data(rand_num);
    let mut kp = "cryptopals__24".as_bytes().to_vec();
    // println!("kp len: {:?}", kp.len());

    let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as u32 & 0xffff;
    println!("seed: {:?}", seed);
    let mut rng_3 = MT19937::new();
    rng_3.init(seed);

    rand_prefix.append(&mut kp.clone());
    let msg = rand_prefix;
    println!("msg len: {:?}", msg.len());

    let ct = stream_cipher::encrypt(&mut rng_3, msg);
    let ct_len = ct.clone().len() - 14;

    let mut is_found:bool = false;
    let mut recovered_seed: u32 = 0;

    for i in 0..4 {
        if is_found {
            break;
        }
        let mut dummy = String::default();

        for _ in 0..(ct_len - i) {
            dummy += "A";
        }
        let mut dummy = dummy.as_bytes().to_vec();
        dummy.append(&mut kp.clone());

        for j in 0..65537 {
            let seed_1 = j as u32 & 0xffff;

            let mut rng_4 = MT19937::new();

            rng_4.init(seed_1);
            
            let test = stream_cipher::encrypt(&mut rng_4, dummy.clone());

            if test[ct_len..] == ct[ct_len..] {
                is_found = true;
                recovered_seed = seed_1;
                break
            }
        }
    }

    println!("[+] recovered seed: 0x{:08x?}", recovered_seed);
}
