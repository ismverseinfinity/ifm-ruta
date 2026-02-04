//! MCP server implementation - Async version (Phase 1, Task 1.2)

use serde_json::json;
use std::sync::Arc;
use tokio::sync::RwLock;

use ifm_ruta_core::{
    models::AppError,
    traits::{AsyncTool, EventBus, ProcessManager, SettingsManager, Tool},
};

/// Re-export protocol types from protocol module
use super::protocol;

/// MCP Request struct (JSON-RPC 2.0)
pub use super::protocol::MCPRequest;

/// MCP Response struct (JSON-RPC 2.0)
pub use super::protocol::MCPResponse;

/// MCP Error struct (JSON-RPC 2.0)
pub use super::protocol::MCPError as ProtocolError;

/// Async MCP server for handling concurrent requests
pub struct MCPServer {
    /// Store for legacy synchronous tools (to be migrated)
    #[allow(clippy::arc_with_non_send_sync)]
    tools: Arc<RwLock<std::collections::HashMap<String, Box<dyn Tool>>>>,

    /// Store for async tools (MCP 1.0)
    async_tools: Arc<RwLock<std::collections::HashMap<String, Arc<dyn AsyncTool>>>>,

    #[allow(dead_code)]
    settings_manager: Arc<dyn SettingsManager>,

    #[allow(dead_code)]
    process_manager: Arc<dyn ProcessManager>,

    #[allow(dead_code)]
    event_bus: Arc<dyn EventBus>,
}

impl MCPServer {
    /// Create a new async MCP server
    pub fn new(
        settings_manager: Arc<dyn SettingsManager>,
        process_manager: Arc<dyn ProcessManager>,
        event_bus: Arc<dyn EventBus>,
    ) -> Self {
        Self {
            tools: Arc::new(RwLock::new(std::collections::HashMap::new())),
            async_tools: Arc::new(RwLock::new(std::collections::HashMap::new())),
            settings_manager,
            process_manager,
            event_bus,
        }
    }

    /// Register a legacy synchronous tool (deprecated)
    pub async fn register_tool(&self, tool: Box<dyn Tool>) {
        let name = tool.name().to_string();
        let mut tools = self.tools.write().await;
        tools.insert(name, tool);
    }

    /// Register an async tool (MCP 1.0)
    pub async fn register_async_tool(&self, name: &str, tool: Arc<dyn AsyncTool>) {
        let mut async_tools = self.async_tools.write().await;
        async_tools.insert(name.to_string(), tool);
    }

    /// Handle a JSON-RPC 2.0 request (async)
    pub async fn handle_request(
        &self,
        request: MCPRequest,
    ) -> Result<Option<MCPResponse>, AppError> {
        // Check if this is a notification (no id field)
        if request.id.is_none() {
            // Handle notifications silently (no response needed per JSON-RPC 2.0 spec)
            match request.method.as_str() {
                "notifications/initialized" => {
                    tracing::debug!("Received notifications/initialized");
                    return Ok(None);
                }
                _ => {
                    tracing::debug!("Received unknown notification: {}", request.method);
                    return Ok(None);
                }
            }
        }

        // This is a request (has id) - send response
        let response = match request.method.as_str() {
            "initialize" => self.handle_initialize(request).await,
            "tools/list" => self.handle_tools_list(request).await,
            "tools/call" => self.handle_tool_call(request).await,
            "resources/list" => self.handle_resources_list(request).await,
            "sampling" => self.handle_sampling(request).await,
            _ => Ok(MCPResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: None,
                error: Some(ProtocolError {
                    code: -32601,
                    message: "Method not found".to_string(),
                    data: None,
                }),
            }),
        };

