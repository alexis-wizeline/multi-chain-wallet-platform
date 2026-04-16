use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::Keypair;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    pub wallets: Arc<RwLock<HashMap<String, Vec<Keypair>>>>,
    pub rpc: Arc<RpcClient>,
}
