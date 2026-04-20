use actix_web::{
    HttpResponse, get, post,
    web::{Data, Json, Path},
};
use uuid::Uuid;

use super::{
    model::{CreateSaleRequest, Sale, SaleDetailResponse, SaleResponse},
    service,
};
use crate::db::AppState;
use crate::modules::inventory::model::InventoryMovement;
use crate::shared::errors::AppError;
use crate::shared::repository::PgRepository;

#[utoipa::path(
    post,
    path = "/api/v1/sales",
    tag = "Sales",
    request_body = CreateSaleRequest,
    responses(
        (status = 201, description = "Sale created", body = SaleResponse),
        (status = 400, description = "Bad request"),
    )
)]
#[post("/sales")]
pub async fn create_sale(
    state: Data<AppState>,
    body: Json<CreateSaleRequest>,
) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<Sale>::new(state.db.clone());
    let inv_repo = PgRepository::<InventoryMovement>::new(state.db.clone());
    let res = service::create_sale(&repo, &inv_repo, body.into_inner()).await?;
    Ok(HttpResponse::Created().json(res))
}

#[utoipa::path(
    get,
    path = "/api/v1/sales/stores/{store_id}",
    tag = "Sales",
    params(("store_id" = Uuid, Path, description = "Store ID")),
    responses(
        (status = 200, description = "List of sales", body = Vec<SaleResponse>),
    )
)]
#[get("/sales/stores/{store_id}")]
pub async fn list_sales_by_store(
    state: Data<AppState>,
    path: Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<Sale>::new(state.db.clone());
    let store_id = path.into_inner();
    let res = service::list_by_store(&repo, store_id).await?;
    Ok(HttpResponse::Ok().json(res))
}

#[utoipa::path(
    get,
    path = "/api/v1/sales/{sale_id}",
    tag = "Sales",
    params(("sale_id" = Uuid, Path, description = "Sale ID")),
    responses(
        (status = 200, description = "Sale detail", body = SaleDetailResponse),
        (status = 404, description = "Not found"),
    )
)]
#[get("/sales/{sale_id}")]
pub async fn get_sale(state: Data<AppState>, path: Path<Uuid>) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<Sale>::new(state.db.clone());
    let sale_id = path.into_inner();
    let res = service::get_sale_detail(&repo, sale_id).await?;
    Ok(HttpResponse::Ok().json(res))
}
