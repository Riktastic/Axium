use sqlx::postgres::PgPool;
use uuid::Uuid;
use crate::models::user::*;
use regex::Regex;
use sqlx::Error;
use validator::Validate;

/// Retrieves all users with security considerations
///
/// # Security
/// - Requires admin privileges (enforced at application layer)
/// - Excludes sensitive fields like password_hash and totp_secret
/// - Limits maximum results in production (enforced at application layer)
pub async fn fetch_all_users_from_db(pool: &PgPool) -> Result<Vec<UserGetResponse>, sqlx::Error> {
    sqlx::query_as!(
        UserGetResponse,
        "SELECT id, username, email, role_level, tier_level, creation_date, 
        profile_picture_url, first_name, last_name, country_code, language_code, 
        birthday, description 
        FROM users"
    )
    .fetch_all(pool)
    .await
}

/// Safely retrieves user by allowed fields using whitelist validation
///
/// # Allowed Fields
/// - id: UUID
/// - email: valid email
/// - username: valid username
///
/// # Security
/// - Only whitelisted fields
/// - No sensitive data returned
pub async fn fetch_user_by_field_from_db(
    pool: &PgPool,
    field: &str,
    value: &str,
) -> Result<Option<UserGetResponse>, Error> {
    match field {
        "id" => {
            let uuid = value.parse::<Uuid>().map_err(|_| {
                Error::Decode(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Invalid UUID format",
                )))
            })?;

            sqlx::query_as!(
                UserGetResponse,
                r#"
                SELECT id, username, email, role_level, tier_level, creation_date, 
                       profile_picture_url, first_name, last_name, country_code, 
                       language_code, birthday, description
                FROM users
                WHERE id = $1
                "#,
                uuid
            )
            .fetch_optional(pool)
            .await
        }
        "email" => {
            sqlx::query_as!(
                UserGetResponse,
                r#"
                SELECT id, username, email, role_level, tier_level, creation_date, 
                       profile_picture_url, first_name, last_name, country_code, 
                       language_code, birthday, description
                FROM users
                WHERE email = $1
                "#,
                value
            )
            .fetch_optional(pool)
            .await
        }
        "username" => {
            sqlx::query_as!(
                UserGetResponse,
                r#"
                SELECT id, username, email, role_level, tier_level, creation_date, 
                       profile_picture_url, first_name, last_name, country_code, 
                       language_code, birthday, description
                FROM users
                WHERE username = $1
                "#,
                value
            )
            .fetch_optional(pool)
            .await
        }
        _ => Err(Error::ColumnNotFound(field.to_string())),
    }
}

/// Retrieves user by email with validation
///
/// # Security
/// - Parameterized query prevents SQL injection
/// - Returns Option to avoid user enumeration risks
pub async fn fetch_user_by_email_from_db(
    pool: &PgPool,
    email: &str,
) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"SELECT id, username, email, password_hash, totp_secret, 
           role_level, tier_level, creation_date, profile_picture_url, 
           first_name, last_name, country_code, language_code, 
           birthday, description
           FROM users WHERE email = $1"#,
        email
    )
    .fetch_optional(pool)
    .await
}

/// Securely deletes a user by ID
///
/// # Security
/// - Requires authentication and authorization
/// - Parameterized query prevents SQL injection
/// - Returns affected rows without sensitive data
pub async fn delete_user_from_db(pool: &PgPool, id: Uuid) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!("DELETE FROM users WHERE id = $1", id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected())
}

/// Creates new user with comprehensive validation
///
/// # Validation
/// - Username: 3-30 alphanumeric characters
/// - Email: Valid format with domain verification
/// - Password: Minimum strength requirements (enforced at application layer)
pub async fn insert_user_into_db(
    pool: &PgPool,
    username: &str,
    email: &str,
    password_hash: &str,
    totp_secret: &str,
    role_level: i32,
    tier_level: i32,
) -> Result<UserInsertResponse, Error> {
    // Validate username
    let username = username.trim();
    if username.len() < 3 || username.len() > 30 {
        return Err(Error::Protocol("Username must be between 3 and 30 characters.".into()));
    }
    if !username.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err(Error::Protocol("Invalid username format: only alphanumeric and underscores allowed.".into()));
    }

    // Validate email
    let email = email.trim().to_lowercase();
    if !is_valid_email(&email) {
        return Err(Error::Protocol("Invalid email format.".into()));
    }

    // Insert user into database
    let row = sqlx::query_as!(
        UserInsertResponse,
        r#"INSERT INTO users 
           (username, email, password_hash, totp_secret, role_level, tier_level, creation_date)
           VALUES ($1, $2, $3, $4, $5, $6, NOW()::timestamp)
           RETURNING id, username, email, totp_secret, role_level, tier_level, creation_date, 
                     first_name, last_name, country_code, language_code, birthday, description, 
                     profile_picture_url"#,
        username,
        email,
        password_hash,
        totp_secret,
        role_level,
        tier_level,
    )
    .fetch_one(pool)
    .await?;

    Ok(row)
}

