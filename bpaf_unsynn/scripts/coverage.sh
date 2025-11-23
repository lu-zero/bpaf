#!/bin/bash
# Code coverage script for bpaf_unsynn using cargo-llvm-cov
#
# IMPORTANT: This is a proc-macro crate. Proc-macros run at compile time,
# so coverage measures the test code execution, not the proc-macro source.
# For proc-macros, test pass/fail is the primary validation metric.
#
# Prerequisites:
#   cargo install cargo-llvm-cov
#
# Usage:
#   ./scripts/coverage.sh          # Generate HTML report
#   ./scripts/coverage.sh --lcov   # Generate LCOV report
#   ./scripts/coverage.sh --open   # Generate and open HTML report
#   ./scripts/coverage.sh --help   # Show help

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
COVERAGE_DIR="$PROJECT_DIR/target/llvm-cov"

# Parse arguments
OUTPUT_FORMAT="html"
OPEN_REPORT=""

for arg in "$@"; do
    case $arg in
        --lcov)
            OUTPUT_FORMAT="lcov"
            ;;
        --open)
            OPEN_REPORT="--open"
            ;;
        --help|-h)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Code coverage using cargo-llvm-cov."
            echo ""
            echo "Options:"
            echo "  --lcov    Generate LCOV format instead of HTML"
            echo "  --open    Open HTML report in browser after generation"
            echo "  --help    Show this help message"
            echo ""
            echo "Note: This is a proc-macro crate. Test pass/fail is the primary"
            echo "validation that the macro generates correct code."
            exit 0
            ;;
    esac
done

cd "$PROJECT_DIR"

if ! command -v cargo-llvm-cov &> /dev/null; then
    echo "cargo-llvm-cov not found. Install with: cargo install cargo-llvm-cov"
    exit 1
fi

echo "=== bpaf_unsynn Coverage ==="
echo ""

mkdir -p "$COVERAGE_DIR"

# Only include bpaf_unsynn/src/ - exclude bpaf library, tests, rustc, cargo, etc.
IGNORE_REGEX="(^/rustc|/\.cargo/|/target/|/bpaf/src/|tests/|examples/)"

if [ "$OUTPUT_FORMAT" = "lcov" ]; then
    echo "Generating LCOV report..."
    cargo llvm-cov --lcov --output-path "$COVERAGE_DIR/lcov.info" \
        --ignore-filename-regex "$IGNORE_REGEX"

    echo ""
    echo "=== Coverage report generated ==="
    echo "LCOV report: $COVERAGE_DIR/lcov.info"

    # Show summary
    if [ -s "$COVERAGE_DIR/lcov.info" ]; then
        echo ""
        echo "=== Coverage Summary ==="
        TOTAL_LINES=$(grep -c "^DA:" "$COVERAGE_DIR/lcov.info" 2>/dev/null || echo "0")
        COVERED_LINES=$(grep "^DA:" "$COVERAGE_DIR/lcov.info" 2>/dev/null | grep -v ",0$" | wc -l | tr -d ' ')
        if [ "$TOTAL_LINES" -gt 0 ]; then
            PERCENT=$((COVERED_LINES * 100 / TOTAL_LINES))
            echo "Lines: $COVERED_LINES / $TOTAL_LINES ($PERCENT%)"
        fi
    fi
else
    echo "Generating HTML report..."
    cargo llvm-cov --html --output-dir "$COVERAGE_DIR" \
        --ignore-filename-regex "$IGNORE_REGEX" $OPEN_REPORT

    echo ""
    echo "=== Coverage report generated ==="
    echo "HTML report: $COVERAGE_DIR/html/index.html"
fi
