use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::NaiveDate;
use utoipa::ToSchema;
use validator::Validate;

use crate::utils::validate::validate_future_date;

/// Represents an API key in the system.
#[derive(Deserialize, Debug, Serialize, FromRow, Clone, ToSchema)]
#[sqlx(rename_all = "snake_case")]
pub struct ApiKey {
    /// The unique id of the API key.
    pub id: Uuid,
    /// The hashed value of the API key.
    pub key_hash: String,
    /// The id of the user who owns the API key.
    pub user_id: Uuid,
    /// The description/name of the API key.
    pub description: Option<String>,
    /// The expiration date of the API key.
    pub expiration_date: Option<NaiveDate>,
    /// The creation date of the API key (default is the current date).
    pub creation_date: NaiveDate,
    /// Whether the API key is disabled (default is false).
    pub disabled: bool,
    /// Whether the API key has read access (default is true).
    pub access_read: bool,
    /// Whether the API key has modify access (default is false).
    pub access_modify: bool,
}

/// Request body for creating a new API key.
#[derive(Deserialize, Validate, ToSchema)]
pub struct ApiKeyInsertBody {
    /// Optional description of the API key (max 50 characters).
    #[validate(length(min = 0, max = 50))]
    pub description: Option<String>,
    /// Optional expiration date of the API key (must be in the future).
    #[validate(custom(function = "validate_future_date"))]
    pub expiration_date: Option<String>,
}

/// Response body for creating a new API key.
#[derive(Serialize, ToSchema)]
pub struct ApiKeyInsertResponse {
    /// The unique id of the created API key.
    pub id: Uuid,
    /// The actual API key value.
    pub api_key: String,
    /// The description of the API key.
    pub description: String,
    /// The expiration date of the API key.
    pub expiration_date: String,
}

/// Response body for retrieving an API key.
#[derive(Serialize, ToSchema)]
pub struct ApiKeyResponse {
    /// The unique id of the API key.
    pub id: Uuid,
    /// The id of the user who owns the API key.
    pub user_id: Uuid,
    /// The description of the API key.
    pub description: Option<String>,
    /// The expiration date of the API key.
    pub expiration_date: Option<NaiveDate>,
    /// The creation date of the API key.
    pub creation_date: NaiveDate,
}

/// Response body for retrieving an API key by its ID.
#[derive(Serialize, ToSchema)]
pub struct ApiKeyByIDResponse {
    /// The unique id of the API key.
    pub id: Uuid,
    /// The description of the API key.
    pub description: Option<String>,
    /// The expiration date of the API key.
    pub expiration_date: Option<NaiveDate>,
    /// The creation date of the API key.
    pub creation_date: NaiveDate,
}

/// Response body for retrieving active API keys for a user.
#[derive(Serialize, ToSchema)]
pub struct ApiKeyGetActiveForUserResponse {
    /// The unique id of the API key.
    pub id: Uuid,
    /// The description of the API key.
    pub description: Option<String>,
}

/// Response body for retrieving API keys by user ID.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ApiKeyByUserIDResponse {
    /// The unique id of the API key.
    pub id: Uuid,
    /// The hashed value of the API key.
    pub key_hash: String,
    /// The expiration date of the API key.
    pub expiration_date: Option<NaiveDate>,
}

/// Request body for creating a new API key (deprecated).
#[derive(serde::Serialize, ToSchema)]
pub struct ApiKeyNewBody {
    /// The description of the API key.
    pub description: Option<String>,
    /// The expiration date of the API key.
    pub expiration_date: Option<NaiveDate>
}

#[derive(Serialize, ToSchema)]
pub struct ApiKeyRotateResponse {
    pub id: Uuid,
    pub api_key: String,
    pub description: String,
    pub expiration_date: NaiveDate,
    pub rotation_info: ApiKeyRotateResponseInfo,
}

#[derive(Serialize, ToSchema)]
pub struct ApiKeyRotateResponseInfo {
    pub original_key: Uuid,
    pub disabled_at: NaiveDate,
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct ApiKeyRotateBody {
    #[validate(length(min = 1, max = 255))]
    pub description: Option<String>,
    pub expiration_date: Option<String>,
}