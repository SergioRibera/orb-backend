use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;
use utoipa::ToSchema;
use uuid::Uuid;

// ── DB structs ────────────────────────────────────────────────────────────────

#[allow(dead_code)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
}

#[allow(dead_code)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub barcode: Option<String>,
    pub description: Option<String>,
    pub cost: Option<Decimal>,
    pub created_at: PrimitiveDateTime,
    pub updated_at: Option<PrimitiveDateTime>,
}

#[allow(dead_code)]
pub struct ProductPrice {
    pub id: Uuid,
    pub product_id: Uuid,
    pub store_id: Uuid,
    pub price: Decimal,
}

#[allow(dead_code)]
pub struct ProductCategory {
    pub product_id: Uuid,
    pub category_id: Uuid,
}

// ── Category DTOs ─────────────────────────────────────────────────────────────

#[derive(Deserialize, ToSchema)]
pub struct CreateCategoryRequest {
    pub name: String,
}

#[derive(Serialize, ToSchema)]
pub struct CategoryResponse {
    pub id: String,
    pub name: String,
}

// ── Product DTOs ──────────────────────────────────────────────────────────────

#[derive(Deserialize, ToSchema)]
pub struct CreateProductRequest {
    pub name: String,
    pub barcode: Option<String>,
    pub description: Option<String>,
    pub cost: Option<f64>,
}

#[derive(Serialize, ToSchema)]
pub struct ProductResponse {
    pub id: String,
    pub name: String,
    pub barcode: Option<String>,
    pub description: Option<String>,
    pub cost: Option<f64>,
}

// ── ProductPrice DTOs ─────────────────────────────────────────────────────────

#[derive(Deserialize, ToSchema)]
pub struct SetProductPriceRequest {
    pub price: f64,
}

#[derive(Serialize, ToSchema)]
pub struct ProductPriceResponse {
    pub id: String,
    pub product_id: String,
    pub store_id: String,
    pub price: f64,
}
