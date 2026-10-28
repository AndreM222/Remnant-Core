//! Application configuration loaded from environment variables.
//! All values come from .env (development) or environment (production).

use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub environment: String,
    pub database_url: String,
    pub redis_url: String,
    pub tmdb_api_key: String,
    pub tmdb_base_url: String,
    pub tmdb_image_base_url: String,
    pub supabase_jwt_secret: String,
    pub supabase_url: String,
    pub supabase_anon_key: String,
    pub rate_limit_per_second: u64,
    pub rate_limit_burst_size: u32,
    pub cache_ttl_movie: u64,
    pub cache_ttl_availability: u64,
    pub cache_ttl_search: u64,
    pub cache_ttl_device_session: u64,
}

impl Config {
    /// Load configuration from environment variables.
    /// Requires dotenvy to have already loaded the .env file.
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok(); // ok() because .env may not exist in production

        Ok(Self {
            host: std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .context("PORT must be a valid number")?,
            environment: std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string()),
            database_url: std::env::var("DATABASE_URL").context("DATABASE_URL is required")?,
            redis_url: std::env::var("REDIS_URL").context("REDIS_URL is required")?,
            tmdb_api_key: std::env::var("TMDB_API_KEY").context("TMDB_API_KEY is required")?,
            tmdb_base_url: std::env::var("TMDB_BASE_URL")
                .unwrap_or_else(|_| "https://api.themoviedb.org/3".to_string()),
            tmdb_image_base_url: std::env::var("TMDB_IMAGE_BASE_URL")
                .unwrap_or_else(|_| "https://image.tmdb.org/t/p".to_string()),
            supabase_jwt_secret: std::env::var("SUPABASE_JWT_SECRET")
                .context("SUPABASE_JWT_SECRET is required")?,
            supabase_url: std::env::var("SUPABASE_URL").context("SUPABASE_URL is required")?,
            supabase_anon_key: std::env::var("SUPABASE_ANON_KEY")
                .context("SUPABASE_ANON_KEY is required")?,
            rate_limit_per_second: std::env::var("RATE_LIMIT_PER_SECOND")
                .unwrap_or_else(|_| "50".to_string())
                .parse()
                .context("RATE_LIMIT_PER_SECOND must be a valid number")?,
            rate_limit_burst_size: std::env::var("RATE_LIMIT_BURST_SIZE")
                .unwrap_or_else(|_| "100".to_string())
                .parse()
                .context("RATE_LIMIT_BURST_SIZE must be a valid number")?,
            cache_ttl_movie: std::env::var("CACHE_TTL_MOVIE")
                .unwrap_or_else(|_| "604800".to_string())
                .parse()
                .context("CACHE_TTL_MOVIE must be a valid number")?,
            cache_ttl_availability: std::env::var("CACHE_TTL_AVAILABILITY")
                .unwrap_or_else(|_| "259200".to_string())
                .parse()
                .context("CACHE_TTL_AVAILABILITY must be a valid number")?,
            cache_ttl_search: std::env::var("CACHE_TTL_SEARCH")
                .unwrap_or_else(|_| "3600".to_string())
                .parse()
                .context("CACHE_TTL_SEARCH must be a valid number")?,
            cache_ttl_device_session: std::env::var("CACHE_TTL_DEVICE_SESSION")
                .unwrap_or_else(|_| "3600".to_string())
                .parse()
                .context("CACHE_TTL_DEVICE_SESSION must be a valid number")?,
        })
    }

    pub fn is_production(&self) -> bool {
        self.environment == "production"
    }
}
