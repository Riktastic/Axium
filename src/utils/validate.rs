use chrono::{NaiveDate, Utc};
use validator::ValidationError;
use regex::Regex;

pub fn validate_future_date(date_str: &str) -> Result<(), ValidationError> {
    // Attempt to parse the date string into a NaiveDate (date only, no time)
    if let Ok(parsed_date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        // Convert NaiveDate to DateTime<Utc> at the start of the day (00:00:00)
        let datetime_utc = match parsed_date.and_hms_opt(0, 0, 0) {
            Some(dt) => dt.and_utc(),
            None => return Err(ValidationError::new("Invalid time components.")),
        };

        // Get the current time in UTC
        let now_utc = Utc::now();

        // Check if the parsed date is in the future
        if datetime_utc > now_utc {
            Ok(())
        } else {
            Err(ValidationError::new("The date should be in the future."))
        }
    } else {
        Err(ValidationError::new("Invalid date format. Use YYYY-MM-DD."))
    }
}


pub fn validate_username(username: &str) -> Result<(), ValidationError> {
    let re = Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
    if !re.is_match(username) {
        return Err(ValidationError::new("Invalid username format. Only alphanumeric characters, dashes, and underscores are allowed."));
    }
    Ok(())
}

pub fn validate_password(password: &str) -> Result<(), ValidationError> {
    if password.len() < 8 {
        return Err(ValidationError::new("Password too short. Minimum length is 8 characters."));
    }
    
    let re = Regex::new(r"^[a-zA-Z0-9!@#$%^&*()_+\-=\[\]{};:'\'|,.<>/?]+$").unwrap();
    if !re.is_match(password) {
        return Err(ValidationError::new("Password contains invalid characters. Only alphanumeric characters and special characters are allowed."));
    }
    
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