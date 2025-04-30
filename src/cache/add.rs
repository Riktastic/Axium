use deadpool_redis::Pool;
use deadpool_redis::redis::AsyncCommands;

/// Adds a value to Redis under the specified key.
/// Returns Ok(()) on success, or Err(String) with error details.
#[allow(dead_code)]
pub async fn add_to_cache(
    redis_pool: &Pool,
    key: &str,
    value: &str,
) -> Result<(), String> {
    // Input validation
    if key.trim().is_empty() {
        return Err("Redis set error: key is empty".to_string());
    }
    if value.is_empty() {
        return Err("Redis set error: value is empty".to_string());
    }

    // Get a connection from the pool
    let mut conn = redis_pool.get().await
        .map_err(|e| format!("Failed to get Redis connection: {e}"))?;

    // Set the key-value pair asynchronously, explicitly specify return type
    let _: () = conn.set(key, value).await
        .map_err(|e| format!("Failed to set value in Redis: {e}"))?;

    Ok(())
}
