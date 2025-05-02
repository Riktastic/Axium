use aws_sdk_s3::presigning::PresigningConfig;
use std::time::Duration;

use crate::storage::StorageState;

/// Generates a pre-signed (pre-authenticated) URL for accessing a private S3/MinIO object.
///
/// This function creates a temporary, signed URL that allows anyone with the link to access
/// the specified object in your storage bucket, even if the bucket is not public. The URL
/// is valid for the given number of seconds (`expires_in_seconds`).
///
/// # Arguments
///
/// * `state` - Reference to the `StorageState` containing the S3 client.
/// * `bucket` - The name of the S3/MinIO bucket.
/// * `object_key` - The key (path) of the object in the bucket.
/// * `expires_in_seconds` - Duration in seconds for which the URL will remain valid.
///
/// # Returns
///
/// * `Ok(String)` containing the pre-signed URL if successful.
/// * `Err(String)` with an error message if the URL could not be generated.
///
/// # Examples
///
/// ```
/// # use your_crate::{StorageState, generate_presigned_url};
/// # async fn example(state: &StorageState) -> Result<(), String> {
/// let url = generate_presigned_url(state, "mybucket", "path/to/object.jpg", 900).await?;
/// println!("Pre-signed URL: {}", url);
/// # Ok(())
/// # }
/// ```
///
/// # Errors
///
/// Returns an error if the presigning configuration cannot be created or if the S3 client fails to generate the URL.
///
/// # See also
///
/// - [AWS S3 Pre-signed URLs](https://docs.aws.amazon.com/AmazonS3/latest/userguide/ShareObjectPreSignedURL.html)
///
pub async fn generate_presigned_url(
    state: &StorageState,
    bucket: &str,
    object_key: &str,
    expires_in_seconds: u64,
) -> Result<String, String> {
    let presign_config = PresigningConfig::expires_in(Duration::from_secs(expires_in_seconds))
        .map_err(|e| format!("Failed to create presign config: {}", e))?;

    let presigned_req = state
        .client
        .get_object()
        .bucket(bucket)
        .key(object_key)
        .presigned(presign_config)
        .await
        .map_err(|e| format!("Failed to presign URL: {}", e))?;

    Ok(presigned_req.uri().to_string())
}

