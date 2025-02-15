use axum::http::StatusCode;
use utoipa::ToSchema;
use serde::{Serialize, Deserialize};

/// Represents the claims to be included in a JWT payload.
#[derive(Serialize, Deserialize, ToSchema)]
pub struct Claims {
    /// Subject of the token (e.g., user ID or email).
    pub sub: String,
    
    /// Timestamp when the token was issued.
    pub iat: usize,
    
    /// Timestamp when the token will expire.
    pub exp: usize,
    
    /// Issuer of the token (optional).
    pub iss: String,
    
    /// Intended audience for the token (optional).
    pub aud: String,
}

/// Custom error type for handling authentication-related errors.
pub struct AuthError {
    /// Descriptive error message.
    pub message: String,
    
    /// HTTP status code to be returned with the error.
    pub status_code: StatusCode,
}

// Implement Display trait for AuthError if needed
// impl std::fmt::Display for AuthError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.message)
//     }
// }

// Implement Error trait for AuthError if needed
// impl std::error::Error for AuthError {}
