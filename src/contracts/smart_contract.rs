// smart_contract.rs - Smart Contract-Funktionalität
#[derive(Debug)]
pub struct SmartContract {
    pub name: String,
    pub conditions: Vec<String>, // Bedingungen des Smart Contracts
}

impl SmartContract {
    /// Erstellt einen neuen Smart Contract.
    pub fn new(name: String, conditions: Vec<String>) -> Self {
        SmartContract { name, conditions }
    }

    /// Führt den Smart Contract aus und überprüft die Bedingungen.
    pub fn execute(&self, input: &str) -> bool {
        for condition in &self.conditions {
            if !condition.contains(input) {
                return false; // Wenn eine Bedingung nicht erfüllt ist, wird false zurückgegeben.
            }
        }
        true // Alle Bedingungen wurden erfüllt.
    }
}