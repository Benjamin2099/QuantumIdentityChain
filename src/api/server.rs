// server.rs - API-Server für die Blockchain
use warp::Filter;
use crate::core::blockchain::Blockchain;
use crate::contracts::smart_contract::SmartContract;

/// Startet den API-Server.
pub async fn start_server(blockchain: Blockchain, contract: SmartContract) {
    let chain_route = warp::path!("chain")
        .and(warp::any().map(move || blockchain.clone()))
        .and_then(get_chain);

    let contract_route = warp::path!("contract" / String)
        .and(warp::any().map(move || contract.clone()))
        .and_then(execute_smart_contract);

    warp::serve(chain_route.or(contract_route)).run(([127, 0, 0, 1], 3030)).await;
}

/// Gibt die aktuelle Blockchain-Kette zurück.
async fn get_chain(blockchain: Blockchain) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&blockchain.chain))
}

/// Führt einen Smart Contract aus.
async fn execute_smart_contract(contract: SmartContract, input: String) -> Result<impl warp::Reply, warp::Rejection> {
    let result = contract.execute(&input);
    Ok(warp::reply::json(&result))
}
