mod block;
mod blockchain;

use blockchain::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new();

    println!("Mining block 1...");
    blockchain.add_block("Block 1: Learning Rust".to_string());

    println!("Mining block 2...");
    blockchain.add_block("Block 2: Building Blockchain".to_string());

    for block in blockchain.chain {
        println!("{:#?}", block);
    }
}