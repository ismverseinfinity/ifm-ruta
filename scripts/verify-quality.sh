#!/bin/bash
# Quality verification script for Phase 4

set -e

echo "=========================================="
echo "IFM-Ruta Quality Verification"
echo "=========================================="
echo ""

FAILED=0
PASSED=0

check() {
    local name="$1"
    local cmd="$2"
    
    echo -n "Checking $name... "
    if eval "$cmd" > /dev/null 2>&1; then
        echo "✓"
        ((PASSED++))
    else
        echo "✗"
        ((FAILED++))
    fi
}

echo "Code Quality"
echo "============"

check "Formatting" "cargo fmt --all -- --check"
check "Clippy" "cargo clippy --all --all-targets -- -D warnings"
check "Tests" "cargo test --workspace"

echo ""
echo "Documentation"
echo "=============="

check "API Documentation" "cargo doc --workspace --no-deps"

echo ""
echo "Security"
echo "========"

check "Audit dependencies" "cargo audit"
check "No unsafe blocks" "! grep -r 'unsafe {' core/src/security/ --include='*.rs'"

echo ""
echo "Performance"
echo "==========="

check "Binary size < 50MB" "[[ $(stat -f%z target/release/ifm-ruta 2>/dev/null || stat -c%s target/release/ifm-ruta) -lt 52428800 ]]"
check "Build succeeds" "cargo build --release"

echo ""
echo "Version Check"
echo "============="

check "Version is 1.0.0" "grep -q 'version = \"1.0.0\"' Cargo.toml"

echo ""
echo "=========================================="
echo "Results: $PASSED passed, $FAILED failed"
echo "=========================================="

if [ $FAILED -gt 0 ]; then
    echo ""
    echo "Failing checks:"
    echo "  - Format: cargo fmt --all"
    echo "  - Lint: cargo clippy --all --all-targets -- -D warnings"
    echo "  - Tests: cargo test --workspace"
    echo "  - Audit: cargo audit"
    exit 1
fi

echo ""
echo "All quality checks passed! ✓"
exit 0
