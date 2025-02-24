// routes.rs - REST API-Routen
use warp::Filter;
use crate::core::blockchain::Blockchain;

/// Definiert Routen für die API.
pub fn define_routes(blockchain: Blockchain) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let chain_route = warp::path!("chain")
        .and(warp::any().map(move || blockchain.clone()))
        .and_then(get_chain);

    chain_route
}

/// Gibt die aktuelle Blockchain-Kette zurück.
async fn get_chain(blockchain: Blockchain) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&blockchain.chain))
}
