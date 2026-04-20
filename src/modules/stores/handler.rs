use actix_web::{
    HttpResponse, get, post,
    web::{Data, Json, Path},
};
use uuid::Uuid;

use super::{
    model::{
        CreateDeviceRequest, CreateStoreRequest, Device, DeviceResponse, Store, StoreResponse,
    },
    service,
};
use crate::db::AppState;
use crate::shared::errors::AppError;
use crate::shared::repository::PgRepository;

// ── Stores ────────────────────────────────────────────────────────────────────

#[utoipa::path(
    post,
    path = "/api/v1/stores",
    tag = "Stores",
    request_body = CreateStoreRequest,
    responses(
        (status = 201, description = "Store created", body = StoreResponse),
        (status = 409, description = "Conflict"),
    )
)]
#[post("/stores")]
pub async fn create_store(
    state: Data<AppState>,
    body: Json<CreateStoreRequest>,
) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<Store>::new(state.db.clone());
    let res = service::create_store(&repo, body.into_inner()).await?;
    Ok(HttpResponse::Created().json(res))
}

#[utoipa::path(
    get,
    path = "/api/v1/stores",
    tag = "Stores",
    responses(
        (status = 200, description = "List of stores", body = Vec<StoreResponse>),
    )
)]
#[get("/stores")]
pub async fn list_stores(state: Data<AppState>) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<Store>::new(state.db.clone());
    let res = service::list_stores(&repo).await?;
    Ok(HttpResponse::Ok().json(res))
}

// ── Devices ───────────────────────────────────────────────────────────────────

#[utoipa::path(
    post,
    path = "/api/v1/stores/{store_id}/devices",
    tag = "Stores",
    params(("store_id" = Uuid, Path, description = "Store ID")),
    request_body = CreateDeviceRequest,
    responses(
        (status = 201, description = "Device created", body = DeviceResponse),
        (status = 404, description = "Store not found"),
    )
)]
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

#[utoipa::path(
    get,
    path = "/api/v1/stores/{store_id}/devices",
    tag = "Stores",
    params(("store_id" = Uuid, Path, description = "Store ID")),
    responses(
        (status = 200, description = "List of devices", body = Vec<DeviceResponse>),
    )
)]
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
