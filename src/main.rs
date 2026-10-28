#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

use anyhow::{Ok, Result};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use remnant_core::{api, cache, config::Config, db};
// use remnant_core::config::Config;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialise tracing
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "remnant_core=debug,info".into()))
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    // Load configuration from environment
    let config = Config::from_env()?;

    info!(
        host = %config.host,
        port = %config.port,
        environment = %config.environment,
        "Starting remnant-core"
    );

    // Connect to PostgreSQL
    let db = db::connection::connect(&config.database_url).await?;
    // info!("Connected to PostgreSQL");

    // Connect to Redis
    let cache = cache::client::connect(&config.redis_url).await?;
    info!("Connected to Redis");

    // Build and start the Axum router
    let app = api::router(db, cache, config.clone()).await?;

    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", config.host, config.port)).await?;

    info!("remnant-core listening on {}:{}", config.host, config.port);

    axum::serve(listener, app).await?;

    Ok(())
}
