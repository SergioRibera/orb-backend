use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;
use utoipa::ToSchema;
use uuid::Uuid;

// ── DB structs ────────────────────────────────────────────────────────────────

#[allow(dead_code)]
pub struct Sale {
    pub id: Uuid,
    pub store_id: Uuid,
    pub user_id: Uuid,
    pub device_id: Uuid,
    pub folio: Option<String>,
    pub total: Decimal,
    pub status: String,
    pub created_at: PrimitiveDateTime,
}

#[allow(dead_code)]
pub struct SaleItem {
    pub id: Uuid,
    pub sale_id: Uuid,
    pub product_id: Uuid,
    pub quantity: Decimal,
    pub price: Decimal,
    pub subtotal: Decimal,
}

#[allow(dead_code)]
pub struct Payment {
    pub id: Uuid,
    pub sale_id: Uuid,
    pub method: String,
    pub amount: Decimal,
    pub reference: Option<String>,
}

// ── DTOs ──────────────────────────────────────────────────────────────────────

#[derive(Deserialize, ToSchema)]
pub struct SaleItemInput {
    pub product_id: String,
    pub quantity: f64,
    pub price: f64,
}

#[derive(Deserialize, ToSchema)]
pub struct PaymentInput {
    pub method: String,
    pub amount: f64,
    pub reference: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateSaleRequest {
    pub store_id: String,
    pub user_id: String,
    pub device_id: String,
    pub folio: Option<String>,
    pub items: Vec<SaleItemInput>,
    pub payments: Vec<PaymentInput>,
}

#[derive(Serialize, ToSchema)]
pub struct SaleResponse {
    pub id: String,
    pub store_id: String,
    pub user_id: String,
    pub device_id: String,
    pub folio: Option<String>,
    pub total: f64,
    pub status: String,
    pub created_at: String,
}

#[derive(Serialize, ToSchema)]
pub struct SaleItemResponse {
    pub id: String,
    pub product_id: String,
    pub quantity: f64,
    pub price: f64,
    pub subtotal: f64,
}

#[derive(Serialize, ToSchema)]
pub struct PaymentResponse {
    pub id: String,
    pub method: String,
    pub amount: f64,
    pub reference: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct SaleDetailResponse {
    pub sale: SaleResponse,
    pub items: Vec<SaleItemResponse>,
    pub payments: Vec<PaymentResponse>,
}
