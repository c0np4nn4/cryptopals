use utils::{attack::padding_oracle_attack::padding_oracle_attack, oracles::Oracle17};

#[test]
fn chal_17() {
    let oracle = Oracle17::new();

    let ct = oracle.ciphertext.clone();
    let iv = oracle.iv.clone();

    let pt = padding_oracle_attack(&oracle, ct, iv);

    let res = oracle.solve(pt);

    // let res = String::from_utf8(base64_dec(String::from_utf8(pt).unwrap()).unwrap()).unwrap();

    assert_eq!(res, true);
}
