use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;
use utoipa::ToSchema;
use uuid::Uuid;

// ── DB structs ────────────────────────────────────────────────────────────────

#[allow(dead_code)]
pub struct InventoryMovement {
    pub id: Uuid,
    pub product_id: Uuid,
    pub store_id: Uuid,
    pub r#type: String,
    pub quantity: Decimal,
    pub reference_id: Option<Uuid>,
    pub created_at: PrimitiveDateTime,
}

// ── DTOs ──────────────────────────────────────────────────────────────────────

#[derive(Deserialize, ToSchema)]
pub struct CreateMovementRequest {
    pub product_id: String,
    pub store_id: String,
    pub r#type: String,
    pub quantity: f64,
    pub reference_id: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct MovementResponse {
    pub id: String,
    pub product_id: String,
    pub store_id: String,
    pub r#type: String,
    pub quantity: f64,
    pub reference_id: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct StockResponse {
    pub product_id: String,
    pub store_id: String,
    pub quantity: f64,
}
