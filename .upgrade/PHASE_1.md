# Phase 1: Foundation & Protocol Update (Weeks 1-4)

**Objective**: Update MCP protocol from 0.1.0 to 1.0+ specification and establish async foundation

**Duration**: 4 weeks  
**Risk Level**: Medium (protocol breaking changes)  
**Success Gate**: Code compiles, basic tests pass

---

## 📋 Overview

Phase 1 focuses on three core areas:
1. **MCP Protocol Upgrade** - Update to MCP 1.0 spec
2. **Async Architecture** - Foundation for streaming & concurrent operations
3. **Input Validation** - JSON schema validation framework

---

## 1.1 Upgrade MCP Protocol

**Goal**: Migrate from MCP 0.1.0 to 1.0+ specification

### Task 1.1.1: Research MCP 1.0 Specification
**Timeline**: 3 days  
**Owner**: Lead Developer

**Checklist**:
- [ ] Read official MCP 1.0 documentation
- [ ] Analyze breaking changes from previous version
- [ ] Identify new capabilities to implement:
  - [ ] Resource management endpoints
  - [ ] Prompt templates
  - [ ] Server-side caching directives
  - [ ] Sampling/cost management
- [ ] Document findings in research note
- [ ] Create breaking changes checklist

**Deliverables**:
- Research document (MCP_1.0_RESEARCH.md)
- Breaking changes list
- Implementation roadmap

---

### Task 1.1.2: Update Dependencies

**Timeline**: 2 days  
**Owner**: Backend Developer

**Changes Required**:

```toml
# Cargo.toml workspace.dependencies
[workspace.dependencies]
# Core dependencies
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.0", features = ["rt", "rt-multi-thread", "macros", "sync", "process", "io-util", "io-std"] }
anyhow = "1.0"
thiserror = "1.0"
uuid = { version = "1.0", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
dirs = "5.0"
toml = "0.8"

# MCP dependencies - UPDATE
mcp = "1.0"  # From "0.1.0"

# Async support - ADD
async-trait = "0.1"
futures = { version = "0.3", features = ["io-compat"] }
pin-project = "1.1"

# GUI dependencies
eframe = "0.28"
egui = "0.28"

# Validation - ADD
jsonschema = "0.16"

# Process management
sysinfo = "0.30"

# Logging
tracing = "0.1"
tracing-subscriber = "0.3"

# Serialization
serde_yaml = "0.9"
```

**Steps**:
- [ ] Update `Cargo.toml` with new versions
- [ ] Run `cargo update`
- [ ] Verify all dependencies resolve
- [ ] Check for security advisories: `cargo audit`
- [ ] Document any version constraints needed

**Verification**:
```bash
cargo check --workspace
cargo build --workspace
```

---

### Task 1.1.3: Refactor Protocol Types

**Timeline**: 5 days  
**Owner**: Lead Developer  
**Files**: `unified/src/mcp/protocol.rs`

**Objective**: Add new types required by MCP 1.0 spec

#### New Types to Add

