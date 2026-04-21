use reqwest::Client;

use super::model::{LoginRequest, LoginResponse, RefreshRequest, VaultaraTokenResponse};
use crate::shared::errors::AppError;

const OAUTH_CLIENT_ID: &str = "orb-api";

/// Login via Vaultara's OAuth2 password grant flow.
pub async fn login(
    vaultara_url: &str,
    client_secret: &str,
    req: LoginRequest,
) -> Result<LoginResponse, AppError> {
    let token_endpoint = format!("{}/oauth/token", vaultara_url.trim_end_matches('/'));

    let params = [
        ("grant_type", "password"),
        ("username", &req.email),
        ("password", &req.password),
        ("client_id", OAUTH_CLIENT_ID),
        ("client_secret", client_secret),
    ];

    let client = Client::new();
    let response = client
        .post(&token_endpoint)
        .form(&params)
        .send()
        .await
        .map_err(|e| AppError::Internal(format!("Failed to contact Vaultara: {}", e)))?;

    if !response.status().is_success() {
        return Err(AppError::Unauthorized);
    }

    let token_resp: VaultaraTokenResponse = response
        .json()
        .await
        .map_err(|e| AppError::Internal(format!("Failed to parse token response: {}", e)))?;

    Ok(LoginResponse {
        access_token: token_resp.access_token,
        refresh_token: token_resp.refresh_token.unwrap_or_default(),
        token_type: token_resp.token_type,
    })
}

/// Refresh token via Vaultara's OAuth2 refresh grant flow.
pub async fn refresh(
    vaultara_url: &str,
    client_secret: &str,
    req: RefreshRequest,
) -> Result<LoginResponse, AppError> {
    let token_endpoint = format!("{}/oauth/token", vaultara_url.trim_end_matches('/'));

    let params = [
        ("grant_type", "refresh_token"),
        ("refresh_token", &req.refresh_token),
        ("client_id", OAUTH_CLIENT_ID),
        ("client_secret", client_secret),
    ];

    let client = Client::new();
    let response = client
        .post(&token_endpoint)
        .form(&params)
        .send()
        .await
        .map_err(|e| AppError::Internal(format!("Failed to contact Vaultara: {}", e)))?;

    if !response.status().is_success() {
        return Err(AppError::Unauthorized);
    }

    let token_resp: VaultaraTokenResponse = response
        .json()
        .await
        .map_err(|e| AppError::Internal(format!("Failed to parse token response: {}", e)))?;

    Ok(LoginResponse {
        access_token: token_resp.access_token,
        refresh_token: token_resp.refresh_token.unwrap_or_default(),
        token_type: token_resp.token_type,
    })
}
