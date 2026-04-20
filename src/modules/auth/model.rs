use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// ── JWT ───────────────────────────────────────────────────────────────────────
#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    pub exp: usize,
}

#[derive(Serialize, Deserialize)]
pub struct RefreshClaims {
    pub sub: String,
    pub exp: usize,
}

// ── DTOs ──────────────────────────────────────────────────────────────────────
#[derive(Deserialize, ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, ToSchema)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
}

#[derive(Deserialize, ToSchema)]
pub struct RefreshRequest {
    pub refresh_token: String,
}
