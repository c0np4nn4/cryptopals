use crate::hamming_distance::get_hamming_distance;

#[test]
fn test_hamming_distance() {
    let l = "this is a test".to_string().as_bytes().to_vec();
    let r = "wokka wokka!!!".to_string().as_bytes().to_vec();

    let res = get_hamming_distance(&l, &r).unwrap();
    println!("{:?}", res);
}
