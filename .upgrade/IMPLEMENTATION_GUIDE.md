# IFM-Ruta MCP v1.0.0 - Implementation Guide for Coding Agents

**Status**: Ready for Implementation  
**Current Version**: 0.1.0  
**Target Version**: 1.0.0  
**Total Duration**: 16 weeks (4 phases)

---

## 🎯 Quick Overview

This guide helps AI coding agents implement the IFM-Ruta MCP v1.0.0 upgrade. The project requires:
- **MCP Protocol 1.0** support
- **Streaming responses** for real-time feedback
- **Multi-tool framework** for extensibility
- **Async/concurrent execution** with Tokio

---

## 📂 Document Structure

```
.upgrade/
├── IMPLEMENTATION_GUIDE.md     ← You are here
├── UPGRADE_README.md           ← Overview & navigation
├── UPGRADE_PLAN_2026.md        ← Executive summary
├── UPGRADE_INDEX.md            ← Document index
├── UPGRADE_SUMMARY.txt         ← Quick reference
├── PHASE_1.md                  ← Foundation & Protocol (Weeks 1-4)
├── PHASE_2.md                  ← Core Features (Weeks 5-8)
├── PHASE_3.md                  ← Testing & Documentation (Weeks 9-12)
└── PHASE_4.md                  ← Polish & Release (Weeks 13-16)
```

---

## 🚀 How to Use This Guide

### For Phase Implementation
1. Read the corresponding PHASE_*.md file
2. Follow task checklist in order
3. Implement code examples provided
4. Run tests before advancing
5. Verify gate criteria before next phase

### For Code References
- Each phase includes detailed code snippets
- Files to modify: `unified/src/`, `core/src/`
- Dependencies in: `Cargo.toml`
- Tests in: `tests/`, `benches/`

### For Question Resolution
- Check PHASE_*.md first for your current task
- Reference code examples provided
- Consult external links in References section

---

## 📋 Phase-by-Phase Implementation

### PHASE 1: Foundation & Protocol Update (Weeks 1-4)
**Reference**: [PHASE_1.md](./PHASE_1.md)

**Key Tasks**:
1. **Task 1.1**: Upgrade MCP Protocol to 1.0
   - Research MCP 1.0 spec
   - Update Cargo.toml dependencies
   - Refactor `unified/src/mcp/protocol.rs` with new types

2. **Task 1.2**: Design Async Architecture
   - Create `core/src/traits/async_tool.rs`
   - Refactor `unified/src/mcp/server.rs` to async
   - Implement Tokio runtime

3. **Task 1.3**: Input Validation Framework
   - Create `core/src/utils/validator.rs`
   - Add JSON schema validation
   - Define tool schemas

**Deliverables**:
- ✅ Updated Cargo.toml
- ✅ New protocol types (ResourceRequest, SamplingRequest, etc.)
- ✅ AsyncTool and StreamingTool traits
- ✅ Input validator with JSON schema support

**Success Gate**: Code compiles, basic tests pass

**Commands to Run**:
```bash
cargo check --workspace
cargo test --lib
cargo clippy --all
```

---

### PHASE 2: Core Features (Weeks 5-8)
**Reference**: [PHASE_2.md](./PHASE_2.md)

**Key Tasks**:
1. **Task 2.1**: Implement Streaming Responses
   - Create `unified/src/mcp/streaming.rs`
   - Implement StreamingResponse wrapper
   - Convert interactive_feedback tool to streaming

2. **Task 2.2**: Build Multi-Tool Framework
   - Create `core/src/services/tool_registry.rs`
   - Implement ToolRegistry struct
   - Refactor existing tools to use new trait

3. **Task 2.3**: Add Sampling & Metrics
   - Create `unified/src/mcp/sampling.rs`
   - Create `core/src/services/metrics.rs`
   - Implement cost tracking

**Deliverables**:
- ✅ StreamingResponse infrastructure
- ✅ ToolRegistry with registration API
- ✅ SamplingHandler for AI calls
- ✅ Metrics collection system

**Success Gate**: Streaming works end-to-end, multi-tool registration functional

**Commands to Run**:
```bash
cargo build --release
cargo test --all
cargo bench
```

---

### PHASE 3: Integration & Testing (Weeks 9-12)
**Reference**: [PHASE_3.md](./PHASE_3.md)

**Key Tasks**:
1. **Task 3.1**: Write Comprehensive Tests
   - Create `tests/protocol_tests.rs`
   - Create `tests/async_tool_tests.rs`
   - Create `tests/tool_registry_tests.rs`
   - Create `tests/streaming_tests.rs`
   - Create integration tests

2. **Task 3.2**: Generate Documentation
   - Create API_REFERENCE.md
   - Create MIGRATION.md
   - Create TOOL_DEVELOPMENT.md

3. **Task 3.3**: Optional Compatibility Layer
   - Create `core/src/compat/v0_tool_wrapper.rs`

