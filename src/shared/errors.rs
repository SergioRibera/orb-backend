use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Not found")]
    NotFound,
    #[error("Conflict: {0}")]
    Conflict(String),
    #[error("Internal error: {0}")]
    Internal(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::Unauthorized => {
                HttpResponse::Unauthorized().json(serde_json::json!({ "error": "Unauthorized" }))
            }
            AppError::NotFound => {
                HttpResponse::NotFound().json(serde_json::json!({ "error": "Not found" }))
            }
            AppError::Conflict(msg) => {
                HttpResponse::Conflict().json(serde_json::json!({ "error": msg }))
            }
            AppError::Internal(_) => HttpResponse::InternalServerError()
                .json(serde_json::json!({ "error": "Internal server error" })),
        }
    }
}
