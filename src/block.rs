use hex::encode;
use std::fmt::{Debug, Display, Formatter, Result};

use super::hashable::Hashable;
use super::byteable::Byteable;
use super::primitives::*;
use super::transaction::Transaction;

#[derive(Debug)]
pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub hash: YHash,
    pub previous_block_hash: YHash,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
    pub difficulty: u128,
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "Block[{}]: {} at: {} with {:?}",
            &self.index,
            encode(&self.hash),
            &self.timestamp,
            &self.transactions
        )
    }
}

impl Block {
    pub fn new(
        index: u32,
        timestamp: u128,
        previous_block_hash: YHash,
        transactions: Vec<Transaction>,
        difficulty: u128,
    ) -> Self {
        Block {
            index,
            timestamp,
            hash: vec![0; 32],
            previous_block_hash,
            nonce: 0,
            transactions,
            difficulty,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.difficulty > difficulty_bytes_as_u128(&self.hash)
    }

    pub fn populate_hash(&mut self) {
        self.hash = self.hash()
    }

    pub fn mine(&mut self) {
        for nonce_attempt in 0..(u64::max_value()) {
            self.nonce = nonce_attempt;
            self.populate_hash();
            if self.is_valid() {
                return;
            }
        }
    }
}

impl Byteable for Block {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(&u32_bytes(&self.index));
        bytes.extend(&u128_bytes(&self.timestamp));
        bytes.extend(&self.previous_block_hash);
        bytes.extend(&u64_bytes(&self.nonce));
        bytes.extend(self.transactions.bytes());
        bytes.extend(&u128_bytes(&self.difficulty));
        bytes
    }
}
