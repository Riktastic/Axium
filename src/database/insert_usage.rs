use sqlx::postgres::PgPool;
use uuid::Uuid;

pub async fn insert_usage(pool: &PgPool, user_id: Uuid, endpoint: String) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"INSERT INTO usage 
        (endpoint, user_id) 
        VALUES ($1, $2)"#,
        endpoint,
        user_id
    )
    .execute(pool)
    .await?;

    Ok(())
}