use actix_web::{App, HttpResponse, HttpServer, get, web::Data};
use tracing::info;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::{EnvFilter, fmt::format::FmtSpan};

mod config;
mod db;
use config::Config;
use db::{AppState, create_pool};

#[get("/hc")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok"
    }))
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

    info!("Server running on {}:{}", cfg.host, cfg.port);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { db: pool.clone() }))
            .wrap(TracingLogger::default())
            .service(health_check)
    })
    .bind((cfg.host, cfg.port))?
    .run()
    .await
}
