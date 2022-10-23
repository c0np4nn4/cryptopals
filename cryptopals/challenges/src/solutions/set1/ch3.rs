use utils::{
    attack::{single_char_key_attack, SingleCharKeyAttackResult},
    types::decode_hex,
};

#[test]
fn chal_3() {
    let ct: String = String::from(
        "\
        1b37373331363f78151b7f2b78343133\
        3d78397828372d363c78373e783a393b\
        3736\
        ",
    );

    let ct = decode_hex(ct).unwrap();

    let SingleCharKeyAttackResult { pt, .. } = single_char_key_attack(ct).unwrap();

    println!("[set1/ch3] res: {:?}", pt);

    assert_eq!("Cooking MC's like a pound of bacon", pt.as_str());
}
