use axum::{extract::{Extension, Path, State}, Json};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use chrono::{Duration, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::postgres::PgPool;
use tracing::instrument;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::handlers::validate::validate_future_date;
use crate::middlewares::auth::{generate_api_key, hash_password};
use crate::models::user::User;

#[derive(Deserialize, Validate, ToSchema)]
pub struct ApiKeyBody {
    #[validate(length(min = 0, max = 50))]
    pub description: Option<String>,
    #[validate(custom(function = "validate_future_date"))]
    pub expiration_date: Option<String>,
}


#[derive(Serialize, ToSchema)]
pub struct ApiKeyResponse {
    pub id: Uuid,
    pub description: Option<String>,
}

#[utoipa::path(
    post,
    path = "/apikeys/rotate/{id}",
    tag = "apikey",
    request_body = ApiKeyBody,
    responses(
        (status = 200, description = "API key rotated successfully", body = ApiKeyResponse),
        (status = 400, description = "Validation error", body = String),
        (status = 404, description = "API key not found", body = String),
        (status = 500, description = "Internal server error", body = String)
    ),
    params(
        ("id" = String, Path, description = "API key identifier")
    )
)]
#[instrument(skip(pool, user, apikeybody))]
pub async fn rotate_apikey(
    State(pool): State<PgPool>,
    Extension(user): Extension<User>,
    Path(id): Path<String>,
    Json(apikeybody): Json<ApiKeyBody>
) -> impl IntoResponse {
    // Validate input
    if let Err(errors) = apikeybody.validate() {
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

    // Validate UUID format
    let uuid = match Uuid::parse_str(&id) {
        Ok(uuid) => uuid,
        Err(_) => return Err((StatusCode::BAD_REQUEST, 
            Json(json!({ "error": "Invalid API key identifier format" })))),
    };

    // Verify ownership of the API key
    let existing_key = sqlx::query_as!(ApiKeyResponse,
        "SELECT id, description FROM apikeys 
        WHERE user_id = $1 AND id = $2 AND disabled = FALSE",
        user.id,
        uuid
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, 
         Json(json!({ "error": "Internal server error" })))
    })?;

    let existing_key = existing_key.ok_or_else(|| 
        (StatusCode::NOT_FOUND, 
         Json(json!({ "error": "API key not found or already disabled" })))
    )?;

    // Validate expiration date format
    let expiration_date = match &apikeybody.expiration_date {
        Some(date_str) => NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
            .map_err(|_| (StatusCode::BAD_REQUEST, 
                Json(json!({ "error": "Invalid expiration date format. Use YYYY-MM-DD" }))))?,
        None => (Utc::now() + Duration::days(365 * 2)).naive_utc().date(),
    };

    // Validate expiration date is in the future
    if expiration_date <= Utc::now().naive_utc().date() {
        return Err((StatusCode::BAD_REQUEST, 
            Json(json!({ "error": "Expiration date must be in the future" }))));
    }

    // Generate new secure API key
    let api_key = generate_api_key();
    let key_hash = hash_password(&api_key)
        .map_err(|e| {
            tracing::error!("Hashing error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, 
             Json(json!({ "error": "Internal server error" })))
        })?;

    // Begin transaction
    let mut tx = pool.begin().await.map_err(|e| {
        tracing::error!("Transaction error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, 
         Json(json!({ "error": "Internal server error" })))
    })?;

    // Disable old key
    let disable_result = sqlx::query!(
        "UPDATE apikeys SET 
            disabled = TRUE, 
            expiration_date = CURRENT_DATE + INTERVAL '1 day'
        WHERE id = $1 AND user_id = $2",
        uuid,
        user.id
    )
    .execute(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, 
         Json(json!({ "error": "Internal server error" })))
    })?;

    if disable_result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, 
            Json(json!({ "error": "API key not found or already disabled" }))));
    }

    // Create new key with automatic description
    let description = apikeybody.description.unwrap_or_else(|| 
        format!("Rotated from key {} - {}", 
            existing_key.id, 
            Utc::now().format("%Y-%m-%d"))
    );

    let new_key = sqlx::query!(
        "INSERT INTO apikeys 
            (key_hash, description, expiration_date, user_id, access_read, access_modify)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, description, expiration_date",
        key_hash,
        description,
        expiration_date,
        user.id,
        true,  // Default read access
        false  // Default no modify access
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, 
         Json(json!({ "error": "Internal server error" })))
    })?;

    tx.commit().await.map_err(|e| {
        tracing::error!("Transaction commit error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, 
         Json(json!({ "error": "Internal server error" })))
    })?;

    Ok(Json(json!({
        "id": new_key.id,
        "api_key": api_key,
        "description": new_key.description,
        "expiration_date": new_key.expiration_date,
        "warning": "Store this key securely - it won't be shown again",
        "rotation_info": {
            "original_key": existing_key.id,
            "disabled_at": Utc::now().to_rfc3339()
        }
    })))
}