// pbft.rs - Practical Byzantine Fault Tolerance
use std::collections::HashMap;

/// Repräsentiert einen PBFT-Knoten.
#[derive(Debug)]
pub struct PBFTRound {
    pub phase: String,          // Aktuelle Phase (z. B. "PREPARE", "COMMIT")
    pub messages: HashMap<String, String>, // Empfangene Nachrichten
    pub validators: Vec<String>, // Liste der Validatoren
}

impl PBFTRound {
    /// Initialisiert einen neuen PBFT-Rundenzyklus.
    pub fn new(validators: Vec<String>) -> Self {
        PBFTRound {
            phase: "PREPARE".to_string(),
            messages: HashMap::new(),
            validators,
        }
    }

    /// Verarbeitet eine eingehende Nachricht.
    pub fn process_message(&mut self, sender: String, message: String) {
        if !self.validators.contains(&sender) {
            println!("Ungültiger Sender: {}", sender);
            return;
        }

        match self.phase.as_str() {
            "PREPARE" => {
                if !self.messages.contains_key(&sender) {
                    self.messages.insert(sender, message);
                    println!("Nachricht in PREPARE-Phase empfangen von {}", sender);
                }
            }
            "COMMIT" => {
                if !self.messages.contains_key(&sender) {
                    self.messages.insert(sender, message);
                    println!("Nachricht in COMMIT-Phase empfangen von {}", sender);
                }
            }
            _ => println!("Unbekannte Phase: {}", self.phase),
        }

        // Wechsle zur nächsten Phase, wenn genügend Nachrichten empfangen wurden.
        if self.messages.len() >= (2 * self.validators.len() / 3) {
            self.next_phase();
        }
    }

    /// Wechselt zur nächsten Phase des PBFT-Protokolls.
    fn next_phase(&mut self) {
        match self.phase.as_str() {
            "PREPARE" => {
                self.phase = "COMMIT".to_string();
                println!("Wechsel zur COMMIT-Phase.");
            }
            "COMMIT" => {
                self.phase = "DECIDE".to_string();
                println!("Konsens erreicht: DECIDE-Phase.");
            }
            _ => println!("Konsens abgeschlossen."),
        }
    }
}