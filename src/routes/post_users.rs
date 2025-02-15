use axum::{extract::State, Json, response::IntoResponse};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::postgres::PgPool;
use tracing::instrument;
use uuid::Uuid;
use utoipa::ToSchema;
use validator::Validate;

use crate::handlers::validate::{validate_password, validate_username};
use crate::middlewares::auth::{hash_password, generate_totp_secret};

// Define the request body structure
#[derive(Deserialize, Validate, ToSchema)]
pub struct UserBody {
    #[validate(length(min = 3, max = 50), custom(function = "validate_username"))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(custom(function = "validate_password"))]
    pub password: String,
    pub totp: Option<String>,
}

// Define the response body structure
#[derive(Serialize, ToSchema)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub totp_secret: Option<String>,
    pub role_level: i32,
}

// Define the API endpoint
#[utoipa::path(
    post,
    path = "/users",
    tag = "user",
    request_body = UserBody,
    responses(
        (status = 200, description = "User created successfully", body = UserResponse),
        (status = 400, description = "Validation error", body = String),
        (status = 500, description = "Internal server error", body = String)
    )
)]
#[instrument(skip(pool, user))]
pub async fn post_user(
    State(pool): State<PgPool>,
    Json(user): Json<UserBody>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Validate input
    if let Err(errors) = user.validate() {
        let error_messages: Vec<String> = errors
            .field_errors()
            .iter()
            .flat_map(|(_, errors)| errors.iter().map(|e| e.message.clone().unwrap_or_default().to_string()))
            .collect();
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": error_messages.join(", ") }))
        ));
    }

    // Hash the password before saving it
    let hashed_password = hash_password(&user.password)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "Failed to hash password." }))))?;

    // Generate TOTP secret if totp is Some("true")
    let totp_secret = if user.totp.as_deref() == Some("true") {
        Some(generate_totp_secret())
    } else {
        None
    };

    let row = sqlx::query!(
        "INSERT INTO users (username, email, password_hash, totp_secret, role_level) 
         VALUES ($1, $2, $3, $4, $5) 
         RETURNING id, username, email, password_hash, totp_secret, role_level",
        user.username,
        user.email,
        hashed_password,
        totp_secret,
        1, // Default role_level
    )
    .fetch_one(&pool)
    .await
    .map_err(|_err| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "Could not create the user."}))))?;

    Ok(Json(UserResponse {
        id: row.id,
        username: row.username,
        email: row.email,
        totp_secret: row.totp_secret,
        role_level: row.role_level,
    }))
}
