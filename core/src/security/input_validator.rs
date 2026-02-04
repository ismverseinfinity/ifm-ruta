/// Input validation utilities for preventing injection attacks
///
/// Validates user input to prevent command injection and other attacks

/// Validates command-line input for potential injection attacks
pub struct InputValidator;

impl InputValidator {
    /// Validate user input for shell injection prevention
    /// 
    /// # Checks
    /// - No shell metacharacters
    /// - Length limits
    /// - Valid UTF-8
    pub fn validate_input(input: &str) -> Result<(), String> {
        // 1. Check for null bytes
        if input.contains('\0') {
            return Err("Input contains null bytes".to_string());
        }

        // 2. Check length
        if input.is_empty() || input.len() > 10000 {
            return Err("Input length invalid (0 < len < 10000)".to_string());
        }

        // 3. Check for shell metacharacters that could cause injection
        let dangerous_chars = ['|', ';', '&', '$', '`', '\n', '\r'];
        if input.chars().any(|c| dangerous_chars.contains(&c)) {
            return Err("Input contains potentially dangerous characters".to_string());
        }

        Ok(())
    }

    /// Validate URL/path for security
    pub fn validate_url(url: &str) -> Result<(), String> {
        // 1. Check for null bytes
        if url.contains('\0') {
            return Err("URL contains null bytes".to_string());
        }

        // 2. Check length
        if url.is_empty() || url.len() > 2048 {
            return Err("URL length invalid".to_string());
        }

        // 3. Parse as URL
        match url::Url::parse(url) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Invalid URL: {}", e)),
        }
    }

    /// Validate JSON field name to prevent key injection
    pub fn validate_field_name(name: &str) -> Result<(), String> {
        // 1. Check for null bytes
        if name.contains('\0') {
            return Err("Field name contains null bytes".to_string());
        }

        // 2. Check length
        if name.is_empty() || name.len() > 255 {
            return Err("Field name length invalid".to_string());
        }

        // 3. Alphanumeric and underscore only
        if !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err("Field name contains invalid characters".to_string());
        }

        Ok(())
    }

    /// Check for buffer overflow attempts
    pub fn check_buffer_size(size: usize, max: usize) -> Result<(), String> {
        if size > max {
            return Err(format!(
                "Buffer size {} exceeds maximum {}",
                size, max
            ));
        }
        Ok(())
    }

    /// Check for integer overflow
    pub fn check_integer_overflow(a: u64, b: u64) -> Result<u64, String> {
        a.checked_add(b)
            .ok_or_else(|| "Integer overflow detected".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_input() {
        assert!(InputValidator::validate_input("hello world").is_ok());
    }

    #[test]
    fn test_shell_injection_prevention() {
        assert!(InputValidator::validate_input("test; rm -rf /").is_err());
        assert!(InputValidator::validate_input("test | cat").is_err());
        assert!(InputValidator::validate_input("test && echo bad").is_err());
        assert!(InputValidator::validate_input("test $(whoami)").is_err());
    }

    #[test]
    fn test_null_byte_prevention() {
        assert!(InputValidator::validate_input("test\0bad").is_err());
    }

    #[test]
    fn test_length_validation() {
        assert!(InputValidator::validate_input("").is_err());
        assert!(InputValidator::validate_input(&"x".repeat(10001)).is_err());
    }

    #[test]
    fn test_field_name_validation() {
        assert!(InputValidator::validate_field_name("valid_name").is_ok());
        assert!(InputValidator::validate_field_name("name123").is_ok());
        assert!(InputValidator::validate_field_name("name-invalid").is_err());
        assert!(InputValidator::validate_field_name("").is_err());
    }

    #[test]
    fn test_buffer_overflow_check() {
        assert!(InputValidator::check_buffer_size(100, 200).is_ok());
        assert!(InputValidator::check_buffer_size(300, 200).is_err());
    }

    #[test]
    fn test_integer_overflow() {
        assert!(InputValidator::check_integer_overflow(100, 200).is_ok());
        assert!(InputValidator::check_integer_overflow(u64::MAX, 1).is_err());
    }
}
