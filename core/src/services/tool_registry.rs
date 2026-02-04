//! Tool registry for managing multiple tools dynamically

use crate::traits::async_tool::{AsyncTool, MCPError, MCPResult, StreamingTool, ToolMetadata};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Tool registry for managing async tools
pub struct ToolRegistry {
    tools: Arc<RwLock<HashMap<String, Arc<dyn AsyncTool>>>>,
    streaming_tools: Arc<RwLock<HashMap<String, Arc<dyn StreamingTool>>>>,
}

impl ToolRegistry {
    /// Create a new tool registry
    pub fn new() -> Self {
        Self {
            tools: Arc::new(RwLock::new(HashMap::new())),
            streaming_tools: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register an async tool
    pub async fn register_tool(
        &self,
        name: impl Into<String>,
        tool: Arc<dyn AsyncTool>,
    ) -> MCPResult<()> {
        let name = name.into();
        let mut tools = self.tools.write().await;

        if tools.contains_key(&name) {
            return Err(MCPError::InvalidParams(format!(
                "Tool '{}' already registered",
                name
            )));
        }

        tools.insert(name, tool);
        Ok(())
    }

    /// Register a streaming tool
    pub async fn register_streaming_tool(
        &self,
        name: impl Into<String>,
        tool: Arc<dyn StreamingTool>,
    ) -> MCPResult<()> {
        let name = name.into();
        let mut tools = self.streaming_tools.write().await;

        if tools.contains_key(&name) {
            return Err(MCPError::InvalidParams(format!(
                "Streaming tool '{}' already registered",
                name
            )));
        }

        tools.insert(name, tool);
        Ok(())
    }

    /// Get a registered async tool
    pub async fn get_tool(&self, name: &str) -> MCPResult<Arc<dyn AsyncTool>> {
        let tools = self.tools.read().await;
        tools
            .get(name)
            .cloned()
            .ok_or_else(|| MCPError::NotFound(format!("Tool '{}' not found", name)))
    }

    /// Get a registered streaming tool
    pub async fn get_streaming_tool(&self, name: &str) -> MCPResult<Arc<dyn StreamingTool>> {
        let tools = self.streaming_tools.read().await;
        tools
            .get(name)
            .cloned()
            .ok_or_else(|| MCPError::NotFound(format!("Streaming tool '{}' not found", name)))
    }

    /// List all available tools (async and streaming)
    pub async fn list_tools(&self) -> MCPResult<Vec<ToolMetadata>> {
        let mut metadata = Vec::new();

        // Get async tools
        let async_tools = self.tools.read().await;
        for tool in async_tools.values() {
            metadata.push(tool.metadata());
        }

        // Get streaming tools
        let streaming_tools = self.streaming_tools.read().await;
        for tool in streaming_tools.values() {
            metadata.push(tool.metadata());
        }

        Ok(metadata)
    }

    /// Get tool metadata by name
    pub async fn get_tool_metadata(&self, name: &str) -> MCPResult<ToolMetadata> {
        // Try async tools first
        let async_tools = self.tools.read().await;
        if let Some(tool) = async_tools.get(name) {
            return Ok(tool.metadata());
        }

        // Try streaming tools
        drop(async_tools);
        let streaming_tools = self.streaming_tools.read().await;
        if let Some(tool) = streaming_tools.get(name) {
            return Ok(tool.metadata());
        }

        Err(MCPError::NotFound(format!("Tool '{}' not found", name)))
    }

    /// Check if a tool is registered
    pub async fn has_tool(&self, name: &str) -> bool {
        let async_tools = self.tools.read().await;
        if async_tools.contains_key(name) {
            return true;
        }

        drop(async_tools);
        let streaming_tools = self.streaming_tools.read().await;
        streaming_tools.contains_key(name)
    }

    /// Unregister an async tool
    pub async fn unregister_tool(&self, name: &str) -> MCPResult<()> {
        let mut tools = self.tools.write().await;
        tools
            .remove(name)
            .ok_or_else(|| MCPError::NotFound(format!("Tool '{}' not found", name)))?;
        Ok(())
    }

    /// Unregister a streaming tool
    pub async fn unregister_streaming_tool(&self, name: &str) -> MCPResult<()> {
        let mut tools = self.streaming_tools.write().await;
        tools
            .remove(name)
            .ok_or_else(|| MCPError::NotFound(format!("Tool '{}' not found", name)))?;
        Ok(())
    }

    /// Clear all registered tools
    pub async fn clear(&self) {
        self.tools.write().await.clear();
        self.streaming_tools.write().await.clear();
    }

    /// Get count of registered tools
    pub async fn tool_count(&self) -> usize {
        let async_count = self.tools.read().await.len();
        let streaming_count = self.streaming_tools.read().await.len();
        async_count + streaming_count
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::async_tool::ToolResponse;
    use async_trait::async_trait;
    use serde_json::Value;

    struct MockTool;

    #[async_trait]
    impl AsyncTool for MockTool {
        async fn execute(&self, _args: Value) -> MCPResult<ToolResponse> {
            Ok(ToolResponse {
                content: "mock result".to_string(),
                is_error: false,
            })
        }

        fn metadata(&self) -> ToolMetadata {
            ToolMetadata {
                name: "mock_tool".to_string(),
                description: "A mock tool for testing".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "message": {"type": "string"}
                    }
                }),
                version: "1.0.0".to_string(),
            }
        }
    }

