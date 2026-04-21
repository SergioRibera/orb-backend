use crate::modules::iam::model::User;
use crate::shared::errors::AppError;
use crate::shared::repository::PgRepository;

pub trait UserRepository: Send + Sync {
    async fn email_exists(&self, email: &str) -> Result<bool, AppError>;
    async fn create(
        &self,
        name: &str,
        second_name: Option<&str>,
        first_surname: Option<&str>,
        second_surname: Option<&str>,
        email: &str,
        password_hash: &str,
    ) -> Result<User, AppError>;
    async fn list(&self) -> Result<Vec<User>, AppError>;
}

impl UserRepository for PgRepository<User> {
    async fn email_exists(&self, email: &str) -> Result<bool, AppError> {
        let row = sqlx::query_scalar!("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)", email)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        Ok(row.unwrap_or(false))
    }

    async fn create(
        &self,
        name: &str,
        second_name: Option<&str>,
        first_surname: Option<&str>,
        second_surname: Option<&str>,
        email: &str,
        password_hash: &str,
    ) -> Result<User, AppError> {
        sqlx::query_as!(
            User,
            "INSERT INTO users (name, second_name, first_surname, second_surname, email, password)
             VALUES ($1, $2, $3, $4, $5, $6)
             RETURNING id, name, second_name, first_surname, second_surname, email, password, created_at, updated_at",
            name, second_name, first_surname, second_surname, email, password_hash,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))
    }

    async fn list(&self) -> Result<Vec<User>, AppError> {
        sqlx::query_as!(
            User,
            "SELECT id, name, second_name, first_surname, second_surname, email, password, created_at, updated_at
             FROM users ORDER BY name"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))
    }
}
