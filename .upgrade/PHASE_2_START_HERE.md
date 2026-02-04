# Phase 2 Quick Start - For Next Implementer

**Previous Status**: Phase 1 COMPLETE ✅  
**Current Phase**: Phase 2 - Core Features (Weeks 5-8)  
**Duration**: 4 weeks  
**What You're Building**: Streaming responses, multi-tool framework, sampling/metrics

---

## What Was Done in Phase 1

Phase 1 (Foundation & Protocol Update) is 100% complete:

✅ **MCP Protocol 1.0 Types** - All 9 types implemented in `unified/src/mcp/protocol.rs`
✅ **Async Architecture** - AsyncTool trait in `core/src/traits/async_tool.rs`
✅ **Input Validation** - JSON schema validator in `core/src/utils/validator.rs`
✅ **Server Refactored** - Async server in `unified/src/mcp/server.rs`
✅ **Tool Schemas** - All schemas in `unified/src/tools/schemas.rs`

**Key Files Modified**:
- `Cargo.toml` - Added async-trait, jsonschema, pin-project
- `unified/src/mcp/protocol.rs` - 9 new MCP 1.0 types
- `unified/src/mcp/server.rs` - Converted to async with tokio
- `core/src/traits/async_tool.rs` - NEW async tool traits
- `core/src/utils/validator.rs` - NEW input validation
- `unified/src/tools/schemas.rs` - NEW tool schemas

---

## Your Phase 2 Mission

### Task 2.1: Streaming Responses (5 days)
**File**: `unified/src/mcp/streaming.rs` (NEW)

What to build:
- `StreamingResponse` struct that wraps a futures::Stream
- Methods: `send_all()`, `send_chunk()`, `send_error()`, `send_completion()`
- JSON streaming protocol (stream_chunk messages)
- Builder pattern for easy construction

Example from PHASE_2.md task 2.1.1 has complete code.

### Task 2.2: Multi-Tool Framework (6 days)
**File**: `core/src/services/tool_registry.rs` (NEW)

What to build:
- `ToolRegistry` struct with RwLock for concurrent access
- Methods: `register_tool()`, `execute()`, `list_tools()`, `has_tool()`
- Support for both AsyncTool and legacy Tool traits
- Error handling and logging

Example from PHASE_2.md task 2.2.1 has complete code.

### Task 2.3: Sampling & Metrics (7 days)

**2.3.1 Sampling**: `unified/src/mcp/sampling.rs` (NEW)
- `SamplingHandler` for AI model calls
- Request validation
- Cost tracking preparation

**2.3.2 Metrics**: `core/src/services/metrics.rs` (NEW)
- `ToolMetrics` for tracking execution
- Count calls, errors, duration
- Stats aggregation

Examples in PHASE_2.md have complete code.

---

## Quick Checklist

### Before Starting
- [ ] Read `PHASE_2.md` completely (30 min)
- [ ] Review Phase 1 completion summary
- [ ] Verify Rust toolchain installed
- [ ] Run `cargo test --lib` to verify Phase 1

### Task 2.1 Checklist
- [ ] Create `unified/src/mcp/streaming.rs`
- [ ] Implement StreamingResponse struct
- [ ] Add chunk sending methods
- [ ] Create StreamingResponseBuilder
- [ ] Write unit tests
- [ ] Update `unified/src/mcp/mod.rs`

### Task 2.2 Checklist
- [ ] Create `core/src/services/mod.rs` (if not exists)
- [ ] Create `core/src/services/tool_registry.rs`
- [ ] Implement ToolRegistry struct
- [ ] Add register/execute/list/has methods
- [ ] Write comprehensive tests
- [ ] Update `core/src/lib.rs` exports

### Task 2.3 Checklist
- [ ] Create `unified/src/mcp/sampling.rs`
- [ ] Implement SamplingHandler
- [ ] Add validation methods
- [ ] Create `core/src/services/metrics.rs`
- [ ] Implement ToolMetrics
- [ ] Write tests for both

---

## Files You'll Create in Phase 2