    #[tokio::test]
    async fn test_register_tool() {
        let registry = ToolRegistry::new();
        let tool = Arc::new(MockTool);

        let result = registry.register_tool("mock_tool", tool).await;
        assert!(result.is_ok());

        // Try registering again - should fail
        let tool = Arc::new(MockTool);
        let result = registry.register_tool("mock_tool", tool).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_tool() {
        let registry = ToolRegistry::new();
        let tool = Arc::new(MockTool);

        registry.register_tool("mock_tool", tool).await.unwrap();
        let retrieved = registry.get_tool("mock_tool").await;

        assert!(retrieved.is_ok());
    }

    #[tokio::test]
    async fn test_get_nonexistent_tool() {
        let registry = ToolRegistry::new();
        let result = registry.get_tool("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_tools() {
        let registry = ToolRegistry::new();
        let tool = Arc::new(MockTool);

        registry.register_tool("mock_tool", tool).await.unwrap();
        let tools = registry.list_tools().await.unwrap();

        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0].name, "mock_tool");
    }

    #[tokio::test]
    async fn test_has_tool() {
        let registry = ToolRegistry::new();
        let tool = Arc::new(MockTool);

        registry.register_tool("mock_tool", tool).await.unwrap();
        assert!(registry.has_tool("mock_tool").await);
        assert!(!registry.has_tool("nonexistent").await);
    }

    #[tokio::test]
    async fn test_unregister_tool() {
        let registry = ToolRegistry::new();
        let tool = Arc::new(MockTool);

        registry.register_tool("mock_tool", tool).await.unwrap();
        assert!(registry.has_tool("mock_tool").await);

        registry.unregister_tool("mock_tool").await.unwrap();
        assert!(!registry.has_tool("mock_tool").await);
    }

    #[tokio::test]
    async fn test_tool_count() {
        let registry = ToolRegistry::new();
        assert_eq!(registry.tool_count().await, 0);

        let tool = Arc::new(MockTool);
        registry.register_tool("tool1", tool.clone()).await.unwrap();
        assert_eq!(registry.tool_count().await, 1);

        registry.register_tool("tool2", tool.clone()).await.unwrap();
        assert_eq!(registry.tool_count().await, 2);
    }

    #[tokio::test]
    async fn test_clear_registry() {
        let registry = ToolRegistry::new();
        let tool = Arc::new(MockTool);

        registry.register_tool("tool1", tool.clone()).await.unwrap();
        registry.register_tool("tool2", tool.clone()).await.unwrap();

        assert_eq!(registry.tool_count().await, 2);
        registry.clear().await;
        assert_eq!(registry.tool_count().await, 0);
    }

    #[tokio::test]
    async fn test_get_tool_metadata() {
        let registry = ToolRegistry::new();
        let tool = Arc::new(MockTool);

        registry.register_tool("mock_tool", tool).await.unwrap();
        let metadata = registry.get_tool_metadata("mock_tool").await.unwrap();

        assert_eq!(metadata.name, "mock_tool");
        assert_eq!(metadata.version, "1.0.0");
    }
}
