use utils::base64;

#[test]
fn chal_1() {
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
