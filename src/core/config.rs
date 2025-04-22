// Import the standard library's environment module
use std::env;

/// Retrieves the value of an environment variable as a `String`.
/// 
/// # Arguments
/// * `key` - The name of the environment variable to retrieve.
/// 
/// # Returns
/// * The value of the environment variable if it exists.
/// * Panics if the environment variable is missing.
pub fn get_env(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| panic!("Missing required environment variable: {}", key))
}

/// Retrieves the value of an environment variable as a `String`, with a default value if not found.
/// 
/// # Arguments
/// * `key` - The name of the environment variable to retrieve.
/// * `default` - The value to return if the environment variable is not found.
/// 
/// # Returns
/// * The value of the environment variable if it exists.
/// * The `default` value if the environment variable is missing.
pub fn get_env_with_default(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}

/// Retrieves the value of an environment variable as a `bool`, with a default value if not found.
/// 
/// The environment variable is considered `true` if its value is "true" (case-insensitive), otherwise `false`.
/// 
/// # Arguments
/// * `key` - The name of the environment variable to retrieve.
/// * `default` - The value to return if the environment variable is not found.
/// 
/// # Returns
/// * `true` if the environment variable is "true" (case-insensitive).
/// * `false` otherwise, or if the variable is missing, the `default` value is returned.
pub fn get_env_bool(key: &str, default: bool) -> bool {
    env::var(key).map(|v| v.to_lowercase() == "true").unwrap_or(default)
}

/// Retrieves the value of an environment variable as a `u16`, with a default value if not found.
/// 
/// # Arguments
/// * `key` - The name of the environment variable to retrieve.
/// * `default` - The value to return if the environment variable is not found or cannot be parsed.
/// 
/// # Returns
/// * The parsed `u16` value of the environment variable if it exists and is valid.
/// * The `default` value if the variable is missing or invalid.
pub fn get_env_u16(key: &str, default: u16) -> u16 {
    env::var(key).unwrap_or_else(|_| default.to_string()).parse().unwrap_or(default)
}

/// Retrieves the value of an environment variable as a `u64`, with a default value if not found.
/// 
/// # Arguments
/// * `key` - The name of the environment variable to retrieve.
/// * `default` - The value to return if the environment variable is not found or cannot be parsed.
/// 
/// # Returns
/// * The parsed `u64` value of the environment variable if it exists and is valid.
/// * The `default` value if the variable is missing or invalid.
pub fn get_env_u64(key: &str, default: u64) -> u64 {
    env::var(key).unwrap_or_else(|_| default.to_string()).parse().unwrap_or(default)
}
