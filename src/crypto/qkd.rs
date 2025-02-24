// qkd.rs - Simulierte Quantum Key Distribution
use std::collections::HashMap;

/// Simuliert den QKD-Prozess zwischen zwei Parteien.
pub struct QKDSession {
    pub keys: HashMap<String, String>, // Teilnehmer und ihre Schlüssel
}

impl QKDSession {
    /// Initialisiert eine neue QKD-Sitzung.
    pub fn new() -> Self {
        QKDSession {
            keys: HashMap::new(),
        }
    }

    /// Simuliert den Austausch eines Schlüssels zwischen zwei Parteien.
    pub fn exchange_key(&mut self, sender: String, receiver: String, key: String) {
        if !self.keys.contains_key(&sender) {
            self.keys.insert(sender.clone(), key.clone());
            println!("Sender {} hat Schlüssel {} generiert.", sender, key);
        }

        if !self.keys.contains_key(&receiver) {
            self.keys.insert(receiver.clone(), key);
            println!("Empfänger {} hat Schlüssel empfangen.", receiver);
        } else {
            println!("Schlüsselaustausch abgeschlossen.");
        }
    }

    /// Überprüft, ob ein Abhörversuch stattgefunden hat.
    pub fn detect_eavesdropping(&self) -> bool {
        let mut keys_iter = self.keys.values();
        if let (Some(first), Some(second)) = (keys_iter.next(), keys_iter.next()) {
            first == second
        } else {
            false
        }
    }
}