use super::profile_for;
use crate::{crypto::aes::get_random_aes_key, profile::Profile};

#[test]
pub fn test_profile_for() {
    let email_address = "aaa@bbb.com".to_string();

    let res = profile_for(email_address).unwrap();

    let encoded_profile = res.encode();

    let res = Profile::parsing_profile(encoded_profile).unwrap();

    println!("{:#?}", res);
}

#[test]
pub fn test_profile_with_aes() {
    let email_address = "aaa@bbb.com".to_string();

    let profile = profile_for(email_address).unwrap();

    let key = get_random_aes_key();

    let encrypted_profile = profile.encrypt_aes(key.to_vec());

    let res = Profile::decrypt_aes(key.to_vec(), encrypted_profile).unwrap();

    println!("{:#?}", res);
}
