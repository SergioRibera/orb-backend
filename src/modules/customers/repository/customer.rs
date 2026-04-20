use uuid::Uuid;

use crate::modules::customers::model::Customer;
use crate::shared::errors::AppError;
use crate::shared::repository::PgRepository;

pub trait CustomerRepository: Send + Sync {
    async fn create(
        &self,
        name: Option<&str>,
        phone: Option<&str>,
        email: Option<&str>,
    ) -> Result<Customer, AppError>;
    async fn list(&self) -> Result<Vec<Customer>, AppError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Customer>, AppError>;
}

impl CustomerRepository for PgRepository<Customer> {
    async fn create(
        &self,
        name: Option<&str>,
        phone: Option<&str>,
        email: Option<&str>,
    ) -> Result<Customer, AppError> {
        sqlx::query_as!(
            Customer,
            "INSERT INTO customers (name, phone, email) VALUES ($1, $2, $3)
             RETURNING id, name, phone, email",
            name,
            phone,
            email,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))
    }

    async fn list(&self) -> Result<Vec<Customer>, AppError> {
        sqlx::query_as!(
            Customer,
            "SELECT id, name, phone, email FROM customers ORDER BY name"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Customer>, AppError> {
        sqlx::query_as!(
            Customer,
            "SELECT id, name, phone, email FROM customers WHERE id = $1",
            id,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))
    }
}
