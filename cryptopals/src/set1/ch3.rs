use crate::single_char_key_attack;

#[test]
fn test_single_byte_xor_cipher() {
    let ct: String = String::from(
        "\
        1b37373331363f78151b7f2b78343133\
        3d78397828372d363c78373e783a393b\
        3736\
        ",
    );

    let res = single_char_key_attack(ct).unwrap();

    println!("[set1/ch3] res: {:?}", res.0);

    assert_eq!("Cooking MC's like a pound of bacon", res.0.as_str());
}
