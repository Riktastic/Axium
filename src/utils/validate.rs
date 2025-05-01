use chrono::{NaiveDate, Utc, Datelike};
use validator::ValidationError;
use regex::Regex;
use lazy_static::lazy_static;
use std::collections::HashSet;

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
        err.message = Some("Birthday must be within the last 120 years and not in the future".into());
        return Err(err);
    }
    Ok(())
}

// ISO 3166-1 alpha-2 country codes
lazy_static! {
    static ref ISO_COUNTRY_CODES: HashSet<&'static str> = {
        let codes = [
            // Full list of ISO 3166-1 alpha-2 codes
            "AD", "AE", "AF", "AG", "AI", "AL", "AM", "AO", "AQ", "AR", "AS", "AT", "AU", "AW",
            "AX", "AZ", "BA", "BB", "BD", "BE", "BF", "BG", "BH", "BI", "BJ", "BL", "BM", "BN",
            "BO", "BQ", "BR", "BS", "BT", "BV", "BW", "BY", "BZ", "CA", "CC", "CD", "CF", "CG",
            "CH", "CI", "CK", "CL", "CM", "CN", "CO", "CR", "CU", "CV", "CW", "CX", "CY", "CZ",
            "DE", "DJ", "DK", "DM", "DO", "DZ", "EC", "EE", "EG", "EH", "ER", "ES", "ET", "FI", "FJ",
            "FK", "FM", "FO", "FR", "GA", "GB", "GD", "GE", "GF", "GG", "GH", "GI", "GL", "GM",
            "GN", "GP", "GQ", "GR", "GS", "GT", "GU", "GW", "GY", "HK", "HM", "HN", "HR", "HT",
            "HU", "ID", "IE", "IL", "IM", "IN", "IO", "IQ", "IR", "IS", "IT", "JE", "JM", "JO",
            "JP", "KE", "KG", "KH", "KI", "KM", "KN", "KP", "KR", "KW", "KY", "KZ", "LA", "LB",
            "LC", "LI", "LK", "LR", "LS", "LT", "LU", "LV", "LY", "MA", "MC", "MD", "ME", "MF",
            "MG", "MH", "MK", "ML", "MM", "MN", "MO", "MP", "MQ", "MR", "MS", "MT", "MU", "MV",
            "MW", "MX", "MY", "MZ", "NA", "NC", "NE", "NF", "NG", "NI", "NL", "NO", "NP", "NR",
            "NU", "NZ", "OM", "PA", "PE", "PF", "PG", "PH", "PK", "PL", "PM", "PN", "PR", "PS",
            "PT", "PW", "PY", "QA", "RE", "RO", "RS", "RU", "RW", "SA", "SB", "SC", "SD", "SE",
            "SG", "SH", "SI", "SJ", "SK", "SL", "SM", "SN", "SO", "SR", "SS", "ST", "SV", "SX",
            "SY", "SZ", "TC", "TD", "TF", "TG", "TH", "TJ", "TK", "TL", "TM", "TN", "TO", "TR",
            "TT", "TV", "TW", "TZ", "UA", "UG", "UM", "US", "UY", "UZ", "VA", "VC", "VE", "VG",
            "VI", "VN", "VU", "WF", "WS", "YE", "YT", "ZA", "ZM", "ZW",
        ];
        codes.iter().cloned().collect()
    };
}

/// Validates ISO 3166-1 alpha-2 country codes
#[allow(dead_code)]
pub fn validate_country_code(code: &str) -> Result<(), ValidationError> {
    if !ISO_COUNTRY_CODES.contains(code) {
        let mut err = ValidationError::new("invalid_iso3166_country_code");
        err.message = Some(format!("'{}' is not a valid ISO 3166-1 alpha-2 country code", code).into());
        return Err(err);
    }
    Ok(())
}

// Common language-region codes (combination of IETF language tags)
lazy_static! {
    static ref LANGUAGE_CODES: HashSet<&'static str> = {
        let codes = [
            "af-ZA", "am-ET", "ar-SA", "as-IN", "az-AZ", 
            "be-BY", "bg-BG", "bn-BD", "bn-IN", "bs-BA",
            "ca-ES", "cs-CZ", "cy-GB", "da-DK", "de-AT",
            "de-CH", "de-DE", "el-GR", "en-AU", "en-CA",
            "en-GB", "en-IE", "en-IN", "en-NZ", "en-US",
            "es-AR", "es-CL", "es-CO", "es-ES", "es-MX",
            "es-US", "et-EE", "eu-ES", "fa-IR", "fi-FI",
            "fil-PH", "fr-BE", "fr-CA", "fr-CH", "fr-FR",
            "ga-IE", "gl-ES", "gu-IN", "he-IL", "hi-IN",
            "hr-HR", "hu-HU", "hy-AM", "id-ID", "is-IS",
            "it-CH", "it-IT", "ja-JP", "jv-ID", "ka-GE",
            "kk-KZ", "km-KH", "kn-IN", "ko-KR", "lo-LA",
            "lt-LT", "lv-LV", "mk-MK", "ml-IN", "mn-MN",
            "mr-IN", "ms-MY", "my-MM", "nb-NO", "ne-NP",
            "nl-BE", "nl-NL", "nn-NO", "or-IN", "pa-IN",
            "pl-PL", "ps-AF", "pt-BR", "pt-PT", "ro-RO",
            "ru-RU", "sd-PK", "si-LK", "sk-SK", "sl-SI",
            "so-SO", "sq-AL", "sr-RS", "sv-SE", "sw-KE",
            "ta-IN", "te-IN", "th-TH", "tr-TR", "uk-UA",
            "ur-PK", "uz-UZ", "vi-VN", "zh-CN", "zh-HK", "zh-TW", "zu-ZA"
            ];
            codes.iter().cloned().collect()
        };
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
    if !LANGUAGE_CODES.contains(code) {
        let mut err = ValidationError::new("invalid_language_code");
        err.message = Some(
            format!("'{}' is not a valid language-region code (e.g., en_US, nl-NL)", code).into()
        );
        return Err(err);
    }
    Ok(())
}

// For flexible format validation (accepts both en-US and en_US)
#[allow(dead_code)]
pub fn validate_language_code_flexible(code: &str) -> Result<(), ValidationError> {
    let normalized = code.replace('_', "-");
    if !LANGUAGE_CODES.contains(normalized.as_str()) {
        let mut err = ValidationError::new("invalid_language_code");
        err.message = Some(
            format!("'{}' is not a recognized language-region combination", code).into()
        );
        return Err(err);
    }
    Ok(())
}

/// Strict language code validation with format checking
/// 
/// Requires either:
/// - Language code only (en)
/// - Language with region (en-US or en_US)
/// 
/// Uses regex pattern: ^[a-z]{2,3}([-_][A-Z]{2})?$
#[allow(dead_code)]
pub fn validate_language_code_strict(code: &str) -> Result<(), ValidationError> {
    lazy_static! {
        static ref LANG_REGEX: Regex = Regex::new(r"^[a-z]{2,3}([-_][A-Z]{2})?$").unwrap();
    }

    if !LANG_REGEX.is_match(code) {
        let mut err = ValidationError::new("invalid_language_format");
        err.message = Some(
            "Must be in format: language_REGION (en_US) or language-REGION (en-US)".into()
        );
        return Err(err);
    }

    validate_language_code_flexible(code)
}