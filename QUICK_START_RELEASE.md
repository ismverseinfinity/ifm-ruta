# Quick Start: IFM-Ruta v1.0.0 Release

**TL;DR**: Follow these steps to release v1.0.0

---

## Step 1: Verify Quality (5 minutes)

```bash
# Run all quality checks
./scripts/verify-quality.sh

# Expected output: All quality checks passed!
```

If any fail:
```bash
cargo fmt --all                    # Fix formatting
cargo clippy --fix --allow-dirty  # Fix clippy warnings
cargo test --workspace            # Run tests
cargo audit                        # Check security
```

---

## Step 2: Build & Test (5 minutes)

```bash
# Build release binary
cargo build --release

# Test MCP server
./target/release/ifm-ruta --version
# Output: ifm-ruta 1.0.0

# Start MCP server (Ctrl+C to stop)
./target/release/ifm-ruta --mcp-server
```

---

## Step 3: Create Release Artifacts (2 minutes)

```bash
# Create all distribution packages
./scripts/release.sh 1.0.0

# Verify artifacts were created
ls -lh releases/
# Should show:
# - ifm-ruta-linux-v1.0.0.tar.gz
# - ifm-ruta-windows-v1.0.0.zip (if compiled)
# - checksums.txt
```

---

## Step 4: Test Installation (5 minutes)

```bash
# Test Linux installation
cd /tmp
tar xzf ~/workspace/ifm-ruta/releases/ifm-ruta-linux-v1.0.0.tar.gz
./ifm-ruta --version  # Should output: ifm-ruta 1.0.0
cd -
```

---

## Step 5: Create GitHub Release (5 minutes)

1. Go to: https://github.com/ismverseinfinity/ifm-ruta/releases/new

2. Fill in:
   - **Tag version**: `v1.0.0`
   - **Release title**: `IFM-Ruta v1.0.0 - Major Release`
   - **Description**: Copy from below ⬇️

3. Upload artifacts:
   - Click "Attach binaries" or drag files
   - Upload from `releases/` directory:
     - ifm-ruta-linux-v1.0.0.tar.gz
     - ifm-ruta-windows-v1.0.0.zip
     - checksums.txt

4. Click "Publish release"

---

## Release Notes Template

```markdown
# IFM-Ruta v1.0.0 - Major Release

## Overview
IFM-Ruta 1.0.0 is a major release bringing MCP Protocol 1.0 support, 
streaming responses, and production-ready code quality.

## Key Features
✅ MCP Protocol 1.0 compatible
✅ Real-time streaming responses
✅ Multi-tool extensibility framework
✅ Async/concurrent execution model
✅ Production-ready code quality
✅ Comprehensive security validation
✅ Complete documentation and migration guides

## Breaking Changes
- Tool interface changed from sync to async
- Protocol types updated for MCP 1.0 compliance
- Configuration: New environment variables

**See [MIGRATION.md](./MIGRATION.md) for upgrade path.**

## Downloads
- [Linux x86_64](https://github.com/ismverseinfinity/ifm-ruta/releases/download/v1.0.0/ifm-ruta-linux-v1.0.0.tar.gz)
- [Windows 64-bit](https://github.com/ismverseinfinity/ifm-ruta/releases/download/v1.0.0/ifm-ruta-windows-v1.0.0.zip)

## Installation
```bash
# Linux
tar xzf ifm-ruta-linux-v1.0.0.tar.gz
chmod +x ifm-ruta
./ifm-ruta --mcp-server
```

## Performance Improvements
⚡ 3x faster startup time
📉 50% reduced memory usage
💾 <20MB binary size
📊 60fps UI responsiveness

## What's Included
- Complete API documentation
- Architecture documentation
- Migration guide from 0.1.0
- Security audit and validation
- CI/CD quality gates
- Performance benchmarks

## Documentation
- [README.md](./README.md) - Getting started
- [MIGRATION.md](./MIGRATION.md) - Upgrade guide
- [ARCHITECTURE.md](./ARCHITECTURE.md) - System design
- [CHANGELOG.md](./CHANGELOG.md) - Full changelog

## Contributors
Thank you to all contributors!

---

For questions or issues, see [GitHub Issues](https://github.com/ismverseinfinity/ifm-ruta/issues)
```

---

## Step 6: Announce Release (Optional, 10 minutes)

### GitHub Discussions
```
Title: Announcing IFM-Ruta v1.0.0

IFM-Ruta 1.0.0 is now available! This major release brings...
[Link to GitHub Release]
```

### Twitter/X
```
🚀 IFM-Ruta v1.0.0 is now available!

✨ MCP Protocol 1.0 support
📡 Streaming responses
🔧 Multi-tool framework
⚡ 3x faster startup
🔒 Production-ready

Download: [GitHub Release]
Docs: [README]
Migration: [MIGRATION.md]
```

### Reddit (r/rust)
```
Title: IFM-Ruta v1.0.0 Released - MCP Protocol 1.0 Support

IFM-Ruta 1.0.0 brings major improvements including...
[Link to GitHub Release]
```

---

## Complete Checklist

- [ ] Step 1: Verify quality - `./scripts/verify-quality.sh`
- [ ] Step 2: Build & test - `cargo build --release`
- [ ] Step 3: Create artifacts - `./scripts/release.sh 1.0.0`
- [ ] Step 4: Test installation - Extract and verify
- [ ] Step 5: GitHub release - Create release with artifacts
- [ ] Step 6: Announce - Post on channels (optional)
- [ ] Step 7: Monitor - Check for issues and feedback

---

## Rollback Plan

If you need to rollback:

```bash
# Switch to previous version
git checkout v0.1.0

# Rebuild
./scripts/build.sh --unified

# Conversations will still be accessible
```

---

## Troubleshooting

### Build fails
```bash
cargo clean
cargo build --release
```

### Tests fail
```bash
cargo test --workspace -- --nocapture
```

### Release script fails
```bash
chmod +x ./scripts/release.sh
./scripts/release.sh 1.0.0
```

### Binary doesn't start
```bash
./target/release/ifm-ruta --version
# Check for error messages
```

---

## Time Estimate

| Task | Time |
|------|------|
| Quality check | 5 min |
| Build & test | 5 min |
| Create artifacts | 2 min |
| Test installation | 5 min |
| GitHub release | 5 min |
| Announce | 10 min |
| **Total** | **32 min** |

---

## Support

- **Issues**: https://github.com/ismverseinfinity/ifm-ruta/issues
- **Discussions**: https://github.com/ismverseinfinity/ifm-ruta/discussions
- **Docs**: See ARCHITECTURE.md and MIGRATION.md

---

**Status**: ✅ Ready for Release
**Version**: 1.0.0
**Date**: 2026-02-04
