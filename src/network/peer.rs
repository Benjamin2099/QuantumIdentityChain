// peer.rs - Peer-Management für P2P-Netzwerke
use std::net::SocketAddr;
use tokio::net::TcpStream;
use log::{info, error};
use std::time::Duration;

/// Repräsentiert einen Peer im Netzwerk.
#[derive(Debug, Clone)]
pub struct Peer {
    pub address: SocketAddr,
}

impl Peer {
    /// Versucht, eine Verbindung zu einem Peer herzustellen.
    /// Bei einem Fehler wird ein erneuter Verbindungsversuch nach kurzer Wartezeit durchgeführt.
    pub async fn connect(address: SocketAddr) -> Result<Self, Box<dyn std::error::Error>> {
        const MAX_RETRIES: usize = 3;
        for attempt in 1..=MAX_RETRIES {
            match TcpStream::connect(address).await {
                Ok(_) => {
                    info!("Verbindung zu {} erfolgreich (Versuch {})", address, attempt);
                    return Ok(Peer { address });
                },
                Err(e) => {
                    error!("Verbindungsversuch {} zu {} fehlgeschlagen: {}", attempt, address, e);
                    tokio::time::sleep(Duration::from_secs(2)).await;
                },
            }
        }
        Err(format!("Verbindung zu {} konnte nach {} Versuchen nicht hergestellt werden", address, MAX_RETRIES).into())
    }

    /// Gibt die Adresse des Peers zurück.
    pub fn get_address(&self) -> SocketAddr {
        self.address
    }
}