```rust
// unified/src/mcp/protocol.rs

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// MCP Protocol version
pub const PROTOCOL_VERSION: &str = "1.0.0";

/// Client capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caching: Option<bool>,
}

/// Server capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<ToolsCapability>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<ResourceCapability>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsCapability {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_changed: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceCapability {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe: Option<bool>,
}

/// Initialize request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitializeRequest {
    pub protocol_version: String,
    pub capabilities: ClientCapabilities,
    pub client_info: ClientInfo,
}

/// Client information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientInfo {
    pub name: String,
    pub version: String,
}

/// Initialize response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitializeResponse {
    pub protocol_version: String,
    pub capabilities: ServerCapabilities,
    pub server_info: ServerInfo,
}

/// Server information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    pub name: String,
    pub version: String,
}

/// Resource request (new in 1.0)
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

/// Sampling request (new in 1.0)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SamplingRequest {
    pub model: String,
    pub max_tokens: u32,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    
    pub messages: Vec<SamplingMessage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SamplingMessage {
    pub role: String,  // "user" or "assistant"
    pub content: String,
}

/// Sampling response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SamplingResponse {
    pub model: String,
    pub content: String,
    pub stop_reason: String,
}

/// Tool list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolListResponse {
    pub tools: Vec<ToolDefinition>,
}

/// Tool definition with full schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Schema,
}

/// MCP Error (enhanced)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

impl MCPError {
    pub fn new(code: i32, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            data: None,
        }
    }
    
    pub fn with_data(code: i32, message: impl Into<String>, data: Value) -> Self {
        Self {
            code,
            message: message.into(),
            data: Some(data),
        }
    }
    
    // Standard error codes
    pub fn parse_error() -> Self {
        Self::new(-32700, "Parse error")
    }
    
    pub fn invalid_request() -> Self {
        Self::new(-32600, "Invalid Request")
    }
    
    pub fn method_not_found() -> Self {
        Self::new(-32601, "Method not found")
    }
    
    pub fn invalid_params() -> Self {
        Self::new(-32602, "Invalid params")
    }
    
    pub fn internal_error() -> Self {
        Self::new(-32603, "Internal error")
    }
}

/// MCP Request (updated)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPRequest {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Value>,
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
}

impl MCPRequest {
    pub fn new(method: impl Into<String>, params: Option<Value>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: None,
            method: method.into(),
            params,
        }
    }
}

/// MCP Response (updated)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPResponse {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<MCPError>,
}

impl MCPResponse {
    pub fn success(id: Option<Value>, result: Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(result),
            error: None,
        }
    }
    
    pub fn error(id: Option<Value>, error: MCPError) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: None,
            error: Some(error),
        }
    }
}

/// Tool result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub content: String,
    pub is_error: bool,
}
```

**Checklist**:
- [ ] Add all new types above
- [ ] Update existing MCPRequest/MCPResponse
- [ ] Add helper methods (constructors, constants)
- [ ] Update module exports in `mod.rs`
- [ ] Verify serialization/deserialization roundtrips
- [ ] Add unit tests for new types

**Tests to Add**:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_serialize_initialize_request() {
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
    }
    
    #[test]
    fn test_mcp_error_codes() {
        assert_eq!(MCPError::parse_error().code, -32700);
        assert_eq!(MCPError::invalid_request().code, -32600);
        assert_eq!(MCPError::method_not_found().code, -32601);
    }
}
```

---

## 1.2 Design Async Architecture

**Goal**: Foundation for streaming & concurrent operations

### Task 1.2.1: Create Async Traits

**Timeline**: 4 days  
**Owner**: Lead Developer  
**Files**: `core/src/traits/async_tool.rs` (NEW)

```rust
// core/src/traits/async_tool.rs

use async_trait::async_trait;
use serde_json::Value;
use std::pin::Pin;
use futures::stream::Stream;

/// Result type for MCP operations
pub type MCPResult<T> = Result<T, MCPError>;

/// Streaming data from tools
pub type ToolStream = Pin<Box<dyn Stream<Item = MCPResult<String>> + Send>>;

/// Tool metadata
#[derive(Debug, Clone)]
pub struct ToolMetadata {
    pub name: String,
    pub description: String,
    pub input_schema: Value,  // JSON Schema
    pub version: String,
}

/// Tool response
#[derive(Debug, Clone)]
pub struct ToolResponse {
    pub content: String,
    pub is_error: bool,
}

/// Async tool trait - non-streaming execution
#[async_trait]
pub trait AsyncTool: Send + Sync {
    /// Execute tool with given arguments
    async fn execute(&self, args: Value) -> MCPResult<ToolResponse>;
    
    /// Get tool metadata
    fn metadata(&self) -> ToolMetadata;
    
    /// Optional: Validate inputs before execution
    fn validate_inputs(&self, args: &Value) -> MCPResult<()> {
        Ok(())
    }
}

/// Streaming tool trait - streaming execution
#[async_trait]
pub trait StreamingTool: Send + Sync {
    /// Execute tool and return streaming responses
    async fn execute_streaming(&self, args: Value) -> MCPResult<ToolStream>;
    
    /// Get tool metadata
    fn metadata(&self) -> ToolMetadata;
    
    /// Optional: Validate inputs before execution
    fn validate_inputs(&self, args: &Value) -> MCPResult<()> {
        Ok(())
    }
}

/// Tool executor abstraction
#[async_trait]
pub trait ToolExecutor: Send + Sync {
    /// Execute tool by name
    async fn execute(
        &self,
        name: &str,
        args: Value,
    ) -> MCPResult<ToolResponse>;
    
