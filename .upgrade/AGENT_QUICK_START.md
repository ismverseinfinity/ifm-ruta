# Quick Start for Coding Agent - IFM-Ruta v1.0.0 Upgrade

**Current Task**: Implement IFM-Ruta MCP v1.0.0 upgrade  
**Total Duration**: 16 weeks | **4 Phases** | **40+ Tasks**  
**Status**: Ready for implementation

---

## 🎯 Your Mission

Upgrade IFM-Ruta MCP from v0.1.0 to v1.0.0 by implementing:
1. **MCP Protocol 1.0** support
2. **Streaming responses** (real-time feedback)
3. **Multi-tool framework** (extensible architecture)
4. **Async/concurrent execution** (Tokio-based)

---

## 📍 Where You Are

```
You are starting PHASE 1: Foundation & Protocol Update (Weeks 1-4)
├─ Task 1.1: MCP Protocol Upgrade
├─ Task 1.2: Async Architecture
└─ Task 1.3: Input Validation Framework
```

---

## 🚀 Immediate Action Items

### Step 1: Read Overview (5 min)
Open and read: **[IMPLEMENTATION_GUIDE.md](./IMPLEMENTATION_GUIDE.md)**

This file explains:
- How to use all documents
- Phase-by-phase workflow
- File organization
- Commands to run

### Step 2: Read Your Phase (30 min)
Open: **[PHASE_1.md](./PHASE_1.md)**

This contains:
- Detailed tasks with timelines
- Code examples and implementations
- Testing requirements
- Completion checklists

### Step 3: Start Coding

Follow task 1.1.1 in PHASE_1.md:
```
Task 1.1.1: Research MCP 1.0 Specification (3 days)
├─ Read official MCP 1.0 documentation
├─ Analyze breaking changes
├─ Identify new capabilities to implement
└─ Document findings in research note
```

---

## 📚 Reference Folder Structure

```
.upgrade/
├── AGENT_QUICK_START.md ←━━ You are here
├── IMPLEMENTATION_GUIDE.md   Main implementation guide
├── UPGRADE_README.md         Overview & navigation
├── UPGRADE_PLAN_2026.md      Executive summary
├── UPGRADE_INDEX.md          Document index
├── UPGRADE_SUMMARY.txt       Quick reference
├── PHASE_1.md                Weeks 1-4 (START HERE)
├── PHASE_2.md                Weeks 5-8
├── PHASE_3.md                Weeks 9-12
└── PHASE_4.md                Weeks 13-16
```

---

## 🎯 Phase 1 Overview (Weeks 1-4)

**Goal**: Update MCP protocol and build async foundation

### Task 1.1: MCP Protocol Upgrade (10 days)
- Research MCP 1.0 spec
- Update dependencies in Cargo.toml
- Implement new protocol types (ResourceRequest, SamplingRequest, etc.)
- **File**: `unified/src/mcp/protocol.rs`

### Task 1.2: Async Architecture (9 days)
- Create AsyncTool and StreamingTool traits
- Refactor MCP server to async
- Setup Tokio runtime
- **Files**: `core/src/traits/async_tool.rs`, `unified/src/mcp/server.rs`

### Task 1.3: Input Validation (7 days)
- Implement JSON schema validation
- Create validator framework
- Define tool schemas
- **Files**: `core/src/utils/validator.rs`, `unified/src/tools/schemas.rs`

**Success Gate**: Code compiles, basic tests pass ✅

---

## ⚡ Quick Commands

### Setup
```bash
cd /home/ismhac/workspace/ifm-ruta
git checkout -b feature/mcp-1.0
cargo --version  # Should be 1.70+
```

### Development
```bash
cargo fmt --all                    # Format code
cargo clippy --all                 # Lint check
cargo test --lib                   # Run tests
cargo doc --no-deps                # Generate docs
```

### Quality Checks
```bash
cargo test --all --verbose         # All tests
cargo clippy --all -- -D warnings  # Strict linting
cargo audit                        # Security check
```

---

## 📂 Key Files to Modify

| File | Phase | Task | Purpose |
|------|-------|------|---------|
| `Cargo.toml` | 1 | 1.1 | Update dependencies |
| `unified/src/mcp/protocol.rs` | 1 | 1.1 | New protocol types |
| `core/src/traits/async_tool.rs` | 1 | 1.2 | Async traits (NEW) |
| `unified/src/mcp/server.rs` | 1 | 1.2 | Async server |
| `core/src/utils/validator.rs` | 1 | 1.3 | Input validation (NEW) |
| `unified/src/mcp/streaming.rs` | 2 | 2.1 | Streaming (NEW) |
| `core/src/services/tool_registry.rs` | 2 | 2.2 | Multi-tool (NEW) |

---

## 💡 Key Implementation Details

### MCP Protocol Types (Phase 1, Task 1.1)
```rust
// Add to protocol.rs
pub struct InitializeRequest { ... }
pub struct ResourceRequest { ... }
pub struct SamplingRequest { ... }
pub struct ClientCapabilities { ... }
```
See PHASE_1.md for complete code.

