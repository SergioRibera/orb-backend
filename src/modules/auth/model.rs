use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

// ── DTOs ──────────────────────────────────────────────────────────────────

#[derive(Serialize, ToSchema)]
pub struct AuthorizeResponse {
    pub url: String,
}

#[derive(Deserialize, IntoParams)]
pub struct CallbackQuery {
    pub code: String,
    pub state: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
}

#[derive(Deserialize, ToSchema)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

// ── Internal ──────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct VaultaraTokenResponse {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub token_type: String,
    pub expires_in: Option<i64>,
}
