use chrono::{NaiveDate, Utc, Datelike};
use validator::ValidationError;
use regex::Regex;
use lazy_static::lazy_static;

use crate::referencedata::countries::countries;
use crate::referencedata::languages::languages;


/// Validates that a date string is in the future
/// 
/// # Arguments
/// * `date_str` - Date string in format YYYY-MM-DD
/// 
/// # Returns
/// `Ok(())` if valid future date, `ValidationError` otherwise
#[allow(dead_code)]
pub fn validate_future_date(date_str: &str) -> Result<(), ValidationError> {
    // Parse date using chrono's NaiveDate parser
    if let Ok(parsed_date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        // Convert to UTC datetime at midnight for comparison
        let datetime_utc = match parsed_date.and_hms_opt(0, 0, 0) {
            Some(dt) => dt.and_utc(),
            None => return Err(ValidationError::new("Invalid time components.")),
        };

        // Compare with current UTC time
        if datetime_utc > Utc::now() {
            Ok(())
        } else {
            Err(ValidationError::new("The date should be in the future."))
        }
    } else {
        Err(ValidationError::new("Invalid date format. Use YYYY-MM-DD."))
    }
}

/// Validates username format requirements
/// 
/// Requirements:
/// - Only alphanumeric, dash, and underscore characters
/// 
/// # Arguments
/// * `username` - String to validate
#[allow(dead_code)]
pub fn validate_username(username: &str) -> Result<(), ValidationError> {
    let re = Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
    if !re.is_match(username) {
        return Err(ValidationError::new("Invalid username format. Only alphanumeric characters, dashes, and underscores are allowed."));
    }
    Ok(())
}

/// Validates password complexity requirements
/// 
/// Requirements:
/// - Minimum 8 characters
/// - At least one uppercase letter
/// - At least one lowercase letter
/// - At least one number
/// - At least one special character
/// - Only allowed special characters
#[allow(dead_code)]
pub fn validate_password(password: &str) -> Result<(), ValidationError> {
    // Length check
    if password.len() < 8 {
        return Err(ValidationError::new("Password too short. Minimum length is 8 characters."));
    }
    
    // Character whitelist check
    let re = Regex::new(r"^[a-zA-Z0-9!@#$%^&*()_+\-=\[\]{};:'\'|,.<>/?]+$").unwrap();
    if !re.is_match(password) {
        return Err(ValidationError::new("Password contains invalid characters. Only alphanumeric characters and special characters are allowed."));
    }
    
    // Complexity requirements
    if !password.chars().any(|c| c.is_uppercase()) {
        return Err(ValidationError::new("Password must contain an uppercase letter."));
    }
    if !password.chars().any(|c| c.is_lowercase()) {
        return Err(ValidationError::new("Password must contain a lowercase letter."));
    }
    if !password.chars().any(|c| c.is_numeric()) {
        return Err(ValidationError::new("Password must contain a number."));
    }
    if !password.chars().any(|c| "!@#$%^&*()_+-=[]{};:'\"\\|,.<>/?".contains(c)) {
        return Err(ValidationError::new("Password must contain at least one special character."));
    }
    Ok(())
}

/// Validates birthday date range
/// 
/// Requirements:
/// - Must be within last 120 years
/// - Cannot be in the future
#[allow(dead_code)]
pub fn validate_birthday(birthday: &NaiveDate) -> Result<(), ValidationError> {
    let today = Utc::now().date_naive();
    let earliest = NaiveDate::from_ymd_opt(today.year() - 120, today.month(), today.day())
        .unwrap_or_else(|| NaiveDate::from_ymd_opt(1900, 1, 1).unwrap());

    if *birthday < earliest || *birthday > today {
        let mut err = ValidationError::new("birthday_out_of_range");
        err.message = Some("Birthday must be within the last 120 years and not in the future.".into());
        return Err(err);
    }
    Ok(())
}

/// Validates ISO 3166-1 alpha-2 country codes
#[allow(dead_code)]
pub fn validate_country_code(code: &str) -> Result<(), ValidationError> {
    if !countries().contains_key(code) {
        let mut err = ValidationError::new("invalid_iso3166_country_code");
        err.message = Some(format!("'{}' is not a valid ISO 3166-1 alpha-2 country code.", code).into());
        return Err(err);
    }
    Ok(())
}
    
/// Validates language-region code format (IETF BCP 47 format variants)
/// 
/// Supports common hyphen-separated codes (en-US) and underscore variants (en_US)
/// 
/// # Arguments
/// * `code` - Language code string to validate
/// 
/// # Returns
/// `Ok(())` if valid code, `ValidationError` with details otherwise
#[allow(dead_code)]
pub fn validate_language_code(code: &str) -> Result<(), ValidationError> {
    if !languages().contains_key(code) {
        let mut err = ValidationError::new("invalid_language_code");
        err.message = Some(
            format!("'{}' is not a valid language-region code (e.g., en_US, nl-NL).", code).into()
        );
        return Err(err);
    }
    Ok(())
}