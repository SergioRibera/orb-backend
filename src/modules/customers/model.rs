use serde::{Deserialize, Serialize};
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

#[derive(Deserialize)]
pub struct CreateCustomerRequest {
    pub name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
}

#[derive(Serialize)]
pub struct CustomerResponse {
    pub id: String,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
}
