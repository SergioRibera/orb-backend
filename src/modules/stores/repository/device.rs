use uuid::Uuid;

use crate::modules::stores::model::Device;
use crate::shared::errors::AppError;
use crate::shared::repository::PgRepository;

pub trait DeviceRepository: Send + Sync {
    async fn create(&self, store_id: Uuid, name: Option<&str>) -> Result<Device, AppError>;
    async fn list_by_store(&self, store_id: Uuid) -> Result<Vec<Device>, AppError>;
}

impl DeviceRepository for PgRepository<Device> {
    async fn create(&self, store_id: Uuid, name: Option<&str>) -> Result<Device, AppError> {
        sqlx::query_as!(
            Device,
            "INSERT INTO devices (store_id, name) VALUES ($1, $2)
             RETURNING id, store_id, name, created_at",
            store_id,
            name,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))
    }

    async fn list_by_store(&self, store_id: Uuid) -> Result<Vec<Device>, AppError> {
        sqlx::query_as!(
            Device,
            "SELECT id, store_id, name, created_at FROM devices WHERE store_id = $1 ORDER BY name",
            store_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))
    }
}
