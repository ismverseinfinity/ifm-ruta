//! Tool input/output schemas (Phase 1, Task 1.3.2)

use serde_json::{json, Value};

/// Get JSON schema for interactive_feedback tool
#[allow(dead_code)]
pub fn interactive_feedback_schema() -> Value {
    json!({
        "type": "object",
        "title": "interactive_feedback",
        "description": "Request interactive feedback for a project",
        "properties": {
            "projectDirectory": {
                "type": "string",
                "description": "Full path to the project directory"
            },
            "prompt": {
                "type": "string",
                "description": "The prompt to show to the user for feedback"
            },
            "previousUserRequest": {
                "type": "string",
                "description": "The previous user request that triggered this feedback"
            }
        },
        "required": ["projectDirectory", "prompt"],
        "additionalProperties": false
    })
}

/// Get JSON schema for echo tool
#[allow(dead_code)]
pub fn echo_schema() -> Value {
    json!({
        "type": "object",
        "title": "echo",
        "description": "Echo back the input message",
        "properties": {
            "message": {
                "type": "string",
                "description": "The message to echo"
            }
        },
        "required": ["message"],
        "additionalProperties": false
    })
}

/// Get output schema for tool results
#[allow(dead_code)]
pub fn tool_result_schema() -> Value {
    json!({
        "type": "object",
        "title": "ToolResult",
        "description": "Response from tool execution",
        "properties": {
            "content": {
                "type": "string",
                "description": "The output content from the tool"
            },
            "is_error": {
                "type": "boolean",
                "description": "Whether this is an error response"
            }
        },
        "required": ["content", "is_error"],
        "additionalProperties": false
    })
}

/// Get JSON schema for sampling request
#[allow(dead_code)]
pub fn sampling_request_schema() -> Value {
    json!({
        "type": "object",
        "title": "SamplingRequest",
        "description": "Request for AI model sampling",
        "properties": {
            "model": {
                "type": "string",
                "description": "The model to use for sampling"
            },
            "max_tokens": {
                "type": "integer",
                "description": "Maximum tokens to generate",
                "minimum": 1
            },
            "system": {
                "type": "string",
                "description": "System prompt for the model (optional)"
            },
            "messages": {
                "type": "array",
                "description": "Messages to send to the model",
                "items": {
                    "type": "object",
                    "properties": {
                        "role": {
                            "type": "string",
                            "enum": ["user", "assistant"]
                        },
                        "content": {
                            "type": "string"
                        }
                    },
                    "required": ["role", "content"]
                },
                "minItems": 1
            }
        },
        "required": ["model", "max_tokens", "messages"],
        "additionalProperties": false
    })
}

/// Get all available tool schemas
#[allow(dead_code)]
pub fn all_schemas() -> std::collections::HashMap<String, Value> {
    let mut schemas = std::collections::HashMap::new();
    schemas.insert("interactive_feedback".to_string(), interactive_feedback_schema());
    schemas.insert("echo".to_string(), echo_schema());
    schemas.insert("tool_result".to_string(), tool_result_schema());
    schemas.insert("sampling_request".to_string(), sampling_request_schema());
    schemas
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interactive_feedback_schema() {
        let schema = interactive_feedback_schema();
        assert_eq!(schema["type"], "object");
        assert_eq!(schema["title"], "interactive_feedback");
        assert!(schema["properties"]["projectDirectory"].is_object());
        assert!(schema["properties"]["prompt"].is_object());
    }

    #[test]
    fn test_echo_schema() {
        let schema = echo_schema();
        assert_eq!(schema["type"], "object");
        assert_eq!(schema["title"], "echo");
        assert!(schema["properties"]["message"].is_object());
    }

    #[test]
    fn test_tool_result_schema() {
        let schema = tool_result_schema();
        assert_eq!(schema["type"], "object");
        assert_eq!(schema["title"], "ToolResult");
        assert!(schema["properties"]["content"].is_object());
        assert!(schema["properties"]["is_error"].is_object());
    }

    #[test]
    fn test_sampling_request_schema() {
        let schema = sampling_request_schema();
        assert_eq!(schema["type"], "object");
        assert_eq!(schema["title"], "SamplingRequest");
        assert!(schema["properties"]["model"].is_object());
        assert!(schema["properties"]["max_tokens"].is_object());
        assert!(schema["properties"]["messages"].is_object());
    }

    #[test]
    fn test_all_schemas() {
        let schemas = all_schemas();
        assert_eq!(schemas.len(), 4);
        assert!(schemas.contains_key("interactive_feedback"));
        assert!(schemas.contains_key("echo"));
        assert!(schemas.contains_key("tool_result"));
        assert!(schemas.contains_key("sampling_request"));
    }

    #[test]
    fn test_schema_has_required_fields() {
        let schema = interactive_feedback_schema();
        let required = &schema["required"];
        assert!(required.is_array());
        assert!(required[0].as_str().is_some());
    }

    #[test]
    fn test_schema_serialization() {
        let schema = echo_schema();
        let json_str = serde_json::to_string(&schema).unwrap();
        let deserialized: Value = serde_json::from_str(&json_str).unwrap();
        assert_eq!(schema, deserialized);
    }
}
