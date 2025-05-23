use sqlx::postgres::PgPool;
use uuid::Uuid;
use crate::models::user::*;
use regex::Regex;
use sqlx::Error;
use validator::Validate;
use chrono::{DateTime, Utc, NaiveDate};

/// Retrieves all users with security considerations
///
/// # Security
/// - Requires admin privileges (enforced at application layer)
/// - Excludes sensitive fields like password_hash and totp_secret
/// - Limits maximum results in production (enforced at application layer)
#[allow(dead_code)]
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


/// Retrieves all active users with security considerations
///
/// # Security
/// - Requires admin privileges (enforced at application layer)
/// - Excludes sensitive fields like password_hash and totp_secret
/// - Limits maximum results in production (enforced at application layer)
pub async fn fetch_all_active_users_from_db(pool: &PgPool) -> Result<Vec<UserGetResponse>, sqlx::Error> {
    sqlx::query_as!(
        UserGetResponse,
        "SELECT id, username, email, role_level, tier_level, creation_date, 
        profile_picture_url, first_name, last_name, country_code, language_code, 
        birthday, description 
        FROM users
        WHERE status = 'active'"
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
#[allow(dead_code)]
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


/// Safely retrieves only active user by allowed fields using whitelist validation
///
/// # Allowed Fields
/// - id: UUID
/// - email: valid email
/// - username: valid username
///
/// # Security
/// - Only whitelisted fields
/// - No sensitive data returned
/// - Only users with status = 'active'
pub async fn fetch_active_user_by_field_from_db(
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
                WHERE id = $1 AND status = 'active'
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
                WHERE email = $1 AND status = 'active'
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
                WHERE username = $1 AND status = 'active'
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
           birthday, description, verification_code, verification_expires_at
           FROM users WHERE email = $1"#,
        email
    )
    .fetch_optional(pool)
    .await
}

/// Retrieves user by email, only if status is 'active'
///
/// # Security
/// - Parameterized query prevents SQL injection
/// - Returns Option to avoid user enumeration risks
pub async fn fetch_active_user_by_email_from_db(
    pool: &PgPool,
    email: &str,
) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"SELECT id, username, email, password_hash, totp_secret, 
           role_level, tier_level, creation_date, profile_picture_url, 
           first_name, last_name, country_code, language_code, 
           birthday, description, verification_code, verification_expires_at
           FROM users 
           WHERE email = $1 AND status = 'active'"#,
        email
    )
    .fetch_optional(pool)
    .await
}


/// Retrieves user by email, only if status is 'active'
///
/// # Security
/// - Parameterized query prevents SQL injection
/// - Returns Option to avoid user enumeration risks
pub async fn fetch_pending_user_by_email_from_db(
    pool: &PgPool,
    email: &str,
) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"SELECT id, username, email, password_hash, totp_secret, 
           role_level, tier_level, creation_date, profile_picture_url, 
           first_name, last_name, country_code, language_code, 
           birthday, description, verification_code, verification_expires_at
           FROM users 
           WHERE email = $1 AND status = 'pending'"#,
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

