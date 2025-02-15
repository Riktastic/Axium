use std::{collections::HashSet, env};
use axum::http::StatusCode;
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, Error},
    Argon2, Params, Version,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use totp_rs::{Secret, TOTP};
use rand::{rngs::OsRng, Rng};
use tracing::{warn, error, instrument};
use tokio::task;
use moka::future::Cache;
use lazy_static::lazy_static;

use crate::models::auth::Claims;

// Standard library imports for working with HTTP, environment variables, and other necessary utilities

// Importing necessary libraries for password hashing, JWT handling, and date/time management

// Cache for storing successful password verifications
lazy_static! {
    static ref PASSWORD_CACHE: Cache<String, bool> = Cache::builder()
        .time_to_live(std::time::Duration::from_secs(300))  // 5 minutes
        .build();
}

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

// Function to hash a password using Argon2 and a salt retrieved from the environment variables
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
    let mut rng = rand::thread_rng();
    (0..5)
        .map(|_| {
            (0..8)
                .map(|_| format!("{:02x}", rng.gen::<u8>()))
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("-")
}

// Function to encode a JWT token for the given email address
#[instrument(skip(email))]
pub fn encode_jwt(email: String) -> Result<String, StatusCode> {
    // Load secret key from environment variable for better security
    let secret_key = env::var("JWT_SECRET_KEY")
        .map_err(|_| {
            error!("JWT_SECRET_KEY not set in environment variables");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let now = Utc::now();
    let expire = Duration::hours(24);
    let exp: usize = (now + expire).timestamp() as usize;
    let iat: usize = now.timestamp() as usize;

    let claim = Claims {
        sub: email.clone(),
        iat,
        exp,
        iss: "your_issuer".to_string(),   // Add issuer if needed
        aud: "your_audience".to_string(), // Add audience if needed
    };

    // Use a secure HMAC algorithm (e.g., HS256) for signing the token
    encode(
        &Header::new(jsonwebtoken::Algorithm::HS256),
        &claim,
        &EncodingKey::from_secret(secret_key.as_ref()),
    )
    .map_err(|e| {
        error!("Failed to encode JWT: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

// Function to decode a JWT token and extract the claims
#[instrument(skip(jwt))]
pub fn decode_jwt(jwt: String) -> Result<TokenData<Claims>, StatusCode> {
    // Load secret key from environment variable for better security
    let secret_key = env::var("JWT_SECRET_KEY")
        .map_err(|_| {
            error!("JWT_SECRET_KEY not set in environment variables");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Set up validation rules (e.g., check if token has expired, is from a valid issuer, etc.)
    let mut validation = Validation::default();
    
    // Use a HashSet for the audience and issuer validation
    let mut audience_set = HashSet::new();
    audience_set.insert("your_audience".to_string());

    let mut issuer_set = HashSet::new();
    issuer_set.insert("your_issuer".to_string());

    // Set up the validation with the HashSet for audience and issuer
    validation.aud = Some(audience_set);
    validation.iss = Some(issuer_set);

    // Decode the JWT and extract the claims
    decode::<Claims>(
        &jwt,
        &DecodingKey::from_secret(secret_key.as_ref()),
        &validation,
    )
    .map_err(|e| {
        warn!("Failed to decode JWT: {:?}", e);
        StatusCode::UNAUTHORIZED
    })
}

// Function to verify password asynchronously
#[instrument(skip(password, hash))]
pub async fn verify_password(password: String, hash: String) -> Result<bool, Error> {
    verify_hash(&password, &hash).await
}

#[instrument(skip(password, hash))]
pub async fn verify_api_key(password: String, hash: String) -> Result<bool, Error> {
    verify_hash(&password, &hash).await
}