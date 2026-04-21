use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use vaultara_sdk::VaultaraClient;

pub struct AppState {
    pub db: Pool<Postgres>,
    pub jwt_secret: String,
    pub vaultara: VaultaraClient,
    pub vaultara_url: String,
}

pub async fn create_pool(database_url: &str) -> Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("Error creating database pool")
}