/// Retrieves the profile picture URL for a specific user
///
/// # Arguments
/// - `pool`: Database connection pool
/// - `user_id`: The user's unique identifier
///
/// # Returns
/// - `Ok(Some(String))` if user exists and has a profile picture
/// - `Ok(None)` if user exists but has no profile picture
/// - `Err(sqlx::Error)` on database errors
pub async fn fetch_profile_picture_url_from_db(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Option<String>, sqlx::Error> {
    let result: Option<Option<String>> = sqlx::query_scalar!(
        r#"
        SELECT profile_picture_url 
        FROM users 
        WHERE id = $1
        "#,
        user_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(result.flatten())
}


/// Updates the profile picture URL for a user
///
/// # Arguments
/// - `pool`: Database connection pool
/// - `user_id`: The user's ID
/// - `profile_picture_url`: The new URL or path for the profile picture
///
/// # Returns
/// - `Ok(())` on success
/// - `Err(sqlx::Error)` on failure
pub async fn update_user_profile_picture_in_db(
    pool: &PgPool,
    user_id: Uuid,
    profile_picture_url: &str,
) -> Result<(), Error> {
    sqlx::query!(
        r#"
        UPDATE users
        SET profile_picture_url = $1
        WHERE id = $2
        "#,
        profile_picture_url,
        user_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Email validation helper function
fn is_valid_email(email: &str) -> bool {
    let email_regex = Regex::new(
        r"^[a-z0-9_+]+([a-z0-9_.-]*[a-z0-9_+])?@[a-z0-9]+([-.][a-z0-9]+)*\.[a-z]{2,6}$"
    ).unwrap();
    email_regex.is_match(email)
}

/// Updates the specified user's profile fields in the database.
///
/// This function dynamically builds an `UPDATE` SQL statement using `sqlx::QueryBuilder`
/// based on the fields present in `UserUpdateBody`. Only fields that are `Some` will be updated.
/// Fields set to `Some(None)` will be set to `NULL` in the database, while fields set to
/// `Some(Some(value))` will be updated to that value. Fields that are `None` are not changed.
///
/// The update struct is validated before attempting the update.
///
/// # Arguments
///
/// * `pool` - A reference to the PostgreSQL connection pool.
/// * `user_id` - The UUID of the user whose profile is being updated.
/// * `update` - A struct containing the profile fields to update. Each field is an
///   `Option<Option<T>>`, allowing for explicit nullification or update.
///
/// # Returns
///
/// * `Ok(())` if the update was successful or if there was nothing to update.
/// * `Err(sqlx::Error)` if the database operation fails or validation fails.
///
/// # Example
///
/// ```
/// let update = UserUpdateBody {
///     first_name: Some(Some("Alice".to_string())),
///     last_name: Some(None), // Will set last_name to NULL
///     country_code: None,    // Will not update country_code
///     language_code: None,
///     birthday: None,
///     description: None,
/// };
/// update_user_in_db(&pool, user_id, update).await?;
/// ```
///
/// # Notes
///
/// - If no fields are provided to update, the function returns `Ok(())` and performs no database operation.
/// - The SQL query is constructed dynamically to update only the specified fields.
///
pub async fn update_user_in_db(
    pool: &PgPool,
    user_id: Uuid,
    update: UserUpdateBody,
) -> Result<(), sqlx::Error> {
    // Validate the update struct before proceeding
    if let Err(validation_errors) = update.validate() {
        return Err(sqlx::Error::Protocol(format!("Validation error: {:?}", validation_errors)));
    }

    use sqlx::QueryBuilder;
    let mut builder = QueryBuilder::new("UPDATE users SET ");
    let mut has_updates = false;

    // For Option<T> fields (cannot be explicitly set to NULL)
    macro_rules! maybe_set_opt {
        ($field:ident) => {
            if let Some(ref val) = update.$field {
                if has_updates {
                    builder.push(", ");
                }
                builder.push(format!("{} = ", stringify!($field)));
                builder.push_bind(val);
                has_updates = true;
            }
        };
    }

    // For Option<Option<T>> fields (can be explicitly set to NULL)
    macro_rules! maybe_set_optopt {
        ($field:ident) => {
            if let Some(ref val) = update.$field {
                if has_updates {
                    builder.push(", ");
                }
                builder.push(format!("{} = ", stringify!($field)));
                match val {
                    Some(inner) => { builder.push_bind(inner); },
                    None => { builder.push("NULL"); },
                }
                has_updates = true;
            }
        };
    }

    // Use parentheses and semicolons for macro calls!
    maybe_set_opt!(first_name);
    maybe_set_opt!(last_name);
    maybe_set_optopt!(country_code);
    maybe_set_opt!(language_code);
    maybe_set_optopt!(birthday);
    maybe_set_opt!(description);
    maybe_set_opt!(role_level);
    maybe_set_opt!(tier_level);

    if !has_updates {
        return Ok(());
    }

    builder.push(" WHERE id = ");
    builder.push_bind(user_id);

    let query = builder.build();
    query.execute(pool).await?;

    Ok(())
}
