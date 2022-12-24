use utils::{attack::aes_bit_flipping::bit_flipping_attack, oracles::Oracle16};

#[test]
fn chal_16() {
    let oracle = Oracle16::new();

    let inject_data = ";admin=true;";

    let delimiter: char = ';';

    let payload = bit_flipping_attack(&oracle, inject_data, delimiter).unwrap();

    let res = oracle.try_to_access_as_admin(payload).unwrap();

    assert_eq!(res, true);
}
