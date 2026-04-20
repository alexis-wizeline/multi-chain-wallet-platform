use crate::{
    error::AppError,
    models::{
        requests::{SignTxRequest, SingMsRequest},
        responses::{SignedMsResponse, SignedTXResponse},
    },
    services::crypto_service::{sign_and_send_trasaction, sign_message},
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
) -> Result<Json<SignedTXResponse>, AppError> {
    let wallets = state.wallets.read().await;

    let wallet = &wallets
        .get(&req.wallet_id)
        .ok_or(AppError::WalletNotFound)?;

    let account = wallet
        .get(req.account_index)
        .ok_or(AppError::AccountNotFound)?;

    let signature = sign_and_send_trasaction(&state.rpc, account, req.serialize_tx)?;

    Ok(Json(SignedTXResponse { signature }))
}

async fn sign_msg(
    State(state): State<AppState>,
    Json(req): Json<SingMsRequest>,
) -> Result<Json<SignedMsResponse>, AppError> {
    let wallets = state.wallets.read().await;

    let wallet = wallets
        .get(&req.wallet_id)
        .ok_or(AppError::WalletNotFound)?;

    let account = wallet
        .get(req.account_index)
        .ok_or(AppError::AccountNotFound)?;

    let signed_tx = sign_message(account, req.message.as_bytes());

    Ok(Json(SignedMsResponse { signed_tx }))
}
