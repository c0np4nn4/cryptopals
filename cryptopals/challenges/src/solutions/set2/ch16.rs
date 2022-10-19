use utils::{crypto::aes::get_random_aes_iv, oracles::Oracle16};

#[test]
fn chal_16() {
    let oracle = Oracle16::new();

    // let data = "AAAAAAAAA;admin=true;AAAAAAA".as_bytes().to_vec();
    let data = "AAAAAAAAAAAAAAAA".as_bytes().to_vec();

    let iv = get_random_aes_iv();

    let ct = oracle.encrypt(data, iv);

    let pt = oracle.decrypt(ct, iv);

    println!("pt: {:?}", String::from_utf8(pt));
}
