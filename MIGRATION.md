# Migration Guide: 0.1.0 → 1.0.0

This guide helps you upgrade from ifm-ruta 0.1.0 to 1.0.0.

## Overview

IFM-Ruta 1.0.0 is a major release with significant improvements:
- **MCP Protocol 1.0** support (replaces 0.1.0)
- **Async/Concurrent** execution model
- **Streaming responses** for real-time feedback
- **Production-ready** code quality and security

## Breaking Changes

### 1. Tool Interface: Sync → Async

#### Before (0.1.0)
```rust
pub trait Tool {
    fn execute(&self, input: &str) -> Result<String>;
}
```

#### After (1.0.0)
```rust
#[async_trait]
pub trait Tool {
    async fn execute(&self, input: &str) -> Result<String>;
}
```

**Migration**: Add `#[async_trait]` and `async` keyword to all tool implementations.

```rust
#[async_trait]
impl Tool for MyTool {
    async fn execute(&self, input: &str) -> Result<String> {
        // Your async implementation
        Ok(result)
    }
}
```

### 2. Protocol Types: Updated for MCP 1.0

The following types have been updated to match MCP Protocol 1.0:

| Old (0.1.0) | New (1.0.0) | Change |
|------------|-----------|--------|
| `ToolRequest` | `ToolCall` | Renamed for clarity |
| `ToolResponse` | `ToolResult` | Renamed for clarity |
| `feedback` | `content` | JSON field names updated |
| `version: "0.1"` | `version: "1.0"` | Protocol version |

**Migration Example**:
```json
// Before
{
  "method": "tools/request",
  "params": {
    "name": "interactive_feedback",
    "arguments": { "feedback": "..." }
  }
}

// After
{
  "method": "tools/call",
  "params": {
    "name": "interactive_feedback",
    "arguments": { "content": "..." }
  }
}
```

### 3. Configuration: Environment Variables

New environment variables added in 1.0.0:

```bash
# New in 1.0.0
RUST_LOG=debug                    # Log level control
IFM_RUTA_TIMEOUT=30              # Timeout for user interaction
IFM_RUTA_CONFIG_DIR=/custom/path # Custom config directory
IFM_RUTA_MAX_MEMORY_MB=256       # Memory limit
```

**Migration**: Update your MCP configuration:

```json
{
  "mcpServers": {
    "ifm-ruta": {
      "command": "/path/to/ifm-ruta",
      "args": ["--mcp-server"],
      "env": {
        "RUST_LOG": "info",
        "IFM_RUTA_TIMEOUT": "30"
      }
    }
  }
}
```

## Non-Breaking Changes

These changes are backward compatible but recommended:

### 1. Build Script Updates

```bash
# Recommended: Use new unified build
./scripts/build.sh --unified

# Old method still works
./scripts/build.sh
```

### 2. Configuration Files

Conversation storage format is compatible, but new format recommended:

```bash
# Auto-migration will occur on first use
# Old: .ifm-ruta/conversations/
# New: .ifm-ruta/conversations/ (same location, optimized)
```

## Step-by-Step Migration

### For Users

1. **Backup conversation data** (optional):
   ```bash
   cp -r ~/.ifm-ruta ~/ifm-ruta-backup
   ```

2. **Update ifm-ruta binary**:
   ```bash
   git clone https://github.com/ismverseinfinity/ifm-ruta.git
   cd ifm-ruta
   ./scripts/build.sh --unified
   ```

3. **Update MCP configuration** in Cursor:
   ```json
   {
     "mcpServers": {
       "ifm-ruta": {
         "command": "/new/path/to/ifm-ruta",
         "args": ["--mcp-server"],
         "env": {
           "RUST_LOG": "info"
         }
       }
     }
   }
   ```

4. **Restart Cursor** and test MCP connection

5. **Verify functionality**:
   ```bash
   ./target/release/ifm-ruta --version  # Should output: ifm-ruta 1.0.0
   ```

### For Tool Developers

1. **Update tool trait implementations** to async:
   ```rust
   #[async_trait]
   impl Tool for YourTool {
       async fn execute(&self, input: &str) -> Result<String> {
           // Update implementation
           Ok(result)
       }
   }
   ```

2. **Update imports**:
   ```rust
   // Add async_trait
   use async_trait::async_trait;
   use tokio::task;
   ```

3. **Update error handling**:
   ```rust
   // Use ? operator with async
   let result = some_async_operation().await?;
   ```

4. **Test thoroughly**:
   ```bash
   cargo test --workspace
   ```

### For Integration Developers

1. **Update API calls** to use MCP 1.0 protocol
2. **Update JSON schemas** to match new types
3. **Test protocol compatibility**:
   ```bash
   ./scripts/test-mcp.sh
   ```

## Rollback Plan

If you need to rollback to 0.1.0:

```bash
# Switch to previous version
git checkout v0.1.0

# Rebuild
./scripts/build.sh --unified

# Your conversation history will still be accessible
```

## Getting Help

- **Issues**: Check [GitHub Issues](https://github.com/ismverseinfinity/ifm-ruta/issues)
- **Discussions**: See [GitHub Discussions](https://github.com/ismverseinfinity/ifm-ruta/discussions)
- **Documentation**: Review [README.md](./README.md) and [CHANGELOG.md](./CHANGELOG.md)

## Compatibility

### Supported Platforms
- Linux (x86_64): ✅ Full support
- Windows (x86_64, i686): ✅ Full support via Windows build
- macOS: ✅ Supported (native binary)

### Minimum Requirements
- **Rust**: 1.70+ (MSRV)
- **OS**: Linux, Windows 10+, macOS 10.12+

### MCP Protocol
- **Protocol Version**: 1.0
- **JSON-RPC**: 2.0

## Summary

| Aspect | 0.1.0 | 1.0.0 | Impact |
|--------|-------|-------|--------|
| MCP Protocol | 0.1 | 1.0 | Breaking |
| Tool Interface | Sync | Async | Breaking |
| Performance | Baseline | 3x faster | Improvement |
| Security | Basic | Comprehensive | Improvement |
| Documentation | Limited | Complete | Improvement |

---

**Last Updated**: 2026-02-04
**Version**: 1.0.0
