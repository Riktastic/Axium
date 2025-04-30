use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{NaiveDate, NaiveDateTime};
use utoipa::ToSchema;
use validator::Validate;

use crate::utils::validate::{validate_password, validate_username, validate_birthday, validate_country_code, validate_language_code};

/// Database model (SQLx compatible)
#[derive(Debug, FromRow)]
pub struct UserRow {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub role_level: i32,
    pub tier_level: i32,
    pub creation_date: Option<NaiveDate>,
    pub profile_picture_url: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub country_code: Option<String>,
    pub language_code: Option<String>,
    pub birthday: Option<NaiveDate>,
    pub description: Option<String>,
    pub password_hash: String,
    pub totp_secret: Option<String>,
}

/// Internal domain model (non-SQLx)
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub role_level: i32,
    pub tier_level: i32,
    pub creation_date: Option<NaiveDate>,
    pub profile_picture_url: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub country_code: Option<String>,
    pub language_code: Option<String>,
    pub birthday: Option<NaiveDate>,
    pub description: Option<String>,
    #[serde(skip)]
    pub password_hash: String,
    #[serde(skip)]
    pub totp_secret: Option<String>,
}

/// Public user response
#[derive(Debug, Serialize, Clone, ToSchema)]
pub struct UserGetResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub role_level: i32,
    pub tier_level: i32,
    pub creation_date: Option<NaiveDate>,
    pub profile_picture_url: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub country_code: Option<String>,
    pub language_code: Option<String>,
    pub birthday: Option<NaiveDate>,
    pub description: Option<String>,
}

/// Request body for user creation
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UserInsertBody {
    #[validate(length(min = 3, max = 50), custom(function = "validate_username"))]
    pub username: String,
    
    #[validate(email)]
    pub email: String,
    
    #[validate(custom(function = "validate_password"))]
    pub password: String,
    
    pub totp: Option<String>,
    
    #[validate(length(min = 1, max = 50))]
    pub first_name: Option<String>,
    
    #[validate(length(min = 1, max = 50))]
    pub last_name: Option<String>,
    
    #[validate(length(equal = 2), custom(function = "validate_country_code"))]
    pub country_code: Option<String>,
    
    #[validate(length(min = 2, max = 5), custom(function = "validate_language_code"))]
    pub language_code: Option<String>,
    
    #[validate(custom(function = "validate_birthday"))]
    pub birthday: Option<NaiveDate>,
    
    #[validate(length(max = 1000))]
    pub description: Option<String>,
    
    #[validate(url)]
    pub profile_picture_url: Option<String>,
}

/// Response for user creation
#[derive(Debug, Serialize, ToSchema)]
pub struct UserInsertResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub role_level: i32,
    pub tier_level: i32,
    pub creation_date: NaiveDateTime,
    pub profile_picture_url: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub country_code: Option<String>,
    pub language_code: Option<String>,
    pub birthday: Option<NaiveDate>,
    pub description: Option<String>,
    pub totp_secret: Option<String>,
}

/// Request body for user updates
#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")] 
pub struct UserUpdateBody {
    #[validate(length(min = 1, max = 50))]
    pub first_name: Option<String>,
    
    #[validate(length(min = 1, max = 50))]
    pub last_name: Option<String>,
    
    #[validate(
        custom(function = "validate_country_code"),
        length(equal = 2)
    )]
    pub country_code: Option<Option<String>>,
    
    #[validate(
        length(min = 2, max = 5),
        custom(function = "validate_language_code")
    )]
    pub language_code: Option<String>,
    
    #[validate(custom(function = "validate_birthday"))]
    pub birthday: Option<Option<NaiveDate>>,
    
    #[validate(length(max = 1000))]
    pub description: Option<String>,

    pub role_level: Option<i32>,  // Added the role_level field to the update body

    pub tier_level: Option<i32>,  // Added the role_level field to the update body
}

/// Response for user updates
#[derive(Debug, Serialize, ToSchema)]
pub struct UserUpdateResponse {
    pub success: bool,
}

/// Profile picture upload handling
#[allow(dead_code)]
#[derive(Debug, Deserialize, ToSchema)]
pub struct UserProfilePictureUploadBody {
    #[serde(rename = "profile_picture")]
    pub profile_picture: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ProfilePictureUploadResponse {
    pub url: String,
}

// Conversion implementations
impl From<UserRow> for User {
    fn from(row: UserRow) -> Self {
        Self {
            id: row.id,
            username: row.username,
            email: row.email,
            role_level: row.role_level,
            tier_level: row.tier_level,
            creation_date: row.creation_date,
            profile_picture_url: row.profile_picture_url,
            first_name: row.first_name,
            last_name: row.last_name,
            country_code: row.country_code,
            language_code: row.language_code,
            birthday: row.birthday,
            description: row.description,
            password_hash: row.password_hash,
            totp_secret: row.totp_secret,
        }
    }
}

impl From<User> for UserGetResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            role_level: user.role_level,
            tier_level: user.tier_level,
            creation_date: user.creation_date,
            profile_picture_url: user.profile_picture_url,
            first_name: user.first_name,
            last_name: user.last_name,
            country_code: user.country_code,
            language_code: user.language_code,
            birthday: user.birthday,
            description: user.description,
        }
    }
}

impl From<User> for UserInsertResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            role_level: user.role_level,
            tier_level: user.tier_level,
            creation_date: user.creation_date
                .and_then(|d| NaiveDateTime::from_timestamp_opt(d.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp(), 0))
                .expect("Invalid creation date"),
            profile_picture_url: user.profile_picture_url,
            first_name: user.first_name,
            last_name: user.last_name,
            country_code: user.country_code,
            language_code: user.language_code,
            birthday: user.birthday,
            description: user.description,
            totp_secret: user.totp_secret,
        }
    }
}

// Additional conversions for handler convenience
impl From<UserRow> for UserGetResponse {
    fn from(row: UserRow) -> Self {
        UserGetResponse::from(User::from(row))
    }
}

impl From<UserRow> for UserInsertResponse {
    fn from(row: UserRow) -> Self {
        UserInsertResponse::from(User::from(row))
    }
}
