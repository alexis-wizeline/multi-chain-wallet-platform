use serde::Serialize;

#[derive(Serialize)]
pub struct CreateWalletResponse {
    pub wallet_id: String,
    pub accounts: Vec<AccountResponse>,
    pub mnemonic: String,
}

#[derive(Serialize)]
pub struct AccountResponse {
    pub index: u32,
    pub pubkey: String,
}

#[derive(Serialize)]
pub struct SignedTXResponse {
    pub signature: String,
}

#[derive(Serialize)]
pub struct SignedMsResponse {
    pub signed_tx: String,
}

#[derive(Serialize)]
pub struct AirdropResponse {
    pub signature: String,
}

#[derive(Serialize)]
pub struct BalanceResponse {
    pub balance: u64,
}
