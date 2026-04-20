use uuid::Uuid;

use crate::modules::catalog::model::Category;
use crate::shared::errors::AppError;
use crate::shared::repository::PgRepository;

pub trait CategoryRepository: Send + Sync {
    async fn create(&self, name: &str) -> Result<Category, AppError>;
    async fn list(&self) -> Result<Vec<Category>, AppError>;
}

impl CategoryRepository for PgRepository<Category> {
    async fn create(&self, name: &str) -> Result<Category, AppError> {
        sqlx::query_as!(
            Category,
            "INSERT INTO categories (name) VALUES ($1) RETURNING id, name",
            name,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))
    }

    async fn list(&self) -> Result<Vec<Category>, AppError> {
        sqlx::query_as!(Category, "SELECT id, name FROM categories ORDER BY name")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))
    }
}
