use axum::{extract::State, Json};
use axum::http::StatusCode;
use serde_json::json;
use sqlx::postgres::PgPool;
use tracing::instrument;
use validator::Validate;

use crate::utils::auth::{hash_password, generate_totp_secret};
use crate::database::users::insert_user_into_db;
use crate::models::user::{UserInsertResponse, UserInsertBody};

// --- Route Handler ---

// Define the API endpoint
#[utoipa::path(
    post,
    path = "/users",
    tag = "user",
    security(
        ("jwt_token" = [])
    ),
    request_body = UserInsertBody,
    responses(
        (status = 200, description = "User created successfully", body = UserInsertResponse),
        (status = 400, description = "Validation error", body = String),
        (status = 401, description = "Unauthorized", body = serde_json::Value),
        (status = 500, description = "Internal server error", body = String)
    )
)]
#[instrument(skip(pool, user))]
pub async fn post_user(
    State(pool): State<PgPool>,
    Json(user): Json<UserInsertBody>,
) -> Result<Json<UserInsertResponse>, (StatusCode, Json<serde_json::Value>)> {
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
        generate_totp_secret()
    } else {
        String::new() // or some other default value
    };

    match insert_user_into_db(&pool, &user.username, &user.email, &hashed_password, &totp_secret, 1, 1).await {
        Ok(new_user) => Ok(Json(new_user)),
        Err(_err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Could not create the user." }))
        )),
    }
}