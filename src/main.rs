// main.rs - Einstiegspunkt der Anwendung
use std::sync::{Arc, Mutex};
use warp::Filter;
use crate::core::blockchain::Blockchain;
use crate::api::routes::define_routes;
use crate::consensus::pow::proof_of_work;

mod core;
mod api;
mod consensus;
mod crypto;
mod network;
mod utils;

#[tokio::main]
async fn main() {
    // Initialisiere die Blockchain
    let blockchain = Arc::new(Mutex::new(Blockchain::new()));

    // Starte den API-Server
    let routes = define_routes(blockchain.clone(), ());

    println!("Starting API server on http://127.0.0.1:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}