**Deliverables**:
- ✅ 50+ unit tests
- ✅ Integration test suite
- ✅ Performance benchmarks
- ✅ Complete documentation (5+ pages)

**Success Gate**: 85%+ test coverage, all integrations tested

**Commands to Run**:
```bash
cargo test --workspace
cargo tarpaulin --out Html
cargo doc --no-deps
```

---

### PHASE 4: Polish & Release (Weeks 13-16)
**Reference**: [PHASE_4.md](./PHASE_4.md)

**Key Tasks**:
1. **Task 4.1**: Final Optimization
   - Profile with flamegraph
   - Security audit
   - Code quality checks

2. **Task 4.2**: Release Preparation
   - Version bump: 0.1.0 → 1.0.0
   - Generate CHANGELOG.md
   - Build release artifacts

3. **Task 4.3**: Launch Release
   - Create GitHub release
   - Publish artifacts
   - Announce

**Deliverables**:
- ✅ Optimized binaries
- ✅ Release artifacts
- ✅ GitHub release
- ✅ Announcements

**Success Gate**: Production-ready v1.0.0 released

**Commands to Run**:
```bash
cargo audit
cargo fmt --check
cargo clippy --all -- -D warnings
./scripts/build.sh --unified
```

---

## 🔧 Implementation Workflow

### Before Starting Any Task
```bash
# 1. Create feature branch
git checkout -b feature/mcp-1.0

# 2. Update repository
git pull origin main

# 3. Verify environment
cargo --version  # Should be 1.70+
rustup update
```

### During Implementation
```bash
# 1. After code changes
cargo fmt --all
cargo clippy --all

# 2. Verify tests
cargo test --workspace

# 3. Check documentation
cargo doc --no-deps

# 4. Commit changes
git add .
git commit -m "Phase 1, Task 1.1: Update MCP protocol types"
```

### Before Phase Gate
```bash
# 1. Run full test suite
cargo test --all --verbose

# 2. Check code quality
cargo clippy --all -- -D warnings

# 3. Format verification
cargo fmt --check

# 4. Security audit
cargo audit
```

---

## 📊 File Organization Reference

### Files to Modify
```
core/
├── src/
│   ├── traits/
│   │   ├── mod.rs                    ← Export AsyncTool, StreamingTool
│   │   ├── async_tool.rs             ← NEW: Async traits
│   │   └── [existing traits]
│   ├── services/
│   │   ├── mod.rs                    ← Export ToolRegistry, Metrics
│   │   ├── tool_registry.rs          ← NEW: Multi-tool framework
│   │   └── metrics.rs                ← NEW: Metrics collection
│   └── utils/
│       ├── mod.rs                    ← Export Validator
│       ├── validator.rs              ← NEW: JSON schema validation
│       └── [existing utils]
└── Cargo.toml

unified/
├── src/
│   ├── mcp/
│   │   ├── protocol.rs               ← UPDATE: New MCP 1.0 types
│   │   ├── server.rs                 ← UPDATE: Async refactor
│   │   ├── streaming.rs              ← NEW: Streaming infrastructure
│   │   ├── sampling.rs               ← NEW: Sampling handler
│   │   └── mod.rs
│   ├── tools/
│   │   ├── interactive_feedback.rs   ← UPDATE: Use StreamingTool
│   │   └── schemas.rs                ← NEW: Tool input schemas
│   └── main.rs
└── Cargo.toml
```

### New Files to Create
```
core/src/traits/async_tool.rs
core/src/services/tool_registry.rs
core/src/services/metrics.rs
core/src/utils/validator.rs
unified/src/mcp/streaming.rs
unified/src/mcp/sampling.rs
unified/src/tools/schemas.rs
tests/protocol_tests.rs
tests/async_tool_tests.rs
tests/tool_registry_tests.rs
tests/streaming_tests.rs
```

---

## 💡 Key Implementation Tips

### Dependency Management
- Add new dependencies to `[workspace.dependencies]` in Cargo.toml
- Use `cargo update` to resolve versions
- Run `cargo audit` before releasing

### Async/Await Patterns
- Use `#[async_trait]` for trait implementations
- Prefer `tokio::spawn` for concurrent tasks
- Use `Arc<RwLock<T>>` for shared mutable state

### Testing
- Write tests in same file (mod tests { })
- Use `#[tokio::test]` for async tests
- Aim for 85%+ coverage
- Run `cargo tarpaulin` to measure coverage

### Error Handling
- Define custom error types
- Use `Result<T, CustomError>`
- Provide detailed error messages
- Log errors with `tracing::error!`

### Documentation
- Add doc comments to public APIs
- Include examples in doc comments
- Generate with `cargo doc --no-deps`
- Update README.md with new features

---

## 🔗 External References

**MCP Protocol**:
- https://modelcontextprotocol.io/

