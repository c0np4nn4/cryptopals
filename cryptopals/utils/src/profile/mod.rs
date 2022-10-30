use core::fmt;

use crate::{
    crypto::aes::{decrypt_ecb, encrypt_ecb},
    padding::pkcs7::trim_pkcs7,
    BoxedError,
};

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub enum Role {
    User,
    Admin,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Role::User => write!(f, "user"),
            Role::Admin => write!(f, "admin"),
        }
    }
}

#[derive(Debug)]
pub struct Profile {
    pub email: String,
    pub uid: u32,
    pub role: Role,
}

impl Profile {
    pub fn encode(&self) -> String {
        format!(
            "\
            email={}&\
            uid={}&\
            role={}",
            self.email, self.uid, self.role
        )
    }

    pub fn parsing_profile(encoded_profile: String) -> Result<Profile, BoxedError> {
        let mut profile = encoded_profile.split('&').collect::<Vec<&str>>();

        if profile.len() != 3 {
            return Err(format!("expected encoded_profile").into());
        }

        profile.iter_mut().for_each(|field| {
            let data = field.split('=').collect::<Vec<&str>>();

            *field = data[1];
        });

        let email = profile[0].to_string();

        let uid = profile[1].parse::<u32>()?;

        let role = {
            match profile[2] {
                "user" => Role::User,
                "admin" => Role::Admin,
                _ => {
                    return Err(format!("invalid role has been found, role: {}", profile[2]).into())
                }
            }
        };

        let res = Profile { email, uid, role };

        Ok(res)
    }

    // suppose that security key is managed by the person who had been created the profile
    pub fn encrypt_aes(&self, key: Vec<u8>) -> Vec<u8> {
        encrypt_ecb(key, &mut self.encode().as_bytes().to_vec())
    }

    pub fn decrypt_aes(key: Vec<u8>, encrypted_profile: Vec<u8>) -> Result<Profile, BoxedError> {
        let mut res = decrypt_ecb(key, encrypted_profile);

        trim_pkcs7(&mut res, 16).unwrap();

        // Profile::parsing_profile(res)
        Profile::parsing_profile(String::from_utf8(res).unwrap())
    }
}

fn check_email_address_validity(email_address: &String) -> bool {
    if email_address.split('@').collect::<Vec<&str>>().len() != 2 {
        return false;
    }

    if email_address.contains('&') {
        return false;
    }

    if email_address.contains('=') {
        return false;
    }

    true
}

pub fn profile_for(email_address: String) -> Result<Profile, BoxedError> {
    if !check_email_address_validity(&email_address) {
        return Err(format!("Invalid character has been found in parameter",).into());
    }

    let profile = Profile {
        email: email_address,
        uid: 10,
        role: Role::User,
    };

    Ok(profile)
}
