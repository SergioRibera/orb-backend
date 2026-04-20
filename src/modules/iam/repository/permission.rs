use crate::modules::iam::model::Permission;
use crate::shared::errors::AppError;
use crate::shared::repository::PgRepository;

pub trait PermissionRepository: Send + Sync {
    async fn create(&self, name: &str) -> Result<Permission, AppError>;
    async fn list(&self) -> Result<Vec<Permission>, AppError>;
}

impl PermissionRepository for PgRepository<Permission> {
    async fn create(&self, name: &str) -> Result<Permission, AppError> {
        sqlx::query_as!(
            Permission,
            "INSERT INTO catalogs.permissions (name) VALUES ($1)
             RETURNING id, name, created_at, updated_at",
            name
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))
    }

    async fn list(&self) -> Result<Vec<Permission>, AppError> {
        sqlx::query_as!(
            Permission,
            "SELECT id, name, created_at, updated_at FROM catalogs.permissions ORDER BY name"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))
    }
}
