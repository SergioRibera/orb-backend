use actix_web::{
    HttpResponse, get, post,
    web::{Data, Json, Path},
};
use uuid::Uuid;

use super::{
    model::{CreateMovementRequest, InventoryMovement, MovementResponse, StockResponse},
    service,
};
use crate::db::AppState;
use crate::shared::errors::AppError;
use crate::shared::repository::PgRepository;

#[utoipa::path(
    post,
    path = "/api/v1/inventory/movements",
    tag = "Inventory",
    request_body = CreateMovementRequest,
    responses(
        (status = 201, description = "Movement created", body = MovementResponse),
    )
)]
#[post("/inventory/movements")]
pub async fn create_movement(
    state: Data<AppState>,
    body: Json<CreateMovementRequest>,
) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<InventoryMovement>::new(state.db.clone());
    let res = service::create_movement(&repo, body.into_inner()).await?;
    Ok(HttpResponse::Created().json(res))
}

#[utoipa::path(
    get,
    path = "/api/v1/inventory/movements/stores/{store_id}",
    tag = "Inventory",
    params(("store_id" = Uuid, Path, description = "Store ID")),
    responses(
        (status = 200, description = "List of movements", body = Vec<MovementResponse>),
    )
)]
#[get("/inventory/movements/stores/{store_id}")]
pub async fn list_movements_by_store(
    state: Data<AppState>,
    path: Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<InventoryMovement>::new(state.db.clone());
    let store_id = path.into_inner();
    let res = service::list_by_store(&repo, store_id).await?;
    Ok(HttpResponse::Ok().json(res))
}

#[utoipa::path(
    get,
    path = "/api/v1/inventory/stock/{product_id}/stores/{store_id}",
    tag = "Inventory",
    params(
        ("product_id" = Uuid, Path, description = "Product ID"),
        ("store_id" = Uuid, Path, description = "Store ID"),
    ),
    responses(
        (status = 200, description = "Stock level", body = StockResponse),
        (status = 404, description = "Not found"),
    )
)]
#[get("/inventory/stock/{product_id}/stores/{store_id}")]
pub async fn get_stock(
    state: Data<AppState>,
    path: Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<InventoryMovement>::new(state.db.clone());
    let (product_id, store_id) = path.into_inner();
    let res = service::get_stock(&repo, product_id, store_id).await?;
    Ok(HttpResponse::Ok().json(res))
}
