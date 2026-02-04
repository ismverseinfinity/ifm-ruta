//! Async tool trait for MCP 1.0

use async_trait::async_trait;
use serde_json::Value;
use std::error::Error;

/// Tool metadata
#[derive(Debug, Clone)]
pub struct ToolMetadata {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
    pub version: String,
}

/// Tool response
#[derive(Debug, Clone)]
pub struct ToolResponse {
    pub content: String,
    pub is_error: bool,
}

/// MCP Result type
pub type MCPResult<T> = Result<T, MCPError>;

/// MCP Error type for async operations
#[derive(Debug, Clone)]
pub enum MCPError {
    /// Invalid parameters
    InvalidParams(String),
    /// Validation error
    ValidationError(String),
    /// Execution error
    ExecutionError(String),
    /// Timeout error
    TimeoutError,
    /// Not found
    NotFound(String),
    /// Internal server error
    InternalError(String),
}

impl std::fmt::Display for MCPError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MCPError::InvalidParams(msg) => write!(f, "Invalid parameters: {}", msg),
            MCPError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            MCPError::ExecutionError(msg) => write!(f, "Execution error: {}", msg),
            MCPError::TimeoutError => write!(f, "Tool execution timed out"),
            MCPError::NotFound(msg) => write!(f, "Not found: {}", msg),
            MCPError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl Error for MCPError {}

/// Basic async tool trait for MCP 1.0
#[async_trait]
pub trait AsyncTool: Send + Sync {
    /// Execute the tool with given arguments
    async fn execute(&self, args: Value) -> MCPResult<ToolResponse>;

    /// Get tool metadata
    fn metadata(&self) -> ToolMetadata;

    /// Validate input (optional, default implementation does nothing)
    fn validate_input(&self, _args: &Value) -> MCPResult<()> {
        Ok(())
    }
}

/// Streaming tool trait for tools that support streaming responses
pub type ToolStream =
    Box<dyn futures::stream::Stream<Item = Result<String, String>> + Send + Unpin>;

#[async_trait]
pub trait StreamingTool: Send + Sync {
    /// Execute the tool with streaming output
    async fn execute_streaming(&self, args: Value) -> MCPResult<ToolStream>;

    /// Get tool metadata
    fn metadata(&self) -> ToolMetadata;

    /// Validate input (optional)
    fn validate_input(&self, _args: &Value) -> MCPResult<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_metadata_creation() {
        let metadata = ToolMetadata {
            name: "test_tool".to_string(),
            description: "A test tool".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "message": {"type": "string"}
                }
            }),
            version: "1.0.0".to_string(),
        };

        assert_eq!(metadata.name, "test_tool");
        assert_eq!(metadata.version, "1.0.0");
    }

    #[test]
    fn test_tool_response_creation() {
        let response = ToolResponse {
            content: "test result".to_string(),
            is_error: false,
        };

        assert_eq!(response.content, "test result");
        assert!(!response.is_error);
    }

    #[test]
    fn test_mcp_error_display() {
        let err = MCPError::InvalidParams("test param".to_string());
        assert_eq!(err.to_string(), "Invalid parameters: test param");

        let err = MCPError::TimeoutError;
        assert_eq!(err.to_string(), "Tool execution timed out");
    }

    #[test]
    fn test_mcp_result_type() {
        let result: MCPResult<String> = Ok("success".to_string());
        assert!(result.is_ok());

        let error: MCPResult<String> = Err(MCPError::ExecutionError("failed".to_string()));
        assert!(error.is_err());
    }
}