### Async Traits (Phase 1, Task 1.2)
```rust
// Create async_tool.rs
#[async_trait]
pub trait AsyncTool: Send + Sync {
    async fn execute(&self, args: Value) -> MCPResult<ToolResponse>;
    fn metadata(&self) -> ToolMetadata;
}
```
See PHASE_1.md for complete code.

### Validation Framework (Phase 1, Task 1.3)
```rust
// Create validator.rs
pub struct InputValidator {
    schemas: HashMap<String, JSONSchema>,
}
```
See PHASE_1.md for complete code.

---

## ✅ Phase 1 Checklist

### Task 1.1: MCP Protocol
- [ ] Research MCP 1.0 spec (3 days)
- [ ] Update Cargo.toml with new crates (2 days)
- [ ] Implement protocol types (5 days)
- [ ] Write tests for new types
- [ ] All code compiles without warnings

### Task 1.2: Async Architecture
- [ ] Define AsyncTool trait (2 days)
- [ ] Define StreamingTool trait (1 day)
- [ ] Refactor server to async (4 days)
- [ ] Add proper error handling
- [ ] Test async execution

### Task 1.3: Validation Framework
- [ ] Create validator struct (2 days)
- [ ] Implement JSON schema support (2 days)
- [ ] Define tool input schemas (1 day)
- [ ] Add validation tests

### Gate Check
- [ ] `cargo test --all` passes ✅
- [ ] `cargo clippy --all` has no warnings ✅
- [ ] Basic tests for each component ✅

---

## 🔗 File References

**For detailed implementation code**: See [PHASE_1.md](./PHASE_1.md)
- Task 1.1.3 has complete protocol.rs code
- Task 1.2.1 has complete async_tool.rs code
- Task 1.3.1 has complete validator.rs code

**For workflow and commands**: See [IMPLEMENTATION_GUIDE.md](./IMPLEMENTATION_GUIDE.md)
**For timeline and overview**: See [UPGRADE_PLAN_2026.md](./UPGRADE_PLAN_2026.md)

---

## 📊 Progress Tracking

**Week 1**: Task 1.1 (Research & Cargo.toml)
**Week 2**: Task 1.1 (Protocol types) + Task 1.2 (Async traits start)
**Week 3**: Task 1.2 (Server refactor) + Task 1.3 (Validation framework)
**Week 4**: Task 1.3 (Tool schemas) + Testing + Gate check

---

## 🚀 Getting Started RIGHT NOW

1. Open [IMPLEMENTATION_GUIDE.md](./IMPLEMENTATION_GUIDE.md) ← Read first
2. Read [PHASE_1.md](./PHASE_1.md) ← Your detailed guide
3. Start Task 1.1.1 in PHASE_1.md ← Begin coding
4. Run: `cargo test --lib` ← Verify setup
5. Create git commit ← Track progress

---

## 🆘 If You Get Stuck

1. **Check PHASE_1.md** - Has all code examples
2. **Run commands** - `cargo check`, `cargo test`
3. **Read error messages** - Rust compiler is helpful
4. **Reference code** - Code snippets in each task
5. **Check links** - External refs at bottom of guides

---

## 📞 Quick Reference

| Topic | File | Section |
|-------|------|---------|
| Overview | UPGRADE_README.md | - |
| Executive Summary | UPGRADE_PLAN_2026.md | - |
| Workflow | IMPLEMENTATION_GUIDE.md | How to Use |
| Phase 1 Tasks | PHASE_1.md | Each task (1.1, 1.2, 1.3) |
| Code Examples | PHASE_1.md | **Code** blocks |
| Testing | PHASE_1.md | Each **Task** |
| Success Gate | PHASE_1.md | Summary & Validation |

---

## ✨ What You'll Build in Phase 1

### Updated Cargo.toml
```toml
[workspace.dependencies]
mcp = "1.0"              # NEW
async-trait = "0.1"      # NEW
jsonschema = "0.16"      # NEW
```

### New Files Created
```
core/src/traits/async_tool.rs          ← Async trait definitions
core/src/utils/validator.rs            ← Input validation
unified/src/tools/schemas.rs           ← Tool input schemas
```

### Modified Files
```
unified/src/mcp/protocol.rs            ← New types added
unified/src/mcp/server.rs              ← Converted to async
Cargo.toml                             ← New dependencies
```

---

## 🎓 Learning Resources

- **MCP Protocol**: https://modelcontextprotocol.io/
- **Rust Async**: https://tokio.rs/tokio/tutorial
- **JSON Schema**: https://json-schema.org/
- **Testing in Rust**: https://doc.rust-lang.org/book/ch11-00-testing.html

---

**STATUS**: Ready to start Phase 1, Task 1.1  
**NEXT STEP**: Open [PHASE_1.md](./PHASE_1.md)  
**ESTIMATED TIME**: 4 weeks to completion

---

Created: 2026-02-04 | Ready for Implementation
