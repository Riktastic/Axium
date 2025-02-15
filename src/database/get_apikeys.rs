use sqlx::postgres::PgPool;
use uuid::Uuid;
use crate::models::apikey::*;

pub async fn get_active_apikeys_by_user_id(pool: &PgPool, user_id: Uuid) -> Result<Vec<ApiKeyByUserIDResponse>, sqlx::Error> {
    sqlx::query_as!(ApiKeyByUserIDResponse,
        r#"
        SELECT id, key_hash, expiration_date::DATE 
        FROM apikeys
        WHERE user_id = $1 AND (expiration_date IS NULL OR expiration_date > NOW()::DATE)
        "#,
        user_id
    )
    .fetch_all(pool)
    .await
}