// Imports grouped by functionality
use std::env;
use axum::http::{StatusCode, Request};
use axum::body::Body;
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, Error},
    Argon2, Params, Version,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation, errors::ErrorKind};
use totp_rs::{Secret, TOTP};
use rand::{rngs::OsRng, Rng};
use tracing::{warn, error, debug, instrument};
use tokio::task;
use moka::future::Cache;
use lazy_static::lazy_static;

use crate::models::auth::{AuthError, Claims}; 
use crate::core::config::{get_env, get_env_with_default};

// Constants and lazy_static variables
lazy_static! {
    static ref PASSWORD_CACHE: Cache<String, bool> = Cache::builder()
        .time_to_live(std::time::Duration::from_secs(300))  // 5 minutes
        .build();

    static ref SECRET_KEY: String = get_env("JWT_SECRET_KEY");
}

// Password hashing and verification
#[instrument(skip(password, hash))]
pub async fn verify_hash(password: &str, hash: &str) -> Result<bool, Error> {
    // Check cache first
    if let Some(result) = PASSWORD_CACHE.get(password).await {
        return Ok(result);
    }

    let password_owned = password.to_string();
    let hash_owned = hash.to_string();
    let password_clone = password_owned.clone();

    let result = task::spawn_blocking(move || {
        let parsed_hash = PasswordHash::new(&hash_owned)?;
        
        Argon2::default()
            .verify_password(password_owned.as_bytes(), &parsed_hash)
            .map(|_| true)  // Remove the map_err conversion
    })
    .await
    .map_err(|_| argon2::Error::AlgorithmInvalid)??; // Keep double question mark

    if result {
        PASSWORD_CACHE.insert(password_clone, true).await;
    }

    Ok(result)
}

#[instrument(skip(password))]
pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    // Generate random salt
    let salt = SaltString::generate(&mut OsRng);
    
    // Configure Argon2id with recommended parameters
    let argon2 = Argon2::new(
        argon2::Algorithm::Argon2id,  // Explicitly use Argon2id variant
        Version::V0x13,       // Latest version
        Params::new(           // OWASP-recommended parameters
            15360,  // 15 MiB memory cost
            2,       // 2 iterations
            1,       // 1 parallelism
            None     // Default output length
        )?
    );

    // Hash password with configured parameters
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?.to_string();
    Ok(password_hash)
}

