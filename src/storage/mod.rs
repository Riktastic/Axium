// Module declarations
pub mod connect;
pub mod upload;
pub mod delete;
pub mod presign_url;

use aws_sdk_s3::Client as S3Client;

#[derive(Clone, Debug)]
pub struct StorageState {
    pub client: S3Client,
    pub endpoint_url: String, // e.g. "http://127.0.0.1:9000"
}
