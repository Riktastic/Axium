use aws_sdk_s3::{
    Client as S3Client,
    error::ProvideErrorMetadata,
};

/// Deletes an object from S3/MinIO storage
///
/// # Arguments
/// - `s3_client`: Configured S3 client
/// - `bucket`: Target bucket name
/// - `object_key`: Object identifier to delete
/// - `endpoint`: Endpoint URL (for error message context)
///
/// # Returns
/// - `Ok(())` on successful deletion
/// - `Err(String)` with detailed error message on failure
pub async fn delete_from_storage(
    s3_client: &S3Client,
    bucket: &str,
    object_key: &str,
    endpoint: &str,
) -> Result<(), String> {
    // Input validation
    if bucket.trim().is_empty() {
        return Err("Delete error: bucket name is empty".to_string());
    }
    if object_key.trim().is_empty() {
        return Err("Delete error: object key is empty".to_string());
    }
    if endpoint.trim().is_empty() {
        return Err("Delete error: endpoint is empty".to_string());
    }

    let delete_result = s3_client
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
