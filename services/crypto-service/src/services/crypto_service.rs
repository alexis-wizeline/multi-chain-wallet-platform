use base64::{Engine, prelude::BASE64_STANDARD};
use bincode::deserialize;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
};

use crate::{
    error::AppError, models::signing::SignIntent,
    services::validation_service::validate_transaction,
};

pub fn sign_and_send_trasaction(
    client: &RpcClient,
    keypair: &Keypair,
    serialized_tx: String,
    intent: SignIntent,
) -> Result<String, AppError> {
    let tx_bytes = BASE64_STANDARD
        .decode(serialized_tx)
        .map_err(|e| AppError::Serialzation(format!("base64 decode failed: {}", e)))?;

    let mut tx: Transaction = deserialize(&tx_bytes)
        .map_err(|e| AppError::Serialzation(format!("transaction decode failed: {}", e)))?;

    validate_transaction(&tx, &keypair.pubkey(), &intent)?;

    let recent_blockhash = client
        .get_latest_blockhash()
        .map_err(|e| AppError::Rpc(format!("get_latest_blockhash failed: {}", e)))?;
    tx.sign(&[keypair], recent_blockhash);

    let signature = client
        .send_and_confirm_transaction(&tx)
        .map_err(|e| AppError::Rpc(format!("send_and_confirm_transaction failed: {}", e)))?;

    Ok(signature.to_string())
}

pub fn sign_message(keypair: &Keypair, message: &[u8]) -> String {
    BASE64_STANDARD.encode(keypair.sign_message(message).as_ref())
}
