use axum::{extract::{Extension, State}, Json};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::postgres::PgPool;
use tracing::{error, info};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::handlers::validate::validate_future_date;
use crate::middlewares::auth::{generate_api_key, hash_password};
use crate::models::user::User;

// Define the request body structure
#[derive(Deserialize, Validate, ToSchema)]
pub struct ApiKeyBody {
    #[validate(length(min = 0, max = 50))]
    pub description: Option<String>,
    #[validate(custom(function = "validate_future_date"))]
    pub expiration_date: Option<String>,
}

// Define the response body structure
#[derive(Serialize, ToSchema)]
pub struct ApiKeyResponse {
    pub id: Uuid,
    pub api_key: String,
    pub description: String,
    pub expiration_date: String,
}

// Define the API endpoint
#[utoipa::path(
    post,
    path = "/apikeys",
    tag = "apikey",
    request_body = ApiKeyBody,
    responses(
        (status = 200, description = "API key created successfully", body = ApiKeyResponse),
        (status = 400, description = "Validation error", body = String),
        (status = 500, description = "Internal server error", body = String)
    )
)]
pub async fn post_apikey(
    State(pool): State<PgPool>, 
    Extension(user): Extension<User>,
    Json(api_key_request): Json<ApiKeyBody>
) -> impl IntoResponse {
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
    let existing_keys_count = sqlx::query!(
        "SELECT COUNT(*) as count FROM apikeys WHERE user_id = $1 AND expiration_date >= CURRENT_DATE",
        user.id
    )
    .fetch_one(&pool)
    .await;

    match existing_keys_count {
        Ok(row) if row.count.unwrap_or(0) >= 5 => {
            info!("User {} already has 5 API keys.", user.id);
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "You already have 5 API keys. Please delete an existing key before creating a new one." }))
            ));
        }
        Err(_err) => {
            error!("Failed to check the amount of API keys for user {}.", user.id);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Could not check the amount of API keys registered." }))
            ));
        }
        _ => {} // Proceed if the user has fewer than 5 keys
    }

    let current_date = Utc::now().naive_utc();
    let description = api_key_request.description
        .unwrap_or_else(|| format!("API key created on {}", current_date.format("%Y-%m-%d")));
    
    let expiration_date = api_key_request.expiration_date
        .and_then(|date| date.parse::<chrono::NaiveDate>().ok())
        .unwrap_or_else(|| (current_date + Duration::days(365 * 2)).date());

    let api_key = generate_api_key();

    let key_hash = hash_password(&api_key).expect("Failed to hash password.");

    let row = sqlx::query!(
        "INSERT INTO apikeys (key_hash, description, expiration_date, user_id) VALUES ($1, $2, $3, $4) RETURNING id, key_hash, description, expiration_date, user_id",
        key_hash,
        description,
        expiration_date,
        user.id
    )
    .fetch_one(&pool)
    .await;

    match row {
        Ok(row) => {
            info!("Successfully created API key for user: {}", user.id);
            Ok(Json(ApiKeyResponse {
                id: row.id,
                api_key: api_key,
                description: description.to_string(),
                expiration_date: expiration_date.to_string()
            }))
        },
        Err(err) => {
            error!("Error creating API key for user {}: {}", user.id, err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Error creating API key: {}.", err) }))
            ))
        },
    }
}
