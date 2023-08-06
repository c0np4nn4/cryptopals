use num_bigint::BigUint;
use num_traits::{Num, Zero};
use std::str::FromStr;
use utils::crypto::DH::dh::DH;

struct MockClient<'a> {
    name: &'a str,
    cipher: DH,
}

#[test]
fn chal_33() {
    let p: BigUint= BigUint::from_str_radix("ffffffffffffffffc90fdaa22168c234c4c6628b80dc1cd129024e088a67cc74020bbea63b139b22514a08798e3404ddef9519b3cd3a431b302b0a6df25f14374fe1356d6d51c245e485b576625e7ec6f44c42e9a637ed6b0bff5cb6f406b7edee386bfb5a899fa5ae9f24117c4b1fe649286651ece45b3dc2007cb8a163bf0598da48361c55d39a69163fa8fd24cf5f83655d23dca3ad961c62f356208552bb9ed529077096966d670c354e4abc9804f1746c08ca237327ffffffffffffffff", 16).unwrap();
    let g: BigUint = BigUint::from_str("2").unwrap();

    // generate clients
    let mut alice: MockClient = MockClient {
        name: "alice",
        cipher: DH::new(&p, &g),
    };

    let mut bob: MockClient = MockClient {
        name: "bob",
        cipher: DH::new(&p, &g),
    };

    // generate public key
    alice.cipher.generate_pub_key().unwrap();
    bob.cipher.generate_pub_key().unwrap();

    // generate shared key
    alice.cipher.generate_shared_key().unwrap();
    bob.cipher.generate_shared_key().unwrap();

    assert_eq!(
        //
        alice.cipher.shared_key,
        bob.cipher.shared_key
    );
}
