use std::{collections::HashMap, sync::Arc};

use axum::{Router, routing::get};
use solana_client::rpc_client::RpcClient;
use tokio::sync::RwLock;

use crate::{
    routes::{account, devnet, signing, wallet},
    state::AppState,
};

pub async fn create_app(rpc_url: String) -> Router {
    let state = AppState {
        wallets: Arc::new(RwLock::new(HashMap::new())),
        rpc: Arc::new(RpcClient::new(rpc_url)),
    };

    Router::new()
        .route("/health", get(|| async { "ok" }))
        .nest("/wallets", wallet::routes())
        .nest("/accounts", account::routes())
        .nest("/sining", signing::routes())
        .nest("/devnet", devnet::routes())
        .with_state(state)
}
