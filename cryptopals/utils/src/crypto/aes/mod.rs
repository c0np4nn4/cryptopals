mod decrypt;
mod encrypt;
mod key;
pub mod mode;
mod state;

#[cfg(test)]
mod test;

use self::{mode::Mode, state::State};
use crate::{xor::fixed_xor, BoxedError};

type RoundKey = [u8; 16];

pub struct AES {
    pub iv: Option<State>,
    pub mode: Mode,
}

impl AES {
    pub fn new(iv: Option<State>, mode: Mode) -> AES {
        AES { iv, mode }
    }

    fn add_round_key(
        &self,
        prev_state: State,
        round_key: RoundKey,
    ) -> Result<State, BoxedError> {
        let mut next_state =
            fixed_xor(prev_state.state.to_vec(), round_key.to_vec())?;

        Ok(State::from_vec(next_state)?)
    }

    pub fn encrypt(
        &self,
        data: Vec<u8>,
        key: [u8; 16],
    ) -> Result<Vec<u8>, BoxedError> {
        // for test
        println!(
            "\
            [debug::encrypt]\
            \n\tdata (len: {:?}): {:?}\
            \n\tkey (len: {:?}):  {:?}\
            ",
            data.len(),
            data,
            key.len(),
            key,
        );

        let res = self.mix_columns(State::from_vec(data.clone())?)?;

        Ok(res.into_vec()?)
    }

    // pub fn decrypt(&self, key: [u8; 16]) -> Vec<u8> {
    //     self.data.clone()
    // }

    fn xtime(&self, x: u8, mut word: Vec<u8>) -> Result<Vec<u8>, BoxedError> {
        let res = match x {
            1 => self.xtime_1(word)?,
            2 => self.xtime_2(word)?,
            3 => self.xtime_3(word)?,
            _ => {
                return Err(format!("Not implemented yet").into());
            }
        };

        Ok(res)
    }

    #[inline]
    fn xtime_1(&self, word: Vec<u8>) -> Result<Vec<u8>, BoxedError> {
        println!("[debug] xtime_1 has been called");

        Ok(word)
    }

    #[inline]
    fn xtime_2(&self, mut word: Vec<u8>) -> Result<Vec<u8>, BoxedError> {
        println!("[debug] xtime_2 has been called");
        let mut res: Vec<_> = word.drain(1..).collect();

        res.push(0);

        Ok(res)
    }

    #[inline]
    fn xtime_3(&self, word: Vec<u8>) -> Result<Vec<u8>, BoxedError> {
        println!("[debug] xtime_3 has been called");
        let l = self.xtime_1(word.clone())?;
        let r = self.xtime_2(word.clone())?;

        let res = self.addition(l, r)?;

        Ok(res)
    }

    fn addition(&self, l: Vec<u8>, r: Vec<u8>) -> Result<Vec<u8>, BoxedError> {
        Ok(fixed_xor(l, r)?)
    }
}
