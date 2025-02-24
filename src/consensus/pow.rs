// pow.rs - Proof-of-Work Implementierung
use crate::core::block::Block;

/// Führt den Proof-of-Work-Mining-Prozess für einen Block aus.
pub fn proof_of_work(block: &mut Block, difficulty: u64) {
    while !block.calculate_hash().starts_with(&"0".repeat(difficulty as usize)) {
        block.nonce += 1;
    }
    block.hash = block.calculate_hash();
}