#!/bin/bash
# Generate files for comparison between implemented types and OpenRPC spec

set -e

# 1. Generate generated.rs from OpenRPC.json spec
echo "==> Generating generated.rs from OpenRPC.json..."
cd codegen && cargo run -- --input ../OpenRPC.json --output .. --core-version 30 --single-file && cd ..

# 2. Generate flattened.rs from the actual source code (all versions, extracts just structs)
echo "==> Extracting structs from source code..."
python3 flatten_mods.py --all-versions --types-dir types/src -o flattened.rs

# 3. Run comparison
echo "" 
python3 compare_types.py --repo flattened.rs --spec generated.rs  > compare.txt

echo ""
echo "Files generated:"
echo "  generated.rs  - types from OpenRPC.json spec ($(grep -c '^pub struct' generated.rs) structs)"
echo "  flattened.rs  - types from this repo's source code ($(grep -c '^pub struct' flattened.rs) structs)"
echo ""
echo "For manual diff: diff -u flattened.rs generated.rs | less"

cat compare.txt