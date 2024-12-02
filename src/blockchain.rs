use crate::block::Block;

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    /// 创建一个新的区块链
    pub fn new() -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            difficulty: 4, // 设置工作量证明的难度
        };
        blockchain.create_genesis_block();
        blockchain
    }

    /// 创建创世区块
    fn create_genesis_block(&mut self) {
        let genesis_block = Block::new(0, "Genesis Block".to_string(), "0".to_string());
        self.chain.push(genesis_block);
    }

    /// 获取最后一个区块
    pub fn get_latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    /// 添加新的区块
    pub fn add_block(&mut self, data: String) {
        let previous_block = self.get_latest_block();
        let mut new_block = Block::new(
            previous_block.index + 1,
            data,
            previous_block.hash.clone(),
        );
        new_block.mine_block(self.difficulty);
        self.chain.push(new_block);
    }
}
