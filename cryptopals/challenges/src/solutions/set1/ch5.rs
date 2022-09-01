use utils::xor::repeating_key_xor;

#[test]
fn test_repeating_key_xor() {
    //
    let pt = "Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal"
        .to_string();

    let key = String::from("ICE");

    let res = repeating_key_xor(pt, key).unwrap();

    println!("[set1/ch5] res: {:?}", res);

    assert_eq!(
        res,
        "\
        0b3637272a2b2e63622c2e69692a2369\
        3a2a3c6324202d623d63343c2a262263\
        24272765272a282b2f20430a652e2c65\
        2a3124333a653e2b2027630c692b2028\
        3165286326302e27282f"
    );
}
