use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;
use utoipa::ToSchema;
use uuid::Uuid;

// ── DB structs ────────────────────────────────────────────────────────────────

#[allow(dead_code)]
pub struct CashSession {
    pub id: Uuid,
    pub store_id: Uuid,
    pub user_id: Uuid,
    pub device_id: Uuid,
    pub opened_at: PrimitiveDateTime,
    pub closed_at: Option<PrimitiveDateTime>,
    pub opening_amount: Decimal,
    pub closing_amount: Option<Decimal>,
    pub expected_amount: Option<Decimal>,
}

// ── DTOs ──────────────────────────────────────────────────────────────────────

#[derive(Deserialize, ToSchema)]
pub struct OpenSessionRequest {
    pub store_id: String,
    pub user_id: String,
    pub device_id: String,
    pub opening_amount: f64,
}

#[derive(Deserialize, ToSchema)]
pub struct CloseSessionRequest {
    pub closing_amount: f64,
    pub expected_amount: f64,
}

#[derive(Serialize, ToSchema)]
pub struct SessionResponse {
    pub id: String,
    pub store_id: String,
    pub user_id: String,
    pub device_id: String,
    pub status: String,
    pub opened_at: String,
    pub closed_at: Option<String>,
    pub opening_amount: f64,
    pub closing_amount: Option<f64>,
    pub expected_amount: Option<f64>,
}
