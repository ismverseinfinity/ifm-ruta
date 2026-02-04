//! Protocol tests for MCP 1.0 implementation

#[cfg(test)]
mod protocol_tests {
    use ifm_ruta_core::traits::async_tool::{MCPError, MCPResult, ToolMetadata, ToolResponse};
    use serde_json::json;

    // Test error code generation
    #[test]
    fn test_mcp_error_display_messages() {
        let errors = vec![
            (
                MCPError::InvalidParams("test".to_string()),
                "Invalid parameters: test",
            ),
            (
                MCPError::ValidationError("bad".to_string()),
                "Validation error: bad",
            ),
            (
                MCPError::ExecutionError("fail".to_string()),
                "Execution error: fail",
            ),
            (MCPError::TimeoutError, "Tool execution timed out"),
            (MCPError::NotFound("item".to_string()), "Not found: item"),
            (
                MCPError::InternalError("err".to_string()),
                "Internal error: err",
            ),
        ];

        for (error, expected) in errors {
            assert_eq!(error.to_string(), expected);
        }
    }

    // Test tool metadata
    #[test]
    fn test_tool_metadata_creation() {
        let metadata = ToolMetadata {
            name: "test_tool".to_string(),
            description: "A test tool".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "message": {"type": "string"}
                }
            }),
            version: "1.0.0".to_string(),
        };

        assert_eq!(metadata.name, "test_tool");
        assert_eq!(metadata.version, "1.0.0");
        assert_eq!(metadata.description, "A test tool");
        assert!(metadata.input_schema.is_object());
    }

    // Test tool response
    #[test]
    fn test_tool_response_success() {
        let response = ToolResponse {
            content: "success".to_string(),
            is_error: false,
        };

        assert_eq!(response.content, "success");
        assert!(!response.is_error);
    }

    #[test]
    fn test_tool_response_error() {
        let response = ToolResponse {
            content: "error message".to_string(),
            is_error: true,
        };

        assert_eq!(response.content, "error message");
        assert!(response.is_error);
    }

    // Test MCPResult type
    #[test]
    fn test_mcp_result_ok() {
        let result: MCPResult<String> = Ok("success".to_string());
        assert!(result.is_ok());
        match result {
            Ok(val) => assert_eq!(val, "success"),
            Err(_) => panic!("Expected Ok result"),
        }
    }

    #[test]
    fn test_mcp_result_error() {
        let result: MCPResult<String> = Err(MCPError::ExecutionError("failed".to_string()));
        assert!(result.is_err());
    }

    // Test tool metadata fields
    #[test]
    fn test_tool_metadata_fields() {
        let metadata = ToolMetadata {
            name: "field_test".to_string(),
            description: "Test fields".to_string(),
            input_schema: json!({"type": "string"}),
            version: "1.0.0".to_string(),
        };

        assert_eq!(metadata.name, "field_test");
        assert_eq!(metadata.description, "Test fields");
        assert_eq!(metadata.version, "1.0.0");
        assert!(metadata.input_schema.get("type").is_some());
    }

    // Test schema validation structure
    #[test]
    fn test_input_schema_with_properties() {
        let schema = json!({
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "age": {"type": "number"},
                "active": {"type": "boolean"}
            },
            "required": ["name"]
        });

        assert!(schema.get("properties").is_some());
        assert!(schema.get("required").is_some());

        let props = schema.get("properties").unwrap();
        assert!(props.get("name").is_some());
        assert!(props.get("age").is_some());
        assert!(props.get("active").is_some());
    }
}
