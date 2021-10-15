mod block;
mod blockchain;
mod hashable;
mod primitives;
mod transaction;
mod byteable;

use block::Block;
use blockchain::Blockchain;
use std::collections::HashSet;
use transaction::{Transaction, Output};
use primitives::now;

fn main() {
    let difficulty = 0x0000ffffffffffffffffffffffffffff;
    let mut block = Block::new(
        0,
        0,
        vec![0; 32],
        vec![
            Transaction {
                inputs: vec![],
                outputs: vec![
                    Output {
                        to_addr: "Alice".to_owned(),
                        value: 50
                    },
                    Output {
                        to_addr: "Bob".to_owned(),
                        value: 50
                    }
                ]
            }
        ],
        difficulty,
    );
    block.mine();
    println!("Genesis block: {}", &block);

    // Initiate chain with genesis
    let mut prev_hash = block.hash.clone();
    let mut blockchain = Blockchain::new();
    blockchain.update_with_block(block).unwrap();

    for i in 1..10 {
        block = Block::new(i, now(), prev_hash, vec![
            Transaction {
                inputs: vec![],
                outputs: vec![
                    Output {
                        to_addr: "Chris".to_owned(),
                        value: 1234
                    }
                ]
            },

        ], difficulty);

        block.mine();
        prev_hash = block.hash.clone();
        blockchain.update_with_block(block).unwrap();
        println!("Block {} mined!", i);
    }

    println!("Blockchain: \n{}", &blockchain);
    println!("Valid: {:?}", &blockchain.verify());
}
