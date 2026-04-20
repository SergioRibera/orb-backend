use uuid::Uuid;

use crate::modules::stores::model::Store;
use crate::shared::errors::AppError;
use crate::shared::repository::PgRepository;

pub trait StoreRepository: Send + Sync {
    async fn create(&self, name: &str, address: Option<&str>) -> Result<Store, AppError>;
    async fn list(&self) -> Result<Vec<Store>, AppError>;
    #[allow(dead_code)]
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Store>, AppError>;
}

impl StoreRepository for PgRepository<Store> {
    async fn create(&self, name: &str, address: Option<&str>) -> Result<Store, AppError> {
        sqlx::query_as!(
            Store,
            "INSERT INTO stores (name, address) VALUES ($1, $2)
             RETURNING id, name, address, created_at, updated_at",
            name,
            address,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))
    }

    async fn list(&self) -> Result<Vec<Store>, AppError> {
        sqlx::query_as!(
            Store,
            "SELECT id, name, address, created_at, updated_at FROM stores ORDER BY name"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Store>, AppError> {
        sqlx::query_as!(
            Store,
            "SELECT id, name, address, created_at, updated_at FROM stores WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))
    }
}
