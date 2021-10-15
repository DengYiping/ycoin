use super::hashable::Hashable;
use super::byteable::Byteable;
use super::primitives::*;
use std::collections::HashSet;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Output {
    pub to_addr: Address,
    pub value: u64,
}

#[derive(Debug)]
pub struct Transaction {
    pub inputs: Vec<Output>,
    pub outputs: Vec<Output>,
}

impl Byteable for Output {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(self.to_addr.as_bytes());
        bytes.extend(&u64_bytes(&self.value));
        bytes
    }
}

impl Byteable for Transaction {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(self.outputs.bytes());
        bytes.extend(self.inputs.bytes());
        bytes
    }
}

impl Transaction {
    pub fn input_value(&self) -> u64 {
        self.inputs.iter().map(|input| input.value).sum()
    }

    pub fn output_value(&self) -> u64 {
        self.outputs.iter().map(|output| output.value).sum()
    }

    pub fn input_hashes(&self) -> HashSet<YHash> {
        self.inputs
            .iter()
            .map(|input| input.hash())
            .collect::<HashSet<YHash>>()
    }

    pub fn output_hashes(&self) -> HashSet<YHash> {
        self.outputs
            .iter()
            .map(|output| output.hash())
            .collect::<HashSet<YHash>>()
    }

    pub fn is_coinbase(&self) -> bool {
        self.inputs.len() == 0
    }
}
