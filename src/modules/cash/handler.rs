use actix_web::{
    HttpResponse, get, post,
    web::{Data, Json, Path},
};
use uuid::Uuid;

use super::{
    model::{CashSession, CloseSessionRequest, OpenSessionRequest},
    service,
};
use crate::db::AppState;
use crate::shared::errors::AppError;
use crate::shared::repository::PgRepository;

#[post("/cash/sessions")]
pub async fn open_session(
    state: Data<AppState>,
    body: Json<OpenSessionRequest>,
) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<CashSession>::new(state.db.clone());
    let res = service::open_session(&repo, body.into_inner()).await?;
    Ok(HttpResponse::Created().json(res))
}

#[post("/cash/sessions/{id}/close")]
pub async fn close_session(
    state: Data<AppState>,
    path: Path<Uuid>,
    body: Json<CloseSessionRequest>,
) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<CashSession>::new(state.db.clone());
    let id = path.into_inner();
    let res = service::close_session(&repo, id, body.into_inner()).await?;
    Ok(HttpResponse::Ok().json(res))
}

#[get("/cash/sessions/stores/{store_id}")]
pub async fn list_sessions(
    state: Data<AppState>,
    path: Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<CashSession>::new(state.db.clone());
    let store_id = path.into_inner();
    let res = service::list_sessions_by_store(&repo, store_id).await?;
    Ok(HttpResponse::Ok().json(res))
}

#[get("/cash/sessions/{id}")]
pub async fn get_session(
    state: Data<AppState>,
    path: Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<CashSession>::new(state.db.clone());
    let id = path.into_inner();
    let res = service::get_session(&repo, id).await?;
    Ok(HttpResponse::Ok().json(res))
}
