use actix_web::{
    HttpResponse, get, post,
    web::{Data, Json, Path},
};
use uuid::Uuid;

use super::{
    model::{
        Category, CategoryResponse, CreateCategoryRequest, CreateProductRequest, Product,
        ProductPriceResponse, ProductResponse, SetProductPriceRequest,
    },
    service,
};
use crate::db::AppState;
use crate::shared::errors::AppError;
use crate::shared::repository::PgRepository;

// ── Categories ────────────────────────────────────────────────────────────────

#[utoipa::path(
    post,
    path = "/api/v1/categories",
    tag = "Catalog",
    request_body = CreateCategoryRequest,
    responses(
        (status = 201, description = "Category created", body = CategoryResponse),
        (status = 409, description = "Conflict"),
    )
)]
#[post("/categories")]
pub async fn create_category(
    state: Data<AppState>,
    body: Json<CreateCategoryRequest>,
) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<Category>::new(state.db.clone());
    let res = service::create_category(&repo, body.into_inner()).await?;
    Ok(HttpResponse::Created().json(res))
}

#[utoipa::path(
    get,
    path = "/api/v1/categories",
    tag = "Catalog",
    responses(
        (status = 200, description = "List of categories", body = Vec<CategoryResponse>),
    )
)]
#[get("/categories")]
pub async fn list_categories(state: Data<AppState>) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<Category>::new(state.db.clone());
    let res = service::list_categories(&repo).await?;
    Ok(HttpResponse::Ok().json(res))
}

// ── Products ──────────────────────────────────────────────────────────────────

#[utoipa::path(
    post,
    path = "/api/v1/products",
    tag = "Catalog",
    request_body = CreateProductRequest,
    responses(
        (status = 201, description = "Product created", body = ProductResponse),
        (status = 409, description = "Conflict"),
    )
)]
#[post("/products")]
pub async fn create_product(
    state: Data<AppState>,
    body: Json<CreateProductRequest>,
) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<Product>::new(state.db.clone());
    let res = service::create_product(&repo, body.into_inner()).await?;
    Ok(HttpResponse::Created().json(res))
}

#[utoipa::path(
    get,
    path = "/api/v1/products",
    tag = "Catalog",
    responses(
        (status = 200, description = "List of products", body = Vec<ProductResponse>),
    )
)]
#[get("/products")]
pub async fn list_products(state: Data<AppState>) -> Result<HttpResponse, AppError> {
    let repo = PgRepository::<Product>::new(state.db.clone());
    let res = service::list_products(&repo).await?;
    Ok(HttpResponse::Ok().json(res))
}

#[utoipa::path(
    post,
    path = "/api/v1/products/{product_id}/categories/{category_id}",
    tag = "Catalog",
    params(
        ("product_id" = Uuid, Path, description = "Product ID"),
        ("category_id" = Uuid, Path, description = "Category ID"),
    ),
    responses(
        (status = 204, description = "Category assigned"),
        (status = 404, description = "Not found"),
    )
)]
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

#[utoipa::path(
    post,
    path = "/api/v1/products/{product_id}/stores/{store_id}/price",
    tag = "Catalog",
    params(
        ("product_id" = Uuid, Path, description = "Product ID"),
        ("store_id" = Uuid, Path, description = "Store ID"),
    ),
    request_body = SetProductPriceRequest,
    responses(
        (status = 200, description = "Price set", body = ProductPriceResponse),
        (status = 404, description = "Not found"),
    )
)]
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
