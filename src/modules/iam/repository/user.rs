use uuid::Uuid;

use crate::modules::iam::model::{User, UserAccess};
use crate::shared::errors::AppError;
use crate::shared::repository::PgRepository;

pub trait UserRepository: Send + Sync {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError>;
    async fn email_exists(&self, email: &str) -> Result<bool, AppError>;
    async fn create(
        &self,
        name: &str,
        second_name: Option<&str>,
        first_surname: Option<&str>,
        second_surname: Option<&str>,
        email: &str,
        password_hash: &str,
    ) -> Result<User, AppError>;
    async fn find_access(&self, user_id: Uuid) -> Result<UserAccess, AppError>;
    async fn list(&self) -> Result<Vec<User>, AppError>;
}

impl UserRepository for PgRepository<User> {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        sqlx::query_as!(
            User,
            "SELECT id, name, second_name, first_surname, second_surname, email, password, created_at, updated_at
             FROM users WHERE email = $1",
            email
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError> {
        sqlx::query_as!(
            User,
            "SELECT id, name, second_name, first_surname, second_surname, email, password, created_at, updated_at
             FROM users WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))
    }

    async fn email_exists(&self, email: &str) -> Result<bool, AppError> {
        let row = sqlx::query_scalar!("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)", email)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        Ok(row.unwrap_or(false))
    }

    async fn create(
        &self,
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
            name, second_name, first_surname, second_surname, email, password_hash,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))
    }

    async fn find_access(&self, user_id: Uuid) -> Result<UserAccess, AppError> {
        let roles = sqlx::query_scalar!(
            "SELECT r.name FROM catalogs.roles r
             JOIN user_roles ur ON ur.role_id = r.id
             WHERE ur.user_id = $1",
            user_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

        let permissions = sqlx::query_scalar!(
            "SELECT DISTINCT p.name FROM catalogs.permissions p
             JOIN catalogs.role_permissions rp ON rp.permission_id = p.id
             JOIN user_roles ur ON ur.role_id = rp.role_id
             WHERE ur.user_id = $1",
            user_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(UserAccess { roles, permissions })
    }

    async fn list(&self) -> Result<Vec<User>, AppError> {
        sqlx::query_as!(
            User,
            "SELECT id, name, second_name, first_surname, second_surname, email, password, created_at, updated_at
             FROM users ORDER BY name"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))
    }
}
