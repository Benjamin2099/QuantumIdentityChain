// encryption.rs - Verschlüsselungslogik für sichere Kommunikation
extern crate aes_gcm;
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};
use crate::utils::error::AppError; // Unser benutzerdefinierter Fehler-Typ

/// Verschlüsselt Daten mit AES-256-GCM.
/// Gibt ein Result zurück, um Fehler während der Verschlüsselung zu behandeln.
pub fn encrypt_data(key: &[u8], nonce: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, AppError> {
    let key = Key::from_slice(key);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce); // 12 bytes
    cipher.encrypt(nonce, plaintext)
         .map_err(|e| AppError::InternalError(format!("Encryption failed: {:?}", e)))
}

/// Entschlüsselt Daten mit AES-256-GCM.
/// Gibt ein Result zurück, um Fehler während der Entschlüsselung zu behandeln.
pub fn decrypt_data(key: &[u8], nonce: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, AppError> {
    let key = Key::from_slice(key);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce); // 12 bytes
    cipher.decrypt(nonce, ciphertext)
         .map_err(|e| AppError::InternalError(format!("Decryption failed: {:?}", e)))
}
