use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use vaultara_sdk::VaultaraClient;

pub struct AppState {
    pub db: Pool<Postgres>,
    pub vaultara: VaultaraClient,
    pub vaultara_url: String,
    pub vaultara_client_id: String,
    pub vaultara_client_secret: String,
    pub vaultara_redirect_uri: String,
    pub vaultara_api_key: Option<String>,
    pub vaultara_tenant_id: Option<String>,
}

pub async fn create_pool(database_url: &str) -> Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("Error creating database pool")
}
