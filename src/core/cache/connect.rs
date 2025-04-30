use deadpool_redis::{Config as RedisConfig, Pool, Runtime};
use thiserror::Error;
use url::Url;

use crate::core::config::{get_env, get_env_with_default};

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum RedisStorageError {
    #[error("❌  Environment error: {0}")]
    EnvError(String),

    #[error("❌  URL parse error: {0}")]
    UrlParseError(#[from] url::ParseError),

    #[error("❌  Redis config error: {0}")]
    RedisConfigError(String),

    #[error("❌  Redis connection error: {0}")]
    ConnectionError(String),
}

pub async fn connect_to_cache() -> Result<Pool, RedisStorageError> {
    // Load environment variables
    let endpoint_base = get_env("CACHE_ENDPOINT");
    let port = get_env_with_default("CACHE_PORT", "6379");
    let username = get_env_with_default("CACHE_USERNAME", "");
    let password = get_env_with_default("CACHE_PASSWORD", "");
    let db = get_env_with_default("CACHE_DB", "0");

    // Build Redis URL
    let mut url = String::from("redis://");
    if !username.is_empty() && !password.is_empty() {
        url.push_str(&format!("{}:{}@", username, password));
    } else if !password.is_empty() {
        url.push_str(&format!(":{}@", password));
    }
    url.push_str(&format!("{}:{}/{}", endpoint_base.trim_end_matches('/'), port, db));
    Url::parse(&url)?; // Validate URL

    // Build deadpool-redis config using only the URL field
    let mut cfg = RedisConfig::default();
    cfg.url = Some(url);
    cfg.connection = None; // Explicitly ensure this is None (should be by default)

    // Create connection pool
    let pool = cfg
        .create_pool(Some(Runtime::Tokio1))
        .map_err(|e| RedisStorageError::RedisConfigError(e.to_string()))?;

    // Test connection (map errors to your error type)
    {
        let mut conn = pool
            .get()
            .await
            .map_err(|e| RedisStorageError::ConnectionError(e.to_string()))?;
        use deadpool_redis::redis::AsyncCommands;
        let _: () = conn.ping()
            .await
            .map_err(|e| RedisStorageError::ConnectionError(e.to_string()))?;
    }

    Ok(pool)
}
