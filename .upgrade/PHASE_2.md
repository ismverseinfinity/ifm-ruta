# Phase 2: Core Features (Weeks 5-8)

**Objective**: Implement streaming responses, multi-tool framework, and sampling/cost management

**Duration**: 4 weeks  
**Risk Level**: High (core architecture changes)  
**Success Gate**: Streaming works end-to-end, multi-tool registration functional

---

## 📋 Overview

Phase 2 focuses on three major features:
1. **Streaming Responses** - Real-time feedback streaming
2. **Multi-Tool Framework** - Support multiple tools dynamically
3. **Sampling & Cost Management** - Agent cost control

---

## 2.1 Implement Streaming Responses

**Goal**: Support real-time streaming of tool outputs to clients

### Task 2.1.1: Streaming Infrastructure

**Timeline**: 5 days  
**Owner**: Lead Developer  
**Files**: `unified/src/mcp/streaming.rs` (NEW)

```rust
// unified/src/mcp/streaming.rs

use futures::stream::{Stream, StreamExt};
use serde_json::Value;
use std::pin::Pin;
use tokio::io::AsyncWrite;

/// Wrapper for streaming tool responses
pub struct StreamingResponse {
    id: Option<Value>,
    stream: Box<dyn Stream<Item = Result<String, String>> + Send + Unpin>,
}

impl StreamingResponse {
    pub fn new(
        id: Option<Value>,
        stream: Box<dyn Stream<Item = Result<String, String>> + Send + Unpin>,
    ) -> Self {
        Self { id, stream }
    }
    
    /// Send all chunks from stream to writer
    pub async fn send_all<W: AsyncWrite + Unpin>(
        mut self,
        writer: &mut W,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut chunk_index = 0;
        
        while let Some(result) = self.stream.next().await {
            match result {
                Ok(chunk) => {
                    self.send_chunk(writer, chunk_index, &chunk).await?;
                    chunk_index += 1;
                }
                Err(e) => {
                    self.send_error(writer, &e).await?;
                    return Err(e.into());
                }
            }
        }
        
        self.send_completion(writer).await?;
        Ok(())
    }
    
    async fn send_chunk<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        index: u32,
        chunk: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let response = serde_json::json!({
            "jsonrpc": "2.0",
            "id": self.id,
            "result": {
                "type": "stream_chunk",
                "index": index,
                "content": chunk
            }
        });
        
        let json = serde_json::to_string(&response)?;
        writer.write_all(json.as_bytes()).await?;
        writer.write_all(b"\n").await?;
        writer.flush().await?;
        
        Ok(())
    }
    
    async fn send_error<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        error: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let response = serde_json::json!({
            "jsonrpc": "2.0",
            "id": self.id,
            "error": {
                "code": -32603,
                "message": error
            }
        });
        
        let json = serde_json::to_string(&response)?;
        writer.write_all(json.as_bytes()).await?;
        writer.write_all(b"\n").await?;
        writer.flush().await?;
        
        Ok(())
    }
    
    async fn send_completion<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let response = serde_json::json!({
            "jsonrpc": "2.0",
            "id": self.id,
            "result": {
                "type": "stream_complete"
            }
        });
        
        let json = serde_json::to_string(&response)?;
        writer.write_all(json.as_bytes()).await?;
        writer.write_all(b"\n").await?;
        writer.flush().await?;
        
        Ok(())
    }
}

/// Builder for streaming responses
pub struct StreamingResponseBuilder {
    id: Option<Value>,
}

impl StreamingResponseBuilder {
    pub fn new(id: Option<Value>) -> Self {
        Self { id }
    }
    
    pub fn with_stream(
        self,
        stream: Box<dyn Stream<Item = Result<String, String>> + Send + Unpin>,
    ) -> StreamingResponse {
        StreamingResponse::new(self.id, stream)
    }
}
```

**Checklist**:
- [ ] Create StreamingResponse struct
- [ ] Implement chunk sending
- [ ] Implement error handling
- [ ] Implement completion signaling
- [ ] Add builder pattern
- [ ] Add comprehensive tests

---

### Task 2.1.2: Convert interactive_feedback to Streaming

**Timeline**: 4 days  
**Owner**: Backend Developer  
**Files**: `unified/src/tools/interactive_feedback.rs`

