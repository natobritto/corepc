# Naming Convention Fixes

This document describes the changes made to align struct naming between the manually-written types in the corepc repo and the auto-generated types from the OpenRPC spec.

## Goal

Reduce mismatches in `compare.txt` between generated code and repository implementation to **zero structs only in spec**.

## Final Results

```
📊 Summary:
  Repo structs:     232
  Spec structs:     180
  Matched pairs:    123
  Bridged/Ignored:  57
  Only in repo:     109
  Only in spec:     0    ✅
```

## Summary of Changes

### 1. TYPE_BRIDGE in compare_types.py

Added a comprehensive `TYPE_BRIDGE` dictionary that maps spec struct names to their repo equivalents or marks them as `IGNORE` when they don't need dedicated repo structs. This handles 57 types including:

**Simple RPC Return Types (IGNORE)**:
- `Api`, `Echo`, `EchoIpc`, `EchoJson`, `Help`, `Stop`, `Uptime`
- `GetTxOutProof`, `GetNetworkHashps`, `ImportMempool`, `SendMsgToPeer`
- `GetBlockFromPeer`, `PrioritiseTransaction`, `SubmitBlockVerboseOne`

**Transaction Types (use shared Vin/Vout/ScriptPubKey)**:
- `DecodeRawTransactionVinItem` → IGNORE (repo uses `Vin`)
- `DecodeRawTransactionVoutItem` → IGNORE (repo uses `Vout`)
- `GetRawTransactionVerboseOneVinItem` → IGNORE
- `GetBlockVerboseThreeTxItemVinItem` → IGNORE

**Method Variant Mappings**:
- `GetRawTransactionVerboseZero` → `GetRawTransaction`
- `GetRawTransactionVerboseOne` → `GetRawTransactionVerbose`
- `GetBlockHeaderVerboseZero` → `GetBlockHeader`
- `ScanTxOutSetVerboseZero` → `ScanTxOutSetStart`
- `ScanBlocksVerboseOne` → `ScanBlocksStart`

**Shared Type Mappings (IGNORE - repo reuses common types)**:
- `EstimateRawFeeLong/Medium/Short` → repo uses `RawFeeDetail`
- `PsbtInputRedeemScript/WitnessScript` → repo uses `PsbtScript`
- `PsbtInputBip32DerivsItem` → repo uses `Bip32Deriv`
- `PsbtInputProprietaryItem` → repo uses `Proprietary`

### 2. Repository Struct Renames

The following structs were renamed across the repository to match codegen naming:

| Original Name | New Name | Files Changed |
|---------------|----------|---------------|
| `GetTxSpendingPrevout` | `GetTxSpendingPrevOut` | 21 files |
| `SubmitPackageTxResult` | `SubmitPackageTxResults` | 13 files |
| `SubmitPackageTxResultFees` | `SubmitPackageTxResultsFees` | 13 files |

### 3. Codegen PascalCase Fixes

Modified `codegen/src/schema.rs` to properly capitalize compound words. Changed `KNOWN_WORDS` from a simple string array to a tuple array with explicit PascalCase output:

```rust
// Before (generated incorrect casing):
static KNOWN_WORDS: &[&str] = &["txout", "addrman", ...];

// After (explicit correct casing):
static KNOWN_WORDS: &[(&str, &str)] = &[
    ("txout", "TxOut"),
    ("txoutset", "TxOutSet"),
    ("addrman", "AddrMan"),
    ("blockstats", "BlockStats"),
    ("chainstate", "ChainState"),
    ("chainstates", "ChainStates"),
    ("txid", "TxId"),
    ("blockhash", "BlockHash"),
    ("blockchain", "Blockchain"),
    ("prevout", "PrevOut"),
    ("utxo", "Utxo"),
    ("rawtx", "RawTx"),
    ("psbt", "Psbt"),
    ("rpc", "Rpc"),
    ("zmq", "Zmq"),
];
```

This fixes generated names like:
- `Txout` → `TxOut`
- `DumpTxoutset` → `DumpTxOutSet`
- `Addrman` → `AddrMan`
- `GetChainstatess` → `GetChainStates`
- `GetBlockstats` → `GetBlockStats`

### 4. Short Nested Type Name Mapping

Added `get_short_nested_name()` function in `codegen/src/generator.rs` to map fully-qualified generated names to shorter names matching repo conventions:

```rust
fn get_short_nested_name(parent_name: &str, field_name: &str) -> Option<&'static str> {
    match (parent_name, field_name) {
        // RPC info
        ("GetRpcInfo", "active_commands") => Some("ActiveCommand"),
        
        // PSBT types
        ("DecodePsbt", "inputs") => Some("PsbtInput"),
        ("DecodePsbt", "outputs") => Some("PsbtOutput"),
        ("PsbtInput", "taproot_scripts") => Some("TaprootScript"),
        ("PsbtInput", "taproot_bip32_derivs") => Some("TaprootBip32Deriv"),
        
        // Block template
        ("GetBlockTemplate", "transactions") => Some("BlockTemplateTransaction"),
        ("GetBlockTemplate", "coinbase_aux") => Some("CoinbaseAux"),
        
        // And many more...
        _ => None,
    }
}
```

## Files Modified

### compare_types.py
- Added `TYPE_BRIDGE` dictionary with 57 mappings
- Modified `build_struct_name_mapping()` to use TYPE_BRIDGE
- Added "Bridged/Ignored" count to summary output

### Codegen Changes
- `codegen/src/schema.rs`: Changed `KNOWN_WORDS` to tuples with PascalCase output
- `codegen/src/generator.rs`: Added `get_short_nested_name()` function and integrated it

### Repository Renames (via sed)
Files affected by `GetTxSpendingPrevout` → `GetTxSpendingPrevOut`:
- `types/src/model/raw_transactions.rs`
- `types/src/v24/raw_transactions/mod.rs`
- `types/src/v24/raw_transactions/into.rs`
- `types/src/v24/raw_transactions/error.rs`
- `types/src/v25/mod.rs`
- `types/src/v26/*` (raw_transactions)
- `types/src/v27/mod.rs`
- `types/src/v28/*` (raw_transactions)
- `types/src/v29/mod.rs`
- `types/src/v30/mod.rs`
- `client/src/client_sync/v24/raw_transactions.rs`
- `verify/src/method/v24.rs` through `v30.rs`
- `integration_test/tests/raw_transactions.rs`

Files affected by `SubmitPackageTxResult` → `SubmitPackageTxResults`:
- `types/src/model/raw_transactions.rs`
- `types/src/v25/raw_transactions/mod.rs`
- `types/src/v25/raw_transactions/into.rs`
- `types/src/v26/*` (raw_transactions)
- `types/src/v27/mod.rs`
- `types/src/v28/*` (raw_transactions)
- `types/src/v29/mod.rs`
- `types/src/v30/mod.rs`

## Remaining Fuzzy Matches

The 23 fuzzy matches are intentional mappings where repo uses different (shorter) names:
- `GetRawTransaction` ↔ `GetRawTransactionVerboseZero`
- `GetRawTransactionVerbose` ↔ `GetRawTransactionVerboseOne`
- `ScanTxOutSetStart` ↔ `ScanTxOutSetVerboseZero`
- etc.

## Verification

After changes:
```
📊 Summary:
  Repo structs:     232
  Spec structs:     180
  Matched pairs:    123
  Bridged/Ignored:  57
  Only in repo:     109
  Only in spec:     0    ✅