// JWT encoding and decoding
#[instrument(skip(email))]
pub fn encode_jwt(email: String) -> Result<String, StatusCode> {
    // Get the current time and expiration time
    let now = Utc::now();
    let expire = Duration::hours(24);
    let exp: usize = (now + expire).timestamp() as usize;
    let iat: usize = now.timestamp() as usize;

    // Get issuer and audience from environment variables
    let issuer = get_env("JWT_ISSUER"); // Fetching the issuer from environment variables
    let audience = get_env("JWT_AUDIENCE"); // Fetching the audience from environment variables

    // Create claims using the fetched issuer and audience
    let claim = Claims {
        sub: email,
        iat,
        exp,
        iss: issuer,   // Set the issuer from the environment
        aud: audience, // Set the audience from the environment
    };

    // Sign the token using the secret key and the default algorithm (HS256)
    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(SECRET_KEY.as_ref()),
    )
    .map_err(|e| {
        error!("Failed to encode JWT: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[instrument(skip(jwt))]
pub fn decode_jwt(jwt: String) -> Result<TokenData<Claims>, AuthError> {
    let secret_key = get_env("JWT_SECRET_KEY");

    // Get issuer and audience from environment variables
    let issuer = get_env("JWT_ISSUER"); // Fetching the issuer from environment variables
    let audience = get_env("JWT_AUDIENCE"); // Fetching the audience from environment variables

    // Configure validation
    let mut validation = Validation::default();

    // Optional: enforce audience and issuer using the environment variables
    validation.set_issuer(&[issuer]);
    validation.set_audience(&[audience]);

    // Add a small leeway to account for clock skew (e.g., 5 minutes)
    validation.leeway = 300;  // Allow up to 5 minutes of clock skew

    // Decode and validate token
    match decode::<Claims>(
        &jwt,
        &DecodingKey::from_secret(secret_key.as_ref()),
        &validation,
    ) {
        Ok(token_data) => Ok(token_data),
        Err(err) => {
            let message = match err.kind() {
                ErrorKind::InvalidToken => "Invalid token format.",
                ErrorKind::InvalidSignature => "Invalid token signature.",
                ErrorKind::ExpiredSignature => "Token has expired.",
                ErrorKind::InvalidIssuer => "Invalid token issuer.",
                ErrorKind::InvalidAudience => "Invalid token audience.",
                _ => "Failed to decode token.",
            };

            warn!("JWT decode error: {:?}", err);

            Err(AuthError {
                message: message.to_string(),
                status_code: StatusCode::UNAUTHORIZED,
            })
        }
    }
}

// Token extraction
pub fn extract_bearer_token(header: &str) -> Result<&str, AuthError> {
    let parts: Vec<&str> = header.splitn(2, ' ').collect();
    if parts.len() != 2 || parts[0] != "Bearer" {
        return Err(AuthError {
            message: "Authorization header must be in Bearer format.".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
        });
    }
    Ok(parts[1])
}

pub fn extract_token_from_header(req: &Request<Body>) -> Option<String> {
    let header = req.headers().get(axum::http::header::AUTHORIZATION);
    debug!("Authorization header: {:?}", header);

    let token = header
        .and_then(|header| header.to_str().ok())
        .and_then(|auth_header| {
            debug!("Authorization header (as str): {:?}", auth_header);
            extract_bearer_token(auth_header).ok()
        })
        .map(|t| t.to_string());

    debug!("Extracted token from header: {:?}", token);
    token
}

pub fn extract_token_from_cookie(req: &Request<Body>) -> Option<String> {
    let cookie_name = get_env_with_default("JWT_COOKIE_NAME", "auth_token");

    // Log the entire headers to see if the Cookie header is present
    debug!("All headers: {:?}", req.headers());

    // Get the cookie header
    let header = req.headers().get(axum::http::header::COOKIE);
    debug!("Cookie header: {:?}", header);

    // If there's no cookie header, return None
    if let Some(header_value) = header {
        if let Ok(cookie_str) = header_value.to_str() {
            debug!("Cookie header as str: {}", cookie_str);

            // Split cookies by ';' and trim them, then log and check each cookie
            let token = cookie_str.split(';')
                .map(|cookie| cookie.trim())
                .filter_map(|cookie| {
                    let (name, value) = cookie.split_once('=')?;
                    debug!("Found cookie: name='{}', value='{}'", name, value);
                    if name == cookie_name {
                        Some(value.to_string())
                    } else {
                        None
                    }
                })
                .next();  // Just get the first matching cookie, if any

            debug!("Extracted token from cookie: {:?}", token);
            return token;
        }
    }

    // If no token found, log it and return None
    debug!("No cookies found or token not present in cookies.");
    None
}


// TOTP and API key generation
#[instrument]
pub fn generate_totp_secret() -> String {    
    let totp = TOTP::new(
        totp_rs::Algorithm::SHA512,
        8,
        1,
        30,
        Secret::generate_secret().to_bytes().unwrap(),
    ).expect("Failed to create TOTP.");

    totp.generate_current().unwrap()
}

#[instrument]
pub fn generate_api_key() -> String {
    // Use OsRng for cryptographically secure random number generation
    let mut rng = OsRng;
    (0..5)
        .map(|_| {
            (0..8)
                .map(|_| format!("{:02x}", rng.gen::<u8>()))
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("-")
}

// Asynchronous password and API key verification
#[instrument(skip(password, hash))]
pub async fn verify_password(password: String, hash: String) -> Result<bool, Error> {
    verify_hash(&password, &hash).await
}

#[instrument(skip(password, hash))]
pub async fn verify_api_key(password: String, hash: String) -> Result<bool, Error> {
    verify_hash(&password, &hash).await
}