**Changes**:
```rust
// unified/src/tools/interactive_feedback.rs

use async_trait::async_trait;
use futures::stream::Stream;
use serde_json::Value;
use crate::core::traits::{StreamingTool, ToolMetadata, MCPResult, ToolStream};

pub struct InteractiveFeedbackTool {
    // Tool state
}

#[async_trait]
impl StreamingTool for InteractiveFeedbackTool {
    async fn execute_streaming(&self, args: Value) -> MCPResult<ToolStream> {
        // Validate inputs
        self.validate_inputs(&args)?;
        
        // Parse arguments
        let project_dir = args["projectDirectory"]
            .as_str()
            .ok_or(MCPError::InvalidParams("Missing projectDirectory".into()))?;
        let prompt = args["prompt"]
            .as_str()
            .ok_or(MCPError::InvalidParams("Missing prompt".into()))?;
        
        // Create streaming response
        let stream = self.create_feedback_stream(project_dir, prompt).await?;
        
        Ok(stream)
    }
    
    fn metadata(&self) -> ToolMetadata {
        ToolMetadata {
            name: "interactive_feedback".to_string(),
            description: "Request interactive feedback for a project".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "projectDirectory": {"type": "string"},
                    "prompt": {"type": "string"},
                    "previousUserRequest": {"type": "string"}
                },
                "required": ["projectDirectory", "prompt"]
            }),
            version: "1.0.0".to_string(),
        }
    }
    
    fn validate_inputs(&self, args: &Value) -> MCPResult<()> {
        if !args.get("projectDirectory").map(|v| v.is_string()).unwrap_or(false) {
            return Err(MCPError::InvalidParams("projectDirectory must be string".into()));
        }
        if !args.get("prompt").map(|v| v.is_string()).unwrap_or(false) {
            return Err(MCPError::InvalidParams("prompt must be string".into()));
        }
        Ok(())
    }
}

impl InteractiveFeedbackTool {
    async fn create_feedback_stream(
        &self,
        project_dir: &str,
        prompt: &str,
    ) -> MCPResult<ToolStream> {
        let project_dir = project_dir.to_string();
        let prompt = prompt.to_string();
        
        let stream = async_stream::stream! {
            // Yield initial status
            yield Ok("Starting interactive feedback...".to_string());
            
            // Load conversation history
            yield Ok("Loading conversation history...".to_string());
            
            // Get user feedback
            yield Ok("Waiting for user feedback...".to_string());
            
            // Process feedback
            yield Ok("Processing feedback...".to_string());
            
            // Return results
            yield Ok("Feedback received successfully".to_string());
        };
        
        Ok(Box::pin(stream))
    }
}
```

**Dependencies to Add**:
```toml
async-stream = "0.1"
```

**Checklist**:
- [ ] Implement StreamingTool trait
- [ ] Convert to async streaming
- [ ] Test end-to-end streaming
- [ ] Add proper error handling
- [ ] Document streaming behavior
- [ ] Performance test streaming

---

## 2.2 Multi-Tool Framework

**Goal**: Support dynamic tool registration and discovery

### Task 2.2.1: Tool Registry

**Timeline**: 5 days  
**Owner**: Lead Developer  
**Files**: `core/src/services/tool_registry.rs` (NEW)

