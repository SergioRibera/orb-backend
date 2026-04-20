use uuid::Uuid;

use crate::modules::iam::model::Role;
use crate::shared::errors::AppError;
use crate::shared::repository::PgRepository;

pub trait RoleRepository: Send + Sync {
    async fn create(&self, name: &str) -> Result<Role, AppError>;
    async fn list(&self) -> Result<Vec<Role>, AppError>;
    async fn assign_to_user(&self, user_id: Uuid, role_id: Uuid) -> Result<(), AppError>;
    async fn assign_permission(&self, role_id: Uuid, permission_id: Uuid) -> Result<(), AppError>;
}

impl RoleRepository for PgRepository<Role> {
    async fn create(&self, name: &str) -> Result<Role, AppError> {
        sqlx::query_as!(
            Role,
            "INSERT INTO catalogs.roles (name) VALUES ($1)
             RETURNING id, name, created_at, updated_at",
            name
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))
    }

    async fn list(&self) -> Result<Vec<Role>, AppError> {
        sqlx::query_as!(
            Role,
            "SELECT id, name, created_at, updated_at FROM catalogs.roles ORDER BY name"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))
    }

    async fn assign_to_user(&self, user_id: Uuid, role_id: Uuid) -> Result<(), AppError> {
        sqlx::query!(
            "INSERT INTO user_roles (user_id, role_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
            user_id,
            role_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
        Ok(())
    }

    async fn assign_permission(&self, role_id: Uuid, permission_id: Uuid) -> Result<(), AppError> {
        sqlx::query!(
            "INSERT INTO catalogs.role_permissions (role_id, permission_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
            role_id,
            permission_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
        Ok(())
    }
}
