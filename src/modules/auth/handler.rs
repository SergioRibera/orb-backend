use actix_web::{post, web::{Data, Json}, HttpResponse};

use crate::db::AppState;
use crate::shared::errors::AppError;
use super::{model::{LoginRequest, RefreshRequest, RegisterRequest}, service};

#[post("/login")]
pub async fn login(
    state: Data<AppState>,
    body: Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    let response = service::login(&state.db, &state.jwt_secret, body.into_inner()).await?;
    Ok(HttpResponse::Ok().json(response))
}

#[post("/refresh")]
pub async fn refresh(
    state: Data<AppState>,
    body: Json<RefreshRequest>,
) -> Result<HttpResponse, AppError> {
    let response = service::refresh(&state.db, &state.jwt_secret, body.into_inner()).await?;
    Ok(HttpResponse::Ok().json(response))
}

#[post("/register")]
pub async fn register(
    state: Data<AppState>,
    body: Json<RegisterRequest>,
) -> Result<HttpResponse, AppError> {
    let response = service::register(&state.db, body.into_inner()).await?;
    Ok(HttpResponse::Created().json(response))
}
