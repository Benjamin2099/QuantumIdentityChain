// identity.rs - Verwaltung digitaler Identitäten
use serde::{Serialize, Deserialize};
use crate::crypto::pqc;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Identity {
    pub id: String,
    pub public_key: Vec<u8>,
    pub attributes: Vec<String>, // Zusätzliche Attribute der Identität (z. B. Name, Alter)
}

impl Identity {
    /// Erstellt eine neue digitale Identität.
    pub fn new(id: String, attributes: Vec<String>) -> Self {
        let (public_key, _) = pqc::generate_pqc_keypair();
        Identity {
            id,
            public_key,
            attributes,
        }
    }

    /// Signiert eine Nachricht mit dem privaten Schlüssel der Identität.
    pub fn sign_message(&self, message: &[u8], secret_key: &[u8]) -> Vec<u8> {
        pqc::sign_message(message, secret_key)
    }

    /// Verifiziert eine Signatur basierend auf der öffentlichen Schlüssel der Identität.
    pub fn verify_signature(&self, message: &[u8], signature: &[u8]) -> bool {
        pqc::verify_signature(message, signature, &self.public_key)
    }

    /// Fügt ein neues Attribut zur Identität hinzu.
    pub fn add_attribute(&mut self, attribute: String) {
        self.attributes.push(attribute);
    }
}