#!/bin/bash
# Generate files for comparison between implemented types and OpenRPC spec
#
# Usage: ./cmd.sh [--all]
#   --all    Show all differences including likely naming convention noise
#
# Output:
#   generated.rs  - Structs generated from OpenRPC.json spec
#   flattened.rs  - Structs extracted from this repo's source code
#   compare.txt   - Comparison report

set -e

# Parse args
COMPARE_ARGS=""
if [[ "$1" == "--all" ]]; then
    COMPARE_ARGS="--all"
fi

# 1. Generate generated.rs from OpenRPC.json spec
echo "==> Generating generated.rs from OpenRPC.json..."
cd codegen && cargo run -- --input ../OpenRPC.json --output .. --core-version 30 --single-file && cd ..

# 2. Generate flattened.rs from the actual source code (all versions, extracts just structs)
echo "==> Extracting structs from source code..."
python3 flatten_mods.py --all-versions --types-dir types/src -o flattened.rs

sleep 1

# 3. Run comparison
echo "" 
python3 compare_types.py --all --repo flattened.rs --spec generated.rs $COMPARE_ARGS > compare.txt

echo ""
echo "Files generated:"
echo "  generated.rs  - types from OpenRPC.json spec ($(grep -c '^pub struct' generated.rs) structs)"
echo "  flattened.rs  - types from this repo's source code ($(grep -c '^pub struct' flattened.rs) structs)"
echo "  compare.txt   - Comparison report (saved)"
echo ""
echo "For manual diff: diff -u flattened.rs generated.rs | less"
echo "For full output: ./cmd.sh --all"
echo ""

cat compare.txt