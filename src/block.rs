use std::fmt::{ self, Debug, Formatter };
use super::*;
pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub hash: BlockHash,
    pub previous_hash: BlockHash,
    pub nonce: u64,
    pub payload: String,
}

impl Debug for Block {
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Block")
    }
}

impl Block {
    pub fn new() -> Self {
        Self {
            index: 0,
            timestamp: now(),
            hash: vec![],
            previous_hash: vec![],
            nonce: 0,
            payload: String::new(),
        }
    }
}