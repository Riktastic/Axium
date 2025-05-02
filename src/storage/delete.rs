use aws_sdk_s3::error::ProvideErrorMetadata;

use crate::storage::StorageState;

/// Deletes an object from S3/MinIO storage
///
/// # Arguments
/// - `s3_client`: Configured S3 client
/// - `bucket`: Target bucket name
/// - `object_key`: Object identifier to delete
///
/// # Returns
/// - `Ok(())` on successful deletion
/// - `Err(String)` with detailed error message on failure
#[allow(dead_code)]
pub async fn delete_from_storage(
    state: &StorageState,
    bucket: &str,
    object_key: &str,
) -> Result<(), String> {
    // Input validation
    if bucket.trim().is_empty() {
        return Err("Delete error: bucket name is empty".to_string());
    }
    if object_key.trim().is_empty() {
        return Err("Delete error: object key is empty".to_string());
    }

    let delete_result = state.client
        .delete_object()
        .bucket(bucket)
        .key(object_key)
        .send()
        .await;

    match delete_result {
        Ok(_) => Ok(()),
        Err(err) => {
            let code = err.code().unwrap_or("Unknown");
            let message = err.message().unwrap_or("No error message provided");
            Err(format!(
                "Failed to delete {}/{} (code: {}): {}",
                bucket, object_key, code, message
            ))
        }
    }
}
