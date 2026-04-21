use reqwest::Client;

use super::model::{RefreshRequest, TokenResponse, VaultaraTokenResponse};
use crate::shared::errors::AppError;

pub fn authorize_url(
    vaultara_url: &str,
    client_id: &str,
    redirect_uri: &str,
) -> String {
    let base = vaultara_url.trim_end_matches('/');
    format!(
        "{}/oauth/authorize?response_type=code&client_id={}&redirect_uri={}&scope=openid+profile+email",
        base,
        urlencoding::encode(client_id),
        urlencoding::encode(redirect_uri),
    )
}

pub async fn exchange_code(
    vaultara_url: &str,
    client_id: &str,
    client_secret: &str,
    redirect_uri: &str,
    code: &str,
) -> Result<TokenResponse, AppError> {
    let token_endpoint = format!("{}/oauth/token", vaultara_url.trim_end_matches('/'));

    let params = [
        ("grant_type", "authorization_code"),
        ("code", code),
        ("redirect_uri", redirect_uri),
        ("client_id", client_id),
        ("client_secret", client_secret),
    ];

    let response = Client::new()
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

    Ok(TokenResponse {
        access_token: token_resp.access_token,
        refresh_token: token_resp.refresh_token.unwrap_or_default(),
        token_type: token_resp.token_type,
    })
}

pub async fn refresh(
    vaultara_url: &str,
    client_id: &str,
    client_secret: &str,
    req: RefreshRequest,
) -> Result<TokenResponse, AppError> {
    let token_endpoint = format!("{}/oauth/token", vaultara_url.trim_end_matches('/'));

    let params = [
        ("grant_type", "refresh_token"),
        ("refresh_token", &req.refresh_token),
        ("client_id", client_id),
        ("client_secret", client_secret),
    ];

    let response = Client::new()
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

    Ok(TokenResponse {
        access_token: token_resp.access_token,
        refresh_token: token_resp.refresh_token.unwrap_or_default(),
        token_type: token_resp.token_type,
    })
}