```rust
// core/src/services/tool_registry.rs

use async_trait::async_trait;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::core::traits::{AsyncTool, StreamingTool, ToolMetadata, MCPResult, MCPError};

/// Registry for managing multiple tools
pub struct ToolRegistry {
    tools: Arc<RwLock<HashMap<String, Arc<dyn AsyncTool>>>>,
    streaming_tools: Arc<RwLock<HashMap<String, Arc<dyn StreamingTool>>>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: Arc::new(RwLock::new(HashMap::new())),
            streaming_tools: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Register a non-streaming tool
    pub async fn register_tool(
        &self,
        name: impl Into<String>,
        tool: Arc<dyn AsyncTool>,
    ) {
        let name = name.into();
        let mut tools = self.tools.write().await;
        tools.insert(name, tool);
    }
    
    /// Register a streaming tool
    pub async fn register_streaming_tool(
        &self,
        name: impl Into<String>,
        tool: Arc<dyn StreamingTool>,
    ) {
        let name = name.into();
        let mut tools = self.streaming_tools.write().await;
        tools.insert(name, tool);
    }
    
    /// Execute non-streaming tool
    pub async fn execute(
        &self,
        name: &str,
        args: Value,
    ) -> MCPResult<Value> {
        let tools = self.tools.read().await;
        let tool = tools.get(name)
            .ok_or_else(|| MCPError::ToolNotFound(name.to_string()))?;
        
        let response = tool.execute(args).await?;
        Ok(serde_json::to_value(response)?)
    }
    
    /// Execute streaming tool
    pub async fn execute_streaming(
        &self,
        name: &str,
        args: Value,
    ) -> MCPResult<crate::core::traits::ToolStream> {
        let tools = self.streaming_tools.read().await;
        let tool = tools.get(name)
            .ok_or_else(|| MCPError::ToolNotFound(name.to_string()))?;
        
        tool.execute_streaming(args).await
    }
    
    /// List all available tools
    pub async fn list_tools(&self) -> MCPResult<Vec<ToolMetadata>> {
        let mut metadata = Vec::new();
        
        let tools = self.tools.read().await;
        for tool in tools.values() {
            metadata.push(tool.metadata());
        }
        
        let streaming = self.streaming_tools.read().await;
        for tool in streaming.values() {
            metadata.push(tool.metadata());
        }
        
        Ok(metadata)
    }
    
    /// Get specific tool metadata
    pub async fn get_tool(&self, name: &str) -> MCPResult<ToolMetadata> {
        let tools = self.tools.read().await;
        if let Some(tool) = tools.get(name) {
            return Ok(tool.metadata());
        }
        
        let streaming = self.streaming_tools.read().await;
        if let Some(tool) = streaming.get(name) {
            return Ok(tool.metadata());
        }
        
        Err(MCPError::ToolNotFound(name.to_string()))
    }
    
    /// Check if tool exists
    pub async fn has_tool(&self, name: &str) -> bool {
        let tools = self.tools.read().await;
        if tools.contains_key(name) {
            return true;
        }
        
        let streaming = self.streaming_tools.read().await;
        streaming.contains_key(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_register_and_list_tools() {
        let registry = ToolRegistry::new();
        
        // TODO: Create mock tool and register
        
        let tools = registry.list_tools().await.unwrap();
        assert!(!tools.is_empty());
    }
}
```

**Checklist**:
- [ ] Create ToolRegistry struct
- [ ] Implement tool registration
- [ ] Implement tool execution
- [ ] Implement tool discovery
- [ ] Add thread-safe operations
- [ ] Add comprehensive tests

---

### Task 2.2.2: Refactor Existing Tools

**Timeline**: 4 days  
**Owner**: Backend Developer

**Changes**:
- [ ] Convert InteractiveFeedbackTool to implement AsyncTool/StreamingTool
- [ ] Add proper error handling
- [ ] Implement metadata methods
- [ ] Add input validation
- [ ] Document tool interface

**Checklist**:
- [ ] All tools implement proper traits
- [ ] Metadata complete for all tools
- [ ] Validation working
- [ ] Tests passing

---

### Task 2.2.3: Tool Examples & Documentation

**Timeline**: 3 days  
**Owner**: Backend Developer  
**Files**: `examples/custom_tools/` (NEW)

**Example tool structure**:
```rust
// examples/custom_tools/echo_tool.rs

use async_trait::async_trait;
use serde_json::Value;
use ifm_ruta_core::traits::{AsyncTool, ToolMetadata, ToolResponse, MCPResult};

pub struct EchoTool;

#[async_trait]
impl AsyncTool for EchoTool {
    async fn execute(&self, args: Value) -> MCPResult<ToolResponse> {
        let message = args["message"]
            .as_str()
            .ok_or("Missing message")?;
        
        Ok(ToolResponse {
            content: format!("Echo: {}", message),
            is_error: false,
        })
    }
    
    fn metadata(&self) -> ToolMetadata {
        ToolMetadata {
            name: "echo".to_string(),
            description: "Echo back the input message".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "message": {"type": "string"}
                },
                "required": ["message"]
            }),
            version: "1.0.0".to_string(),
        }
    }
}
```

**Documentation**:
- [ ] Create TOOL_DEVELOPMENT.md guide
- [ ] Add code templates
- [ ] Document best practices
- [ ] Add streaming examples

---

## 2.3 Sampling & Cost Management

**Goal**: Support agent cost control and optimization

### Task 2.3.1: Sampling Handler

**Timeline**: 4 days  
**Owner**: Lead Developer  
**Files**: `unified/src/mcp/sampling.rs` (NEW)

