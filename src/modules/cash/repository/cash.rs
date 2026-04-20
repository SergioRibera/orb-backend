use rust_decimal::Decimal;
use uuid::Uuid;

use crate::modules::cash::model::CashSession;
use crate::shared::errors::AppError;
use crate::shared::repository::PgRepository;

pub trait CashRepository: Send + Sync {
    async fn open(
        &self,
        store_id: Uuid,
        user_id: Uuid,
        device_id: Uuid,
        opening_amount: Decimal,
    ) -> Result<CashSession, AppError>;
    async fn close(
        &self,
        id: Uuid,
        closing_amount: Decimal,
        expected_amount: Decimal,
    ) -> Result<CashSession, AppError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<CashSession>, AppError>;
    async fn list_by_store(&self, store_id: Uuid) -> Result<Vec<CashSession>, AppError>;
}

impl CashRepository for PgRepository<CashSession> {
    async fn open(
        &self,
        store_id: Uuid,
        user_id: Uuid,
        device_id: Uuid,
        opening_amount: Decimal,
    ) -> Result<CashSession, AppError> {
        sqlx::query_as!(
            CashSession,
            "INSERT INTO cash_sessions (store_id, user_id, device_id, opening_amount)
             VALUES ($1, $2, $3, $4)
             RETURNING id, store_id, user_id, device_id, opened_at, closed_at,
                       opening_amount, closing_amount, expected_amount",
            store_id,
            user_id,
            device_id,
            opening_amount,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))
    }

    async fn close(
        &self,
        id: Uuid,
        closing_amount: Decimal,
        expected_amount: Decimal,
    ) -> Result<CashSession, AppError> {
        sqlx::query_as!(
            CashSession,
            "UPDATE cash_sessions
             SET closed_at = NOW(), closing_amount = $2, expected_amount = $3
             WHERE id = $1
             RETURNING id, store_id, user_id, device_id, opened_at, closed_at,
                       opening_amount, closing_amount, expected_amount",
            id,
            closing_amount,
            expected_amount,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<CashSession>, AppError> {
        sqlx::query_as!(
            CashSession,
            "SELECT id, store_id, user_id, device_id, opened_at, closed_at,
                    opening_amount, closing_amount, expected_amount
             FROM cash_sessions WHERE id = $1",
            id,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))
    }

    async fn list_by_store(&self, store_id: Uuid) -> Result<Vec<CashSession>, AppError> {
        sqlx::query_as!(
            CashSession,
            "SELECT id, store_id, user_id, device_id, opened_at, closed_at,
                    opening_amount, closing_amount, expected_amount
             FROM cash_sessions WHERE store_id = $1 ORDER BY opened_at DESC",
            store_id,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))
    }
}
