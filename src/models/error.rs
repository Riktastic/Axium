use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Error response structure to standardize error outputs
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ErrorResponse {
    pub error: String,
    pub details: Option<String>, // Optional field to provide more error details
}

#[allow(dead_code)]
impl ErrorResponse {
    pub fn new(error: &str, details: Option<String>) -> Self {
        Self {
            error: error.to_string(),
            details,
        }
    }

    // For convenience, you could create a helper function for common error messages
    pub fn bad_request(details: Option<String>) -> Self {
        Self::new("Bad request", details)
    }

    pub fn unauthorized(details: Option<String>) -> Self {
        Self::new("Unauthorized", details)
    }

    pub fn forbidden(details: Option<String>) -> Self {
        Self::new("Forbidden", details)
    }

    pub fn internal_server_error(details: Option<String>) -> Self {
        Self::new("Internal server error", details)
    }
}