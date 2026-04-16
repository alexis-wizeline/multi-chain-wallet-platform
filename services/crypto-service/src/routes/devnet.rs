use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{get, post},
};
use solana_sdk::pubkey::Pubkey;

use crate::{
    models::{
        requests::AirdropRequest,
        responses::{AirdropResponse, BalanceResponse},
    },
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/airdrop", post(airdrop))
        .route("/balance/:pubkey", get(balance))
}

async fn airdrop(
    State(state): State<AppState>,
    Json(req): Json<AirdropRequest>,
) -> Json<AirdropResponse> {
    let pubkey: Pubkey = req.pubkey.parse().expect("public key not parsed");

    let sig = state
        .rpc
        .request_airdrop(&pubkey, req.lamports)
        .expect("devnet can only airdrop 1 sol every 8 hours");

    Json(AirdropResponse {
        signature: sig.to_string(),
    })
}

async fn balance(
    Path(pubkey): Path<String>,
    State(state): State<AppState>,
) -> Json<BalanceResponse> {
    let address: Pubkey = pubkey.parse().expect("unable to parse pubkey");

    let balance = state
        .rpc
        .get_balance(&address)
        .expect("not able to get pubkeybalance");

    Json(BalanceResponse { balance })
}
