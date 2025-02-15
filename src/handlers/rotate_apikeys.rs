use axum::{extract::{Extension, Path, State}, Json};
use axum::http::StatusCode;
use chrono::{Duration, NaiveDate, Utc};
use serde_json::json;
use sqlx::postgres::PgPool;
use tracing::instrument;
use uuid::Uuid;
use validator::Validate;

use crate::utils::auth::{generate_api_key, hash_password};
use crate::models::user::User;
use crate::database::apikeys::{fetch_existing_apikey, insert_api_key_into_db, disable_apikey_in_db};
use crate::models::apikey::{ApiKeyRotateBody, ApiKeyRotateResponse, ApiKeyRotateResponseInfo};

#[utoipa::path(
    post,
    path = "/apikeys/rotate/{id}",
    tag = "apikey",
    security(
        ("jwt_token" = [])
    ),
    request_body = ApiKeyRotateBody,
    responses(
        (status = 200, description = "API key rotated successfully", body = ApiKeyRotateResponse),
        (status = 400, description = "Validation error", body = String),
        (status = 404, description = "API key not found", body = String),
        (status = 500, description = "Internal server error", body = String)
    ),
    params(
        ("id" = String, Path, description = "API key identifier")
    )
)]
#[instrument(skip(pool, user, apikeyrotatebody))]
pub async fn rotate_apikey(
    State(pool): State<PgPool>,
    Extension(user): Extension<User>,
    Path(id): Path<String>,
    Json(apikeyrotatebody): Json<ApiKeyRotateBody>
) -> Result<Json<ApiKeyRotateResponse>, (StatusCode, Json<serde_json::Value>)> {
    // Validate input
    if let Err(errors) = apikeyrotatebody.validate() {
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
        Err(_) => return Err((StatusCode::BAD_REQUEST, Json(json!({ "error": "Invalid API key identifier format" })))),
    };

    // Verify ownership of the old API key
    let existing_key = fetch_existing_apikey(&pool, user.id, uuid).await.map_err(|e| {
        tracing::error!("Database error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "Internal server error" })))
    })?.ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({ "error": "API key not found or already disabled" }))))?;

    // Validate expiration date format
    let expiration_date = match &apikeyrotatebody.expiration_date {
        Some(date_str) => NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
            .map_err(|_| (StatusCode::BAD_REQUEST, Json(json!({ "error": "Invalid expiration date format. Use YYYY-MM-DD" }))))?,
        None => (Utc::now() + Duration::days(365 * 2)).naive_utc().date(),
    };

    // Validate expiration date is in the future
    if expiration_date <= Utc::now().naive_utc().date() {
        return Err((StatusCode::BAD_REQUEST, Json(json!({ "error": "Expiration date must be in the future" }))));
    }

    // Generate new secure API key
    let api_key = generate_api_key();
    let key_hash = hash_password(&api_key).map_err(|e| {
        tracing::error!("Hashing error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "Internal server error" })))
    })?;

    // Create new key FIRST
    let description = apikeyrotatebody.description.unwrap_or_else(||
        format!("Rotated from key {} - {}", existing_key.id, Utc::now().format("%Y-%m-%d"))
    );

    let new_key = insert_api_key_into_db(&pool, key_hash, description, expiration_date, user.id).await.map_err(|e| {
        tracing::error!("Database error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "Internal server error" })))
    })?;

    // Attempt to disable old key
    let disable_result = match disable_apikey_in_db(&pool, uuid, user.id).await {
        Ok(res) => res,
        Err(e) => {
            tracing::error!("Database error: {}", e);
            // Rollback: Disable the newly created key
            let _ = disable_apikey_in_db(&pool, new_key.id, user.id).await;
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "Internal server error" }))));
        }
    };

    // Verify old key was actually disabled
    if disable_result == 0 {
        // Rollback: Disable new key
        let _ = disable_apikey_in_db(&pool, new_key.id, user.id).await;
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Old API key not found or already disabled" }))
        ));
    }

    // Create the ApiKeyRotateResponse
    let rotate_response = ApiKeyRotateResponse {
        id: new_key.id,
        api_key,
        description: new_key.description,
        expiration_date: expiration_date,
        rotation_info: ApiKeyRotateResponseInfo {
            original_key: existing_key.id,
            disabled_at: Utc::now().date_naive(),
        },
    };

    Ok(Json(rotate_response))
}
