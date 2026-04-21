use actix_web::{post, web::{Data, Json}, HttpResponse};

use super::model::{LoginRequest, LoginResponse, RefreshRequest};
use super::service;
use crate::db::AppState;
use crate::shared::errors::AppError;

#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    tag = "Auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Successfully authenticated", body = LoginResponse),
        (status = 401, description = "Invalid credentials"),
    )
)]
#[post("/login")]
pub async fn login(
    state: Data<AppState>,
    body: Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    let res = service::login(&state.vaultara_url, &state.jwt_secret, body.into_inner()).await?;
    Ok(HttpResponse::Ok().json(res))
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/refresh",
    tag = "Auth",
    request_body = RefreshRequest,
    responses(
        (status = 200, description = "Successfully refreshed token", body = LoginResponse),
        (status = 401, description = "Invalid refresh token"),
    )
)]
#[post("/refresh")]
pub async fn refresh(
    state: Data<AppState>,
    body: Json<RefreshRequest>,
) -> Result<HttpResponse, AppError> {
    let res = service::refresh(&state.vaultara_url, &state.jwt_secret, body.into_inner()).await?;
    Ok(HttpResponse::Ok().json(res))
}
