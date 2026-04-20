use rust_decimal::Decimal;
use uuid::Uuid;

use crate::modules::inventory::model::InventoryMovement;
use crate::shared::errors::AppError;
use crate::shared::repository::PgRepository;

pub trait InventoryRepository: Send + Sync {
    async fn create(
        &self,
        product_id: Uuid,
        store_id: Uuid,
        movement_type: &str,
        quantity: Decimal,
        reference_id: Option<Uuid>,
    ) -> Result<InventoryMovement, AppError>;
    async fn list_by_store(&self, store_id: Uuid) -> Result<Vec<InventoryMovement>, AppError>;
    async fn stock_by_product_store(
        &self,
        product_id: Uuid,
        store_id: Uuid,
    ) -> Result<f64, AppError>;
}

impl InventoryRepository for PgRepository<InventoryMovement> {
    async fn create(
        &self,
        product_id: Uuid,
        store_id: Uuid,
        movement_type: &str,
        quantity: Decimal,
        reference_id: Option<Uuid>,
    ) -> Result<InventoryMovement, AppError> {
        sqlx::query_as!(
            InventoryMovement,
            r#"INSERT INTO inventory_movements (product_id, store_id, type, quantity, reference_id)
               VALUES ($1, $2, $3, $4, $5)
               RETURNING id, product_id, store_id, type AS "type", quantity, reference_id, created_at"#,
            product_id,
            store_id,
            movement_type,
            quantity,
            reference_id,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))
    }

    async fn list_by_store(&self, store_id: Uuid) -> Result<Vec<InventoryMovement>, AppError> {
        sqlx::query_as!(
            InventoryMovement,
            r#"SELECT id, product_id, store_id, type AS "type", quantity, reference_id, created_at
               FROM inventory_movements WHERE store_id = $1
               ORDER BY created_at DESC"#,
            store_id,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))
    }

    async fn stock_by_product_store(
        &self,
        product_id: Uuid,
        store_id: Uuid,
    ) -> Result<f64, AppError> {
        let row = sqlx::query!(
            r#"SELECT COALESCE(SUM(quantity), 0)::float8 AS "stock!"
               FROM inventory_movements WHERE product_id = $1 AND store_id = $2"#,
            product_id,
            store_id,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
        Ok(row.stock)
    }
}
