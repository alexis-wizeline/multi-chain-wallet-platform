use crate::{
    models::{
        requests::{SignTxRequest, SingMsRequest},
        responses::{SignedMsResponse, SignedTXResponse},
    },
    services::crypto_service::{sign_message, sign_trasaction},
    state::AppState,
};
use axum::{Json, Router, extract::State, routing::post};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/transaction", post(sing_tx))
        .route("message", post(sign_msg))
}

async fn sing_tx(
    State(state): State<AppState>,
    Json(req): Json<SignTxRequest>,
) -> Json<SignedTXResponse> {
    let wallets = state.wallets.read().await;

    let kp = &wallets
        .get(&req.wallet_id)
        .expect("wallet not found")
        .get(req.account_index)
        .expect("Account not found");

    let hash = state.rpc.get_latest_blockhash().expect("latest blockhash");
    let signed = sign_trasaction(kp, req.serialize_tx, hash);

    let mut res = SignedTXResponse {
        signed_tx: String::new(),
        error: String::new(),
    };
    match signed {
        Ok(sig) => res.signed_tx = sig,
        Err(err) => res.error = err,
    };

    Json(res)
}

async fn sign_msg(
    State(state): State<AppState>,
    Json(req): Json<SingMsRequest>,
) -> Json<SignedMsResponse> {
    let wallets = state.wallets.read().await;

    let kp = wallets
        .get(&req.wallet_id)
        .expect("wallet not found")
        .get(req.account_index)
        .expect("Account not found");

    let signed_tx = sign_message(kp, req.message.as_bytes());

    Json(SignedMsResponse { signed_tx })
}
