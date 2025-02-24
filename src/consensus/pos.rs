// pos.rs - Proof-of-Stake Implementierung
use rand::Rng;
use std::collections::HashMap;

/// Wählt einen Validator basierend auf dem Stake-Anteil aus.
pub fn select_validator(stake_distribution: &HashMap<String, u64>) -> Option<String> {
    let total_stake: u64 = stake_distribution.values().sum();
    if total_stake == 0 {
        return None; // Kein Stake vorhanden
    }

    let mut rng = rand::thread_rng();
    let random_value = rng.gen_range(0..total_stake);

    let mut cumulative_stake = 0;
    for (validator, stake) in stake_distribution {
        cumulative_stake += stake;
        if random_value < cumulative_stake {
            return Some(validator.clone());
        }
    }
    None
}

/// Simuliert das Hinzufügen eines Blocks durch einen ausgewählten Validator.
pub fn add_block_by_validator(
    blockchain: &mut Vec<Block>,
    transactions: Vec<String>,
    validator: String,
    previous_hash: String,
    difficulty: u64,
) {
    let mut new_block = Block::new(
        blockchain.len() as u64,
        transactions,
        previous_hash,
    );
    new_block.mine(difficulty);
    println!("Block wurde von Validator {} hinzugefügt.", validator);
    blockchain.push(new_block);
}