    /// Execute tool with streaming
    async fn execute_streaming(
        &self,
        name: &str,
        args: Value,
    ) -> MCPResult<ToolStream>;
    
    /// List all available tools
    fn list_tools(&self) -> MCPResult<Vec<ToolMetadata>>;
    
    /// Get specific tool metadata
    fn get_tool(&self, name: &str) -> MCPResult<ToolMetadata>;
}

/// MCP Error type
#[derive(Debug)]
pub enum MCPError {
    ParseError(String),
    InvalidRequest(String),
    MethodNotFound(String),
    InvalidParams(String),
    InternalError(String),
    ToolNotFound(String),
    ValidationError(String),
    IOError(std::io::Error),
}

impl std::fmt::Display for MCPError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MCPError::ParseError(s) => write!(f, "Parse error: {}", s),
            MCPError::InvalidRequest(s) => write!(f, "Invalid request: {}", s),
            MCPError::MethodNotFound(s) => write!(f, "Method not found: {}", s),
            MCPError::InvalidParams(s) => write!(f, "Invalid params: {}", s),
            MCPError::InternalError(s) => write!(f, "Internal error: {}", s),
            MCPError::ToolNotFound(s) => write!(f, "Tool not found: {}", s),
            MCPError::ValidationError(s) => write!(f, "Validation error: {}", s),
            MCPError::IOError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl std::error::Error for MCPError {}
```

**Checklist**:
- [ ] Define result types and errors
- [ ] Create AsyncTool trait
- [ ] Create StreamingTool trait
- [ ] Create ToolExecutor abstraction
- [ ] Add comprehensive docs
- [ ] Export from `core/src/traits/mod.rs`

---

### Task 1.2.2: Refactor MCP Server

**Timeline**: 5 days  
**Owner**: Lead Developer  
**Files**: `unified/src/mcp/server.rs`

**Objective**: Convert from sync to async request handling

**Key Changes**:
- [ ] Setup Tokio runtime in main
- [ ] Convert all handlers to async functions
- [ ] Implement proper error handling
- [ ] Add logging throughout
- [ ] Support concurrent requests

**Structure**:
```rust
// unified/src/mcp/server.rs

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::Arc;

pub struct MCPServer {
    listener: TcpListener,
    executor: Arc<dyn ToolExecutor>,
}

impl MCPServer {
    pub async fn new(listener: TcpListener, executor: Arc<dyn ToolExecutor>) -> Self {
        Self { listener, executor }
    }
    
    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            let (socket, _) = self.listener.accept().await?;
            let executor = self.executor.clone();
            
            tokio::spawn(async move {
                if let Err(e) = Self::handle_connection(socket, executor).await {
                    eprintln!("Error handling connection: {}", e);
                }
            });
        }
    }
    
    async fn handle_connection(
        socket: tokio::net::TcpStream,
        executor: Arc<dyn ToolExecutor>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (reader, mut writer) = socket.into_split();
        let mut buf_reader = BufReader::new(reader);
        let mut line = String::new();
        
        while buf_reader.read_line(&mut line).await? > 0 {
            let request = Self::parse_request(&line)?;
            let response = Self::handle_request(request, &executor).await;
            
            let json = serde_json::to_string(&response)?;
            writer.write_all(json.as_bytes()).await?;
            writer.write_all(b"\n").await?;
            
            line.clear();
        }
        
        Ok(())
    }
    
    fn parse_request(line: &str) -> Result<MCPRequest, MCPError> {
        serde_json::from_str(line)
            .map_err(|e| MCPError::ParseError(e.to_string()))
    }
    
    async fn handle_request(
        request: MCPRequest,
        executor: &Arc<dyn ToolExecutor>,
    ) -> MCPResponse {
        match request.method.as_str() {
            "initialize" => Self::handle_initialize(request),
            "tools/list" => Self::handle_list_tools(request, executor),
            "tools/call" => Self::handle_tool_call(request, executor).await,
            _ => MCPResponse::error(
                request.id,
                protocol::MCPError::method_not_found(),
            ),
        }
    }
    
    // ... implement handlers
}
```

**Checklist**:
- [ ] Update server structure
- [ ] Implement async request handling
- [ ] Add connection management
- [ ] Implement method handlers
- [ ] Add comprehensive error handling
- [ ] Add logging with tracing
- [ ] Test with concurrent requests

---

## 1.3 Input Validation Framework

**Goal**: Comprehensive JSON schema validation

### Task 1.3.1: JSON Schema Support

**Timeline**: 4 days  
**Owner**: Backend Developer  
**Files**: `core/src/utils/validator.rs` (NEW)

```rust
// core/src/utils/validator.rs

