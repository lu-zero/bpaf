#!/bin/bash
# Code coverage script for bpaf_unsynn using cargo-llvm-cov
#
# Prerequisites:
#   cargo install cargo-llvm-cov

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
        --http)
            OUTPUT_FORMAT="http"
            ;;
        --help|-h)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Code coverage using cargo-llvm-cov."
            echo ""
            echo "Options:"
            echo "  --lcov    Generate lcov report"
            echo "  --http    Open HTML report in browser after generation"
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

mkdir -p "$COVERAGE_DIR"

# Only include bpaf_unsynn/src/ - exclude bpaf library, tests, rustc, cargo, etc.
IGNORE_REGEX="(^/rustc|/\.cargo/|/target/|/bpaf/src/|tests/|examples/)"
CMD="cargo llvm-cov --ignore-filename-regex $IGNORE_REGEX"
if [ "$OUTPUT_FORMAT" = "http" ]; then
    echo "Generating HTML report..."
    $CMD --html --output-dir "$COVERAGE_DIR" $OPEN_REPORT
elif [ "$OUTPUT_FORMAT" = "lcov" ]; then
    echo "Generating LCOV report..."
    $CMD --lcov --output-path "$COVERAGE_DIR/lcov.info" \

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
    $CMD
fi
