use aws_sdk_s3::Client as S3Client;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::error::ProvideErrorMetadata;

/// Uploads a file to S3/MinIO and returns the public URL (or error)
#[allow(dead_code)]
pub async fn upload_to_storage(
    s3_client: &S3Client,
    bucket: &str,
    object_key: &str,
    data: &[u8],
    endpoint: &str, // e.g. "http://127.0.0.1:9000"
) -> Result<String, String> {
    // Input validation
    if bucket.trim().is_empty() {
        return Err("Upload error: bucket name is empty".to_string());
    }
    if object_key.trim().is_empty() {
        return Err("Upload error: object key is empty".to_string());
    }
    if endpoint.trim().is_empty() {
        return Err("Upload error: endpoint is empty".to_string());
    }
    if data.is_empty() {
        return Err("Upload error: data buffer is empty".to_string());
    }

    let body = ByteStream::from(data.to_vec());
    let put_result = s3_client
        .put_object()
        .bucket(bucket)
        .key(object_key)
        .body(body)
        .send()
        .await;

    match put_result {
        Ok(_) => Ok(format!(
            "{}/{}/{}",
            endpoint.trim_end_matches('/'),
            bucket,
            object_key
        )),
        Err(err) => {
            // Try to extract more info from the error, if available
            let code = err.code().unwrap_or("Unknown");
            let message = err.message().unwrap_or("No error message provided");
            Err(format!(
                "Failed to upload to storage (code: {}): {}",
                code, message
            ))
        }
    }
}