```
unified/src/mcp/streaming.rs       (NEW) - StreamingResponse
core/src/services/tool_registry.rs (NEW) - ToolRegistry
unified/src/mcp/sampling.rs        (NEW) - SamplingHandler
core/src/services/metrics.rs       (NEW) - ToolMetrics
```

## Files You'll Modify in Phase 2

```
unified/src/mcp/mod.rs             - Add streaming module
core/src/lib.rs                    - Export services
core/src/services/mod.rs           - Create if needed
unified/src/tools/interactive_feedback.rs - Convert to streaming
```

---

## Key Imports You'll Need

```rust
// For streaming
use futures::stream::{Stream, StreamExt};
use tokio::io::AsyncWrite;

// For async traits
use async_trait::async_trait;

// For concurrency
use tokio::sync::RwLock;
use std::sync::Arc;

// For JSON
use serde_json::{json, Value};

// For logging
use tracing::{info, debug, error};
```

---

## Success Criteria for Phase 2

✅ Streaming works end-to-end
✅ Multi-tool registration functional
✅ Tools can execute concurrently
✅ Metrics are collected and reported
✅ All tests passing (50+ tests now)
✅ Zero clippy warnings
✅ Proper error handling

---

## Common Patterns from Phase 1 (Use These)

### Pattern 1: Async Trait Implementation
```rust
use async_trait::async_trait;

#[async_trait]
pub trait MyTrait: Send + Sync {
    async fn my_method(&self) -> Result<String>;
}
```

### Pattern 2: Concurrent Struct Storage
```rust
use tokio::sync::RwLock;
use std::sync::Arc;

pub struct MyManager {
    items: Arc<RwLock<HashMap<String, Item>>>,
}

// Usage:
let items = self.items.read().await;  // Read-lock
let mut items = self.items.write().await;  // Write-lock
```

### Pattern 3: Error Handling
```rust
use ifm_ruta_core::traits::MCPError;

fn validate(input: &Value) -> Result<(), MCPError> {
    if input.is_null() {
        return Err(MCPError::InvalidParams("input required".into()));
    }
    Ok(())
}
```

### Pattern 4: Logging
```rust
use tracing::{info, debug, error};

info!("Starting operation");
debug!("Processing item: {:?}", item);
error!("Failed: {}", err);
```

---

## Testing Strategy for Phase 2

Each task should have unit tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_feature() {
        // Test async functionality
    }

    #[test]
    fn test_sync_feature() {
        // Test sync functionality
    }
}
```

Target: 50+ total tests (20+ from Phase 1 + 30+ new)

---

## Documentation to Update

After Phase 2, update:
- `PHASE_2_PROGRESS.md` - Track completion
- `README.md` - Document streaming, multi-tool, metrics
- Code comments - Document complex logic

---

## If You Get Stuck

1. **Compilation error**: Check PHASE_2.md code snippets for exact implementation
2. **Test failure**: Add `println!` for debugging, use `cargo test -- --nocapture`
3. **Architecture question**: Review protocol.rs and async_tool.rs from Phase 1
4. **Trait error**: Ensure `#[async_trait]` on all async trait implementations
5. **Tokio error**: Use `#[tokio::test]` for async tests

---

## Timeline

- **Days 1-5**: Task 2.1 (Streaming)
- **Days 6-12**: Task 2.2 (Multi-tool)
- **Days 13-20**: Task 2.3 (Sampling & Metrics)
- **Days 21-28**: Testing, integration, gate check

---

## Reference Documents

- **PHASE_2.md** - Complete task details and code examples
- **IMPLEMENTATION_GUIDE.md** - Overall workflow and commands
- **PHASE_1_IMPLEMENTATION_COMPLETE.md** - What was built
- **Cargo.toml** - All dependencies (asynctrait, jsonschema already added)

---

## Next: Start Reading

1. Open and read: **[PHASE_2.md](./PHASE_2.md)**
2. Start with Task 2.1.1 code examples
3. Create `unified/src/mcp/streaming.rs`
4. Begin implementation!

---

**Ready?** Go to PHASE_2.md and start Task 2.1!
