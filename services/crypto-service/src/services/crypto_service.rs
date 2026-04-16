use base64::{Engine, prelude::BASE64_STANDARD};
use bincode::deserialize;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
};

pub fn sign_and_send_trasaction(
    client: &RpcClient,
    keypair: &Keypair,
    serialized_tx: String,
) -> Result<String, String> {
    let tx_bytes = BASE64_STANDARD
        .decode(serialized_tx)
        .map_err(|e| e.to_string())?;

    let mut tx: Transaction = deserialize(&tx_bytes).map_err(|e| e.to_string())?;

    let recent_blockhash = client.get_latest_blockhash().map_err(|e| e.to_string())?;
    tx.sign(&[keypair], recent_blockhash);

    let signature = client
        .send_and_confirm_transaction(&tx)
        .map_err(|e| e.to_string())?;

    Ok(signature.to_string())
}

pub fn sign_message(keypair: &Keypair, message: &[u8]) -> String {
    BASE64_STANDARD.encode(keypair.sign_message(message).as_ref())
}
