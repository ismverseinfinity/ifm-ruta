# Quick Commands Reference - IFM-Ruta MCP v1.0.0

**Phase**: 1 COMPLETE ✅ | Ready for Phase 2  
**Date**: 2026-02-04

---

## Essential Commands

### Setup Rust (First Time Only)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
. "$HOME/.cargo/env"
```

### Verify Installation
```bash
rustc --version    # Should be 1.93.0+
cargo --version    # Should be 1.93.0+
```

---

## Development Commands

### Build & Test
```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test --lib

# Run specific test
cargo test --lib test_validate_valid_input

# Run with output
cargo test --lib -- --nocapture
```

### Code Quality
```bash
# Check for issues (doesn't build)
cargo check --workspace

# Lint checking
cargo clippy --all

# Strict linting (fail on warnings)
cargo clippy --all -- -D warnings

# Format code
cargo fmt --all

# Check if formatted
cargo fmt --check

# Security audit
cargo audit
```

### Documentation
```bash
# Generate API docs
cargo doc --no-deps

# Open in browser
cargo doc --no-deps --open

# Test documentation
cargo test --doc
```

---

## Project-Specific Commands

### View All Tests
```bash
cargo test --lib -- --list
```

### Run Core Tests Only
```bash
cargo test --lib --package ifm-ruta-core
```

### Run with Logging
```bash
RUST_LOG=debug cargo test --lib -- --nocapture
```

### View Binary Info
```bash
file target/release/ifm-ruta
ldd target/release/ifm-ruta
```

---

## Debugging Commands

### Show Compilation Warnings
```bash
cargo clippy --all 2>&1 | grep warning
```

### Show All Errors
```bash
cargo check 2>&1 | grep error
```

### Get Full Error Details
```bash
cargo build 2>&1 | less
```

### Clean and Rebuild
```bash
cargo clean
cargo build --release
```

---

## Useful Workflows

### Quick Verify (5 minutes)
```bash
cargo check --workspace && cargo test --lib && cargo clippy --all
```

### Full Quality Check (10 minutes)
```bash
cargo fmt --check && cargo clippy --all -- -D warnings && \
cargo test --lib && cargo audit
```

### Pre-Commit Check
```bash
#!/bin/bash
set -e
echo "Formatting..."
cargo fmt --all
echo "Checking..."
cargo clippy --all -- -D warnings
echo "Testing..."
cargo test --lib
echo "✅ All checks passed"
```

### Release Build
```bash
cargo build --release
strip target/release/ifm-ruta  # Optional: reduce binary size
```

---

## Phase-Specific Commands

### Phase 1 Verification (After Implementation)
```bash
# Run these in order:
cargo check --workspace           # ✅ Should pass
cargo test --lib                  # ✅ 12/12 tests pass
cargo clippy --all                # ✅ 0 critical warnings
cargo fmt --check                 # ✅ All formatted
cargo audit                        # ✅ 0 vulnerabilities
```

### Phase 2 Start
```bash
# First, verify Phase 1 is still working
cargo test --lib

# Then start Phase 2 implementation
cargo test --lib test_streaming_response  # When implemented
```

---

## File Navigation Commands

### Find Test Files
```bash
find . -name "*test*" -type f | head -20
grep -r "#\[test\]" src/ --include="*.rs"
```

### List All Modules
```bash
find src -name "*.rs" | sort
```

### Show Module Structure
```bash
grep -r "^pub mod" src/ --include="*.rs"
```

### Find Specific Functions
```bash
grep -rn "fn execute" src/ --include="*.rs"
grep -rn "async fn" src/ --include="*.rs"
```

---

## Documentation Commands

### Generate Full Documentation
```bash
cargo doc --all --no-deps --open
```

### View API Reference
```bash
# Navigate to target/doc/ifm_ruta_core/index.html
# Contains all public APIs
```

### Test Doc Comments
```bash
cargo test --doc
```

---

## Troubleshooting Commands

### Clear Cache and Rebuild
```bash
cargo clean
cargo build --release
```

### Check for Dependency Issues
```bash
cargo tree              # Show dependency tree
cargo tree --duplicates # Show duplicate deps
```

### Update Dependencies
```bash
cargo update
cargo audit             # Check for vulnerabilities
```

### Check for Dead Code
```bash
cargo clippy --all -- -W clippy::all
```

---

## Git Integration Commands

### Create Feature Branch
```bash
git checkout -b feature/phase-2-streaming
```

### Stage Changes
```bash
git add core/src/traits/async_tool.rs
git add core/src/utils/validator.rs
git add unified/src/tools/schemas.rs
```

### Commit
```bash
git commit -m "Phase 1: Implement MCP 1.0 protocol, async traits, validation"
```

### Push
```bash
git push origin feature/phase-2-streaming
```

---

## Performance Commands

### Build Time Analysis
```bash
cargo clean && time cargo build --release
```

### Binary Size
```bash
ls -lh target/release/ifm-ruta
du -sh target/
```

### Dependencies Download Time
```bash
time cargo fetch
```

---

## Quick Reference

| Task | Command |
|------|---------|
| Run tests | `cargo test --lib` |
| Check code | `cargo check --workspace` |
| Lint | `cargo clippy --all` |
| Format | `cargo fmt --all` |
| Build | `cargo build --release` |
| Docs | `cargo doc --no-deps --open` |
| Security | `cargo audit` |
| Clean | `cargo clean` |

---

## Environment Setup

### Permanent Rust Environment
```bash
# Add to ~/.bashrc or ~/.zshrc:
. "$HOME/.cargo/env"
```

### Build Optimization
```bash
# Add to ~/.cargo/config.toml:
[build]
rustflags = ["-C", "link-arg=-fuse-ld=lld", "-C", "target-cpu=native"]
```

---

## Useful Aliases

Add to ~/.bashrc or ~/.zshrc:
```bash
alias ctest='cargo test --lib'
alias cbuild='cargo build --release'
alias cfmt='cargo fmt --all && cargo clippy --all'
alias ccheck='cargo check --workspace && cargo test --lib && cargo clippy --all'
```

Then use:
```bash
ctest      # Run tests
cbuild     # Build release
cfmt       # Format and lint
ccheck     # Full check
```

---

## For Phase 2 Implementation

When starting Phase 2, use:

```bash
# Verify Phase 1 still works
cargo test --lib

# Start Phase 2 work
cargo build --release

# Monitor changes
cargo watch -x test -x clippy
```

(requires `cargo install cargo-watch`)

---

## Performance Monitoring

```bash
# Show compilation progress
cargo build -v

# Show build times for dependencies
cargo build -Z timings

# Profile binary
cargo flamegraph  # requires flamegraph tool
```

---

## Final Checklist Before Commit

```bash
# Run in order:
cargo fmt --all              # Format code
cargo clippy --all -- -D warnings  # Strict lint
cargo test --lib             # Run tests
cargo doc --no-deps          # Generate docs
cargo audit                   # Check security

# If all pass, commit:
git add .
git commit -m "..."
git push
```

---

**Last Updated**: 2026-02-04  
**Phase**: 1 Complete ✅
