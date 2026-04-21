use std::env;

#[allow(dead_code)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub port: u16,
    pub host: String,
    pub allowed_origins: Vec<String>,
    pub allowed_methods: Vec<String>,
    pub rust_log: String,
    pub vaultara_url: String,
    pub vaultara_api_key: Option<String>,
    pub vaultara_tenant_id: Option<String>,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL not set"),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET not set (used as OAuth2 client_secret)"),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .expect("PORT must be a number"),
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            allowed_origins: env::var("ALLOWED_ORIGINS")
                .unwrap_or_else(|_| "*".to_string())
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
            allowed_methods: env::var("ALLOWED_METHODS")
                .unwrap_or_else(|_| "GET,POST,PUT,DELETE".to_string())
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
            rust_log: env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
            vaultara_url: env::var("VAULTARA_URL").expect("VAULTARA_URL not set"),
            vaultara_api_key: env::var("VAULTARA_API_KEY").ok(),
            vaultara_tenant_id: env::var("VAULTARA_TENANT_ID").ok(),
        }
    }
}