use jsonschema::{JSONSchema, Draft};
use serde_json::{json, Value};

/// Validator for tool inputs
pub struct InputValidator {
    schemas: std::collections::HashMap<String, JSONSchema>,
}

impl InputValidator {
    pub fn new() -> Self {
        Self {
            schemas: std::collections::HashMap::new(),
        }
    }
    
    /// Register schema for a tool
    pub fn register_schema(
        &mut self,
        tool_name: &str,
        schema: Value,
    ) -> Result<(), jsonschema::error::CompileError> {
        let compiled = JSONSchema::compile(&schema, Some(Draft::Draft7))?;
        self.schemas.insert(tool_name.to_string(), compiled);
        Ok(())
    }
    
    /// Validate input against registered schema
    pub fn validate(&self, tool_name: &str, input: &Value) -> Result<(), String> {
        match self.schemas.get(tool_name) {
            Some(schema) => {
                schema.validate(input)
                    .map_err(|e| e.to_string())
            }
            None => Err(format!("No schema registered for tool: {}", tool_name)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validate_feedback_input() {
        let mut validator = InputValidator::new();
        let schema = json!({
            "type": "object",
            "properties": {
                "projectDirectory": {"type": "string"},
                "prompt": {"type": "string"},
                "previousUserRequest": {"type": "string"}
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
}
```

**Checklist**:
- [ ] Create validator struct
- [ ] Implement schema registration
- [ ] Implement validation logic
- [ ] Add error reporting
- [ ] Add unit tests
- [ ] Export from utils module

---

### Task 1.3.2: Define Tool Schemas

**Timeline**: 3 days  
**Owner**: Backend Developer  
**Files**: `unified/src/tools/schemas.rs` (NEW)

```rust
// unified/src/tools/schemas.rs

use serde_json::{json, Value};

/// Get JSON schema for interactive_feedback tool
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
                "description": "The prompt to show to the user"
            },
            "previousUserRequest": {
                "type": "string",
                "description": "The previous user request that triggered this feedback"
            }
        },
        "required": ["projectDirectory", "prompt", "previousUserRequest"],
        "additionalProperties": false
    })
}

/// Get output schema for tool results
pub fn tool_result_schema() -> Value {
    json!({
        "type": "object",
        "title": "ToolResult",
        "properties": {
            "content": {
                "type": "string",
                "description": "The output content"
            },
            "is_error": {
                "type": "boolean",
                "description": "Whether this is an error response"
            }
        },
        "required": ["content", "is_error"]
    })
}
```

**Checklist**:
- [ ] Define interactive_feedback schema
- [ ] Define tool result schema
- [ ] Define common constraint schemas
- [ ] Add schema documentation
- [ ] Add validation examples
- [ ] Test schema correctness

---

## Summary & Validation

### Deliverables
- [ ] Updated Cargo.toml with MCP 1.0
- [ ] Protocol types fully implemented
- [ ] Async traits defined
- [ ] Server refactored to async
- [ ] Validator framework created
- [ ] Tool schemas defined

### Testing Checklist
- [ ] `cargo test --workspace` passes
- [ ] `cargo clippy --workspace` has no warnings
- [ ] `cargo fmt --workspace` is clean
- [ ] Protocol serialization tests pass
- [ ] Validator tests pass

### Code Quality
- [ ] All public APIs documented
- [ ] Error handling consistent
- [ ] Logging added throughout
- [ ] No unsafe code
- [ ] All tests passing

### Next Phase Gate
**Proceed to Phase 2 when**:
- ✅ Code compiles without warnings
- ✅ All tests pass
- ✅ Basic protocol communication works
- ✅ Async traits functional

---

## References

- [MCP Protocol 1.0](https://modelcontextprotocol.io/)
- [JSON Schema Draft 7](https://json-schema.org/draft-07/)
- [Tokio Async Runtime](https://tokio.rs/)
- [Async Trait](https://github.com/dtolnay/async-trait)

---

**Created**: 2026-02-04  
**Last Updated**: 2026-02-04
