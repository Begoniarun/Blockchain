use chrono::prelude::*;
use sha2::{Digest, Sha256};
use hex;

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u32,
    pub timestamp: String,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u32,
}

impl Block {
    /// 创建一个新的区块
    pub fn new(index: u32, data: String, previous_hash: String) -> Self {
        let timestamp = Utc::now().to_string();
        let mut block = Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        };
        block.hash = block.calculate_hash();
        block
    }

    /// 计算区块的哈希值
    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!(
            "{}{}{}{}{}",
            self.index, self.timestamp, self.data, self.previous_hash, self.nonce
        ));
        hex::encode(hasher.finalize())
    }

    /// 简单的工作量证明，找到一个满足条件的哈希值
    pub fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        while &self.hash[..difficulty] != target {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
        println!("Block mined: {}", self.hash);
    }
}