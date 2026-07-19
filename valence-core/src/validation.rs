//! Field validation framework
//!
//! Provides built-in validators and support for custom validation functions.
//! Validators are defined in schema definitions and enforced in generated code.
//!
//! # Built-in Validators
//!
//! - `email` - Valid email address format
//! - `phone` - Phone number (E.164 format)
//! - `url` - Valid URL
//! - `non_empty` - String is not empty
//! - `min_length:N` - String has at least N characters
//! - `max_length:N` - String has at most N characters
//! - `min:N` - Number is at least N
//! - `max:N` - Number is at most N
//! - `range:MIN,MAX` - Number is between MIN and MAX (inclusive)
//! - `positive` - Number is greater than 0
//! - `non_negative` - Number is greater than or equal to 0
//! - `enum:A,B,C` - Value is one of the listed options
//! - `pattern:REGEX` - String matches regex pattern
//!
//! # Custom Validators
//!
//! Use `fn:function_name` to call a custom validation function:
//!
//! ```text
//! // In schema:
//! // validations = ["fn:validate_business_name"]
//!
//! // In code:
//! fn validate_business_name(value: &str) -> Result<(), String> {
//!     if value.len() < 3 {
//!         return Err("Business name must be at least 3 characters".into());
//!     }
//!     Ok(())
//! }
//! ```
//!
//! # Inline Expressions
//!
//! Use `expr:EXPRESSION` for simple inline validations:
//!
//! ```text
//! // In schema:
//! // validations = ["expr:$value > 0"]
//! ```

use regex::Regex;
use std::sync::OnceLock;

/// Validation error
#[derive(Debug, Clone)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

impl ValidationError {
    pub fn new(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            message: message.into(),
        }
    }
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Validation error on field '{}': {}",
            self.field, self.message
        )
    }
}

impl std::error::Error for ValidationError {}

/// Email validation regex (simplified RFC 5322)
fn email_regex() -> &'static Regex {
    static EMAIL_REGEX: OnceLock<Regex> = OnceLock::new();
    EMAIL_REGEX
        .get_or_init(|| Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap())
}

/// URL validation regex (simplified)
fn url_regex() -> &'static Regex {
    static URL_REGEX: OnceLock<Regex> = OnceLock::new();
    URL_REGEX.get_or_init(|| Regex::new(r"^https?://[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}(/.*)?$").unwrap())
}

/// Phone validation regex (E.164 format: +[country code][number])
fn phone_regex() -> &'static Regex {
    static PHONE_REGEX: OnceLock<Regex> = OnceLock::new();
    PHONE_REGEX.get_or_init(|| Regex::new(r"^\+[1-9]\d{1,14}$").unwrap())
}

/// Built-in validators for strings
pub mod string {
    use super::*;

    pub fn email(value: &str) -> Result<(), String> {
        if email_regex().is_match(value) {
            Ok(())
        } else {
            Err("Invalid email format".into())
        }
    }

    pub fn phone(value: &str) -> Result<(), String> {
        if phone_regex().is_match(value) {
            Ok(())
        } else {
            Err("Invalid phone number (use E.164 format: +1234567890)".into())
        }
    }

    pub fn url(value: &str) -> Result<(), String> {
        if url_regex().is_match(value) {
            Ok(())
        } else {
            Err("Invalid URL format".into())
        }
    }

    pub fn non_empty(value: &str) -> Result<(), String> {
        if !value.is_empty() {
            Ok(())
        } else {
            Err("Value cannot be empty".into())
        }
    }

    pub fn min_length(value: &str, min: usize) -> Result<(), String> {
        if value.len() >= min {
            Ok(())
        } else {
            Err(format!("Value must be at least {min} characters"))
        }
    }

    pub fn max_length(value: &str, max: usize) -> Result<(), String> {
        if value.len() <= max {
            Ok(())
        } else {
            Err(format!("Value must be at most {max} characters"))
        }
    }

    pub fn pattern(value: &str, pattern: &str) -> Result<(), String> {
        let regex = Regex::new(pattern).map_err(|e| format!("Invalid regex pattern: {e}"))?;

        if regex.is_match(value) {
            Ok(())
        } else {
            Err(format!("Value does not match pattern: {pattern}"))
        }
    }

    pub fn enum_values(value: &str, allowed: &[&str]) -> Result<(), String> {
        if allowed.contains(&value) {
            Ok(())
        } else {
            Err(format!("Value must be one of: {}", allowed.join(", ")))
        }
    }
}

/// Built-in validators for integers
pub mod integer {
    pub fn min(value: i64, min: i64) -> Result<(), String> {
        if value >= min {
            Ok(())
        } else {
            Err(format!("Value must be at least {min}"))
        }
    }

    pub fn max(value: i64, max: i64) -> Result<(), String> {
        if value <= max {
            Ok(())
        } else {
            Err(format!("Value must be at most {max}"))
        }
    }

    pub fn range(value: i64, min: i64, max: i64) -> Result<(), String> {
        if value >= min && value <= max {
            Ok(())
        } else {
            Err(format!("Value must be between {min} and {max}"))
        }
    }

    pub fn positive(value: i64) -> Result<(), String> {
        if value > 0 {
            Ok(())
        } else {
            Err("Value must be positive".into())
        }
    }

    pub fn non_negative(value: i64) -> Result<(), String> {
        if value >= 0 {
            Ok(())
        } else {
            Err("Value must be non-negative".into())
        }
    }
}

