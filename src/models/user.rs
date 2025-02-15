use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{NaiveDate, NaiveDateTime};
use utoipa::ToSchema;
use validator::Validate;

use crate::utils::validate::{validate_password, validate_username};

/// Represents a user in the system.
#[derive(Deserialize, Debug, Serialize, FromRow, Clone, ToSchema)]
#[sqlx(rename_all = "snake_case")]
pub struct User {
    /// The unique identifier for the user.
    pub id: Uuid,
    /// The username of the user.
    pub username: String,
    /// The email of the user.
    pub email: String,
    /// The hashed password for the user.
    pub password_hash: String,
    /// The TOTP secret for the user.
    pub totp_secret: Option<String>,
    /// Current role of the user.
    pub role_level: i32,
    /// Current tier level of the user.
    pub tier_level: i32,
    /// Date when the user was created.
    pub creation_date: Option<NaiveDate>,
}

/// Represents a user response for GET requests.
#[derive(Deserialize, Debug, Serialize, FromRow, Clone, ToSchema)]
#[sqlx(rename_all = "snake_case")]
pub struct UserGetResponse {
    /// The unique identifier for the user.
    pub id: Uuid,
    /// The username of the user.
    pub username: String,
    /// The email of the user.
    pub email: String,
    /// Current role of the user.
    pub role_level: i32,
    /// Current tier level of the user.
    pub tier_level: i32,
    /// Date when the user was created.
    pub creation_date: Option<NaiveDate>,
}

/// Request body for inserting a new user.
#[derive(Deserialize, Validate, ToSchema)]
pub struct UserInsertBody {
    /// The username of the new user.
    #[validate(length(min = 3, max = 50), custom(function = "validate_username"))]
    pub username: String,
    /// The email of the new user.
    #[validate(email)]
    pub email: String,
    /// The password for the new user.
    #[validate(custom(function = "validate_password"))]
    pub password: String,
    /// Optional TOTP secret for the new user.
    pub totp: Option<String>,
}

/// Response body for a successful user insertion.
#[derive(Serialize, ToSchema)]
pub struct UserInsertResponse {
    /// The unique identifier for the newly created user.
    pub id: Uuid,
    /// The username of the newly created user.
    pub username: String,
    /// The email of the newly created user.
    pub email: String,
    /// The TOTP secret for the newly created user, if provided.
    pub totp_secret: Option<String>,
    /// The role level assigned to the newly created user.
    pub role_level: i32,
    /// The tier level assigned to the newly created user.
    pub tier_level: i32,
    /// The creation date and time of the newly created user.
    pub creation_date: NaiveDateTime,
}
