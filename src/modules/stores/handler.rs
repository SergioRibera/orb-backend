use actix_web::{
    HttpResponse, get, post,
    web::{Data, Json, Path},
};
use uuid::Uuid;

use super::{
    model::{CreateDeviceRequest, CreateStoreRequest, Device, Store},
    service,
};
use crate::db::AppState;
use crate::shared::errors::AppError;
use crate::shared::repository::PgRepository;

// ── Stores ────────────────────────────────────────────────────────────────────

#[post("/stores")]
pub async fn create_store(
    state: Data<AppState>,
    body: Json<CreateStoreRequest>,
) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<Store>::new(state.db.clone());
    let res = service::create_store(&repo, body.into_inner()).await?;
    Ok(HttpResponse::Created().json(res))
}

#[get("/stores")]
pub async fn list_stores(state: Data<AppState>) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<Store>::new(state.db.clone());
    let res = service::list_stores(&repo).await?;
    Ok(HttpResponse::Ok().json(res))
}

// ── Devices ───────────────────────────────────────────────────────────────────

#[post("/stores/{store_id}/devices")]
pub async fn create_device(
    state: Data<AppState>,
    path: Path<Uuid>,
    body: Json<CreateDeviceRequest>,
) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<Device>::new(state.db.clone());
    let store_id = path.into_inner();
    let res = service::create_device(&repo, store_id, body.into_inner()).await?;
    Ok(HttpResponse::Created().json(res))
}

#[get("/stores/{store_id}/devices")]
pub async fn list_devices(
    state: Data<AppState>,
    path: Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<Device>::new(state.db.clone());
    let store_id = path.into_inner();
    let res = service::list_devices(&repo, store_id).await?;
    Ok(HttpResponse::Ok().json(res))
}
