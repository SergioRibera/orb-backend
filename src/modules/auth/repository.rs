use sqlx::{Pool, Postgres};
use uuid::Uuid;

use super::model::User;
use crate::shared::errors::AppError;

pub async fn find_by_email(pool: &Pool<Postgres>, email: &str) -> Result<Option<User>, AppError> {
    sqlx::query_as!(
        User,
        "SELECT id, name, second_name, first_surname, second_surname, email, password, created_at, updated_at
         FROM users WHERE email = $1",
        email
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))
}

pub async fn email_exists(pool: &Pool<Postgres>, email: &str) -> Result<bool, AppError> {
    let row = sqlx::query_scalar!("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)", email)
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(row.unwrap_or(false))
}

pub async fn create_user(
    pool: &Pool<Postgres>,
    name: &str,
    second_name: Option<&str>,
    first_surname: Option<&str>,
    second_surname: Option<&str>,
    email: &str,
    password_hash: &str,
) -> Result<User, AppError> {
    sqlx::query_as!(
        User,
        "INSERT INTO users (name, second_name, first_surname, second_surname, email, password)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING id, name, second_name, first_surname, second_surname, email, password, created_at, updated_at",
        name,
        second_name,
        first_surname,
        second_surname,
        email,
        password_hash,
    )
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))
}

pub async fn find_by_id(pool: &Pool<Postgres>, id: Uuid) -> Result<Option<User>, AppError> {
    sqlx::query_as!(
        User,
        "SELECT id, name, second_name, first_surname, second_surname, email, password, created_at, updated_at
         FROM users WHERE id = $1",
        id
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))
}
