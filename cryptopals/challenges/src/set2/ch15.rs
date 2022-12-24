use utils::padding::pkcs7::verify_pkcs7;

#[test]
fn chal_15() {
    // let mut data = "YELLOW SUBMARINE".to_string().as_bytes().to_vec();
    let mut data = [73, 67, 69, 32, 73, 67, 69, 32, 66, 65, 66, 89, 4, 4, 4, 4].to_vec();

    let res = verify_pkcs7(&mut data, 16);

    assert_eq!(res, true)
}
