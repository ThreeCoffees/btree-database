use std::{
    error::Error,
    fmt::{Debug, Display},
};

use rand::{
    Rng,
    distr::{Distribution, StandardUniform},
    seq::IndexedRandom,
};

use crate::consts::{MAX_RECORD_LENGTH, PADDING_CHAR};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Data {
    letters: Vec<u8>,
}

impl Data {
    fn get_random_letter() -> u8 {
        let allowed_data: &[u8] =
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ 1234567890".as_bytes();
        allowed_data.choose(&mut rand::rng()).unwrap().clone()
    }

    pub fn new(data: Option<Vec<u8>>) -> Self {
        let mut letters = data.unwrap_or(Vec::with_capacity(MAX_RECORD_LENGTH));
        letters.resize(MAX_RECORD_LENGTH, PADDING_CHAR);
        Self { letters }
    }
}

impl Distribution<Data> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Data {
        let random_data: Vec<u8> = (0..(rng.random_range(1..MAX_RECORD_LENGTH)))
            .map(|_| Data::get_random_letter())
            .collect();

        Data::new(Some(random_data))
    }
}

impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = String::from_utf8(self.letters.clone()).unwrap();
        let str = string.trim_end_matches(char::from(0));
        f.pad(&format!("{}", str))
    }
}

impl TryFrom<&[u8]> for Data {
    type Error = Box<dyn Error>;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value[0] == PADDING_CHAR {
            return Err("Empty letters field is not allowed".into());
        }
        Ok(Self::new(Some(value.to_vec())))
    }
}

impl From<&Data> for Vec<u8> {
    fn from(value: &Data) -> Self {
        value.letters.clone()
    }
}
