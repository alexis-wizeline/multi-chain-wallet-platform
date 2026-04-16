use base64::{Engine, prelude::BASE64_STANDARD};
use solana_sdk::{hash::Hash, signature::Keypair, signer::Signer, transaction::Transaction};
use wincode::{deserialize, serialize};

pub fn sign_trasaction(
    keypair: &Keypair,
    serialized_tx: String,
    block: Hash,
) -> Result<String, String> {
    let tx_bytes = BASE64_STANDARD
        .decode(serialized_tx)
        .map_err(|e| e.to_string())?;

    let mut tx: Transaction = deserialize(&tx_bytes).map_err(|e| e.to_string())?;

    tx.sign(&[keypair], block);

    let signed_bytes = serialize(&tx).map_err(|e| e.to_string())?;

    Ok(BASE64_STANDARD.encode(signed_bytes))
}

pub fn sign_message(keypair: &Keypair, message: &[u8]) -> String {
    BASE64_STANDARD.encode(keypair.sign_message(message).as_ref())
}
