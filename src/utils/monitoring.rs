// monitoring.rs - Überwachung und Telemetrie
use log::{info, warn, error};

/// Überwacht den Netzwerkstatus und loggt den entsprechenden Status.
pub fn monitor_network_status(status: &str) {
    match status {
        "healthy" => info!("Network is healthy."),
        "warning" => warn!("Network performance degraded."),
        "critical" => error!("Critical network failure detected."),
        _ => warn!("Unbekannter Netzwerkstatus: {}", status),
    }
}

/// Beispiel: Erweiterte Funktion, die auch zusätzliche Metriken loggt.
pub fn log_network_metrics(latency: u64, throughput: u64) {
    info!("Network Metrics - Latency: {}ms, Throughput: {}kbps", latency, throughput);
}
