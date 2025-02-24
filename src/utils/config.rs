// config.rs - Konfigurationsmanagement
use std::env;
use std::error::Error;

/// Liest eine Umgebungsvariable aus und gibt ein Result zurück.
pub fn get_config(key: &str) -> Result<String, Box<dyn Error>> {
    env::var(key).map_err(|e| format!("Konfiguration '{}' nicht gefunden: {}", key, e).into())
}
