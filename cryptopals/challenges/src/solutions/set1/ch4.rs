use std::{collections::HashMap, env::current_dir, fs};

use utils::{
    attack::{single_char_key_attack, SingleCharKeyAttackResult},
    types::decode_hex,
};

#[test]
fn chal_4() {
    println!("{:?}", current_dir().unwrap());

    let ct_candidates = fs::read_to_string("../../data/4.txt")
        .unwrap()
        .trim()
        .to_string();

    let ct_candidates: Vec<String> =
        ct_candidates.split('\n').map(|ct| ct.to_string()).collect();

    let mut attack_result = HashMap::<String, f64>::new();

    for ct in ct_candidates {
        let ct = decode_hex(ct).unwrap();

        let SingleCharKeyAttackResult { pt, score, .. } =
            single_char_key_attack(ct).unwrap();

        attack_result.insert(pt, score);
    }

    let (pt, _score) = attack_result
        .iter()
        .max_by(|a, b| a.1.total_cmp(&b.1))
        .map(|(k, v)| (k, v))
        .ok_or("error")
        .unwrap();

    println!("[set1/ch4] res: {:?}", pt);

    assert_eq!("Now that the party is jumping\n", pt);
}
