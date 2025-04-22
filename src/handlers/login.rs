use axum::{
    extract::State,
    http::{StatusCode, HeaderMap, HeaderValue},
    Json,
    response::IntoResponse,
};
use serde_json::json;
use sqlx::PgPool;
use totp_rs::{Algorithm, TOTP};
use tracing::{error, warn, debug, instrument};

use crate::utils::auth::{encode_jwt, verify_hash};
use crate::database::{apikeys::fetch_active_apikeys_by_user_id_from_db, users::fetch_user_by_email_from_db};
use crate::models::auth::LoginData;
use crate::core::config::{get_env_bool, get_env_with_default, get_env_u64};

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
    path = "/login",
    tag = "auth",
    request_body = LoginData,
    responses(
        (status = 200, description = "Successful sign-in", body = serde_json::Value),
        (status = 400, description = "Bad request", body = serde_json::Value),
        (status = 401, description = "Unauthorized", body = serde_json::Value),
        (status = 500, description = "Internal server error", body = serde_json::Value)
    )
)]
#[instrument(skip(pool, user_data))]
pub async fn login(
    State(pool): State<PgPool>,
    Json(user_data): Json<LoginData>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Fetch the user from the database based on their email.
    let user = match fetch_user_by_email_from_db(&pool, &user_data.email).await {
        Ok(Some(user)) => user,
        Ok(None) | Err(_) => {
            // Log the error for failed login attempt
            error!("Failed to find user with email: {}", user_data.email);
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
            // Log the error fetching API keys
            error!("Error fetching API keys for user: {}", user.id);
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
    let password_valid = match verify_hash(&user_data.password, &user.password_hash).await {
        Ok(valid) => valid,
        Err(_) => {
            // Log the error and return unauthorized response if password verification fails
            error!("Password verification failed for email: {}", user_data.email);
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": "Incorrect credentials." }))
            ));
        }
    };

    // Determine if the credentials are valid based on API keys or password.
    let credentials_valid = any_api_key_valid || password_valid;

    if !credentials_valid {
        // Log invalid credentials attempt
        error!("Invalid credentials for user: {}", user_data.email);
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
                    error!("Error creating TOTP instance for user: {}", user.id);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({ "error": "Internal server error." }))
                    )
                })?;

                // Check if the provided TOTP code is valid.
                if !totp.check_current(&totp_code).unwrap_or(false) {
                    error!("Invalid 2FA code for user: {}", user.id);
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
            error!("Error generating JWT for user: {}", user.id);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Internal server error." }))
            )
        })?;

    // Log the successful sign-in.
    debug!("User signed in: {}", email);

    // Prepare response headers
    let mut headers = HeaderMap::new();

    // Prevent caching of the response
    headers.insert(
        "Cache-Control",
        HeaderValue::from_static("no-store"),
    );

    let allow_cookie_auth = get_env_bool("JWT_ALLOW_COOKIE_AUTH", false);
    let force_cookie_auth = get_env_bool("JWT_FORCE_COOKIE_AUTH", false);
    let cookie_max_age = get_env_u64("JWT_COOKIE_MAX_AGE", 604800); // default: 7 days
    let use_https = get_env_bool("SERVER_HTTPS_ENABLED", false);
    let cookie_name = get_env_with_default("JWT_COOKIE_NAME", "auth_token");
    let samesite_value = get_env_with_default("JWT_COOKIE_SAMESITE", "Lax");
    let (samesite_flag, secure_flag) = match samesite_value.to_lowercase().as_str() {
        "none" if use_https => ("SameSite=None;", "Secure;"),  // Enforce HTTPS requirement
        "none" => {
            warn!("SameSite=None requires HTTPS. Falling back to Lax.");
            ("SameSite=Lax;", "")
        },
        "lax" => ("SameSite=Lax;", ""),
        "strict" => ("SameSite=Strict;", ""),
        _ => {
            warn!(
                "Invalid SameSite value '{}'. Allowed: None/Lax/Strict. Using Lax.",
                samesite_value
            );
            ("SameSite=Lax;", "")
        }
    };
    
    let cookie = format!(
        "{name}={value}; HttpOnly; Path=/; Max-Age={cookie_max_age}; {secure_flag}{samesite_flag}",
        name = cookie_name,
        value = token,
        secure_flag = secure_flag,
        samesite_flag = samesite_flag,
        cookie_max_age = cookie_max_age
    );
    
    if force_cookie_auth {
        headers.insert(
            axum::http::header::SET_COOKIE,
            HeaderValue::from_str(&cookie).unwrap(),
        );
        debug!("Setting cookie: {}", cookie);
        return Ok((StatusCode::OK, headers, Json(json!({ "success": true }))));
    }
    
    if allow_cookie_auth {
        headers.insert(
            axum::http::header::SET_COOKIE,
            HeaderValue::from_str(&cookie).unwrap(),
        );
        debug!("Setting cookie: {}", cookie);
    }
    
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
    );
    
    Ok((StatusCode::OK, headers, Json(json!({
        "access_token": token,
        "token_type": "Bearer"
    }))))
} 