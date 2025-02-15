use axum::{extract::{Extension, State}, Json};
use axum::http::StatusCode;
use chrono::{Duration, Utc};
use serde_json::json;
use sqlx::postgres::PgPool;
use tracing::{error, info};
use validator::Validate;

use crate::utils::auth::{generate_api_key, hash_password};
use crate::models::user::User;
use crate::database::apikeys::{check_existing_api_key_count, insert_api_key_into_db};
use crate::models::apikey::{ApiKeyInsertBody, ApiKeyInsertResponse};

// --- Route Handler ---

// Define the API endpoint
#[utoipa::path(
    post,
    path = "/apikeys",
    tag = "apikey",
    security(
        ("jwt_token" = [])
    ),
    request_body = ApiKeyInsertBody,
    responses(
        (status = 200, description = "API key created successfully", body = ApiKeyInsertResponse),
        (status = 400, description = "Validation error", body = String),
        (status = 401, description = "Unauthorized", body = serde_json::Value),
        (status = 500, description = "Internal server error", body = String)
    )
)]
pub async fn post_apikey(
    State(pool): State<PgPool>,
    Extension(user): Extension<User>,
    Json(api_key_request): Json<ApiKeyInsertBody>
) -> Result<Json<ApiKeyInsertResponse>, (StatusCode, Json<serde_json::Value>)> {
    // Validate input
    if let Err(errors) = api_key_request.validate() {
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

    info!("Received request to create API key for user: {}", user.id);

    // Check if the user already has 5 or more API keys
    let existing_keys_count = match check_existing_api_key_count(&pool, user.id).await {
        Ok(count) => count,
        Err(err) => {
            error!("Failed to check the amount of API keys for user {}: {}", user.id, err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Could not check the amount of API keys registered." }))
            ));
        }
    };

    if existing_keys_count >= 5 {
        info!("User {} already has 5 API keys.", user.id);
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "You already have 5 API keys. Please delete an existing key before creating a new one." }))
        ));
    }

    let current_date = Utc::now().naive_utc();
    let description = api_key_request.description
        .unwrap_or_else(|| format!("API key created on {}", current_date.format("%Y-%m-%d")));

    let expiration_date = api_key_request.expiration_date
        .and_then(|date| date.parse::<chrono::NaiveDate>().ok())
        .unwrap_or_else(|| (current_date + Duration::days(365 * 2)).date());

    let api_key = generate_api_key();
    let key_hash = hash_password(&api_key).expect("Failed to hash password.");

    match insert_api_key_into_db(&pool, key_hash, description, expiration_date, user.id).await {
        Ok(mut api_key_response) => {
            info!("Successfully created API key for user: {}", user.id);
            // Restore generated api_key to response. It is not stored in database for security reasons.
            api_key_response.api_key = api_key;
            Ok(Json(api_key_response))
        }
        Err(err) => {
            error!("Error creating API key for user {}: {}", user.id, err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Error creating API key: {}.", err) }))
            ))
        }
    }
}
