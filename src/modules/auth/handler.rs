use actix_web::{get, post, web::{Data, Json, Query}, HttpResponse};

use super::model::{AuthorizeResponse, CallbackQuery, RefreshRequest, TokenResponse};
use super::service;
use crate::db::AppState;
use crate::shared::errors::AppError;

#[utoipa::path(
    get,
    path = "/api/v1/auth/authorize",
    tag = "Auth",
    responses(
        (status = 200, description = "Vaultara authorization URL", body = AuthorizeResponse),
    )
)]
#[get("/authorize")]
pub async fn authorize(state: Data<AppState>) -> Result<HttpResponse, AppError> {
    let url = service::authorize_url(
        &state.vaultara_url,
        &state.vaultara_client_id,
        &state.vaultara_redirect_uri,
    );
    Ok(HttpResponse::Ok().json(AuthorizeResponse { url }))
}

#[utoipa::path(
    get,
    path = "/api/v1/auth/callback",
    tag = "Auth",
    params(CallbackQuery),
    responses(
        (status = 200, description = "Tokens issued", body = TokenResponse),
        (status = 401, description = "Invalid or expired code"),
    )
)]
#[get("/callback")]
pub async fn callback(
    state: Data<AppState>,
    query: Query<CallbackQuery>,
) -> Result<HttpResponse, AppError> {
    let tokens = service::exchange_code(
        &state.vaultara_url,
        &state.vaultara_client_id,
        &state.vaultara_client_secret,
        &state.vaultara_redirect_uri,
        &query.code,
    )
    .await?;
    Ok(HttpResponse::Ok().json(tokens))
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/refresh",
    tag = "Auth",
    request_body = RefreshRequest,
    responses(
        (status = 200, description = "Tokens refreshed", body = TokenResponse),
        (status = 401, description = "Invalid refresh token"),
    )
)]
#[post("/refresh")]
pub async fn refresh(
    state: Data<AppState>,
    body: Json<RefreshRequest>,
) -> Result<HttpResponse, AppError> {
    let tokens = service::refresh(
        &state.vaultara_url,
        &state.vaultara_client_id,
        &state.vaultara_client_secret,
        body.into_inner(),
    )
    .await?;
    Ok(HttpResponse::Ok().json(tokens))
}
