use rust_decimal::Decimal;
use uuid::Uuid;

use crate::modules::sales::model::{Payment, Sale, SaleItem};
use crate::shared::errors::AppError;
use crate::shared::repository::PgRepository;

pub trait SaleRepository: Send + Sync {
    async fn create(
        &self,
        store_id: Uuid,
        user_id: Uuid,
        device_id: Uuid,
        folio: Option<&str>,
        total: Decimal,
        status: &str,
    ) -> Result<Sale, AppError>;
    async fn list_by_store(&self, store_id: Uuid) -> Result<Vec<Sale>, AppError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Sale>, AppError>;
    async fn add_item(
        &self,
        sale_id: Uuid,
        product_id: Uuid,
        quantity: Decimal,
        price: Decimal,
        subtotal: Decimal,
    ) -> Result<SaleItem, AppError>;
    async fn add_payment(
        &self,
        sale_id: Uuid,
        method: &str,
        amount: Decimal,
        reference: Option<&str>,
    ) -> Result<Payment, AppError>;
    async fn list_items(&self, sale_id: Uuid) -> Result<Vec<SaleItem>, AppError>;
    async fn list_payments(&self, sale_id: Uuid) -> Result<Vec<Payment>, AppError>;
}

impl SaleRepository for PgRepository<Sale> {
    async fn create(
        &self,
        store_id: Uuid,
        user_id: Uuid,
        device_id: Uuid,
        folio: Option<&str>,
        total: Decimal,
        status: &str,
    ) -> Result<Sale, AppError> {
        sqlx::query_as!(
            Sale,
            "INSERT INTO sales (store_id, user_id, device_id, folio, total, status)
             VALUES ($1, $2, $3, $4, $5, $6)
             RETURNING id, store_id, user_id, device_id, folio, total, status, created_at",
            store_id,
            user_id,
            device_id,
            folio,
            total,
            status,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))
    }

    async fn list_by_store(&self, store_id: Uuid) -> Result<Vec<Sale>, AppError> {
        sqlx::query_as!(
            Sale,
            "SELECT id, store_id, user_id, device_id, folio, total, status, created_at
             FROM sales WHERE store_id = $1 ORDER BY created_at DESC",
            store_id,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Sale>, AppError> {
        sqlx::query_as!(
            Sale,
            "SELECT id, store_id, user_id, device_id, folio, total, status, created_at
             FROM sales WHERE id = $1",
            id,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))
    }

    async fn add_item(
        &self,
        sale_id: Uuid,
        product_id: Uuid,
        quantity: Decimal,
        price: Decimal,
        subtotal: Decimal,
    ) -> Result<SaleItem, AppError> {
        sqlx::query_as!(
            SaleItem,
            "INSERT INTO sale_items (sale_id, product_id, quantity, price, subtotal)
             VALUES ($1, $2, $3, $4, $5)
             RETURNING id, sale_id, product_id, quantity, price, subtotal",
            sale_id,
            product_id,
            quantity,
            price,
            subtotal,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))
    }

    async fn add_payment(
        &self,
        sale_id: Uuid,
        method: &str,
        amount: Decimal,
        reference: Option<&str>,
    ) -> Result<Payment, AppError> {
        sqlx::query_as!(
            Payment,
            "INSERT INTO payments (sale_id, method, amount, reference)
             VALUES ($1, $2, $3, $4)
             RETURNING id, sale_id, method, amount, reference",
            sale_id,
            method,
            amount,
            reference,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))
    }

    async fn list_items(&self, sale_id: Uuid) -> Result<Vec<SaleItem>, AppError> {
        sqlx::query_as!(
            SaleItem,
            "SELECT id, sale_id, product_id, quantity, price, subtotal
             FROM sale_items WHERE sale_id = $1",
            sale_id,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))
    }

    async fn list_payments(&self, sale_id: Uuid) -> Result<Vec<Payment>, AppError> {
        sqlx::query_as!(
            Payment,
            "SELECT id, sale_id, method, amount, reference FROM payments WHERE sale_id = $1",
            sale_id,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))
    }
}
