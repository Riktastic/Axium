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
use std::env; // For accessing environment variables
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString}, // For password hashing and verification
    Argon2,
};

use chrono::{Duration, Utc}; // For working with time (JWT expiration, etc.)
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation}; // For encoding and decoding JWT tokens
use serde::{Deserialize, Serialize}; // For serializing and deserializing JSON data
use serde_json::json; // For constructing JSON data
use sqlx::PgPool; // For interacting with PostgreSQL databases asynchronously

// Importing custom database query functions
use crate::database::get_users::get_user_by_email;

// Define the structure for JWT claims to be included in the token payload
#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,   // Expiration timestamp (in seconds)
    pub iat: usize,   // Issued-at timestamp (in seconds)
    pub email: String, // User's email
}

// Custom error type for handling authentication errors
pub struct AuthError {
    message: String,
    status_code: StatusCode, // HTTP status code to be returned with the error
}

// Function to verify a password against a stored hash using the Argon2 algorithm
pub fn verify_password(password: &str, hash: &str) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(hash)?; // Parse the hash
    // Verify the password using Argon2
    Ok(Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok())
}

// Function to hash a password using Argon2 and a salt retrieved from the environment variables
pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    // Get the salt from environment variables (must be set)
    let salt = env::var("AUTHENTICATION_ARGON2_SALT").expect("AUTHENTICATION_ARGON2_SALT must be set");
    let salt = SaltString::from_b64(&salt).unwrap(); // Convert base64 string to SaltString
    let argon2 = Argon2::default(); // Create an Argon2 instance
    // Hash the password with the salt
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?.to_string();
    Ok(password_hash)
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
pub fn encode_jwt(email: String) -> Result<String, StatusCode> {
    let jwt_token: String = "randomstring".to_string(); // Secret key for JWT (should be more secure in production)

    let now = Utc::now(); // Get current time
    let expire = Duration::hours(24); // Set token expiration to 24 hours
    let exp: usize = (now + expire).timestamp() as usize; // Expiration timestamp
    let iat: usize = now.timestamp() as usize; // Issued-at timestamp

    let claim = Claims { iat, exp, email }; // Create JWT claims with timestamps and user email
    let secret = jwt_token.clone(); // Secret key to sign the token

    // Encode the claims into a JWT token
    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR) // Return error if encoding fails
}

// Function to decode a JWT token and extract the claims
pub fn decode_jwt(jwt: String) -> Result<TokenData<Claims>, StatusCode> {
    let secret = "randomstring".to_string(); // Secret key to verify the JWT (should be more secure in production)

    // Decode the JWT token using the secret key and extract the claims
    decode(
        &jwt,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR) // Return error if decoding fails
}

// Middleware for role-based access control (RBAC)
// Ensures that only users with specific roles are authorized to access certain resources
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
    let current_user = match get_user_by_email(&pool, token_data.claims.email).await {
        Ok(user) => user,
        Err(_) => return Err(AuthError {
            message: "Unauthorized user.".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
        }),
    };

    // Check if the user's role is in the list of allowed roles
    if !allowed_roles.contains(&current_user.role_id) {
        return Err(AuthError {
            message: "Forbidden: insufficient role.".to_string(),
            status_code: StatusCode::FORBIDDEN,
        });
    }

    // Insert the current user into the request extensions for use in subsequent handlers
    req.extensions_mut().insert(current_user);

    // Proceed to the next middleware or handler
    Ok(next.run(req).await)
}

// Structure to hold the data from the sign-in request
#[derive(Deserialize)]
pub struct SignInData {
    pub email: String,
    pub password: String,
}

// Handler for user sign-in (authentication)
pub async fn sign_in(
    State(pool): State<PgPool>,  // Database connection pool injected as state
    Json(user_data): Json<SignInData>, // Deserialize the JSON body into SignInData
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    
    // 1. Retrieve user from the database using the provided email
    let user = match get_user_by_email(&pool, user_data.email).await {
        Ok(user) => user,
        Err(_) => return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "Incorrect credentials." }))
        )),
    };

    // 2. Verify the password using the stored hash
    if !verify_password(&user_data.password, &user.password_hash)
        .map_err(|_| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Internal server error." }))
        ))? 
    {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "Incorrect credentials." }))
        ));
    }

    // 3. Generate a JWT token for the authenticated user
    let token = encode_jwt(user.email)
        .map_err(|_| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Internal server error." }))
        ))?;

    // 4. Return the JWT token to the client
    Ok(Json(json!({ "token": token })))
}
