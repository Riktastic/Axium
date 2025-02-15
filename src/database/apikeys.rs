use chrono::NaiveDate;
use sqlx::postgres::PgPool;
use uuid::Uuid;
use crate::models::apikey::{ApiKeyResponse, ApiKeyByIDResponse, ApiKeyByUserIDResponse, ApiKeyInsertResponse, ApiKeyGetActiveForUserResponse};

// ---------------------------
// Key Creation Functions
// ---------------------------

/// Inserts a new API key into the database for the specified user.
/// 
/// # Parameters
/// - `pool`: PostgreSQL connection pool
/// - `key_hash`: SHA-256 hash of the generated API key
/// - `description`: Human-readable key description
/// - `expiration_date`: Optional key expiration date
/// - `user_id`: Owner's user ID
/// 
/// # Returns
/// `ApiKeyInsertResponse` with metadata (actual key not stored in DB)
/// 
/// # Security
/// - Uses parameterized queries to prevent SQL injection
/// - Caller must validate inputs before invocation
pub async fn insert_api_key_into_db(
    pool: &PgPool,
    key_hash: String,
    description: String,
    expiration_date: NaiveDate,
    user_id: Uuid,
) -> Result<ApiKeyInsertResponse, sqlx::Error> {
    let row = sqlx::query!(
        r#"
        INSERT INTO apikeys (key_hash, description, expiration_date, user_id) 
        VALUES ($1, $2, $3, $4)
        RETURNING id, description, expiration_date
        "#,
        key_hash,
        description,
        expiration_date,
        user_id
    )
    .fetch_one(pool)
    .await?;

    Ok(ApiKeyInsertResponse {
        id: row.id,
        api_key: "".to_string(), // Placeholder for post-processing
        description: row.description.unwrap_or_default(),
        expiration_date: row.expiration_date
            .map(|d| d.to_string())
            .unwrap_or_else(|| "Never".to_string()),
    })
}

// ---------------------------
// Key Retrieval Functions
// ---------------------------

/// Retrieves all API keys (including revoked/expired) for a user
/// 
/// # Security
/// - Always filters by user_id to prevent cross-user access
pub async fn fetch_all_apikeys_from_db(
    pool: &PgPool, 
    user_id: Uuid
) -> Result<Vec<ApiKeyResponse>, sqlx::Error> {
    sqlx::query_as!(
        ApiKeyResponse,
        r#"
        SELECT id, user_id, description, expiration_date, creation_date 
        FROM apikeys 
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_all(pool)
    .await
}

/// Gets detailed metadata for a specific API key
/// 
/// # Security
/// - Verifies both key ID and user_id ownership
pub async fn fetch_apikey_by_id_from_db(
    pool: &PgPool, 
    id: Uuid, 
    user_id: Uuid
) -> Result<Option<ApiKeyByIDResponse>, sqlx::Error> {
    sqlx::query_as!(
        ApiKeyByIDResponse,
        r#"
        SELECT id, description, expiration_date, creation_date 
        FROM apikeys 
        WHERE id = $1 AND user_id = $2
        "#,
        id,
        user_id
    )
    .fetch_optional(pool)
    .await
}

/// Retrieves active keys for user with security checks
/// 
/// # Security
/// - Excludes disabled keys and expired keys
pub async fn fetch_active_apikeys_by_user_id_from_db(
    pool: &PgPool, 
    user_id: Uuid
) -> Result<Vec<ApiKeyByUserIDResponse>, sqlx::Error> {
    sqlx::query_as!(
        ApiKeyByUserIDResponse,
        r#"
        SELECT id, key_hash, expiration_date
        FROM apikeys
        WHERE 
            user_id = $1 
            AND disabled = FALSE 
            AND (expiration_date IS NULL OR expiration_date > CURRENT_DATE)
        "#,
        user_id
    )
    .fetch_all(pool)
    .await
}

// ---------------------------
// Key Modification Functions
// ---------------------------

/// Disables an API key and sets short expiration grace period
/// 
/// # Security
/// - Requires matching user_id to prevent unauthorized revocation
pub async fn disable_apikey_in_db(
    pool: &PgPool, 
    apikey_id: Uuid, 
    user_id: Uuid
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        UPDATE apikeys 
        SET 
            disabled = TRUE,
            expiration_date = CURRENT_DATE + INTERVAL '1 day'
        WHERE id = $1 AND user_id = $2
        "#,
        apikey_id,
        user_id
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}

// ---------------------------
// Key Deletion Functions
// ---------------------------

/// Permanently removes an API key from the system
/// 
/// # Security
/// - Requires matching user_id to prevent unauthorized deletion
pub async fn delete_apikey_from_db(
    pool: &PgPool, 
    id: Uuid, 
    user_id: Uuid
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        DELETE FROM apikeys 
        WHERE id = $1 AND user_id = $2
        "#,
        id,
        user_id
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}

// ---------------------------
// Validation Functions
// ---------------------------

/// Checks active key count against rate limits
/// 
/// # Security
/// - Used to enforce business logic limits
pub async fn check_existing_api_key_count(
    pool: &PgPool, 
    user_id: Uuid
) -> Result<i64, sqlx::Error> {
    let row = sqlx::query!(
        r#"
        SELECT COUNT(*) as count
        FROM apikeys
        WHERE 
            user_id = $1 
            AND disabled = FALSE 
            AND (expiration_date IS NULL OR expiration_date >= CURRENT_DATE)
        "#,
        user_id
    )
    .fetch_one(pool)
    .await?;

    Ok(row.count.unwrap_or(0))
}

/// Validates key existence and ownership before operations
pub async fn fetch_existing_apikey(
    pool: &PgPool, 
    user_id: Uuid, 
    apikey_id: Uuid
) -> Result<Option<ApiKeyGetActiveForUserResponse>, sqlx::Error> {
    sqlx::query_as!(
        ApiKeyGetActiveForUserResponse,
        r#"
        SELECT id, description 
        FROM apikeys 
        WHERE user_id = $1 AND id = $2 AND disabled = FALSE
        "#,
        user_id,
        apikey_id
    )
    .fetch_optional(pool)
    .await
}