// blockchain.rs - Management der Blockchain
use crate::core::block::Block;

#[derive(Debug)]
pub struct Blockchain {
    chain: Vec<Block>,
    difficulty: u64,
}

impl Blockchain {
    /// Erstellt eine neue Blockchain mit einem Genesis-Block.
    pub fn new() -> Self {
        let genesis_block = Block::new(0, vec!["Genesis".to_string()], "0".to_string());
        genesis_block.mine(2); // Genesis-Block wird automatisch gemint
        Blockchain {
            chain: vec![genesis_block],
            difficulty: 2,
        }
    }

    /// Fügt einen neuen Block zur Blockchain hinzu.
    pub fn add_block(&mut self, transactions: Vec<String>) {
        let previous_block = self.chain.last().unwrap();
        let mut new_block = Block::new(
            previous_block.index + 1,
            transactions,
            previous_block.hash.clone(),
        );
        new_block.mine(self.difficulty);
        self.chain.push(new_block);
    }

    /// Überprüft die Gültigkeit der Blockchain.
    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            if current_block.hash != current_block.calculate_hash() {
                return false;
            }

            if current_block.previous_hash != previous_block.hash {
                return false;
            }
        }
        true
    }
}