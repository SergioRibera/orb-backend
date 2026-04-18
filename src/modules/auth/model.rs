use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

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

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
}

#[derive(Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub exp: usize,
}

#[derive(Serialize, Deserialize)]
pub struct RefreshClaims {
    pub sub: String,
    pub exp: usize,
}
