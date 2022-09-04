use crate::base64;

#[test]
fn test_base64_encode_with_1_pad() {
    let data = "012345a".to_string();

    let res = base64::base64_enc(data).unwrap();

    println!("[set1/ch1, 1] res: {:?}", res);

    assert_eq!(res, String::from("ASNFCg=="));
}

#[test]
fn test_base64_encode_with_2_pad() {
    let data = "abcdefab".to_string();

    let res = base64::base64_enc(data).unwrap();

    println!("[set1/ch1, 2] res: {:?}", res);

    assert_eq!(res, String::from("q83vqw=="));
}
