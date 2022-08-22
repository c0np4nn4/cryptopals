use crate::{base64_enc, fixed_xor, hex_string_to_u8_vec};

#[test]
fn test_set1_ch1() {
    let data = "\
        49276d206b696c6c696e6720796f75722\
        0627261696e206c696b65206120706f69\
        736f6e6f7573206d757368726f6f6d"
        .to_string();

    let res = base64_enc(data).unwrap();

    println!("res: {:?}", res);

    assert_eq!(
        res,
        String::from(
            "\
            SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsa\
            WtlIGEgcG9pc29ub3VzIG11c2hyb29t\
            "
        )
    );
}

#[test]
fn test_base64_1_pad() {
    let data = "012345a".to_string();

    let res = base64_enc(data).unwrap();

    println!("res: {:?}", res);

    assert_eq!(res, String::from("ASNFCg=="));
}

#[test]
fn test_base64_2_pad() {
    let data = "abcdefab".to_string();

    let res = base64_enc(data).unwrap();

    println!("res: {:?}", res);

    assert_eq!(res, String::from("q83vqw=="));
}

#[test]
fn test_fixed_xor() {
    let left_data = "\
        1c0111001f010100061a024b53535009181c"
        .to_string();

    let right_data = "\
        686974207468652062756c6c277320657965"
        .to_string();

    let res = fixed_xor(left_data, right_data).unwrap();

    println!("[fixed_xor] res: {:?}", res);

    // assert_eq!(
    //     res.as_bytes().to_vec(),
    //     hex_string_to_u8_vec("746865206b696420646f6e277420706c6179".to_string()).unwrap()
    // )

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