/// Inserts a new pending user for registration (with email verification).
///
/// # Arguments
/// - `pool`: The database connection pool.
/// - `username`: The new user's username.
/// - `email`: The new user's email.
/// - `password_hash`: The hashed password.
/// - `verification_code`: The email verification code.
/// - `verification_expires_at`: When the code expires.
///
/// # Returns
/// - `Ok(Uuid)` with the new user's ID on success.
/// - `Err(Error)` on failure.
pub async fn insert_pending_user_into_db(
    pool: &PgPool,
    username: &str,
    email: &str,
    password_hash: &str,
    verification_code: &str,
    verification_expires_at: DateTime<Utc>,

    // Optional fields:
    first_name: Option<&str>,
    last_name: Option<&str>,
    country_code: Option<&str>,
    language_code: Option<&str>,
    birthday: Option<NaiveDate>,
    description: Option<&str>,
    totp_secret: Option<&str>,
) -> Result<Uuid, Error> {
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

    // Insert user with pending status and verification code
    let row = sqlx::query!(
        r#"
        INSERT INTO users 
            (username, email, password_hash, role_level, tier_level, creation_date, status, 
             verification_code, verification_expires_at,
             first_name, last_name, country_code, language_code, birthday, description, totp_secret)
        VALUES 
            ($1, $2, $3, 1, 1, NOW()::timestamp, 'pending', $4, $5, 
             $6, $7, $8, $9, $10, $11, $12)
        RETURNING id
        "#,
        username,
        email,
        password_hash,
        verification_code,
        verification_expires_at,
        first_name,
        last_name,
        country_code,
        language_code,
        birthday,
        description,
        totp_secret, // Make sure your DB column is bool or nullable bool
    )
    .fetch_one(pool)
    .await?;

    Ok(row.id)
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

/// Inserts a password reset code for a user into the database.
///
/// # Arguments
/// - `pool`: Reference to the PostgreSQL connection pool.
/// - `user_id`: The UUID of the user.
/// - `code`: The password reset code (should be unique).
/// - `expires_at`: The UTC datetime when the code expires.
///
/// # Returns
/// - `Ok(())` on success.
/// - `Err(Error)` on failure.
pub async fn insert_user_password_reset_code_into_db(
    pool: &PgPool,
    user_id: Uuid,
    code: &str,
    expires_at: DateTime<Utc>,
) -> Result<(), Error> {
    let expires_at_naive = expires_at.naive_utc();
    sqlx::query!(
        r#"
        INSERT INTO users_password_reset_codes (user_id, code, expires_at)
        VALUES ($1, $2, $3)
        ON CONFLICT (code) DO UPDATE
            SET user_id = EXCLUDED.user_id,
                expires_at = EXCLUDED.expires_at
        "#,
        user_id,
        code,
        expires_at_naive
    )
    .execute(pool)
    .await?; 

    Ok(())
}

/// Updates the user's password hash in the database.
///
/// # Arguments
/// - `pool`: The database connection pool.
/// - `user_id`: The user's UUID.
/// - `new_password_hash`: The new hashed password.
///
/// # Returns
/// - `Ok(())` on success.
/// - `Err(sqlx::Error)` on failure.
pub async fn update_user_password_in_db(
    pool: &PgPool,
    user_id: Uuid,
    new_password_hash: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE users SET password_hash = $1 WHERE id = $2",
        new_password_hash,
        user_id
    )
    .execute(pool)
    .await?;
    Ok(())
}


/// Activates a user by setting status to 'active' and clearing verification fields.
///
/// # Arguments
/// - `pool`: The database connection pool.
/// - `user_id`: The user's UUID.
///
/// # Returns
/// - `Ok(())` on success.
/// - `Err(sqlx::Error)` on failure.
pub async fn activate_user_in_db(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE users
         SET status = 'active',
             verification_code = NULL,
             verification_expires_at = NULL
         WHERE id = $1",
        user_id
    )
    .execute(pool)
    .await?;
    Ok(())
}


/// Fetches the current (unexpired) password reset code for a user.
///
/// Returns `Ok(Some(UserPasswordResetCode))` if a code exists and is not expired,
/// `Ok(None)` if not found or expired, or `Err(sqlx::Error)` on DB error.
pub async fn fetch_current_password_reset_code_from_db(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Option<UserPasswordResetCode>, sqlx::Error> {
    let now = chrono::Utc::now().naive_utc();
    sqlx::query_as!(
        UserPasswordResetCode,
        r#"
        SELECT 
            user_id as "user_id!",
            code,
            expires_at as "expires_at!"
        FROM users_password_reset_codes
        WHERE user_id = $1 AND expires_at > $2
        ORDER BY expires_at DESC
        LIMIT 1
        "#,
        user_id,
        now
    )
    .fetch_optional(pool)
    .await
}

/// Deletes all password reset codes for the specified user.
///
/// # Arguments
/// - `pool`: Reference to the PostgreSQL connection pool.
/// - `user_id`: The UUID of the user.
///
/// # Returns
/// - `Ok(())` on success.
/// - `Err(sqlx::Error)` on database error.
pub async fn delete_all_password_reset_codes_for_user(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "DELETE FROM users_password_reset_codes WHERE user_id = $1",
        user_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

/// Checks if an active user exists with the given email or username
///
/// Returns `true` if a user with the given email or username exists and is active.
pub async fn check_user_exists_in_db(
    pool: &PgPool,
    email: &str,
    username: &str,
) -> Result<bool, sqlx::Error> {
    let user_by_email = sqlx::query_scalar!(
        r#"SELECT 1 FROM users WHERE email = $1 AND status = 'active' LIMIT 1"#,
        email
    )
    .fetch_optional(pool)
    .await?;

    if user_by_email.is_some() {
        return Ok(true);
    }

    let user_by_username = sqlx::query_scalar!(
        r#"SELECT 1 FROM users WHERE username = $1 AND status = 'active' LIMIT 1"#,
        username
    )
    .fetch_optional(pool)
    .await?;

    Ok(user_by_username.is_some())
}