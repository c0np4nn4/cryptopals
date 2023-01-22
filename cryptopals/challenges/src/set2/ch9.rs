use utils::padding::pkcs7::pkcs7;

#[test]
fn chal_9() {
    let mut data = "YELLOW SUBMARINE".to_string().as_bytes().to_vec();

    let res = pkcs7(&data, 20);

    println!("res: {:?}", data);
}