```rust
// unified/src/mcp/sampling.rs

use async_trait::async_trait;
use serde_json::Value;
use crate::mcp::protocol::{SamplingRequest, SamplingResponse, SamplingMessage};

/// Sampling handler for AI model calls
pub struct SamplingHandler {
    // Optional: Anthropic API client
    // client: Option<AnthropicClient>,
}

impl SamplingHandler {
    pub fn new() -> Self {
        Self {}
    }
    
    /// Handle sampling request from client
    pub async fn handle_sampling(
        &self,
        request: SamplingRequest,
    ) -> Result<SamplingResponse, String> {
        // Validate request
        self.validate_sampling_request(&request)?;
        
        // For now, return error indicating sampling not configured
        // In future: integrate with Anthropic SDK
        Err("Sampling not configured".to_string())
    }
    
    fn validate_sampling_request(&self, request: &SamplingRequest) -> Result<(), String> {
        if request.model.is_empty() {
            return Err("Model name required".to_string());
        }
        
        if request.max_tokens == 0 {
            return Err("Max tokens must be > 0".to_string());
        }
        
        if request.messages.is_empty() {
            return Err("At least one message required".to_string());
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_validate_sampling_request() {
        let handler = SamplingHandler::new();
        
        let invalid_request = SamplingRequest {
            model: String::new(),
            max_tokens: 100,
            system: None,
            messages: vec![SamplingMessage {
                role: "user".to_string(),
                content: "test".to_string(),
            }],
        };
        
        assert!(handler.validate_sampling_request(&invalid_request).is_err());
    }
}
```

**Checklist**:
- [ ] Create SamplingHandler struct
- [ ] Implement request validation
- [ ] Implement sampling logic
- [ ] Add error handling
- [ ] Add tests

---

### Task 2.3.2: Metrics & Logging

**Timeline**: 3 days  
**Owner**: Backend Developer  
**Files**: `core/src/services/metrics.rs` (NEW)

```rust
// core/src/services/metrics.rs

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Tool execution metrics
pub struct ToolMetrics {
    call_count: AtomicU64,
    total_duration: parking_lot::Mutex<Duration>,
    error_count: AtomicU64,
}

impl ToolMetrics {
    pub fn new() -> Self {
        Self {
            call_count: AtomicU64::new(0),
            total_duration: parking_lot::Mutex::new(Duration::ZERO),
            error_count: AtomicU64::new(0),
        }
    }
    
    /// Record successful execution
    pub fn record_success(&self, duration: Duration) {
        self.call_count.fetch_add(1, Ordering::Relaxed);
        let mut total = self.total_duration.lock();
        *total += duration;
    }
    
    /// Record failed execution
    pub fn record_error(&self) {
        self.error_count.fetch_add(1, Ordering::Relaxed);
    }
    
    /// Get metrics
    pub fn get_stats(&self) -> MetricsStats {
        let call_count = self.call_count.load(Ordering::Relaxed);
        let error_count = self.error_count.load(Ordering::Relaxed);
        let total_duration = *self.total_duration.lock();
        
        let avg_duration = if call_count > 0 {
            total_duration / call_count as u32
        } else {
            Duration::ZERO
        };
        
        MetricsStats {
            call_count,
            error_count,
            success_count: call_count - error_count,
            total_duration,
            average_duration: avg_duration,
        }
    }
}

pub struct MetricsStats {
    pub call_count: u64,
    pub error_count: u64,
    pub success_count: u64,
    pub total_duration: Duration,
    pub average_duration: Duration,
}
```

**Checklist**:
- [ ] Create metrics collection struct
- [ ] Implement counter logic
- [ ] Implement duration tracking
- [ ] Add retrieval methods
- [ ] Add tests

---

## Summary & Validation

### Deliverables
- [ ] Streaming infrastructure implemented
- [ ] interactive_feedback tool converted to streaming
- [ ] Tool registry functional
- [ ] Multi-tool support working
- [ ] Sampling handler implemented
- [ ] Metrics framework created

### Testing Checklist
- [ ] Streaming end-to-end tests pass
- [ ] Tool registration tests pass
- [ ] Tool execution tests pass
- [ ] Metrics collection tests pass
- [ ] Integration tests pass

### Performance Targets
- [ ] Streaming latency < 100ms
- [ ] Tool execution < 1s (per call)
- [ ] Registry lookup O(1)
- [ ] Memory usage < 50MB

### Code Quality
- [ ] All public APIs documented
- [ ] Error handling comprehensive
- [ ] Logging throughout
- [ ] No unsafe code (except async-stream)
- [ ] All tests passing

### Next Phase Gate
**Proceed to Phase 3 when**:
- ✅ Streaming works end-to-end
- ✅ Multi-tool registration functional
- ✅ All tests passing
- ✅ Performance targets met

---

## References

- [Futures Streams](https://docs.rs/futures/latest/futures/stream/)
- [Tokio RwLock](https://tokio.rs/tokio/tutorial/shared-state)
- [Async Trait](https://github.com/dtolnay/async-trait)

---

**Created**: 2026-02-04  
**Last Updated**: 2026-02-04
