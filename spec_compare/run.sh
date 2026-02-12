#!/bin/bash
# Compare corepc-types structs against the Bitcoin Core OpenRPC specification.
#
# Usage:
#   ./spec_compare/run.sh <version>        Compare a single version (e.g. 30)
#   ./spec_compare/run.sh all              Compare all versions (24-30)
#
# Output (all written to spec_compare/output/):
#   vN_generated.rs  - Structs generated from the OpenRPC spec
#   vN_flattened.rs  - Structs extracted from repo source code (up to vN)
#   vN_compare.txt   - Human-readable comparison report

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

SPECS_DIR="$SCRIPT_DIR/specs"
OUTPUT_DIR="$SCRIPT_DIR/output"

# ── Argument parsing ───────────────────────────────────────────────────────────

if [[ $# -lt 1 ]]; then
    echo "Usage: $0 <version|all>"
    echo ""
    echo "Examples:"
    echo "  $0 30          Compare version 30 only"
    echo "  $0 all         Compare all versions (24-30)"
    exit 1
fi

VERSION_ARG="$1"

if [[ "$VERSION_ARG" == "all" ]]; then
    VERSIONS=(24 25 26 27 28 29 30)
else
    VERSIONS=("$VERSION_ARG")
fi

# ── Setup ──────────────────────────────────────────────────────────────────────

mkdir -p "$OUTPUT_DIR"

# ── Per-version comparison loop ────────────────────────────────────────────────

for VER in "${VERSIONS[@]}"; do
    # Locate the OpenRPC spec file for this version
    OPENRPC_FILE=$(ls "$SPECS_DIR"/v${VER}_*_openrpc.json 2>/dev/null | head -1)
    if [[ -z "$OPENRPC_FILE" ]]; then
        echo "⚠️  No OpenRPC spec found for v${VER} in $SPECS_DIR — skipping."
        continue
    fi

    SPEC_BASENAME=$(basename "$OPENRPC_FILE")
    GENERATED="$OUTPUT_DIR/v${VER}_generated.rs"
    FLATTENED="$OUTPUT_DIR/v${VER}_flattened.rs"
    REPORT="$OUTPUT_DIR/v${VER}_compare.txt"

    echo "━━━ v${VER}: ${SPEC_BASENAME} ━━━"

    # Step 1: Generate structs from the OpenRPC spec via codegen
    python3 "$SCRIPT_DIR/codegen.py" \
        --input "$OPENRPC_FILE" \
        --output "$OUTPUT_DIR" \
        --core-version "${VER}" \
        --single-file

    # codegen writes generated.rs into --output dir; rename to versioned name
    mv "$OUTPUT_DIR/generated.rs" "$GENERATED"

    # Step 2: Flatten repo source code (types/src/v17..vN) into a single file
    python3 "$SCRIPT_DIR/flatten_mods.py" \
        --all-versions \
        --types-dir "$REPO_ROOT/types/src" \
        --up-to-version "${VER}" \
        -o "$FLATTENED" 2>/dev/null

    # Step 3: Run the struct comparison
    python3 "$SCRIPT_DIR/compare_types.py" \
        --all \
        --version "${VER}" \
        --repo "$FLATTENED" \
        --spec "$GENERATED" \
        > "$REPORT" 2>&1

    # Quick summary line
    REPO_COUNT=$(grep -c '^pub struct' "$FLATTENED")
    SPEC_COUNT=$(grep -c '^pub struct' "$GENERATED")
    echo "  ${SPEC_COUNT} spec structs, ${REPO_COUNT} repo structs"
done

# ── Final log ──────────────────────────────────────────────────────────────────

echo ""
echo "════════════════════════════════════════════════════════════════════"
echo "  Comparison complete."
echo ""
echo "  Output files (spec_compare/output/):"
for VER in "${VERSIONS[@]}"; do
    GENERATED="$OUTPUT_DIR/v${VER}_generated.rs"
    FLATTENED="$OUTPUT_DIR/v${VER}_flattened.rs"
    REPORT="$OUTPUT_DIR/v${VER}_compare.txt"
    if [[ -f "$REPORT" ]]; then
        echo "    v${VER}:"
        echo "      codegen  - output/v${VER}_generated.rs"
        echo "      flatten  - output/v${VER}_flattened.rs"
        echo "      report   - output/v${VER}_compare.txt"
    fi
done
echo "════════════════════════════════════════════════════════════════════"

# Print the report(s) to terminal
for VER in "${VERSIONS[@]}"; do
    REPORT="$OUTPUT_DIR/v${VER}_compare.txt"
    if [[ -f "$REPORT" ]]; then
        echo ""
        echo "######################################################################"
        echo "# v${VER} COMPARISON"
        echo "######################################################################"
        cat "$REPORT"
    fi
done
