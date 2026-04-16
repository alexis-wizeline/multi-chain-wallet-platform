use crate::{
    models::responses::{AccountResponse, CreateWalletResponse},
    services::wallet_service::create_wallet,
    state::AppState,
};
use axum::{Json, Router, extract::State, routing::post};
use solana_sdk::signer::Signer;

pub fn routes() -> Router<AppState> {
    Router::new().route("/", post(create_wallet_handler))
}

pub async fn create_wallet_handler(state: State<AppState>) -> Json<CreateWalletResponse> {
    let (wallet_id, keypair, mnemonic) = create_wallet();

    let pubkey = keypair.pubkey().to_string();

    state
        .wallets
        .write()
        .await
        .insert(wallet_id.clone(), vec![keypair]);

    Json(CreateWalletResponse {
        wallet_id,
        accounts: vec![AccountResponse { index: 0, pubkey }],
        mnemonic,
    })
}
