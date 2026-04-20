use solana_client::{
    rpc_client::RpcClient,
    rpc_config::{CommitmentConfig, RpcSimulateTransactionConfig},
};
use solana_sdk::transaction::Transaction;

use crate::{error::AppError, models::simulation::SimulationOutcome};

pub fn simulate_transaction(
    client: &RpcClient,
    tx: &Transaction,
) -> Result<SimulationOutcome, AppError> {
    let config = RpcSimulateTransactionConfig {
        sig_verify: false,
        replace_recent_blockhash: true,
        commitment: Some(CommitmentConfig::processed()),
        ..Default::default()
    };

    let response = client
        .simulate_transaction_with_config(tx, config)
        .map_err(|e| AppError::Rpc(format!("simulate_transaction_config failed: {}", e)))?;

    if let Some(err) = response.value.err {
        let logs = response.value.logs.unwrap_or_default().join("\n");

        return Err(AppError::SimulationFailed(format!(
            "simulation error: {:?}\nlogs:\n{}",
            err, logs
        )));
    }

    Ok(SimulationOutcome {
        logs: response.value.logs.unwrap_or_default(),
        units_consumed: response.value.units_consumed,
    })
}
