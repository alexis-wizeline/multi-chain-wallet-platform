use serde::Deserialize;

use crate::models::signing::SignIntent;

#[derive(Deserialize)]
pub struct SignTxRequest {
    pub wallet_id: String,
    pub account_index: usize,
    pub serialize_tx: String, // base64
    pub intent: SignIntent,
}

#[derive(Deserialize)]
pub struct SingMsRequest {
    pub wallet_id: String,
    pub account_index: usize,
    pub message: String,
}

#[derive(Deserialize)]
pub struct AirdropRequest {
    pub pubkey: String,
    pub lamports: u64,
}
