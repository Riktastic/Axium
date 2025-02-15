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

#[derive(Deserialize, ToSchema)]
pub struct SignInData {
    pub email: String,
    pub password: String,
    pub totp: Option<String>,
}

/// User sign-in endpoint
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
    let user = match fetch_user_by_email_from_db(&pool, &user_data.email).await {
        Ok(Some(user)) => user,
        Ok(None) | Err(_) => return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "Incorrect credentials." }))
        )),
    };

    let api_key_hashes = match fetch_active_apikeys_by_user_id_from_db(&pool, user.id).await {
        Ok(hashes) => hashes,
        Err(_) => return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Internal server error." }))
        )),
    };
    
    // Check API key first (async version)
    let api_key_futures = api_key_hashes.iter().map(|api_key| {
        let password = user_data.password.clone();
        let hash = api_key.key_hash.clone();
        async move {
            verify_hash(&password, &hash)
                .await
                .unwrap_or(false)
        }
    });
    
    let any_api_key_valid = futures::future::join_all(api_key_futures)
        .await
        .into_iter()
        .any(|result| result);
    
    // Check password (async version)
    let password_valid = verify_hash(&user_data.password, &user.password_hash)
        .await
        .map_err(|_| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Internal server error." }))
        ))?;
    
    let credentials_valid = any_api_key_valid || password_valid;
    
    if !credentials_valid {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "Incorrect credentials." }))
        ));
    }

    // Check TOTP if it's set up for the user
    if let Some(totp_secret) = user.totp_secret {
        match user_data.totp {
            Some(totp_code) => {
                let totp = TOTP::new(
                    Algorithm::SHA512,
                    8,
                    1,
                    30,
                    totp_secret.into_bytes(),
                ).map_err(|_| (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": "Internal server error." }))
                ))?;

                if !totp.check_current(&totp_code).unwrap_or(false) {
                    return Err((
                        StatusCode::UNAUTHORIZED,
                        Json(json!({ "error": "Invalid 2FA code." }))
                    ));
                }
            },
            None => return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "2FA code required for this account." }))
            )),
        }
    }

    let email = user.email.clone();
    let token = encode_jwt(user.email)
        .map_err(|_| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Internal server error." }))
        ))?;

    info!("User signed in: {}", email);
    Ok(Json(json!({ "token": token })))
}
