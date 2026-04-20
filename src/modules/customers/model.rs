use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

// ── DB structs ────────────────────────────────────────────────────────────────

#[allow(dead_code)]
pub struct Customer {
    pub id: Uuid,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
}

// ── DTOs ──────────────────────────────────────────────────────────────────────

#[derive(Deserialize, ToSchema)]
pub struct CreateCustomerRequest {
    pub name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct CustomerResponse {
    pub id: String,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
}
