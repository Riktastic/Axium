use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::NaiveDate;
use utoipa::ToSchema;

/// Represents an API key in the system.
#[derive(Deserialize, Debug, Serialize, FromRow, Clone, ToSchema)]
#[sqlx(rename_all = "snake_case")]  // Ensures that field names are mapped to snake_case in SQL
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


#[derive(serde::Serialize, ToSchema)]
pub struct ApiKeyNewBody {
    pub description: Option<String>,
    pub expiration_date: Option<NaiveDate>
}

#[derive(serde::Serialize, ToSchema)]
pub struct ApiKeyResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub description: Option<String>,
    pub expiration_date: Option<NaiveDate>,
    pub creation_date: NaiveDate,
}

#[derive(serde::Serialize, ToSchema)]
pub struct ApiKeyByIDResponse {
    pub id: Uuid,
    pub description: Option<String>,
    pub expiration_date: Option<NaiveDate>,
    pub creation_date: NaiveDate,
}

#[derive(sqlx::FromRow, ToSchema)]
pub struct ApiKeyByUserIDResponse {
    pub id: Uuid,
    pub key_hash: String,
    pub expiration_date: Option<NaiveDate>,
}