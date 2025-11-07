mod config;
mod services;
mod handlers;
mod middleware;
mod utils;

use axum::{
    Router,
    routing::{get, post},
    http::header,
};
use tower_http::cors::{CorsLayer, Any};
use tower_http::trace::TraceLayer;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use config::Config;
use services::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables
    dotenv::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "api_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env()?;
    tracing::info!("Starting Safari Skills Passport API Server");
    tracing::info!("Environment: {}", config.environment);

    // Create database connection pool
    let db_pool = database::create_pool(&config.database_url).await?;
    tracing::info!("Connected to database");

    // Run migrations
    database::run_migrations(&db_pool).await?;
    tracing::info!("Database migrations completed");

    // Initialize services
    let app_state = Arc::new(AppState::new(config.clone(), db_pool).await?);
    tracing::info!("Services initialized");

    // Build application routes
    let app = Router::new()
        // Health check
        .route("/health", get(handlers::health_check))
        
        // Auth routes
        .route("/api/auth/register", post(handlers::auth::register))
        .route("/api/auth/login", post(handlers::auth::login))
        
        // Institution routes
        .route("/api/institutions/register", post(handlers::institutions::register_institution))
        .route("/api/institutions/me", get(handlers::institutions::get_my_institution))
        
        // Credential routes
        .route("/api/credentials/issue", post(handlers::credentials::issue_credential))
        .route("/api/credentials/verify/:credential_id", get(handlers::credentials::verify_credential))
        .route("/api/credentials/verify-qr", post(handlers::credentials::verify_qr_code))
        .route("/api/credentials/my", get(handlers::credentials::get_my_credentials))
        .route("/api/credentials/issued", get(handlers::credentials::get_issued_credentials))
        .route("/api/credentials/:credential_id", get(handlers::credentials::get_credential))
        .route("/api/credentials/:credential_id/revoke", post(handlers::credentials::revoke_credential))
        .route("/api/credentials/:credential_id/qr", get(handlers::credentials::get_credential_qr))
        
        // CORS
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(vec![
                    header::AUTHORIZATION,
                    header::CONTENT_TYPE,
                    header::ACCEPT,
                ]),
        )
        
        // Tracing
        .layer(TraceLayer::new_for_http())
        
        // State
        .with_state(app_state);

    // Start server
    let addr = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("Server listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