**Rust Async**:
- https://tokio.rs/tokio/tutorial
- https://rust-lang.github.io/async-book/

**Testing & Benchmarking**:
- https://bheisler.github.io/criterion.rs/book/
- https://github.com/xd009642/tarpaulin

**JSON Schema**:
- https://json-schema.org/
- https://docs.rs/jsonschema/

---

## 📝 Progress Tracking

### Phase 1 Checklist
- [ ] MCP 1.0 research complete
- [ ] Cargo.toml updated with new dependencies
- [ ] Protocol types implemented (ResourceRequest, SamplingRequest, etc.)
- [ ] AsyncTool and StreamingTool traits created
- [ ] MCP server refactored to async
- [ ] Validator framework implemented
- [ ] All tests passing
- [ ] Gate check: Code compiles, basic tests pass ✅

### Phase 2 Checklist
- [ ] StreamingResponse wrapper implemented
- [ ] interactive_feedback converted to streaming
- [ ] ToolRegistry implemented
- [ ] Tools migrated to new trait system
- [ ] SamplingHandler created
- [ ] Metrics collection added
- [ ] All tests passing
- [ ] Gate check: Streaming works, multi-tool registered ✅

### Phase 3 Checklist
- [ ] Unit tests written (50+ tests)
- [ ] Integration tests written
- [ ] Performance benchmarks created
- [ ] API documentation complete
- [ ] Migration guide written
- [ ] Tool development guide written
- [ ] Coverage ≥ 85%
- [ ] Gate check: All tests pass, docs complete ✅

### Phase 4 Checklist
- [ ] Performance tuning complete
- [ ] Security audit passed
- [ ] Code quality gates met
- [ ] Version bumped to 1.0.0
- [ ] Changelog created
- [ ] Release artifacts built
- [ ] GitHub release published
- [ ] Gate check: v1.0.0 released ✅

---

## ⚡ Quick Commands Reference

```bash
# Build & Test
cargo build --release
cargo test --all --verbose
cargo bench

# Code Quality
cargo fmt --all
cargo clippy --all -- -D warnings
cargo audit
cargo tarpaulin --out Html

# Documentation
cargo doc --no-deps --open
cargo test --doc

# Performance
cargo build --release
./target/release/ifm-ruta --version

# Git Workflow
git checkout -b feature/mcp-1.0
git add .
git commit -m "descriptive message"
git push origin feature/mcp-1.0
```

---

## 🆘 Common Issues & Solutions

**Issue**: Compilation errors with async traits
**Solution**: Ensure `async-trait` crate is in dependencies

**Issue**: Test failures with tokio runtime
**Solution**: Use `#[tokio::test]` attribute on async tests

**Issue**: JSON schema validation failing
**Solution**: Verify schema format matches JSON Schema Draft 7

**Issue**: Streaming tests hanging
**Solution**: Add timeout assertions with `tokio::time::timeout`

---

## 📞 Getting Help

1. **For Phase Details**: Read corresponding PHASE_*.md file
2. **For Code Examples**: Find code snippets in relevant PHASE_*.md section
3. **For Architecture**: Check UPGRADE_README.md section 3
4. **For Timeline**: See UPGRADE_PLAN_2026.md milestones
5. **For Documentation**: Reference UPGRADE_INDEX.md

---

## 🎯 Success Definition

Implementation is complete when:

✅ **All code is written**
- Protocol types updated
- Async traits implemented
- Tool registry working
- Streaming infrastructure operational

✅ **Tests pass**
- 85%+ code coverage
- All unit tests passing
- Integration tests passing
- Performance benchmarks met

✅ **Documentation complete**
- API reference written
- Migration guide available
- Tool development guide ready
- README updated

✅ **Quality gates passed**
- Zero clippy warnings
- All code formatted
- Security audit passed
- Performance targets met

✅ **Release ready**
- Binaries built and tested
- Changelog generated
- GitHub release created
- v1.0.0 published

---

## 🚀 Starting Your Implementation

### RIGHT NOW (Next 30 minutes)
1. Read this file completely ✅
2. Read [UPGRADE_README.md](./UPGRADE_README.md)
3. Skim [PHASE_1.md](./PHASE_1.md)

### NEXT HOUR
1. Create feature branch: `git checkout -b feature/mcp-1.0`
2. Setup IDE/editor with Rust extension
3. Open PHASE_1.md side-by-side with code editor

### WEEK 1 (Start Phase 1)
1. Task 1.1: Research MCP 1.0 spec (3 days)
2. Task 1.2: Update Cargo.toml dependencies (2 days)
3. Task 1.3: Implement protocol types (5 days)

---

**Ready to start? Open [PHASE_1.md](./PHASE_1.md) and begin Task 1.1.1!**

---

**Created**: 2026-02-04  
**Version**: 1.0.0  
**Status**: Ready for Implementation
