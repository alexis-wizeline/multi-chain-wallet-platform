use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SignIntent {
    SolTransfer {
        from: String,
        to: String,
        lamports: u64,
    },
}
