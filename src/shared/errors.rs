use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[allow(dead_code)]
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
        let msg = self.to_string();
        match self {
            AppError::Unauthorized => {
                HttpResponse::Unauthorized().json(serde_json::json!({ "message": msg }))
            }
            AppError::NotFound => {
                HttpResponse::NotFound().json(serde_json::json!({ "message": msg }))
            }
            AppError::Conflict(_) => {
                HttpResponse::Conflict().json(serde_json::json!({ "message": msg }))
            }
            AppError::Internal(_) => HttpResponse::InternalServerError()
                .json(serde_json::json!({ "message": "Internal server error" })),
        }
    }
}
