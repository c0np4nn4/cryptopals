use crate::{fixed_xor, hex_string_to_u8_vec};

#[test]
fn test_fixed_xor() {
    let left_data = "\
        1c0111001f010100061a024b53535009181c"
        .to_string();

    let right_data = "\
        686974207468652062756c6c277320657965"
        .to_string();

    let res = fixed_xor(left_data, right_data).unwrap();

    println!("[set1/ch2] res: {:?}", res);

    assert_eq!(
        res,
        String::from_utf8(
            hex_string_to_u8_vec(
                "\
                746865206b696420646f6e277420706c6179"
                    .to_string()
            )
            .unwrap()
        )
        .unwrap()
    )
}