/// Built-in validators for floats
pub mod decimal {
    pub fn min(value: f64, min: f64) -> Result<(), String> {
        if value >= min {
            Ok(())
        } else {
            Err(format!("Value must be at least {min}"))
        }
    }

    pub fn max(value: f64, max: f64) -> Result<(), String> {
        if value <= max {
            Ok(())
        } else {
            Err(format!("Value must be at most {max}"))
        }
    }

    pub fn range(value: f64, min: f64, max: f64) -> Result<(), String> {
        if value >= min && value <= max {
            Ok(())
        } else {
            Err(format!("Value must be between {min} and {max}"))
        }
    }

    pub fn positive(value: f64) -> Result<(), String> {
        if value > 0.0 {
            Ok(())
        } else {
            Err("Value must be positive".into())
        }
    }

    pub fn non_negative(value: f64) -> Result<(), String> {
        if value >= 0.0 {
            Ok(())
        } else {
            Err("Value must be non-negative".into())
        }
    }
}

fn on_validation_err(validator: &str) -> impl FnOnce(String) -> String {
    let validator = validator.to_string();
    move |e: String| {
        crate::instrumentation::record_validation_failure("", "", &validator);
        e
    }
}

// Convenient top-level wrappers for common validations
pub fn validate_email(value: &str) -> Result<(), String> {
    string::email(value).map_err(on_validation_err("email"))
}

pub fn validate_phone(value: &str) -> Result<(), String> {
    string::phone(value).map_err(on_validation_err("phone"))
}

pub fn validate_non_empty(value: &str) -> Result<(), String> {
    string::non_empty(value).map_err(on_validation_err("non_empty"))
}

pub fn validate_min_length(value: &str, min: usize) -> Result<(), String> {
    string::min_length(value, min).map_err(on_validation_err("min_length"))
}

pub fn validate_max_length(value: &str, max: usize) -> Result<(), String> {
    string::max_length(value, max).map_err(on_validation_err("max_length"))
}

pub fn validate_enum(value: &str, allowed: &[&str]) -> Result<(), String> {
    string::enum_values(value, allowed).map_err(on_validation_err("enum"))
}

pub fn validate_pattern(value: &str, pattern: &str) -> Result<(), String> {
    string::pattern(value, pattern).map_err(on_validation_err("pattern"))
}

pub fn validate_non_negative(value: &i64) -> Result<(), String> {
    integer::non_negative(*value).map_err(on_validation_err("non_negative"))
}

pub fn validate_positive(value: &i64) -> Result<(), String> {
    integer::positive(*value).map_err(on_validation_err("positive"))
}

pub fn validate_min(value: &i64, min: i64) -> Result<(), String> {
    integer::min(*value, min).map_err(on_validation_err("min"))
}

pub fn validate_max(value: &i64, max: i64) -> Result<(), String> {
    integer::max(*value, max).map_err(on_validation_err("max"))
}

pub fn validate_range(value: &i64, min: i64, max: i64) -> Result<(), String> {
    integer::range(*value, min, max).map_err(on_validation_err("range"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_validator() {
        assert!(string::email("test@example.com").is_ok());
        assert!(string::email("user.name+tag@example.co.uk").is_ok());
        assert!(string::email("invalid").is_err());
        assert!(string::email("@example.com").is_err());
        assert!(string::email("test@").is_err());
    }

    #[test]
    fn test_phone_validator() {
        assert!(string::phone("+12345678901").is_ok());
        assert!(string::phone("+442071234567").is_ok());
        assert!(string::phone("123456").is_err());
        assert!(string::phone("+0123456").is_err()); // Invalid: starts with 0
    }

    #[test]
    fn test_url_validator() {
        assert!(string::url("https://example.com").is_ok());
        assert!(string::url("http://example.com/path").is_ok());
        assert!(string::url("example.com").is_err());
        assert!(string::url("ftp://example.com").is_err());
    }

    #[test]
    fn test_string_length() {
        assert!(string::min_length("hello", 3).is_ok());
        assert!(string::min_length("hi", 3).is_err());

        assert!(string::max_length("hello", 10).is_ok());
        assert!(string::max_length("hello world!", 5).is_err());
    }

    #[test]
    fn test_integer_validators() {
        assert!(integer::min(10, 5).is_ok());
        assert!(integer::min(3, 5).is_err());

        assert!(integer::max(10, 15).is_ok());
        assert!(integer::max(20, 15).is_err());

        assert!(integer::range(10, 5, 15).is_ok());
        assert!(integer::range(3, 5, 15).is_err());

        assert!(integer::positive(1).is_ok());
        assert!(integer::positive(0).is_err());
        assert!(integer::positive(-1).is_err());

        assert!(integer::non_negative(0).is_ok());
        assert!(integer::non_negative(1).is_ok());
        assert!(integer::non_negative(-1).is_err());
    }

    #[test]
    fn test_enum_validator() {
        let allowed = &["active", "inactive", "pending"];

        assert!(string::enum_values("active", allowed).is_ok());
        assert!(string::enum_values("invalid", allowed).is_err());
    }

    #[test]
    fn test_pattern_validator() {
        assert!(string::pattern("ABC123", r"^[A-Z]{3}\d{3}$").is_ok());
        assert!(string::pattern("abc123", r"^[A-Z]{3}\d{3}$").is_err());
    }
}
