use actix_web::{
    HttpResponse, post,
    web::{Data, Json},
};

use super::{
    model::{LoginRequest, LoginResponse, RefreshRequest},
    service,
};
use crate::db::AppState;
use crate::modules::iam::model::User;
use crate::shared::errors::AppError;
use crate::shared::repository::PgRepository;

#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    tag = "Auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials"),
    )
)]
#[post("/login")]
pub async fn login(
    state: Data<AppState>,
    body: Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<User>::new(state.db.clone());
    let res = service::login(&repo, &state.jwt_secret, body.into_inner()).await?;
    Ok(HttpResponse::Ok().json(res))
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/refresh",
    tag = "Auth",
    request_body = RefreshRequest,
    responses(
        (status = 200, description = "Token refreshed", body = LoginResponse),
        (status = 401, description = "Invalid or expired refresh token"),
    )
)]
#[post("/refresh")]
pub async fn refresh(
    state: Data<AppState>,
    body: Json<RefreshRequest>,
) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<User>::new(state.db.clone());
    let res = service::refresh(&repo, &state.jwt_secret, body.into_inner()).await?;
    Ok(HttpResponse::Ok().json(res))
}
