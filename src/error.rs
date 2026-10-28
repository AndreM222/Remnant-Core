//! Application error types.
//!
//! Uses thiserror for defining error variants and anyhow for
//! propagating errors through the application layer.
//! API errors implement IntoResponse so Axum can return them directly.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

/// Top-level application error type.
/// All errors implement IntoResponse so handlers can use ? directly.
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Rate limit exceeded")]
    RateLimited,

    #[error("Database error: {0}")]
    Database(#[from] sea_orm::DbErr),

    #[error("Cache error: {0}")]
    Cache(String),

    #[error("TMDB error: {0}")]
    Tmdb(String),

    #[error("Internal server error")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::RateLimited => (
                StatusCode::TOO_MANY_REQUESTS,
                "Rate limit exceeded".to_string(),
            ),
            AppError::Database(e) => {
                tracing::error!("Database error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
            AppError::Cache(e) => {
                tracing::error!("Cache error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
            AppError::Tmdb(e) => {
                tracing::error!("TMDB error: {}", e);
                (
                    StatusCode::BAD_GATEWAY,
                    "Upstream service error".to_string(),
                )
            }
            AppError::Internal(e) => {
                tracing::error!("Internal error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
        };

        let body = Json(json!({
            "error": message,
            "status": status.as_u16()
        }));

        (status, body).into_response()
    }
}

/// Convenience type alias for handler results.
pub type AppResult<T> = Result<T, AppError>;
