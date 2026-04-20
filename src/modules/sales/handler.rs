use actix_web::{
    HttpResponse, get, post,
    web::{Data, Json, Path},
};
use uuid::Uuid;

use super::{
    model::{CreateSaleRequest, Sale},
    service,
};
use crate::db::AppState;
use crate::shared::errors::AppError;
use crate::shared::repository::PgRepository;

#[post("/sales")]
pub async fn create_sale(
    state: Data<AppState>,
    body: Json<CreateSaleRequest>,
) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<Sale>::new(state.db.clone());
    let res = service::create_sale(&repo, body.into_inner()).await?;
    Ok(HttpResponse::Created().json(res))
}

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

#[get("/sales/{sale_id}")]
pub async fn get_sale(state: Data<AppState>, path: Path<Uuid>) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<Sale>::new(state.db.clone());
    let sale_id = path.into_inner();
    let res = service::get_sale_detail(&repo, sale_id).await?;
    Ok(HttpResponse::Ok().json(res))
}
