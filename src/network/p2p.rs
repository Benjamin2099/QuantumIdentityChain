// p2p.rs - P2P-Netzwerklogik
use tokio::net::{TcpListener, TcpStream};
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use log::{info, error};

/// Startet einen P2P-Server.
pub async fn start_server(address: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(address).await?;
    info!("Listening on {}", address);

    loop {
        let (stream, peer_addr) = listener.accept().await?;
        info!("Neue Verbindung von: {}", peer_addr);

        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream).await {
                error!("Fehler bei der Verarbeitung der Verbindung von {}: {}", peer_addr, e);
            }
        });
    }
}

/// Verarbeitet eingehende Verbindungen und liest eine Nachricht.
async fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = [0u8; 1024];
    // Lese Daten aus dem Stream
    let n = stream.read(&mut buffer).await?;
    if n == 0 {
        return Ok(()); // Verbindung wurde geschlossen
    }
    let received = String::from_utf8_lossy(&buffer[..n]);
    info!("Nachricht empfangen: {}", received);

    // Beispiel: Sende eine Bestätigung zurück
    let response = format!("Nachricht erhalten: {}", received);
    stream.write_all(response.as_bytes()).await?;
    Ok(())
}
