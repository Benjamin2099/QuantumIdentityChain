// identity.rs - Verwaltung digitaler Identitäten
use serde::{Serialize, Deserialize};
use crate::crypto::pqc;
use crate::utils::error::AppError; // Angenommener eigener Fehler-Typ

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Identity {
    pub id: String,
    pub public_key: Vec<u8>,
    pub attributes: Vec<String>, // Zusätzliche Attribute der Identität (z. B. Name, Alter)
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
    /// Gibt im Fehlerfall ein AppError zurück.
    pub fn sign_message(&self, message: &[u8], secret_key: &[u8]) -> Result<Vec<u8>, AppError> {
        // Annahme: pqc::sign_message gibt ein Result<Vec<u8>, _> zurück
        pqc::sign_message(message, secret_key)
            .map_err(|e| AppError::InternalError(format!("Signierung fehlgeschlagen: {:?}", e)))
    }

    /// Verifiziert eine Signatur basierend auf dem öffentlichen Schlüssel der Identität.
    /// Gibt ein Result zurück, sodass Fehler explizit behandelt werden können.
    pub fn verify_signature(&self, message: &[u8], signature: &[u8]) -> Result<bool, AppError> {
        // Annahme: pqc::verify_signature gibt ein Result<bool, _> zurück
        pqc::verify_signature(message, signature, &self.public_key)
            .map_err(|e| AppError::InternalError(format!("Verifizierung fehlgeschlagen: {:?}", e)))
    }

    /// Fügt ein neues Attribut zur Identität hinzu.
    pub fn add_attribute(&mut self, attribute: String) {
        self.attributes.push(attribute);
    }
}
