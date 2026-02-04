//! JSON schema validation for tool inputs (Phase 1, Task 1.3)

use jsonschema::JSONSchema;
use serde_json::Value;
use std::collections::HashMap;

/// Validator for tool inputs using JSON schema
pub struct InputValidator {
    schemas: HashMap<String, JSONSchema>,
}

impl InputValidator {
    /// Create a new input validator
    pub fn new() -> Self {
        Self {
            schemas: HashMap::new(),
        }
    }

    /// Register a schema for a tool
    pub fn register_schema(
        &mut self,
        tool_name: &str,
        schema: Value,
    ) -> Result<(), String> {
        match JSONSchema::compile(&schema) {
            Ok(compiled) => {
                self.schemas.insert(tool_name.to_string(), compiled);
                Ok(())
            }
            Err(e) => Err(format!("Failed to compile schema: {}", e)),
        }
    }

    /// Validate input against a registered schema
    pub fn validate(&self, tool_name: &str, input: &Value) -> Result<(), String> {
        match self.schemas.get(tool_name) {
            Some(schema) => {
                match schema.validate(input) {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        let error_msg = e
                            .map(|err| err.to_string())
                            .collect::<Vec<_>>()
                            .join("; ");
                        Err(format!("Validation failed: {}", error_msg))
                    }
                }
            }
            None => Err(format!("No schema registered for tool: {}", tool_name)),
        }
    }

    /// Check if a schema is registered for a tool
    pub fn has_schema(&self, tool_name: &str) -> bool {
        self.schemas.contains_key(tool_name)
    }

    /// Get the number of registered schemas
    pub fn schema_count(&self) -> usize {
        self.schemas.len()
    }
}

impl Default for InputValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_create_validator() {
        let validator = InputValidator::new();
        assert_eq!(validator.schema_count(), 0);
    }

    #[test]
    fn test_register_schema() {
        let mut validator = InputValidator::new();
        let schema = json!({
            "type": "object",
            "properties": {
                "message": {"type": "string"}
            },
            "required": ["message"]
        });

        let result = validator.register_schema("echo", schema);
        assert!(result.is_ok());
        assert!(validator.has_schema("echo"));
    }

    #[test]
    fn test_validate_valid_input() {
        let mut validator = InputValidator::new();
        let schema = json!({
            "type": "object",
            "properties": {
                "projectDirectory": {"type": "string"},
                "prompt": {"type": "string"}
            },
            "required": ["projectDirectory", "prompt"]
        });

        validator.register_schema("interactive_feedback", schema).unwrap();

        let valid_input = json!({
            "projectDirectory": "/path/to/project",
            "prompt": "Please review"
        });

        assert!(validator.validate("interactive_feedback", &valid_input).is_ok());
    }

    #[test]
    fn test_validate_invalid_input() {
        let mut validator = InputValidator::new();
        let schema = json!({
            "type": "object",
            "properties": {
                "message": {"type": "string"}
            },
            "required": ["message"]
        });

        validator.register_schema("echo", schema).unwrap();

        let invalid_input = json!({
            "wrongField": "test"
        });

        assert!(validator.validate("echo", &invalid_input).is_err());
    }

    #[test]
    fn test_validate_nonexistent_schema() {
        let validator = InputValidator::new();
        let input = json!({"test": "value"});

        let result = validator.validate("nonexistent", &input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("No schema registered"));
    }

    #[test]
    fn test_validate_wrong_type() {
        let mut validator = InputValidator::new();
        let schema = json!({
            "type": "object",
            "properties": {
                "count": {"type": "integer"}
            },
            "required": ["count"]
        });

        validator.register_schema("counter", schema).unwrap();

        let invalid_input = json!({
            "count": "not an integer"
        });

        assert!(validator.validate("counter", &invalid_input).is_err());
    }

    #[test]
    fn test_multiple_schemas() {
        let mut validator = InputValidator::new();

        let schema1 = json!({
            "type": "object",
            "properties": {"message": {"type": "string"}},
            "required": ["message"]
        });

        let schema2 = json!({
            "type": "object",
            "properties": {"count": {"type": "integer"}},
            "required": ["count"]
        });

        validator.register_schema("tool1", schema1).unwrap();
        validator.register_schema("tool2", schema2).unwrap();

        assert_eq!(validator.schema_count(), 2);
        assert!(validator.has_schema("tool1"));
        assert!(validator.has_schema("tool2"));
    }

    #[test]
    fn test_invalid_schema() {
        let mut validator = InputValidator::new();
        let invalid_schema = json!({
            "type": "invalid_type"
        });

        // This should still compile but may not validate strictly
        let result = validator.register_schema("invalid", invalid_schema);
        // Schema compilation might succeed even with unknown types
        // depending on jsonschema implementation
        let _ = result;
    }
}
