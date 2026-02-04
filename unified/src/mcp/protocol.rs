//! MCP protocol types - Updated for MCP 1.0

#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// MCP Protocol version
pub const PROTOCOL_VERSION: &str = "1.0.0";

// ============================================================================
// Core Request/Response Types (JSON-RPC 2.0)
// ============================================================================

/// MCP request (JSON-RPC 2.0 compliant)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct MCPRequest {
    pub jsonrpc: String,
    pub id: Option<Value>,
    pub method: String,
    pub params: Option<Value>,
}

/// MCP response (JSON-RPC 2.0 compliant)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct MCPResponse {
    pub jsonrpc: String,
    pub id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<MCPError>,
}

/// MCP error (JSON-RPC 2.0 error codes)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct MCPError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

impl MCPError {
    /// Parse error: Invalid JSON was received by the server
    pub fn parse_error() -> Self {
        Self {
            code: -32700,
            message: "Parse error".to_string(),
            data: None,
        }
    }

    /// Invalid Request: The JSON sent is not a valid Request object
    pub fn invalid_request() -> Self {
        Self {
            code: -32600,
            message: "Invalid Request".to_string(),
            data: None,
        }
    }

    /// Method not found: The method does not exist / is not available
    pub fn method_not_found() -> Self {
        Self {
            code: -32601,
            message: "Method not found".to_string(),
            data: None,
        }
    }

    /// Invalid params: Invalid method parameter(s)
    pub fn invalid_params() -> Self {
        Self {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        }
    }

    /// Internal error: Internal JSON-RPC error
    pub fn internal_error() -> Self {
        Self {
            code: -32603,
            message: "Internal error".to_string(),
            data: None,
        }
    }
}

// ============================================================================
// MCP 1.0 Capability Types
// ============================================================================

/// Client capabilities for MCP 1.0
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caching: Option<bool>,
}

/// Server capabilities for MCP 1.0
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<ToolsCapability>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<ResourceCapability>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling: Option<bool>,
}

/// Tools capability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsCapability {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_changed: Option<bool>,
}

/// Resources capability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceCapability {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe: Option<bool>,
}

// ============================================================================
// MCP 1.0 Initialize Types
// ============================================================================

/// Client information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientInfo {
    pub name: String,
    pub version: String,
}

/// Server information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    pub name: String,
    pub version: String,
}

/// Initialize request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitializeRequest {
    pub protocol_version: String,
    pub capabilities: ClientCapabilities,
    pub client_info: ClientInfo,
}

/// Initialize response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitializeResponse {
    pub protocol_version: String,
    pub capabilities: ServerCapabilities,
    pub server_info: ServerInfo,
}

// ============================================================================
// MCP 1.0 Resource Types
// ============================================================================

/// Resource request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequest {
    pub uri: String,
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Value>,
}

/// Resource response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceResponse {
    pub uri: String,
    pub mime_type: String,
    pub contents: String,
}

// ============================================================================
// MCP 1.0 Sampling Types
// ============================================================================

/// Sampling request for AI model calls
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SamplingRequest {
    pub model: String,
    pub max_tokens: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    pub messages: Vec<SamplingMessage>,
}

/// Message in sampling request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SamplingMessage {
    pub role: String, // "user" or "assistant"
    pub content: String,
}

/// Sampling response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SamplingResponse {
    pub model: String,
    pub content: String,
    pub stop_reason: String,
}

// ============================================================================
// MCP 1.0 Tool Types
// ============================================================================

/// Tool definition with full schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
}

/// Tool list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolListResponse {
    pub tools: Vec<ToolDefinition>,
}

/// Tool call request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Value>,
}

/// Tool call result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallResult {
    pub content: String,
    pub is_error: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_codes() {
        assert_eq!(MCPError::parse_error().code, -32700);
        assert_eq!(MCPError::invalid_request().code, -32600);
        assert_eq!(MCPError::method_not_found().code, -32601);
        assert_eq!(MCPError::invalid_params().code, -32602);
        assert_eq!(MCPError::internal_error().code, -32603);
    }

    #[test]
    fn test_initialize_request_serialization() {
        let req = InitializeRequest {
            protocol_version: "1.0.0".to_string(),
            capabilities: ClientCapabilities {
                sampling: Some(true),
                resources: Some(true),
                caching: Some(true),
            },
            client_info: ClientInfo {
                name: "test-client".to_string(),
                version: "1.0.0".to_string(),
            },
        };

        let json = serde_json::to_string(&req).unwrap();
        let deserialized: InitializeRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(req.protocol_version, deserialized.protocol_version);
        assert_eq!(req.client_info.name, deserialized.client_info.name);
    }

    #[test]
    fn test_tool_definition_serialization() {
        let tool = ToolDefinition {
            name: "test_tool".to_string(),
            description: "A test tool".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "message": {"type": "string"}
                }
            }),
        };

        let json = serde_json::to_string(&tool).unwrap();
        let deserialized: ToolDefinition = serde_json::from_str(&json).unwrap();

        assert_eq!(tool.name, deserialized.name);
        assert_eq!(tool.description, deserialized.description);
    }

    #[test]
    fn test_sampling_request_serialization() {
        let req = SamplingRequest {
            model: "claude-3-opus".to_string(),
            max_tokens: 1000,
            system: Some("You are helpful".to_string()),
            messages: vec![SamplingMessage {
                role: "user".to_string(),
                content: "Hello".to_string(),
            }],
        };

        let json = serde_json::to_string(&req).unwrap();
        let deserialized: SamplingRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(req.model, deserialized.model);
        assert_eq!(req.max_tokens, deserialized.max_tokens);
    }
}
