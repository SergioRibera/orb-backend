use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

// ── DB structs ────────────────────────────────────────────────────────────────

#[allow(dead_code)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub second_name: Option<String>,
    pub first_surname: Option<String>,
    pub second_surname: Option<String>,
    pub email: String,
    pub password: String,
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
}

#[allow(dead_code)]
pub struct Role {
    pub id: Uuid,
    pub name: String,
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
}

#[allow(dead_code)]
pub struct Permission {
    pub id: Uuid,
    pub name: String,
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
}

// ── Internal ──────────────────────────────────────────────────────────────────

pub struct UserAccess {
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}

// ── User DTOs ─────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub second_name: Option<String>,
    pub first_surname: Option<String>,
    pub second_surname: Option<String>,
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub id: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub email: String,
}

// ── Role DTOs ─────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct CreateRoleRequest {
    pub name: String,
}

#[derive(Serialize)]
pub struct RoleResponse {
    pub id: String,
    pub name: String,
}

// ── Permission DTOs ───────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct CreatePermissionRequest {
    pub name: String,
}

#[derive(Serialize)]
pub struct PermissionResponse {
    pub id: String,
    pub name: String,
}
