use deadpool_redis::Pool;
use deadpool_redis::redis::AsyncCommands; 

/// Deletes a key from Redis.
/// Returns Ok(true) if the key was deleted, Ok(false) if the key did not exist,
/// or Err(String) with error details.
#[allow(dead_code)]
pub async fn delete_from_cache(
    redis_pool: &Pool,
    key: &str,
) -> Result<bool, String> {
    // Input validation
    if key.trim().is_empty() {
        return Err("Redis delete error: key is empty".to_string());
    }

    // Get a connection from the pool
    let mut conn = redis_pool.get().await
        .map_err(|e| format!("Failed to get Redis connection: {e}"))?;

    // DEL returns the number of keys removed (0 if the key did not exist)
    let deleted: u64 = conn.del(key).await
        .map_err(|e| format!("Failed to delete key from Redis: {e}"))?;

    Ok(deleted > 0)
}
