use rust_decimal::Decimal;
use uuid::Uuid;

use crate::modules::catalog::model::{Product, ProductPrice};
use crate::shared::errors::AppError;
use crate::shared::repository::PgRepository;

pub trait ProductRepository: Send + Sync {
    async fn create(
        &self,
        name: &str,
        barcode: Option<&str>,
        description: Option<&str>,
        cost: Option<Decimal>,
    ) -> Result<Product, AppError>;
    async fn list(&self) -> Result<Vec<Product>, AppError>;
    #[allow(dead_code)]
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Product>, AppError>;
    async fn assign_category(&self, product_id: Uuid, category_id: Uuid) -> Result<(), AppError>;
    async fn set_price(
        &self,
        product_id: Uuid,
        store_id: Uuid,
        price: Decimal,
    ) -> Result<ProductPrice, AppError>;
}

impl ProductRepository for PgRepository<Product> {
    async fn create(
        &self,
        name: &str,
        barcode: Option<&str>,
        description: Option<&str>,
        cost: Option<Decimal>,
    ) -> Result<Product, AppError> {
        sqlx::query_as!(
            Product,
            "INSERT INTO products (name, barcode, description, cost)
             VALUES ($1, $2, $3, $4)
             RETURNING id, name, barcode, description, cost, created_at, updated_at",
            name,
            barcode,
            description,
            cost,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))
    }

    async fn list(&self) -> Result<Vec<Product>, AppError> {
        sqlx::query_as!(
            Product,
            "SELECT id, name, barcode, description, cost, created_at, updated_at
             FROM products ORDER BY name"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Product>, AppError> {
        sqlx::query_as!(
            Product,
            "SELECT id, name, barcode, description, cost, created_at, updated_at
             FROM products WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))
    }

    async fn assign_category(&self, product_id: Uuid, category_id: Uuid) -> Result<(), AppError> {
        sqlx::query!(
            "INSERT INTO product_categories (product_id, category_id) VALUES ($1, $2)
             ON CONFLICT DO NOTHING",
            product_id,
            category_id,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
        Ok(())
    }

    async fn set_price(
        &self,
        product_id: Uuid,
        store_id: Uuid,
        price: Decimal,
    ) -> Result<ProductPrice, AppError> {
        sqlx::query_as!(
            ProductPrice,
            "INSERT INTO product_prices (product_id, store_id, price)
             VALUES ($1, $2, $3)
             ON CONFLICT (product_id, store_id) DO UPDATE SET price = EXCLUDED.price
             RETURNING id, product_id, store_id, price",
            product_id,
            store_id,
            price,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))
    }
}
