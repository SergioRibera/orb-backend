use actix_cors::Cors;
use actix_web::{
    App, HttpResponse, HttpServer, get,
    http::header,
    web::{self, Data},
};
use actix_web_grants::GrantsMiddleware;
use tracing::info;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::{EnvFilter, fmt::format::FmtSpan};
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};
use vaultara_sdk::{VaultaraClient, VaultaraConfig};

mod api_docs;
mod config;
mod db;
mod middleware;
mod modules;
mod shared;

use config::Config;
use db::{AppState, create_pool};

#[get("/hc")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({ "status": "ok" }))
}

fn init_tracing() {
    let filter = EnvFilter::from_default_env();

    if cfg!(debug_assertions) {
        tracing_subscriber::fmt()
            .pretty()
            .with_env_filter(filter)
            .with_span_events(FmtSpan::CLOSE)
            .init();
    } else {
        tracing_subscriber::fmt()
            .json()
            .flatten_event(true)
            .with_env_filter(filter)
            .with_span_events(FmtSpan::CLOSE)
            .with_current_span(true)
            .with_span_list(true)
            .init();
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    init_tracing();

    let cfg = Config::from_env();
    let pool = create_pool(&cfg.database_url).await;
    let jwt_secret = cfg.jwt_secret.clone();
    let allowed_origins = cfg.allowed_origins.clone();
    let allowed_methods = cfg.allowed_methods.clone();

    let vaultara: Option<VaultaraClient> = cfg.vaultara_url.as_deref().map(|url| {
        let mut config = VaultaraConfig::new(url);
        if let Some(key) = &cfg.vaultara_api_key {
            config = config.with_api_key(key);
        }
        if let Some(tenant) = &cfg.vaultara_tenant_id {
            config = config.with_tenant(tenant);
        }
        VaultaraClient::new(config).expect("failed to create Vaultara client")
    });

    if vaultara.is_some() {
        info!("Vaultara IAM integration enabled");
    } else {
        info!("Vaultara IAM not configured — falling back to local JWT permissions");
    }

    info!("Server running on {}:{}", cfg.host, cfg.port);

    HttpServer::new(move || {
        let mut cors = Cors::default()
            .allowed_headers([header::AUTHORIZATION, header::CONTENT_TYPE, header::ACCEPT])
            .max_age(3600);

        for origin in &allowed_origins {
            if origin == "*" {
                cors = cors.allow_any_origin();
                break;
            } else {
                cors = cors.allowed_origin(origin);
            }
        }

        cors = cors.allowed_methods(
            allowed_methods
                .iter()
                .map(|m| m.as_str())
                .collect::<Vec<_>>(),
        );

        App::new()
            .app_data(Data::new(AppState {
                db: pool.clone(),
                jwt_secret: jwt_secret.clone(),
                vaultara: vaultara.clone(),
            }))
            .wrap(GrantsMiddleware::with_extractor(
                middleware::auth::extract_permissions,
            ))
            .wrap(TracingLogger::default())
            .wrap(cors)
            .service(
                Scalar::with_url("/docs", api_docs::ApiDoc::openapi())
                    .custom_html(api_docs::SCALAR_HTML),
            )
            .service(
                web::scope("/api/v1")
                    .service(health_check)
                    .configure(modules::auth::config)
                    .configure(modules::iam::config)
                    .configure(modules::stores::config)
                    .configure(modules::catalog::config)
                    .configure(modules::inventory::config)
                    .configure(modules::sales::config)
                    .configure(modules::cash::config)
                    .configure(modules::customers::config),
            )
    })
    .bind((cfg.host, cfg.port))?
    .run()
    .await
}
