use utils::{types::hex_string_to_u8_vec, xor};

#[test]
fn chal_2() {
    let left_data = "\
        1c0111001f010100061a024b53535009181c"
        .to_string();

    let right_data = "\
        686974207468652062756c6c277320657965"
        .to_string();

    let l = hex_string_to_u8_vec(left_data).unwrap();

    let r = hex_string_to_u8_vec(right_data).unwrap();

    let res = xor::fixed_xor(l, r).unwrap();

    println!("[set1/ch2] res: {:?}", String::from_utf8(res.clone()));

    assert_eq!(
        res,
        hex_string_to_u8_vec(
            "746865206b696420646f6e277420706c6179".to_string()
        )
        .unwrap()
    );
}
