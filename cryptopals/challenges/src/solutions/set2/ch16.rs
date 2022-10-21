use utils::{
    attack::aes_bit_flipping::bit_flipping_attack, crypto::aes::get_random_aes_iv,
    oracles::Oracle16,
};

#[test]
fn chal_16() {
    let oracle = Oracle16::new();

    // let data = "AAAAAAAAA;admin=true;AAAAAAA".as_bytes().to_vec();

    // let handle_block = [0u8; BLOCK_SIZE];
    // let target_block = [0u8; BLOCK_SIZE];
    let data = vec![0u8; 32];

    let iv = get_random_aes_iv();

    // let ct = oracle.encrypt(data, iv);
    let mut ct = oracle.encrypt(data, iv);

    let pt = oracle.decrypt(ct.clone(), iv);
    {
        // bit flipping

        // inject data have size less than BLOCK_SIZE
        let inject_data = "admin=true";

        let delimiter: char = ';';

        bit_flipping_attack(&mut ct, pt, inject_data, delimiter);
    }
    // let pt = oracle.decrypt(ct, iv);

    // println!("pt: {:?}", String::from_utf8(pt));
}
