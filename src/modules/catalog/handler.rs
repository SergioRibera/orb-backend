use actix_web::{
    HttpResponse, get, post,
    web::{Data, Json, Path},
};
use uuid::Uuid;

use super::{
    model::{
        Category, CreateCategoryRequest, CreateProductRequest, Product, SetProductPriceRequest,
    },
    service,
};
use crate::db::AppState;
use crate::shared::errors::AppError;
use crate::shared::repository::PgRepository;

// ── Categories ────────────────────────────────────────────────────────────────

#[post("/categories")]
pub async fn create_category(
    state: Data<AppState>,
    body: Json<CreateCategoryRequest>,
) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<Category>::new(state.db.clone());
    let res = service::create_category(&repo, body.into_inner()).await?;
    Ok(HttpResponse::Created().json(res))
}

#[get("/categories")]
pub async fn list_categories(state: Data<AppState>) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<Category>::new(state.db.clone());
    let res = service::list_categories(&repo).await?;
    Ok(HttpResponse::Ok().json(res))
}

// ── Products ──────────────────────────────────────────────────────────────────

#[post("/products")]
pub async fn create_product(
    state: Data<AppState>,
    body: Json<CreateProductRequest>,
) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<Product>::new(state.db.clone());
    let res = service::create_product(&repo, body.into_inner()).await?;
    Ok(HttpResponse::Created().json(res))
}

#[get("/products")]
pub async fn list_products(state: Data<AppState>) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<Product>::new(state.db.clone());
    let res = service::list_products(&repo).await?;
    Ok(HttpResponse::Ok().json(res))
}

#[post("/products/{product_id}/categories/{category_id}")]
pub async fn assign_category(
    state: Data<AppState>,
    path: Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<Product>::new(state.db.clone());
    let (product_id, category_id) = path.into_inner();
    service::assign_category(&repo, product_id, category_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[post("/products/{product_id}/stores/{store_id}/price")]
pub async fn set_price(
    state: Data<AppState>,
    path: Path<(Uuid, Uuid)>,
    body: Json<SetProductPriceRequest>,
) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<Product>::new(state.db.clone());
    let (product_id, store_id) = path.into_inner();
    let res = service::set_price(&repo, product_id, store_id, body.into_inner()).await?;
    Ok(HttpResponse::Ok().json(res))
}
