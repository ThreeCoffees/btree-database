use std::{
    array::TryFromSliceError,
    error::Error,
    fmt::Display,
    fs::File,
    io::{Read, Seek, SeekFrom},
};

use serde::{Deserialize, Serialize};

use crate::{
    btree::{self, BTree}, consts::{MAX_RECORD_LENGTH, PADDING_CHAR}, data::Data
};

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
pub struct Record {
    pub key: u64,
    pub data_id: u64,
}

impl Ord for Record {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.key.cmp(&other.key)
    }
}

impl Record {
    pub fn new(key: u64, data_id: u64) -> Self {
        Self { key, data_id }
    }

    pub fn data_address(&self) -> u64 {
        self.data_id * MAX_RECORD_LENGTH as u64
    }

    pub fn get_data(&mut self, btree: &mut BTree) -> Result<Data, Box<dyn Error>> {
        btree.data_file.get_data(self)
    }

    pub fn write_data(&mut self, btree: &mut BTree, data: &Data) -> Result<(), Box<dyn Error>> {
        btree.data_file.write_data(self, data)
    }
}

impl TryFrom<&[u8]> for Record {
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::new(
            u64::from_le_bytes(value[0..8].try_into()?),
            u64::from_le_bytes(value[8..16].try_into()?),
        ))
    }

    type Error = TryFromSliceError;
}

pub const RECORD_SIZE: usize = 16;

impl From<&Record> for Vec<u8> {
    fn from(value: &Record) -> Self {
        [value.key.to_le_bytes(), value.data_id.to_le_bytes()].concat()
    }
}
