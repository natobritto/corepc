# Naming Convention Fixes

This document describes the changes made to align struct naming between the manually-written types in the corepc repo and the auto-generated types from the OpenRPC spec.

## Goal

Reduce mismatches in `compare.txt` between generated code and repository implementation. Started with 92 matched pairs and 30 fuzzy matches; ended with 100 matched pairs and 1 fuzzy match (false positive).

## Summary of Changes

### 1. Repository Struct Renames

The following structs were renamed across the repository to match codegen naming:

| Original Name | New Name | Files Changed |
|---------------|----------|---------------|
| `GetTxSpendingPrevout` | `GetTxSpendingPrevOut` | 21 files |
| `SubmitPackageTxResult` | `SubmitPackageTxResults` | 13 files |
| `SubmitPackageTxResultFees` | `SubmitPackageTxResultsFees` | 13 files |

### 2. Codegen PascalCase Fixes

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

### 3. Short Nested Type Name Mapping

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

This allows generated nested types to use short names like:
- `GetRpcInfoActiveCommandsItem` → `ActiveCommand`
- `DecodePsbtInputsItem` → `PsbtInput`
- `GetBlockTemplateTransactionsItem` → `BlockTemplateTransaction`

## Files Modified

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

## Remaining Fuzzy Match

The one remaining fuzzy match is a **false positive**:
- `TransactionItem` (wallet transaction detail struct)
- `PrioritiseTransaction` (bool wrapper for prioritisetransaction RPC result)

These are completely different structs that happen to have similar-sounding names.

## Verification

After changes:
```
=== Summary ===
Matched pairs: 100
Fuzzy matches: 1
```

To verify codegen changes:
```bash
cd codegen && cargo test
```

To regenerate and compare:
```bash
./cmd.sh
```
