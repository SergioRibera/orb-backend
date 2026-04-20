use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;
use utoipa::ToSchema;
use uuid::Uuid;

// ── DB structs ────────────────────────────────────────────────────────────────

#[allow(dead_code)]
pub struct Store {
    pub id: Uuid,
    pub name: String,
    pub address: Option<String>,
    pub created_at: PrimitiveDateTime,
    pub updated_at: Option<PrimitiveDateTime>,
}

#[allow(dead_code)]
pub struct Device {
    pub id: Uuid,
    pub store_id: Uuid,
    pub name: Option<String>,
    pub created_at: PrimitiveDateTime,
}

#[allow(dead_code)]
pub struct UserStore {
    pub id: Uuid,
    pub user_id: Uuid,
    pub store_id: Uuid,
    pub created_at: PrimitiveDateTime,
    pub updated_at: Option<PrimitiveDateTime>,
}

// ── Store DTOs ────────────────────────────────────────────────────────────────

#[derive(Deserialize, ToSchema)]
pub struct CreateStoreRequest {
    pub name: String,
    pub address: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct StoreResponse {
    pub id: String,
    pub name: String,
    pub address: Option<String>,
    pub created_at: String,
}

// ── Device DTOs ───────────────────────────────────────────────────────────────

#[derive(Deserialize, ToSchema)]
pub struct CreateDeviceRequest {
    pub name: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct DeviceResponse {
    pub id: String,
    pub store_id: String,
    pub name: Option<String>,
    pub created_at: String,
}
