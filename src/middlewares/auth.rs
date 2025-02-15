use std::{collections::HashSet, env};

// Standard library imports for working with HTTP, environment variables, and other necessary utilities
use axum::{
    body::Body,
    response::IntoResponse,
    extract::{Request, Json},   // Extractor for request and JSON body
    http::{self, Response, StatusCode}, // HTTP response and status codes
    middleware::Next,          // For adding middleware layers to the request handling pipeline
};

// Importing `State` for sharing application state (such as a database connection) across request handlers
use axum::extract::State;

// Importing necessary libraries for password hashing, JWT handling, and date/time management
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString}, // For password hashing and verification
    Argon2,
};

use chrono::{Duration, Utc}; // For working with time (JWT expiration, etc.)
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation}; // For encoding and decoding JWT tokens
use serde::Deserialize; // For serializing and deserializing JSON data
use serde_json::json; // For constructing JSON data
use sqlx::PgPool; // For interacting with PostgreSQL databases asynchronously
use totp_rs::{Algorithm, Secret, TOTP}; // For generating TOTP secrets and tokens
use rand::rngs::OsRng; // For generating random numbers
use uuid::Uuid; // For working with UUIDs
use rand::Rng;
use tracing::{info, warn, error, instrument}; // For logging

use utoipa::ToSchema; // Import ToSchema for OpenAPI documentation

// Importing custom database query functions
use crate::database::{get_users::get_user_by_email, get_apikeys::get_active_apikeys_by_user_id, insert_usage::insert_usage};

// Define the structure for JWT claims to be included in the token payload
#[derive(serde::Serialize, serde::Deserialize)]
struct Claims {
    sub: String,      // Subject (e.g., user ID or email)
    iat: usize,       // Issued At (timestamp)
    exp: usize,       // Expiration (timestamp)
    iss: String,      // Issuer (optional)
    aud: String,      // Audience (optional)
}

// Custom error type for handling authentication errors
pub struct AuthError {
    message: String,
    status_code: StatusCode, // HTTP status code to be returned with the error
}

// Function to verify a password against a stored hash using the Argon2 algorithm
#[instrument]
pub fn verify_hash(password: &str, hash: &str) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(hash)?; // Parse the hash
    // Verify the password using Argon2
    Ok(Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok())
}

// Function to hash a password using Argon2 and a salt retrieved from the environment variables
#[instrument]
pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    // Get the salt from environment variables (must be set)
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default(); // Create an Argon2 instance
    // Hash the password with the salt
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?.to_string();
    Ok(password_hash)
}

