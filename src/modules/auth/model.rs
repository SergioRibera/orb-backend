use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// ── DTOs ──────────────────────────────────────────────────────────────────

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

// ── Internal ──────────────────────────────────────────────────────────────

/// Response from Vaultara's OAuth2 token endpoint
#[derive(Deserialize)]
pub struct VaultaraTokenResponse {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub token_type: String,
    pub expires_in: Option<i64>,
}
