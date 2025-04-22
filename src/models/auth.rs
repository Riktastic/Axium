use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
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
/// Struct for authentication and authorization errors
#[derive(Debug, Serialize)]
pub struct AuthError {
    pub message: String,
    #[serde(skip_serializing)]
    pub status_code: StatusCode,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let body = Json(serde_json::json!({
            "error": self.message,
        }));

        (self.status_code, body).into_response()
    }
}

/// Data structure for user sign-in information.
///
/// This includes the user's email, password, and optionally a TOTP code.
#[derive(Deserialize, ToSchema)]
pub struct LoginData {
    /// User's email address.
    pub email: String,
    /// User's password.
    pub password: String,
    /// Optional TOTP code for two-factor authentication.
    pub totp: Option<String>,
}