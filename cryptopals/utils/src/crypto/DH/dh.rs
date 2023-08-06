use num_bigint::{BigUint, RandBigInt};
use num_traits::Zero;

use crate::crypto::CryptoError;

pub struct DH {
    priv_key: BigUint,
    pub p: BigUint,
    pub g: BigUint,
    pub pub_key: BigUint,
    pub shared_key: BigUint,
}

impl DH {
    pub fn new(p: &BigUint, g: &BigUint) -> Self {
        DH {
            p: p.clone(),
            g: g.clone(),
            priv_key: Zero::zero(),
            pub_key: Zero::zero(),
            shared_key: Zero::zero(),
        }
    }

    pub fn generate_pub_key(&mut self) -> Result<(), CryptoError> {
        self.pub_key = self.g.modpow(&self.priv_key, &self.p);

        Ok(())
    }

    pub fn generate_priv_key(&mut self) -> Result<(), CryptoError> {
        let mut rng = rand::thread_rng();

        self.priv_key = rng.gen_biguint(65537);

        Ok(())
    }

    pub fn generate_shared_key(&mut self) -> Result<(), CryptoError> {
        self.shared_key = self.pub_key.modpow(&self.priv_key, &self.p);

        Ok(())
    }

    // pub fn get_shared_key(&self) -> BigUint {
    //     self.shared_key.clone()
    // }

    // pub fn get_pub_key(&self) -> BigUint {
    //     self.pub_key.clone()
    // }
}
