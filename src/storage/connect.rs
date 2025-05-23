use aws_sdk_s3::{
    Client as S3Client,
    config::{Region, Credentials},
};
use thiserror::Error;
use url::Url;

use crate::core::config::{get_env, get_env_with_default};
use crate::storage::StorageState;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum StorageError {
    #[error("❌  Environment error: {0}")]
    EnvError(String),

    #[error("❌  URL parse error: {0}")]
    UrlParseError(#[from] url::ParseError),

    #[error("❌  AWS config error: {0}")]
    AwsConfigError(String),

    #[error("❌  Storage connection error: {0}")]
    ConnectionError(String),

    #[error("❌  Storage operation error: {0}")]
    OperationError(String),
}

pub async fn connect_to_storage() -> Result<StorageState, StorageError> {
    // Load environment variables with clear errors
    let endpoint_base = get_env("STORAGE_HOST");
    let port = get_env_with_default("STORAGE_PORT", "9000"); // Default 
    let region = get_env_with_default("STORAGE_REGION", "us-east-1");
    let access_key = get_env("STORAGE_ACCESS_KEY");
    let secret_key = get_env("STORAGE_SECRET_KEY");

    // Validate endpoint URL
    let endpoint = format!("{}:{}", endpoint_base.trim_end_matches('/'), port);
    let endpoint_url = Url::parse(&endpoint)?;

    // Build base AWS config
    let base_config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .region(Region::new(region.clone()))
        .load()
        .await;

    // Build S3 config with custom endpoint and credentials
    let s3_config = aws_sdk_s3::config::Builder::from(&base_config)
        .region(Region::new(region))
        .endpoint_url(endpoint_url.as_str())
        .credentials_provider(Credentials::new(
            access_key,
            secret_key,
            None, // session_token
            None, // expiration
            "custom", // provider name
        ))
        .build();

    // Create the S3 client
    let client = S3Client::from_conf(s3_config);

    // Verify the connection by listing the buckets
    match client.list_buckets().send().await {
        Ok(response) => {
            if response.buckets().is_empty() {
                Err(StorageError::ConnectionError("No buckets found in storage".to_string()))
            } else {
                Ok(StorageState {
                    client,
                    endpoint_url: endpoint_url.to_string(),
                })
            }
        },
        Err(err) => {
            Err(StorageError::ConnectionError(format!("Failed to connect to storage: {}", err)))
        }
    }
}
