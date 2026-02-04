/// Path validation and security checks
///
/// Prevents path traversal attacks and validates file system access
use std::path::{Path, PathBuf};

/// Validates and sanitizes file paths to prevent directory traversal attacks
pub struct PathValidator;

impl PathValidator {
    /// Validate and normalize a path string
    ///
    /// # Checks
    /// - No null bytes
    /// - No path traversal (`..`)
    /// - Within allowed directory
    ///
    /// # Returns
    /// - `Ok(String)` if path is valid and normalized
    /// - `Err(String)` with error description if validation fails
    pub fn validate(path: &str) -> Result<String, String> {
        // 1. Check for null bytes
        if path.contains('\0') {
            return Err("Path contains null bytes".to_string());
        }

        // 2. Check for path traversal attacks
        if path.contains("..") {
            return Err("Path traversal detected".to_string());
        }

        // 3. Check for empty path
        if path.is_empty() {
            return Err("Path cannot be empty".to_string());
        }

        // 4. Try to canonicalize path
        let path_buf = PathBuf::from(path);
        let canonical = path_buf
            .canonicalize()
            .map_err(|e| format!("Invalid path: {}", e))?;

        // 5. Verify within allowed directory
        let canonical_str = canonical
            .to_str()
            .ok_or_else(|| "Path contains invalid UTF-8".to_string())?;

        if !Self::is_allowed(&canonical_str) {
            return Err("Path outside allowed directory".to_string());
        }

        Ok(canonical_str.to_string())
    }

    /// Validate path without canonicalizing
    /// Useful for paths that don't exist yet
    pub fn validate_path_components(path: &str) -> Result<(), String> {
        // 1. Check for null bytes
        if path.contains('\0') {
            return Err("Path contains null bytes".to_string());
        }

        // 2. Check for path traversal
        if path.contains("..") {
            return Err("Path traversal detected".to_string());
        }

        // 3. Validate each component
        let path_obj = Path::new(path);
        for component in path_obj.components() {
            use std::path::Component;

            if let Component::Normal(name) = component {
                let name_str = name
                    .to_str()
                    .ok_or_else(|| "Invalid UTF-8 in path component".to_string())?;

                // Check for suspicious patterns
                if name_str.starts_with('.') && name_str != "." {
                    return Err("Hidden files not allowed".to_string());
                }
            }
        }

        Ok(())
    }

    /// Check if path is within allowed base directories
    fn is_allowed(path: &str) -> bool {
        // Allow home directory and temp directory
        path.starts_with("/home/")
            || path.starts_with("/tmp/")
            || path.starts_with("/var/tmp/")
            || path.starts_with(std::env::var("HOME").unwrap_or_default().as_str())
            || std::cfg!(windows) && (path.contains("\\Users\\") || path.contains("\\temp\\"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_traversal_prevention() {
        assert!(PathValidator::validate("../../../etc/passwd").is_err());
    }

    #[test]
    fn test_null_byte_prevention() {
        assert!(PathValidator::validate("/path/to/file\0.txt").is_err());
    }

    #[test]
    fn test_empty_path() {
        assert!(PathValidator::validate("").is_err());
    }

    #[test]
    fn test_validate_components() {
        assert!(PathValidator::validate_path_components("valid/path.txt").is_ok());
        assert!(PathValidator::validate_path_components("../invalid").is_err());
        assert!(PathValidator::validate_path_components("path\0invalid").is_err());
    }
}
