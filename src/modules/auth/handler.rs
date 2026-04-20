use actix_web::{
    HttpResponse, post,
    web::{Data, Json},
};

use super::{
    model::{LoginRequest, RefreshRequest},
    service,
};
use crate::db::AppState;
use crate::modules::iam::model::User;
use crate::shared::errors::AppError;
use crate::shared::repository::PgRepository;

#[post("/login")]
pub async fn login(
    state: Data<AppState>,
    body: Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<User>::new(state.db.clone());
    let res = service::login(&repo, &state.jwt_secret, body.into_inner()).await?;
    Ok(HttpResponse::Ok().json(res))
}

#[post("/refresh")]
pub async fn refresh(
    state: Data<AppState>,
    body: Json<RefreshRequest>,
) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<User>::new(state.db.clone());
    let res = service::refresh(&repo, &state.jwt_secret, body.into_inner()).await?;
    Ok(HttpResponse::Ok().json(res))
}
