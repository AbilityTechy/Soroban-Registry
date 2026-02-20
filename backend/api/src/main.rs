mod aggregation;
mod analytics;
mod audit_handlers;
mod audit_routes;
mod benchmark_engine;
mod benchmark_handlers;
mod benchmark_routes;
mod cache;
mod cache_benchmark;
mod checklist;
mod contract_history_handlers;
mod contract_history_routes;
mod detector;
mod error;
mod handlers;
mod models;
mod multisig_handlers;
mod multisig_routes;
mod popularity;
mod rate_limit;
mod routes;
mod state;
mod health_monitor;
mod migration_cli;

use anyhow::Result;
use axum::http::{header, HeaderValue, Method};
use axum::{middleware, Router};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::rate_limit::RateLimitState;
use crate::state::AppState;

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables
    dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let args: Vec<String> = std::env::args().skip(1).collect();
    let migration_command = migration_cli::parse_command(&args)?;

    // Database connection
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Run migrations by default, or execute migration subcommands.
    if let Some(command) = migration_command {
        migration_cli::execute(command, &pool).await?;
        return Ok(());
    }

    sqlx::migrate!("../../database/migrations")
        .run(&pool)
        .await?;

    tracing::info!("Database connected and migrations applied");

    // Spawn background popularity scoring job (runs hourly)
    popularity::spawn_popularity_task(pool.clone());
    // Spawn the hourly analytics aggregation background task
    aggregation::spawn_aggregation_task(pool.clone());

    // Create app state
    let state = AppState::new(pool);
    let rate_limit_state = RateLimitState::from_env();

    let cors = CorsLayer::new()
        .allow_origin([
            HeaderValue::from_static("http://localhost:3000"),
            HeaderValue::from_static("https://soroban-registry.vercel.app"),
        ])
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);

    // Build router
    let app = Router::new()
        .merge(routes::contract_routes())
        .merge(routes::publisher_routes())