#[instrument]
pub fn generate_totp_secret() -> String {    
    let totp = TOTP::new(
        Algorithm::SHA512,
        8,
        1,
        30,
        Secret::generate_secret().to_bytes().unwrap(),
    ).expect("Failed to create TOTP.");

    let token = totp.generate_current().unwrap();

    token
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

// Implement the IntoResponse trait for AuthError to allow it to be returned as a response from the handler
impl IntoResponse for AuthError {
    fn into_response(self) -> Response<Body> {
        let body = Json(json!( { "error": self.message } )); // Create a JSON response body with the error message

        // Return a response with the appropriate status code and error message
        (self.status_code, body).into_response()
    }
}

// Function to encode a JWT token for the given email address
#[instrument]
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
#[instrument]
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

// Middleware for role-based access control (RBAC)
// Ensures that only users with specific roles are authorized to access certain resources
#[instrument(skip(req, next))]
pub async fn authorize(
    mut req: Request<Body>,
    next: Next,
    allowed_roles: Vec<i32>, // Accept a vector of allowed roles
) -> Result<Response<Body>, AuthError> {
    // Retrieve the database pool from request extensions (shared application state)
    let pool = req.extensions().get::<PgPool>().expect("Database pool not found in request extensions");

    // Retrieve the Authorization header from the request
    let auth_header = req.headers().get(http::header::AUTHORIZATION);

    // Ensure the header exists and is correctly formatted
    let auth_header = match auth_header {
        Some(header) => header.to_str().map_err(|_| AuthError {
            message: "Invalid header format".to_string(),
            status_code: StatusCode::FORBIDDEN,
        })?,
        None => return Err(AuthError {
            message: "Authorization header missing.".to_string(),
            status_code: StatusCode::FORBIDDEN,
        }),
    };

    // Extract the token from the Authorization header (Bearer token format)
    let mut header = auth_header.split_whitespace();
    let (_, token) = (header.next(), header.next());

    // Decode the JWT token
    let token_data = match decode_jwt(token.unwrap().to_string()) {
        Ok(data) => data,
        Err(_) => return Err(AuthError {
            message: "Unable to decode token.".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
        }),
    };

    // Fetch the user from the database using the email from the decoded token
    let current_user = match get_user_by_email(&pool, token_data.claims.sub).await {
        Ok(user) => user,
        Err(_) => return Err(AuthError {
            message: "Unauthorized user.".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
        }),
    };

    // Check if the user's role is in the list of allowed roles
    if !allowed_roles.contains(&current_user.role_level) {
        return Err(AuthError {
            message: "Forbidden: insufficient role.".to_string(),
            status_code: StatusCode::FORBIDDEN,
        });
    }

    // Check rate limit.
    check_rate_limit(&pool, current_user.id, current_user.tier_level).await?;

    // Insert the usage record into the database
    insert_usage(&pool, current_user.id, req.uri().path().to_string()).await
        .map_err(|_| AuthError {
            message: "Failed to insert usage record.".to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    // Insert the current user into the request extensions for use in subsequent handlers
    req.extensions_mut().insert(current_user);


    // Proceed to the next middleware or handler
    Ok(next.run(req).await)
}

// Handler for user sign-in (authentication)
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
    path = "/sign_in",
    request_body = SignInData,
    responses(
        (status = 200, description = "Successful sign-in", body = serde_json::Value),
        (status = 400, description = "Bad request", body = serde_json::Value),
        (status = 401, description = "Unauthorized", body = serde_json::Value),
        (status = 500, description = "Internal server error", body = serde_json::Value)
    )
)]
#[instrument(skip(pool, user_data))]
pub async fn sign_in(
    State(pool): State<PgPool>,
    Json(user_data): Json<SignInData>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let user = match get_user_by_email(&pool, user_data.email).await {
        Ok(user) => user,
        Err(_) => return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "Incorrect credentials." }))
        )),
    };

    let api_key_hashes = match get_active_apikeys_by_user_id(&pool, user.id).await {
        Ok(hashes) => hashes,
        Err(_) => return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Internal server error." }))
        )),
    };

    // Check API key first, then password
    let credentials_valid = api_key_hashes.iter().any(|api_key| {
        verify_hash(&user_data.password, &api_key.key_hash).unwrap_or(false)
    }) || verify_hash(&user_data.password, &user.password_hash)
        .map_err(|_| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Internal server error." }))
        ))?;

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

#[instrument(skip(pool))]
async fn check_rate_limit(pool: &PgPool, user_id: Uuid, tier_level: i32) -> Result<(), AuthError> {   
    // Get the user's tier requests_per_day
    let tier_limit = sqlx::query!(
        "SELECT requests_per_day FROM tiers WHERE level = $1",
        tier_level
    )
    .fetch_one(pool)
    .await
    .map_err(|_| AuthError {
        message: "Failed to fetch tier information".to_string(),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
    })?
    .requests_per_day;

    // Count user's requests for today
    let request_count = sqlx::query!(
        "SELECT COUNT(*) as count FROM usage WHERE user_id = $1 AND creation_date > NOW() - INTERVAL '24 hours'",
        user_id
    )
    .fetch_one(pool)
    .await
    .map_err(|_| AuthError {
        message: "Failed to count user requests".to_string(),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
    })?
    .count
    .unwrap_or(0); // Use 0 if count is NULL

    if request_count >= tier_limit as i64 {
        return Err(AuthError {
            message: "Rate limit exceeded".to_string(),
            status_code: StatusCode::TOO_MANY_REQUESTS,
        });
    }

    Ok(())
}