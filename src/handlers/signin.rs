use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use serde_json::json;
use sqlx::PgPool;
use totp_rs::{Algorithm, TOTP};
use tracing::{info, instrument};
use utoipa::ToSchema;

use crate::utils::auth::{encode_jwt, verify_hash};
use crate::database::{apikeys::fetch_active_apikeys_by_user_id_from_db, users::fetch_user_by_email_from_db};
use crate::models::auth::SignInData;

/// User sign-in endpoint.
///
/// This endpoint allows users to sign in using their email, password, and optionally a TOTP code.
///
/// # Parameters
/// - `State(pool)`: The shared database connection pool.
/// - `Json(user_data)`: The user sign-in data (email, password, and optional TOTP code).
///
/// # Returns
/// - `Ok(Json(serde_json::Value))`: A JSON response containing the JWT token if sign-in is successful.
/// - `Err((StatusCode, Json(serde_json::Value)))`: An error response if sign-in fails.
#[utoipa::path(
    post,
    path = "/signin",
    tag = "auth",
    request_body = SignInData,
    responses(
        (status = 200, description = "Successful sign-in", body = serde_json::Value),
        (status = 400, description = "Bad request", body = serde_json::Value),
        (status = 401, description = "Unauthorized", body = serde_json::Value),
        (status = 500, description = "Internal server error", body = serde_json::Value)
    )
)]
#[instrument(skip(pool, user_data))]
pub async fn signin(
    State(pool): State<PgPool>,
    Json(user_data): Json<SignInData>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Fetch the user from the database based on their email.
    let user = match fetch_user_by_email_from_db(&pool, &user_data.email).await {
        Ok(Some(user)) => user,
        Ok(None) | Err(_) => {
            // If the user is not found or there's an error, return an unauthorized response.
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": "Incorrect credentials." }))
            ));
        }
    };

    // Fetch active API keys for the user.
    let api_key_hashes = match fetch_active_apikeys_by_user_id_from_db(&pool, user.id).await {
        Ok(hashes) => hashes,
        Err(_) => {
            // If there's an error fetching API keys, return an internal server error.
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Internal server error." }))
            ));
        }
    };

    // Check if any of the API keys match the provided password.
    let api_key_futures = api_key_hashes.iter().map(|api_key| {
        let password = user_data.password.clone();
        let hash = api_key.key_hash.clone();
        async move {
            // Verify the password against each API key hash.
            verify_hash(&password, &hash)
                .await
                .unwrap_or(false)
        }
    });

    // Wait for all API key verification futures to complete.
    let any_api_key_valid = futures::future::join_all(api_key_futures)
        .await
        .into_iter()
        .any(|result| result);

    // Verify the user's password against their stored password hash.
    let password_valid = verify_hash(&user_data.password, &user.password_hash)
        .await
        .map_err(|_| {
            // If there's an error verifying the password, return an internal server error.
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Internal server error." }))
            )
        })?;

    // Determine if the credentials are valid based on API keys or password.
    let credentials_valid = any_api_key_valid || password_valid;

    if !credentials_valid {
        // If credentials are not valid, return an unauthorized response.
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "Incorrect credentials." }))
        ));
    }

    // Check TOTP if it's set up for the user.
    if let Some(totp_secret) = user.totp_secret {
        match user_data.totp {
            Some(totp_code) => {
                // Create a TOTP instance with the user's secret.
                let totp = TOTP::new(
                    Algorithm::SHA512,
                    8,
                    1,
                    30,
                    totp_secret.into_bytes(),
                ).map_err(|_| {
                    // If there's an error creating the TOTP instance, return an internal server error.
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({ "error": "Internal server error." }))
                    )
                })?;

                // Check if the provided TOTP code is valid.
                if !totp.check_current(&totp_code).unwrap_or(false) {
                    // If the TOTP code is invalid, return an unauthorized response.
                    return Err((
                        StatusCode::UNAUTHORIZED,
                        Json(json!({ "error": "Invalid 2FA code." }))
                    ));
                }
            },
            None => {
                // If TOTP is set up but no code is provided, return a bad request.
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "error": "2FA code required for this account." }))
                ));
            }
        }
    }

    // Generate a JWT token for the user.
    let email = user.email.clone();
    let token = encode_jwt(user.email)
        .map_err(|_| {
            // If there's an error generating the JWT, return an internal server error.
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Internal server error." }))
            )
        })?;

    // Log the successful sign-in.
    info!("User signed in: {}", email);

    // Return the JWT token in a JSON response.
    Ok(Json(json!({ "token": token })))
}
