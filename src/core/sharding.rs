// sharding.rs - Sharding-Logik für Skalierbarkeit
use std::collections::HashMap;

pub struct Shard {
    pub id: u64,
    pub blocks: Vec<String>, // Liste der Blöcke im Shard
}

impl Shard {
    /// Erstellt einen neuen Shard.
    pub fn new(id: u64) -> Self {
        Shard {
            id,
            blocks: Vec::new(),
        }
    }

    /// Fügt einen neuen Block zum Shard hinzu.
    pub fn add_block(&mut self, block: String) {
        self.blocks.push(block);
    }
}

pub struct ShardingSystem {
    shards: HashMap<u64, Shard>,
}

impl ShardingSystem {
    /// Erstellt ein neues Sharding-System.
    pub fn new() -> Self {
        ShardingSystem {
            shards: HashMap::new(),
        }
    }

    /// Erstellt einen neuen Shard.
    pub fn create_shard(&mut self, shard_id: u64) {
        self.shards.entry(shard_id).or_insert_with(|| Shard::new(shard_id));
    }

    /// Fügt einen Block zu einem bestimmten Shard hinzu.
    /// Gibt ein Result zurück, um Fehler (z.B. nicht vorhandener Shard) zu behandeln.
    pub fn add_block_to_shard(&mut self, shard_id: u64, block: String) -> Result<(), String> {
        if let Some(shard) = self.shards.get_mut(&shard_id) {
            shard.add_block(block);
            Ok(())
        } else {
            Err(format!("Shard {} existiert nicht", shard_id))
        }
    }

    /// Gibt einen Shard zurück.
    pub fn get_shard(&self, shard_id: u64) -> Option<&Shard> {
        self.shards.get(&shard_id)
    }
}
