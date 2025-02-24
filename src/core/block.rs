// block.rs - Block-Darstellung
use chrono::Utc;
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: String,
    pub transactions: Vec<String>, // Transaktionen können Identitätsoperationen enthalten
    pub previous_hash: String,
    pub nonce: u64,
    pub hash: String,
}

impl Block {
    /// Erstellt einen neuen Block.
    pub fn new(index: u64, transactions: Vec<String>, previous_hash: String) -> Self {
        let timestamp = Utc::now().to_rfc3339();
        Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            nonce: 0,
            hash: String::new(),
        }
    }

    /// Berechnet den Hash des Blocks.
    pub fn calculate_hash(&self) -> String {
        let data = format!(
            "{}{}{}{}{}",
            self.index,
            self.timestamp,
            self.transactions.join(","),
            self.previous_hash,
            self.nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    /// Minet den Block, indem der Nonce-Wert iterativ erhöht wird.
    pub fn mine(&mut self, difficulty: u64) {
        while !self.calculate_hash().starts_with(&"0".repeat(difficulty as usize)) {
            self.nonce += 1;
        }
        self.hash = self.calculate_hash();
    }
}