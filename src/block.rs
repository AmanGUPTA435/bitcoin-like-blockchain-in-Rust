use std::time::SystemTime;
pub type Result<T> = std::result::Result<T, failure::Error>;
use crypto::{digest::Digest, sha2::Sha256};
use log::info;
use serde::{Deserialize,Serialize};

use crate::transaction::Transaction;
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Block{
    timestamp: u128,
    transactions: Vec<Transaction>,
    prev_block_hash: String,
    hash: String,
    height: usize,
    nonce: i32
}

#[derive(Debug)]
pub struct Blockchain{
    blocks: Vec<Block>
}

const TARGET_HEXT: usize = 4;

impl Block{
    pub fn get_transaction(&self) -> &Vec<Transaction>{
        &self.transactions
    }
    pub fn new_block(data:Vec<Transaction>, prev_block_hash: String, height: usize) -> Result<Block>{
        let timestamp: u128 = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_millis();
        let mut block = Block{
            timestamp: timestamp,
            transactions: data,
            prev_block_hash,
            hash: String::new(),
            height,
            nonce: 0
        };
        block.run_proof_of_work()?;
        Ok(block)
    }
    fn run_proof_of_work(&mut self) -> Result<()>{
        info!("Mining the block");
        while !self.validate()?{
            self.nonce += 1;
        }
        let data = self.prepare_hash_data()?;
        let mut hasher = Sha256::new();
        hasher.input(&data[..]);
        self.hash = hasher.result_str();
        Ok(())
    }
    pub fn get_hash(&self) -> String{
        self.hash.clone()
    }

    fn validate(&self) -> Result<bool>{
        let data = self.prepare_hash_data()?;
        let mut hasher = Sha256::new();
        hasher.input(&data[..]);
        let mut vec1: Vec<u8> = vec![];
        vec1.resize(TARGET_HEXT, '0' as u8);
        Ok(&hasher.result_str()[0..TARGET_HEXT] == String::from_utf8(vec1)?)
    }

    fn prepare_hash_data(&self) -> Result<Vec<u8>>{
        let content = (
            self.prev_block_hash.clone(),
            self.transactions.clone(),
            self.timestamp,
            TARGET_HEXT,
            self.nonce
        );
        let bytes = bincode::serialize(&content)?;
        Ok(bytes)
    }
    pub fn new_genesis_block(coinbase: Transaction) -> Block{
        Block::new_block(vec![coinbase], String::new(), 0).unwrap()
    }
    pub fn get_prev_hash(&self) -> String {
        self.prev_block_hash.clone()
    }
}

// impl Blockchain{
//     pub fn new() -> Blockchain{
//         Blockchain{
//             blocks: vec![Block::new_genesis_block()]
//         }
//     }
//     pub fn add_block(&mut self, data: String) -> Result<(),Box<dyn std::error::Error>>{
//         let prev = self.blocks.last().unwrap();
//         let new_block = Block::new_block(data, prev.get_hash(), prev.height+1)?;
//         self.blocks.push(new_block);
//         Ok(())
//     }
// }

