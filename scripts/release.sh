#!/bin/bash
# Release script for IFM-Ruta
# Usage: ./scripts/release.sh --version 1.0.0

set -e

VERSION=${1:-1.0.0}
REPO_URL="https://github.com/ismverseinfinity/ifm-ruta"
RELEASE_DIR="releases"

echo "=========================================="
echo "IFM-Ruta Release Script"
echo "=========================================="
echo "Version: $VERSION"
echo ""

# Check if version is provided
if [ -z "$VERSION" ] || [ "$VERSION" = "--version" ]; then
    echo "Usage: ./scripts/release.sh <version>"
    echo "Example: ./scripts/release.sh 1.0.0"
    exit 1
fi

# Verify git is clean
if [ -n "$(git status -s)" ]; then
    echo "Error: Working directory is not clean. Commit changes first."
    git status -s
    exit 1
fi

# Create release directory
mkdir -p "$RELEASE_DIR"

echo "Step 1: Building releases..."
echo "----"

# Build unified
echo "Building unified binary..."
./scripts/build.sh --unified
cp target/release/ifm-ruta "$RELEASE_DIR/ifm-ruta-linux-v${VERSION}"
chmod +x "$RELEASE_DIR/ifm-ruta-linux-v${VERSION}"

# Build Windows (if cross compilation available)
if command -v cargo &> /dev/null; then
    echo "Building Windows binary..."
    if ./scripts/build.sh --windows 2>/dev/null; then
        cp target/x86_64-pc-windows-gnu/release/ifm-ruta.exe "$RELEASE_DIR/ifm-ruta-windows-v${VERSION}.exe" 2>/dev/null || echo "Warning: Windows build skipped (MinGW not available)"
    else
        echo "Warning: Windows build skipped (cross-compilation not available)"
    fi
fi

echo ""
echo "Step 2: Creating distribution packages..."
echo "----"

# Create tar.gz for Linux
if [ -f "$RELEASE_DIR/ifm-ruta-linux-v${VERSION}" ]; then
    cd "$RELEASE_DIR"
    tar czf "ifm-ruta-linux-v${VERSION}.tar.gz" "ifm-ruta-linux-v${VERSION}"
    rm "ifm-ruta-linux-v${VERSION}"
    cd ..
    echo "Created: releases/ifm-ruta-linux-v${VERSION}.tar.gz"
fi

# Create zip for Windows
if [ -f "$RELEASE_DIR/ifm-ruta-windows-v${VERSION}.exe" ]; then
    cd "$RELEASE_DIR"
    zip -q "ifm-ruta-windows-v${VERSION}.zip" "ifm-ruta-windows-v${VERSION}.exe"
    rm "ifm-ruta-windows-v${VERSION}.exe"
    cd ..
    echo "Created: releases/ifm-ruta-windows-v${VERSION}.zip"
fi

echo ""
echo "Step 3: Generating checksums..."
echo "----"

cd "$RELEASE_DIR"
if command -v sha256sum &> /dev/null; then
    sha256sum *.tar.gz *.zip 2>/dev/null > checksums.txt || true
    cat checksums.txt
elif command -v shasum &> /dev/null; then
    shasum -a 256 *.tar.gz *.zip 2>/dev/null > checksums.txt || true
    cat checksums.txt
fi
cd ..

echo ""
echo "Step 4: Verification..."
echo "----"

if [ -f "$RELEASE_DIR/ifm-ruta-linux-v${VERSION}.tar.gz" ]; then
    echo "✓ Linux binary package created"
    ls -lh "$RELEASE_DIR/ifm-ruta-linux-v${VERSION}.tar.gz"
fi

if [ -f "$RELEASE_DIR/ifm-ruta-windows-v${VERSION}.zip" ]; then
    echo "✓ Windows binary package created"
    ls -lh "$RELEASE_DIR/ifm-ruta-windows-v${VERSION}.zip"
fi

if [ -f "$RELEASE_DIR/checksums.txt" ]; then
    echo "✓ Checksums generated"
fi

echo ""
echo "=========================================="
echo "Release artifacts ready in ./releases/"
echo "=========================================="
echo ""
echo "Next steps:"
echo "1. Review the artifacts:"
echo "   ls -lh releases/"
echo ""
echo "2. Create GitHub release:"
echo "   gh release create v${VERSION} releases/* --title 'IFM-Ruta v${VERSION}'"
echo ""
echo "3. Or create manually at:"
echo "   ${REPO_URL}/releases/new"
echo ""
echo "Release notes template:"
cat << EOF
# IFM-Ruta v${VERSION} - Major Release

## Overview
IFM-Ruta ${VERSION} is a major upgrade bringing MCP Protocol 1.0 support,
streaming responses, and comprehensive quality improvements.

## Key Features
✅ MCP Protocol 1.0 compatible
✅ Real-time streaming responses
✅ Multi-tool extensibility
✅ Async/concurrent execution
✅ Production-ready code quality

## Downloads
- [Linux x86_64](${REPO_URL}/releases/download/v${VERSION}/ifm-ruta-linux-v${VERSION}.tar.gz)
- [Windows 64-bit](${REPO_URL}/releases/download/v${VERSION}/ifm-ruta-windows-v${VERSION}.zip)

## Installation
See [README.md](./README.md) for installation instructions.

## Breaking Changes
See [MIGRATION.md](./MIGRATION.md) for upgrade path.

## Changelog
See [CHANGELOG.md](./CHANGELOG.md) for detailed changes.

## Contributors
Thank you to all contributors!
EOF
