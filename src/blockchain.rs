use super::block::Block;
use super::hashable::Hashable;
use super::primitives::*;
use std::fmt::{Debug, Display};
use std::collections::HashSet;

pub struct Blockchain {
    pub blocks: Vec<Block>,
    unspent_output: HashSet<YHash>,
}

#[derive(Debug)]
pub enum BlockValidationErr {
    MismatchedIndex,
    MismatchedPreviousHash,
    InvalidHash,
    DifficultyMismatch,
    AchronologicalTimestamp,
    InvalidInput,
    InsufficientInputValue,
    InvalidCoinbaseTransaction,
}

impl Display for Blockchain {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut result = String::new();
        for block in self.blocks.iter() {
            result.push_str(&format!(" - {}\n", &block));
        }
        write!(f, "{}", result)
    }
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            blocks: vec![],
            unspent_output: HashSet::new()
        }
    }

    pub fn get_last_block_hash(&self) -> &YHash {
        self.blocks.last().map(|block| &block.hash).unwrap()
    }

    pub fn update_with_block(&mut self, block: Block) -> Result<(), BlockValidationErr> {
        self.verify()?;
        if let Some((coinbase, transactions)) = block.transactions.split_first() {
            if !coinbase.is_coinbase() {
                return Err(BlockValidationErr::InvalidCoinbaseTransaction);
            }

            let mut block_spent: HashSet<YHash> = HashSet::new();
            let mut block_created: HashSet<YHash> = HashSet::new();

            let mut total_fee = 0;
            for transaction in transactions {
                let input_hashes = transaction.input_hashes();

                // Check that all inputs can be spent, and no double spent in the same block
                if !(&input_hashes - &self.unspent_output).is_empty() || !(&input_hashes & &block_spent).is_empty() {
                    return Err(BlockValidationErr::InvalidInput);
                }

                let input_value = transaction.input_value();
                let output_value = transaction.output_value();

                if output_value > input_value {
                    return Err(BlockValidationErr::InsufficientInputValue)
                }

                let fee = input_value - output_value;

                block_spent.extend(input_hashes);
                block_created.extend(transaction.output_hashes());
                total_fee += fee;
            }

            if coinbase.output_value() < total_fee {
                return Err(BlockValidationErr::InvalidCoinbaseTransaction);
            } else {
                block_created.extend(coinbase.output_hashes());
            }

            self.unspent_output.retain(|output| !block_spent.contains(output));
            self.unspent_output.extend(block_created);
        }

        self.blocks.push(block);
        Ok(())
    }

    pub fn verify(&self) -> Result<(), BlockValidationErr> {
        verify_chain(&vec![0; 32], &0, &self.blocks, 0)
    }
}

fn verify_chain(
    prev_hash: &YHash,
    prev_timestamp: &u128,
    chain: &[Block],
    index: u32
) -> Result<(), BlockValidationErr> {
    if chain.len() == 0 {
        Ok(())
    } else {
        let head = &chain[0];
        if head.index != index {
            Err(BlockValidationErr::MismatchedIndex)
        } else if &head.previous_block_hash != prev_hash {
            Err(BlockValidationErr::MismatchedPreviousHash)
        } else if &head.hash() != &head.hash {
            Err(BlockValidationErr::InvalidHash)
        } else if !head.is_valid() {
            Err(BlockValidationErr::DifficultyMismatch)
        } else if prev_timestamp > &head.timestamp {
            Err(BlockValidationErr::AchronologicalTimestamp)
        } else {
            verify_chain(&head.hash, &head.timestamp, &chain[1..], index + 1)
        }
    }
}
