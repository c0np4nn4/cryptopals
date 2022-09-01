use utils::base64;

#[test]
fn test_base64_enc() {
    let data = "\
        49276d206b696c6c696e6720796f75722\
        0627261696e206c696b65206120706f69\
        736f6e6f7573206d757368726f6f6d"
        .to_string();

    let res = base64::base64_enc(data).unwrap();

    println!("[set1/ch1] res: {:?}", res);

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

    let res = base64::base64_enc(data).unwrap();

    println!("[set1/ch1, 1] res: {:?}", res);

    assert_eq!(res, String::from("ASNFCg=="));
}

#[test]
fn test_base64_2_pad() {
    let data = "abcdefab".to_string();

    let res = base64::base64_enc(data).unwrap();

    println!("[set1/ch1, 2] res: {:?}", res);

    assert_eq!(res, String::from("q83vqw=="));
}
