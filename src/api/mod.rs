mod entities;
mod routes;

use crate::api::routes::health::health;
use crate::config::Config;
use axum::routing::get;
use axum::Router;
use redis_pool::RedisPool;
use sea_orm::DatabaseConnection;

// Check health of API
pub async fn router(
    _db: DatabaseConnection,
    _cache: RedisPool<redis::Client, redis::aio::MultiplexedConnection>,
    _config: Config,
) -> anyhow::Result<Router> {
    let app = Router::new().route("/health", get(health));
    Ok(app)
}
