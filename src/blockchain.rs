use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use log::{info, error};


const DIFFICULTY: &str = "00";


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub id: u32,
    pub timestamp: i64,
    pub header: String,
    pub prev_hash: String,
    pub transactions: Vec<Transaction>,
    pub nonce: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {}

pub struct App {
    pub blocks: Vec<Block>,
}

impl App {
    pub fn new() -> App {
        App {
            blocks: vec![]
        }
    }

    pub fn add_genesis_block(&mut self) {
        self.blocks.push(Block::genesis_block())
    }

    pub fn add_block_to_chain(&mut self, block: Block) { 
        let latest_block = self.blocks.last().unwrap();
        if self.check_block_is_valid(latest_block, &block) {
            self.blocks.push(block)
        } else {
            error!("Received invalid block");
        }
    }

    fn check_block_is_valid(&self, latest_block: &Block, new_block: &Block) -> bool {  
        if latest_block.id + 1 != new_block.id {
            error!("Invalid block ID!");
            return false;
        } else if new_block.timestamp <= latest_block.timestamp {
            error!("Invalid block timestamp!");
            return false;
        } else if new_block.header != hex::encode(calculate_hash(
            &new_block.id, 
            &new_block.timestamp, 
            &new_block.prev_hash, 
            &new_block.transactions, 
            &new_block.nonce
        )) {
            error!("Invalid block header!");
            return false;
        } else if new_block.prev_hash != latest_block.header {
            error!("Previous hash doesn't match!");
            return false; 
        } else if &new_block.header[0..=DIFFICULTY.len()] != DIFFICULTY {
            error!("Difficulty does not match!");
            return false;
        } else {
            return true;
        }
    }

    pub fn choose_chain(&mut self, local_chain: Vec<Block>, new_chain: Vec<Block>) -> Vec<Block> {
        let is_local_valid = self.check_chain_is_valid(&local_chain);
        let is_new_valid = self.check_chain_is_valid(&new_chain);

        if is_local_valid && is_new_valid {
            if new_chain.len() <= local_chain.len() {
                local_chain
            } else {
                new_chain
            }
        } else if is_local_valid && !is_new_valid {
            local_chain
        } else if !is_local_valid && is_new_valid {
            new_chain
        } else {
            panic!("both local and received chains are invalid!");
        }
    }

    fn check_chain_is_valid(&self, chain: &Vec<Block>) -> bool {
        for i in 0..chain.len() {
            if i == 0 {
                continue;
            }

            let first = chain.get(i - 1).expect("prev block must exist");
            let second = chain.get(i).expect("latest block must exist");

            if !self.check_block_is_valid(second, first) {
                return false;
            }
        }

        true
    }
}

impl Block {
    pub fn genesis_block() -> Block {
        Block {
            id: 0,
            timestamp: 0,
            header: String::from("genesis"),
            prev_hash: String::from("genesis"),
            transactions: vec![],
            nonce: 0
        }
    }

    pub fn new(id: u32, prev_hash: String, transactions: Vec<Transaction>) -> Block {
        let timestamp = Utc::now().timestamp();
        let (nonce, header) = mine_block(&id, &timestamp, &prev_hash, &transactions);
        let block = Block {
            id,
            timestamp,
            header,
            prev_hash,
            transactions,
            nonce,
        };

        block
    }
}

pub fn calculate_hash(id: &u32, timestamp: &i64, prev_hash: &String, transactions: &Vec<Transaction>, nonce: &u64) -> Vec<u8> {
    let data = serde_json::json!({
        "id": id,
        "timestamp": timestamp,
        "prev_hash": prev_hash,
        "transactions": transactions,
        "nonce": nonce,
    });

    let mut hasher = Sha256::new();
    hasher.update(data.to_string().as_bytes());
    hasher.finalize().as_slice().to_owned()
}

fn mine_block(id: &u32, timestamp: &i64, prev_hash: &String, transactions: &Vec<Transaction>) -> (u64, String) {
    let mut nonce = 0;

    loop {
        nonce += 1;

        let result = calculate_hash(id, timestamp, prev_hash, transactions, &nonce);

        let r = &result[0..DIFFICULTY.len()]
            .into_iter()
            .map(|n| n.to_string())
            .collect::<String>();

        if r == DIFFICULTY {
            info!("Mined a new block with ID {}", id);
            return (nonce, hex::encode(result))
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}