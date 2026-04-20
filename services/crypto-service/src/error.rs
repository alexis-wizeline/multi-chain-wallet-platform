use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;

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
    #[error("serilazation error: {0}")]
    Serialzation(String),
    #[error("rpc error: {0}")]
    Rpc(String),
    #[error("internal error: {0} ")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::AccountNotFound | AppError::WalletNotFound => StatusCode::NOT_FOUND,
            AppError::InvalidRequest(_)
            | AppError::ValidationFailed(_)
            | AppError::Serialzation(_) => StatusCode::BAD_REQUEST,
            AppError::Rpc(_) | AppError::Internal(_) => StatusCode::BAD_GATEWAY,
        };

        let body = Json(json!({
            "error": self.to_string()
        }));

        (status, body).into_response()
    }
}