        response.map(Some)
    }

    /// Handle initialize request
    async fn handle_initialize(&self, request: MCPRequest) -> Result<MCPResponse, AppError> {
        tracing::info!("Handling initialize request");

        Ok(MCPResponse {
            jsonrpc: "2.0".to_string(),
            id: request.id,
            result: Some(json!({
                "protocolVersion": protocol::PROTOCOL_VERSION,
                "capabilities": {
                    "tools": {
                        "listChanged": true
                    },
                    "resources": {
                        "subscribe": true
                    },
                    "sampling": true
                },
                "serverInfo": {
                    "name": "ifm-ruta-mcp",
                    "version": "1.0.0"
                }
            })),
            error: None,
        })
    }

    /// Handle tools/list request
    async fn handle_tools_list(&self, request: MCPRequest) -> Result<MCPResponse, AppError> {
        tracing::info!("Listing available tools");

        let mut tools_list = Vec::new();

        // Include legacy tools
        let legacy_tools = self.tools.read().await;
        for tool in legacy_tools.values() {
            tools_list.push(json!({
                "name": tool.name(),
                "description": tool.description(),
                "inputSchema": tool.input_schema()
            }));
        }

        // Include async tools
        let async_tools = self.async_tools.read().await;
        for (_, tool) in async_tools.iter() {
            let metadata = tool.metadata();
            tools_list.push(json!({
                "name": metadata.name,
                "description": metadata.description,
                "inputSchema": metadata.input_schema
            }));
        }

        Ok(MCPResponse {
            jsonrpc: "2.0".to_string(),
            id: request.id,
            result: Some(json!({
                "tools": tools_list
            })),
            error: None,
        })
    }

    /// Handle tools/call request
    async fn handle_tool_call(&self, request: MCPRequest) -> Result<MCPResponse, AppError> {
        let params = request.params.unwrap_or(json!({}));
        let tool_name = params
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| AppError::InternalError(anyhow::anyhow!("Missing tool name")))?;

        let arguments = params.get("arguments").cloned().unwrap_or(json!({}));

        tracing::info!("Executing tool: {}", tool_name);

        // Try async tools first
        {
            let async_tools = self.async_tools.read().await;
            if let Some(tool) = async_tools.get(tool_name) {
                match tool.execute(arguments).await {
                    Ok(result) => {
                        return Ok(MCPResponse {
                            jsonrpc: "2.0".to_string(),
                            id: request.id,
                            result: Some(json!({
                                "content": [{
                                    "type": "text",
                                    "text": result.content
                                }]
                            })),
                            error: None,
                        });
                    }
                    Err(e) => {
                        return Ok(MCPResponse {
                            jsonrpc: "2.0".to_string(),
                            id: request.id,
                            result: None,
                            error: Some(ProtocolError {
                                code: -32603,
                                message: format!("Tool execution error: {}", e),
                                data: None,
                            }),
                        });
                    }
                }
            }
        }

        // Fall back to legacy tools
        {
            let legacy_tools = self.tools.read().await;
            if let Some(tool) = legacy_tools.get(tool_name) {
                match tool.execute(arguments) {
                    Ok(tool_result) => {
                        let result_json = serde_json::to_string(&tool_result)?;
                        return Ok(MCPResponse {
                            jsonrpc: "2.0".to_string(),
                            id: request.id,
                            result: Some(json!({
                                "content": [{
                                    "type": "text",
                                    "text": result_json
                                }]
                            })),
                            error: None,
                        });
                    }
                    Err(e) => {
                        return Ok(MCPResponse {
                            jsonrpc: "2.0".to_string(),
                            id: request.id,
                            result: None,
                            error: Some(ProtocolError {
                                code: -32603,
                                message: format!("Tool execution error: {}", e),
                                data: None,
                            }),
                        });
                    }
                }
            }
        }

        // Tool not found
        Ok(MCPResponse {
            jsonrpc: "2.0".to_string(),
            id: request.id,
            result: None,
            error: Some(ProtocolError {
                code: -32601,
                message: format!("Tool not found: {}", tool_name),
                data: None,
            }),
        })
    }

    /// Handle resources/list request (MCP 1.0)
    async fn handle_resources_list(&self, request: MCPRequest) -> Result<MCPResponse, AppError> {
        tracing::info!("Listing resources");

        Ok(MCPResponse {
            jsonrpc: "2.0".to_string(),
            id: request.id,
            result: Some(json!({
                "resources": []
            })),
            error: None,
        })
    }

    /// Handle sampling request (MCP 1.0)
    async fn handle_sampling(&self, request: MCPRequest) -> Result<MCPResponse, AppError> {
        tracing::warn!("Sampling request received but not implemented");

        Ok(MCPResponse {
            jsonrpc: "2.0".to_string(),
            id: request.id,
            result: None,
            error: Some(ProtocolError {
                code: -32603,
                message: "Sampling not configured".to_string(),
                data: None,
            }),
        })
    }

    /// Get count of registered tools
    pub async fn tool_count(&self) -> usize {
        let legacy = self.tools.read().await;
        let async_tools = self.async_tools.read().await;
        legacy.len() + async_tools.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_initialize_request() {
        // This test would require the full core/traits setup
        // Documented here for completeness
    }
}
