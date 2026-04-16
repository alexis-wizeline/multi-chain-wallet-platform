use crate::{
    models::responses::AccountResponse, services::wallet_service::derive_account, state::AppState,
};
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{get, post},
};
use solana_sdk::signer::Signer;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/:wallet_id", post(add_account))
        .route("/:wallet_id", get(list_accounts))
}

async fn add_account(
    Path(wallet_id): Path<String>,
    State(state): State<AppState>,
) -> Json<AccountResponse> {
    let mut wallets = state.wallets.write().await;

    let accounts = wallets.get_mut(&wallet_id).unwrap();

    let index = accounts.len() as u32;
    let base = accounts.get(0).expect("master keypair");

    let new_keyp = derive_account(base, index);
    let pubkey = new_keyp.pubkey().to_string();

    accounts.push(new_keyp);

    Json(AccountResponse { index, pubkey })
}

async fn list_accounts(
    Path(wallet_id): Path<String>,
    State(state): State<AppState>,
) -> Json<Vec<AccountResponse>> {
    let wallets = state.wallets.read().await;

    let accounts = wallets.get(&wallet_id).expect("wallet not found");

    let result: Vec<AccountResponse> = accounts
        .iter()
        .enumerate()
        .map(|(i, kp)| AccountResponse {
            index: (i as u32) + 1,
            pubkey: kp.pubkey().to_string(),
        })
        .collect();

    Json(result)
}
