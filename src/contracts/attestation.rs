// attestation.rs - Attestationssystem zur Identitätsverifikation
use serde::{Serialize, Deserialize};
use chrono::Utc;

#[derive(Serialize, Deserialize, Debug)]
pub struct Attestation {
    pub issuer: String,         // Aussteller der Attestation
    pub recipient: String,      // Empfänger der Attestation
    pub attributes: Vec<String>,// Attribute oder Claims
    pub timestamp: String,      // Zeitstempel der Ausstellung
}

impl Attestation {
    /// Erstellt eine neue Attestation.
    pub fn new(issuer: String, recipient: String, attributes: Vec<String>) -> Self {
        Attestation {
            issuer,
            recipient,
            attributes,
            timestamp: Utc::now().to_rfc3339(),
        }
    }

    /// Überprüft, ob ein bestimmtes Attribut vorhanden ist.
    pub fn has_attribute(&self, attribute: &str) -> bool {
        self.attributes.iter().any(|attr| attr == attribute)
    }
}