use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Serialize)]
pub struct ErrorBody {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum AppError {
    #[error("aallet not found")]
    WalletNotFound,
    #[error("account not found")]
    AccountNotFound,
    #[error("invalid request: {0}")]
    InvalidRequest(String),
    #[error("transaction validation failed: {0}")]
    ValidationFailed(String),
    #[error("transaction simulation failed")]
    SimulationFailed { reason: String, logs: Vec<String> },
    #[error("serilazation error: {0}")]
    Serialzation(String),
    #[error("rpc error: {0}")]
    Rpc(String),
    #[error("internal error: {0} ")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, body) = match self {
            AppError::AccountNotFound => (
                StatusCode::NOT_FOUND,
                ErrorBody {
                    code: "WALLET_NOT_FOUND".into(),
                    message: "wallet not found".into(),
                    details: None,
                },
            ),
            AppError::WalletNotFound => (
                StatusCode::NOT_FOUND,
                ErrorBody {
                    code: "ACCOUNT_NOT_FOUND".into(),
                    message: "Account not found".into(),
                    details: None,
                },
            ),
            AppError::InvalidRequest(msg) => (
                StatusCode::BAD_REQUEST,
                ErrorBody {
                    code: "INVALID_REQUEST".into(),
                    message: msg,
                    details: None,
                },
            ),
            AppError::ValidationFailed(msg) => (
                StatusCode::BAD_REQUEST,
                ErrorBody {
                    code: "VALIDATION_FAILED".into(),
                    message: msg,
                    details: None,
                },
            ),
            AppError::Serialzation(msg) => (
                StatusCode::BAD_REQUEST,
                ErrorBody {
                    code: "SIMULATION_FAILED".into(),
                    message: msg,
                    details: None,
                },
            ),
            AppError::SimulationFailed { reason, logs } => (
                StatusCode::BAD_REQUEST,
                ErrorBody {
                    code: "SIMULATION_FAILED".into(),
                    message: "Transaction simulation failed".into(),
                    details: Some(json!({
                        "reason": reason,
                        "logs": logs
                    })),
                },
            ),
            AppError::Rpc(msg) => (
                StatusCode::BAD_GATEWAY,
                ErrorBody {
                    code: "RPC_ERROR".into(),
                    message: msg,
                    details: None,
                },
            ),
            AppError::Internal(msg) => (
                StatusCode::BAD_GATEWAY,
                ErrorBody {
                    code: "INTERNAL_ERROR".into(),
                    message: msg,
                    details: None,
                },
            ),
        };

        (status, Json(json!({ "error":body }))).into_response()
    }
}
