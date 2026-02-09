#!/usr/bin/env python3
"""
Compare struct definitions between generated.rs (from OpenRPC spec) and 
flattened.rs (from repo source code).

This tool helps identify:
1. Structs that exist in one file but not the other
2. Field differences between structs with the same name

Key features:
- Normalizes naming conventions (snake_case vs camelCase)
- Uses serde rename attributes to match fields
- Handles nested type name differences (e.g., PsbtInput vs DecodePsbtInputsItem)
- TYPE_BRIDGE maps spec types to repo types or marks them as IGNORE
"""
import re
import sys
from pathlib import Path
from dataclasses import dataclass, field
from typing import Dict, List, Optional, Set, Tuple


# ============================================================================
# TYPE BRIDGE: Maps spec struct names to repo struct names
# ============================================================================
# Use "IGNORE" to indicate a spec type that doesn't need a repo equivalent
# (e.g., uses bitcoin crate types, or is a simple wrapper, or doesn't exist in repo)
#
# Use "repo_name" to map a spec type to a differently-named repo type
# ============================================================================

TYPE_BRIDGE: Dict[str, str] = {
    # ========================================================================
    # SIMPLE RPC RETURN TYPES (primitives wrapped in tuple structs)
    # These are simple return types that don't need dedicated structs in repo
    # ========================================================================
    "Api": "IGNORE",  # getzmqnotifications returns Vec<ZmqNotification>, not Api
    "Echo": "IGNORE",  # returns same input
    "EchoIpc": "IGNORE",  # returns same input
    "EchoJson": "IGNORE",  # returns same input
    "Help": "IGNORE",  # returns String
    "Stop": "IGNORE",  # returns String
    "Uptime": "IGNORE",  # returns u64
    "GetTxOutProof": "IGNORE",  # returns String (hex proof)
    "GetNetworkHashps": "IGNORE",  # returns f64
    "ImportMempool": "IGNORE",  # returns empty object
    "SendMsgToPeer": "IGNORE",  # returns bool
    "GetBlockFromPeer": "IGNORE",  # returns empty object
    "PrioritiseTransaction": "IGNORE",  # returns bool
    "SubmitBlockVerboseOne": "IGNORE",  # returns null or string

    # ========================================================================
    # DecodeRawTransaction types - repo uses shared RawTransaction/Vin/Vout types
    # These are covered by the bitcoin crate types used in the repo
    # ========================================================================
    "DecodeRawTransactionVinItem": "IGNORE",  # repo uses Vin
    "DecodeRawTransactionVinItemScriptSig": "IGNORE",  # repo uses ScriptSig  
    "DecodeRawTransactionVoutItem": "IGNORE",  # repo uses Vout
    "DecodeRawTransactionVoutItemScriptPubKey": "IGNORE",  # repo uses ScriptPubKey

    # ========================================================================
    # GetRawTransaction verbose types - repo uses shared types
    # ========================================================================
    "GetRawTransactionVerboseZero": "GetRawTransaction",  # tuple struct returning hex
    "GetRawTransactionVerboseOne": "GetRawTransactionVerbose",
    "GetRawTransactionVerboseOneVinItem": "IGNORE",  # uses Vin
    "GetRawTransactionVerboseOneVinItemScriptSig": "IGNORE",  # uses ScriptSig
    "GetRawTransactionVerboseOneVoutItem": "IGNORE",  # uses Vout
    "GetRawTransactionVerboseOneVoutItemScriptPubKey": "IGNORE",  # uses ScriptPubKey
    "GetRawTransactionVerboseTwo": "GetRawTransactionVerboseWithPrevout",
    "GetRawTransactionVerboseTwoVinItem": "IGNORE",  # uses RawTransactionInputWithPrevout
    "GetRawTransactionVerboseTwoVinItemPrevout": "IGNORE",  # uses Prevout
    "GetRawTransactionVerboseTwoVinItemPrevoutScriptPubKey": "IGNORE",  # uses ScriptPubKey

    # ========================================================================
    # GetBlock verbose types - repo uses shared transaction types
    # ========================================================================
    "GetBlockVerboseTwoTxItem": "IGNORE",  # repo uses GetBlockVerboseTwoTransaction
    "GetBlockVerboseThreeTxItem": "IGNORE",  # repo uses GetBlockVerboseThreeTransaction
    "GetBlockVerboseThreeTxItemVinItem": "IGNORE",  # uses Vin types
    "GetBlockVerboseThreeTxItemVinItemPrevout": "IGNORE",  # uses Prevout
    "GetBlockVerboseThreeTxItemVinItemPrevoutScriptPubKey": "IGNORE",  # uses ScriptPubKey

    # ========================================================================
    # GetBlockHeader types
    # ========================================================================
    "GetBlockHeaderVerboseZero": "GetBlockHeader",  # returns hex string
    "GetBlockHeaderVerboseOne": "GetBlockHeaderVerbose",

    # ========================================================================
    # GetBlockTemplate types
    # ========================================================================
    "GetBlockTemplateVerboseOne": "IGNORE",  # proposal mode returns String, repo handles template mode
    "GetBlockTemplateVerboseTwo": "GetBlockTemplate",  # same struct, different rules param
    "GetBlockTemplateVerboseTwoTransactionsItem": "IGNORE",  # uses BlockTemplateTransaction

    # ========================================================================
    # GetTxOut types
    # ========================================================================
    "GetTxOutVerboseOne": "GetTxOut",
    "GetTxOutVerboseOneScriptPubKey": "IGNORE",  # uses ScriptPubKey

    # ========================================================================
    # Mempool types - repo reuses MempoolEntry
    # ========================================================================
    "GetRawMempoolVerboseOne": "GetRawMempool",  # returns Vec<String>
    "GetRawMempoolVerboseTwo": "GetRawMempoolVerbose",  # returns BTreeMap<String, MempoolEntry>
    "GetMempoolAncestorsVerboseOne": "GetMempoolAncestorsVerbose",  # returns BTreeMap
    "GetMempoolDescendantsVerboseOne": "GetMempoolDescendantsVerbose",  # returns BTreeMap

    # ========================================================================
    # ScanTxOutSet types
    # ========================================================================
    "ScanTxOutSetVerboseZero": "ScanTxOutSetStart",
    "ScanTxOutSetVerboseOne": "ScanTxOutSetStatus",
    "ScanTxOutSetVerboseTwo": "ScanTxOutSetAbort",
    "ScanTxOutSetVerboseZeroUnspentsItem": "ScanTxOutSetUnspent",

    # ========================================================================
    # ScanBlocks types  
    # ========================================================================
    "ScanBlocksVerboseOne": "ScanBlocksStart",
    "ScanBlocksVerboseTwo": "ScanBlocksStatus",
    "ScanBlocksVerboseThree": "ScanBlocksAbort",

    # ========================================================================
    # GetMemoryInfo types
    # ========================================================================
    "GetMemoryInfoVerboseZero": "GetMemoryInfoStats",  # returns stats map
    "GetMemoryInfoVerboseOne": "IGNORE",  # returns malloc_info string
    "GetMemoryInfoVerboseZeroLocked": "Locked",  # repo uses Locked struct

    # ========================================================================
    # GetAddrManInfo / GetRawAddrMan types
    # ========================================================================
    "GetAddrManInfoEntry": "AddrManInfoNetwork",  # repo uses AddrManInfoNetwork
    "GetRawAddrManEntryEntry": "IGNORE",  # uses RawAddrManEntry

    # ========================================================================  
    # GetIndexInfo types
    # ========================================================================
    "GetIndexInfoEntry": "GetIndexInfoName",  # repo uses GetIndexInfoName

    # ========================================================================
    # GetMiningInfo types
    # ========================================================================
    "GetMiningInfoNext": "NextBlockInfo",  # repo uses NextBlockInfo

    # ========================================================================
    # EstimateRawFee types - repo uses RawFeeDetail for all fee detail structs
    # These map to the same repo type, so mark them as covered (IGNORE)
    # ========================================================================
    "EstimateRawFeeLong": "IGNORE",  # repo uses RawFeeDetail
    "EstimateRawFeeMedium": "IGNORE",  # repo uses RawFeeDetail
    "EstimateRawFeeShort": "IGNORE",  # repo uses RawFeeDetail (also fuzzy matched)
    "EstimateRawFeeShortPass": "IGNORE",  # repo uses RawFeeRange
    "EstimateRawFeeShortFail": "IGNORE",  # repo uses RawFeeRange

    # ========================================================================
    # SignRawTransactionWithKey types
    # ========================================================================
    "SignRawTransactionWithKey": "IGNORE",  # repo uses SignRawTransaction (fuzzy matched)
    "SignRawTransactionWithKeyErrorsItem": "IGNORE",  # repo uses SignRawTransactionError

    # ========================================================================
    # PSBT types - repo uses shared PsbtScript for redeem/witness scripts
    # ========================================================================
    "DecodePsbtProprietaryItem": "IGNORE",  # repo uses Proprietary
    "PsbtInputRedeemScript": "IGNORE",  # repo uses PsbtScript
    "PsbtInputWitnessScript": "IGNORE",  # repo uses PsbtScript
    "PsbtOutputRedeemScript": "IGNORE",  # repo uses PsbtScript
    "PsbtOutputWitnessScript": "IGNORE",  # repo uses PsbtScript
    "PsbtInputNonWitnessUtxo": "IGNORE",  # repo uses RawTransaction
    "PsbtInputWitnessUtxo": "IGNORE",  # repo uses WitnessUtxo
    "PsbtInputWitnessUtxoScriptPubKey": "IGNORE",  # uses ScriptPubKey
    "PsbtInputBip32DerivsItem": "IGNORE",  # repo uses Bip32Deriv
    "PsbtOutputBip32DerivsItem": "IGNORE",  # repo uses Bip32Deriv
    "PsbtInputProprietaryItem": "IGNORE",  # repo uses Proprietary
    "PsbtOutputProprietaryItem": "IGNORE",  # repo uses Proprietary (fuzzy matched)
    "PsbtInputFinalScriptSig": "IGNORE",  # repo doesn't have separate type

    # ========================================================================
    # SubmitPackage types
    # ========================================================================
    "SubmitPackageTxResultsFees": "IGNORE",  # typo in repo: SubmitPackageTxResultssFees (fuzzy matched)

    # ========================================================================
    # GetTxOutSetInfo types
    # ========================================================================
    "GetTxOutSetInfoBlockInfoUnspendables": "IGNORE",  # repo uses GetTxOutSetInfoUnspendables (fuzzy matched)
}


@dataclass
class StructField:
    name: str
    type_: str
    serde_rename: Optional[str] = None
    is_optional: bool = False
    
    @property
    def json_name(self) -> str:
        """The name as it appears in JSON (uses serde_rename if present)."""
        return self.serde_rename or self.name
    
    @property 
    def normalized_name(self) -> str:
        """Normalized name for comparison (lowercase, no underscores)."""
        return self.json_name.lower().replace('_', '')
    
    def __str__(self) -> str:
        rename = f' (serde: "{self.serde_rename}")' if self.serde_rename else ""
        return f"  {self.name}: {self.type_}{rename}"


@dataclass 
class StructDef:
    name: str
    fields: List[StructField] = field(default_factory=list)
    is_tuple_struct: bool = False
    tuple_type: Optional[str] = None
    
    @property
    def normalized_name(self) -> str:
        """Normalized name for matching (handles naming convention differences)."""
        # Remove common suffixes for matching
        n = self.name
        # Strip Item suffix for matching (codegen adds Item for array elements)
        if n.endswith('Item'):
            n = n[:-4]
        # Strip Entry suffix  
        if n.endswith('Entry'):
            n = n[:-5]
        return n.lower()
    
    def get_field_by_json_name(self, json_name: str) -> Optional[StructField]:
        """Find a field by its JSON name (considering serde rename)."""
        json_name_lower = json_name.lower().replace('_', '')
        for f in self.fields:
            if f.normalized_name == json_name_lower:
                return f
        return None
    
    def __str__(self) -> str:
        if self.is_tuple_struct:
            return f"struct {self.name}({self.tuple_type})"
        return f"struct {self.name} {{ {len(self.fields)} fields }}"


def parse_structs(text: str) -> Dict[str, StructDef]:
    """Parse struct definitions from Rust source text."""
    structs = {}
    lines = text.splitlines()
    
    i = 0
    while i < len(lines):
        line = lines[i]
        
        # Match struct declaration
        # Tuple struct: pub struct Name(pub Type);
        tuple_match = re.match(r'^pub struct (\w+)\s*\((.*)\)\s*;?\s*$', line)
        if tuple_match:
            name = tuple_match.group(1)
            tuple_type = tuple_match.group(2).strip()
            structs[name] = StructDef(
                name=name,
                fields=[],
                is_tuple_struct=True,
                tuple_type=tuple_type
            )
            i += 1
            continue

        # Multi-line tuple struct:
        # pub struct Name(
        #     /// doc
        #     pub Type,
        # );
        tuple_start = re.match(r'^pub struct (\w+)\s*\(\s*$', line)
        if tuple_start:
            name = tuple_start.group(1)
            tuple_types = []
            i += 1
            while i < len(lines):
                l = lines[i].strip()

                # End of tuple struct
                if l == ');' or l.endswith(');'):
                    # Handle inline type before closing if present: "pub Type);"
                    inline = l[:-2].strip()
                    if inline:
                        field_match = re.match(r'(?:pub(?:\([^)]*\))?\s+)?(.+?)\s*,?\s*$', inline)
                        if field_match:
                            tuple_types.append(field_match.group(1).strip())
                    break

                # Skip docs/attrs/blank lines
                if not l or l.startswith('///') or l.startswith('#['):
                    i += 1
                    continue

                field_match = re.match(r'(?:pub(?:\([^)]*\))?\s+)?(.+?)\s*,?\s*$', l)
                if field_match:
                    tuple_types.append(field_match.group(1).strip())

                i += 1

            tuple_type = ', '.join(t for t in tuple_types if t)
            structs[name] = StructDef(
                name=name,
                fields=[],
                is_tuple_struct=True,
                tuple_type=tuple_type if tuple_type else None
            )
            i += 1
            continue
        
        # Regular struct: pub struct Name {
        struct_match = re.match(r'^pub struct (\w+)\s*\{', line)
        if struct_match:
            name = struct_match.group(1)
            fields = []
            serde_rename = None
            i += 1
            
            # Parse fields until closing brace
            while i < len(lines) and lines[i].strip() != '}':
                field_line = lines[i].strip()
                
                # Check for serde rename attribute
                rename_match = re.search(r'#\[serde\(rename\s*=\s*"([^"]+)"', field_line)
                if rename_match:
                    serde_rename = rename_match.group(1)
                    i += 1
                    continue
                
                # Parse field: pub field_name: Type,
                field_match = re.match(r'pub\s+(\w+):\s*(.+?),?\s*$', field_line)
                if field_match:
                    field_name = field_match.group(1)
                    field_type = field_match.group(2).rstrip(',')
                    is_optional = field_type.startswith('Option<')
                    
                    fields.append(StructField(
                        name=field_name,
                        type_=field_type,
                        serde_rename=serde_rename,
                        is_optional=is_optional
                    ))
                    serde_rename = None  # Reset for next field
                
                i += 1
            
            structs[name] = StructDef(name=name, fields=fields)
        
        i += 1
    
    return structs


def normalize_type_for_comparison(type_str: str) -> str:
    """Normalize a type string for comparison (ignoring naming convention differences)."""
    # Remove whitespace
    t = type_str.replace(' ', '')
    # Lowercase
    t = t.lower()
    # Normalize integer types (i64, u64, usize, isize -> int)
    t = re.sub(r'\b(i64|u64|i32|u32|usize|isize)\b', 'int', t)
    # Normalize float types
    t = re.sub(r'\b(f64|f32)\b', 'float', t)
    # Remove std::collections:: prefix
    t = t.replace('std::collections::', '')
    return t


def types_are_compatible(repo_type: str, spec_type: str, repo_structs: Dict[str, StructDef], spec_structs: Dict[str, StructDef]) -> bool:
    """Check if two types are compatible (accounting for naming differences)."""
    # Normalize both types
    r = normalize_type_for_comparison(repo_type)
    s = normalize_type_for_comparison(spec_type)
    
    if r == s:
        return True
    
    # f64 and i64 are often intentionally different (repo uses floats for precision)
    # This is an ACCEPTABLE difference, not a bug
    if {r, s} <= {'int', 'float'}:
        return True
    
    # Handle Vec<X> vs Vec<Y> where X and Y might be differently named structs
    vec_match_r = re.match(r'vec<(\w+)>', r)
    vec_match_s = re.match(r'vec<(\w+)>', s)
    if vec_match_r and vec_match_s:
        inner_r = vec_match_r.group(1)
        inner_s = vec_match_s.group(1)
        # Check if inner types are compatible struct names
        return structs_are_compatible(inner_r, inner_s)
    
    # Handle Option<X> vs Option<Y>
    opt_match_r = re.match(r'option<(.+)>', r)
    opt_match_s = re.match(r'option<(.+)>', s)
    if opt_match_r and opt_match_s:
        return types_are_compatible(opt_match_r.group(1), opt_match_s.group(1), repo_structs, spec_structs)
    
    # Handle Option<X> vs X (optionality difference)
    if opt_match_r and not opt_match_s:
        return types_are_compatible(opt_match_r.group(1), s, repo_structs, spec_structs)
    if opt_match_s and not opt_match_r:
        return types_are_compatible(r, opt_match_s.group(1), repo_structs, spec_structs)
    
    # HashMap vs BTreeMap are compatible
    if 'hashmap' in r and 'btreemap' in s:
        return True
    if 'btreemap' in r and 'hashmap' in s:
        return True
    
    # Check if they're compatible struct names
    return structs_are_compatible(r, s)


def structs_are_compatible(name1: str, name2: str) -> bool:
    """Check if two struct names refer to the same logical type."""
    # Exact match
    if name1 == name2:
        return True
    
    # Normalize names (remove common suffixes like Item, Entry)
    def normalize(n: str) -> str:
        n = n.lower()
        for suffix in ['item', 'entry', 'sitem', 'sentry']:
            if n.endswith(suffix):
                n = n[:-len(suffix)]
        return n
    
    n1 = normalize(name1)
    n2 = normalize(name2)
    
    # Check if one is a substring of the other (handles nested type naming)
    # e.g., "psbtinput" vs "decodepsbtinputsitem" -> both contain "psbtinput"
    return n1 in n2 or n2 in n1


def build_struct_name_mapping(repo_structs: Dict[str, StructDef], 
                               spec_structs: Dict[str, StructDef]) -> Tuple[Dict[str, str], Set[str]]:
    """Build a mapping from repo struct names to spec struct names.
    
    Uses conservative matching to avoid false positives.
    Also uses TYPE_BRIDGE for explicit mappings.
    
    Returns:
        Tuple of (mapping dict, set of ignored spec structs)
    """
    mapping = {}
    ignored_spec = set()
    
    # Pre-pass: Apply TYPE_BRIDGE mappings
    # This maps spec names -> repo names (or IGNORE)
    for spec_name, target in TYPE_BRIDGE.items():
        if spec_name not in spec_structs:
            continue
        if target == "IGNORE":
            ignored_spec.add(spec_name)
        elif target in repo_structs:
            # Map repo -> spec (we store it this way for compatibility)
            mapping[target] = spec_name
    
    # First pass: exact matches (only for non-bridged structs)
    for repo_name in repo_structs:
        if repo_name in mapping:
            continue  # Already mapped via TYPE_BRIDGE
        if repo_name in spec_structs:
            mapping[repo_name] = repo_name
    
    # Second pass: case-insensitive exact matches
    unmatched_repo = set(repo_structs.keys()) - set(mapping.keys())
    unmatched_spec = set(spec_structs.keys()) - set(mapping.values()) - ignored_spec
    
    spec_lower_map = {name.lower(): name for name in unmatched_spec}
    for repo_name in list(unmatched_repo):
        lower = repo_name.lower()
        if lower in spec_lower_map:
            spec_name = spec_lower_map[lower]
            mapping[repo_name] = spec_name
            unmatched_repo.discard(repo_name)
            unmatched_spec.discard(spec_name)
            del spec_lower_map[lower]
    
    # Third pass: pattern-based matching for codegen naming
    # Codegen produces names like GetMethodNameItem, GetMethodNameEntry, etc.
    # Only match if the names are clearly related (not just substring matches)
    for repo_name in list(unmatched_repo):
        repo_lower = repo_name.lower()
        
        best_match = None
        best_score = 0
        
        for spec_name in unmatched_spec:
            spec_lower = spec_name.lower()
            
            # Check if repo name is contained in spec name (after removing common suffixes)
            # Strip "item", "entry", "s" suffixes from both
            def strip_suffixes(name):
                for suffix in ['item', 'entry']:
                    if name.endswith(suffix) and len(name) > len(suffix) + 4:
                        name = name[:-len(suffix)]
                # Also remove trailing 's' if present (for plurals like "Inputs" -> "Input")
                if name.endswith('s') and len(name) > 5:
                    name = name[:-1]
                return name
            
            repo_stripped = strip_suffixes(repo_lower)
            spec_stripped = strip_suffixes(spec_lower)
            
            # Exact match after stripping (highest confidence)
            if repo_stripped == spec_stripped:
                score = 100
            # Spec ends with repo name (e.g., "GlobalXpub" matches "DecodePsbtGlobalXpubsItem")
            # But require significant length to avoid short substring false positives
            elif spec_stripped.endswith(repo_stripped) and len(repo_stripped) >= 10:
                score = 90
            # Repo ends with spec name
            elif repo_stripped.endswith(spec_stripped) and len(spec_stripped) >= 10:
                score = 90
            else:
                continue
            
            if score > best_score:
                best_score = score
                best_match = spec_name
        
        if best_match and best_score >= 90:
            mapping[repo_name] = best_match
            unmatched_repo.discard(repo_name)
            unmatched_spec.discard(best_match)
    
    # Fourth pass: field-based matching (only for very high overlap)
    # DISABLED - field-based matching produces too many false positives
    # Only use name-based matching for accurate results
    
    return mapping, ignored_spec


def compare_structs(repo_structs: Dict[str, StructDef], 
                    spec_structs: Dict[str, StructDef],
                    show_all: bool = False) -> None:
    """Compare struct definitions and print differences.
    
    Args:
        repo_structs: Structs from repository source
        spec_structs: Structs from OpenRPC spec
        show_all: If True, show all differences. If False, only significant ones.
    """
    
    # Build name mapping
    name_mapping, ignored_spec = build_struct_name_mapping(repo_structs, spec_structs)
    
    # Categorize structs
    matched_pairs = []  # (repo_name, spec_name)
    only_in_repo = []
    only_in_spec = set(spec_structs.keys()) - ignored_spec  # Exclude ignored specs
    
    for repo_name, repo_s in repo_structs.items():
        if repo_name in name_mapping:
            spec_name = name_mapping[repo_name]
            matched_pairs.append((repo_name, spec_name))
            only_in_spec.discard(spec_name)
        else:
            only_in_repo.append(repo_name)
    
    only_in_spec = sorted(only_in_spec)
    only_in_repo = sorted(only_in_repo)
    
    print("=" * 70)
    print("STRUCT COMPARISON REPORT")
    print("=" * 70)
    
    print(f"\n📊 Summary:")
    print(f"  Repo structs:     {len(repo_structs)}")
    print(f"  Spec structs:     {len(spec_structs)}")
    print(f"  Matched pairs:    {len(matched_pairs)}")
    print(f"  Bridged/Ignored:  {len(ignored_spec)}")
    print(f"  Only in repo:     {len(only_in_repo)}")
    print(f"  Only in spec:     {len(only_in_spec)}")
    
    # Analyze field differences for matched pairs
    field_diffs = []
    for repo_name, spec_name in matched_pairs:
        repo_s = repo_structs[repo_name]
        spec_s = spec_structs[spec_name]
        
        # Skip tuple structs for now
        if repo_s.is_tuple_struct or spec_s.is_tuple_struct:
            continue
        
        # Build field maps using normalized JSON names
        repo_fields = {f.normalized_name: f for f in repo_s.fields}
        spec_fields = {f.normalized_name: f for f in spec_s.fields}
        
        repo_field_names = set(repo_fields.keys())
        spec_field_names = set(spec_fields.keys())
        
        missing_in_repo = spec_field_names - repo_field_names
        extra_in_repo = repo_field_names - spec_field_names
        
        # Check for type differences in common fields
        type_diffs = []
        for fname in repo_field_names & spec_field_names:
            rf = repo_fields[fname]
            sf = spec_fields[fname]
            if not types_are_compatible(rf.type_, sf.type_, repo_structs, spec_structs):
                type_diffs.append((rf.name, rf.type_, sf.type_))
        
        if missing_in_repo or extra_in_repo or type_diffs:
            # Use original field names for display
            missing_display = [spec_fields[n].name for n in missing_in_repo]
            extra_display = [repo_fields[n].name for n in extra_in_repo]
            field_diffs.append((repo_name, spec_name, missing_display, extra_display, type_diffs))
    
    # Filter out "noise" structs that are likely intentional naming differences
    # These are structs that exist in repo but have a codegen-style equivalent
    significant_repo_only = []
    naming_diff_repo = []
    for name in only_in_repo:
        # Check if there's a similar name in spec (potential naming convention diff)
        has_similar = False
        name_lower = name.lower()
        for spec_name in only_in_spec:
            spec_lower = spec_name.lower()
            # Check for common patterns
            if name_lower in spec_lower or spec_lower in name_lower:
                has_similar = True
                break
            # Check without suffixes
            name_base = name_lower.rstrip('s')
            spec_base = spec_lower.rstrip('s')
            if len(name_base) > 8 and (name_base in spec_base or spec_base in name_base):
                has_similar = True
                break
        if has_similar:
            naming_diff_repo.append(name)
        else:
            significant_repo_only.append(name)
    
    # Similar for spec-only structs
    significant_spec_only = []
    naming_diff_spec = []
    for name in only_in_spec:
        # Check if it's a nested type (contains "Item" and method prefix)
        if 'Item' in name and any(name.startswith(prefix) for prefix in 
            ['Get', 'List', 'Decode', 'Create', 'Sign', 'Submit', 'Scan', 'Estimate', 'Analyze']):
            naming_diff_spec.append(name)
        else:
            # Check if there's a similar name in repo
            has_similar = False
            name_lower = name.lower()
            for repo_name in only_in_repo:
                repo_lower = repo_name.lower()
                if name_lower in repo_lower or repo_lower in name_lower:
                    has_similar = True
                    break
            if has_similar:
                naming_diff_spec.append(name)
            else:
                significant_spec_only.append(name)
    
    if only_in_repo:
        print(f"\n🔵 Structs only in REPO ({len(only_in_repo)}):")
        if show_all:
            for name in only_in_repo:
                print(f"  - {name}")
        else:
            print(f"  (Showing {len(significant_repo_only)} significant, {len(naming_diff_repo)} likely naming diffs)")
            for name in significant_repo_only:
                print(f"  - {name}")
    
    if only_in_spec:
        print(f"\n🟡 Structs only in SPEC ({len(only_in_spec)}) - may need implementation:")
        if show_all:
            for name in only_in_spec:
                print(f"  + {name}")
        else:
            print(f"  (Showing {len(significant_spec_only)} significant, {len(naming_diff_spec)} likely naming diffs)")
            for name in significant_spec_only:
                print(f"  + {name}")
    
    if field_diffs:
        print(f"\n🔄 Structs with FIELD DIFFERENCES ({len(field_diffs)}):")
        for repo_name, spec_name, missing, extra, type_diffs in field_diffs:
            name_display = repo_name if repo_name == spec_name else f"{repo_name} ↔ {spec_name}"
            print(f"\n  {name_display}:")
            if missing:
                print(f"    Missing in repo: {', '.join(sorted(missing))}")
            if extra:
                print(f"    Extra in repo:   {', '.join(sorted(extra))}")
            if type_diffs:
                for fname, rtype, stype in type_diffs:
                    print(f"    Type diff '{fname}': repo={rtype} vs spec={stype}")
    
    # Show name mappings that weren't exact (filter out likely false positives)
    fuzzy_matches = [(r, s) for r, s in matched_pairs if r != s]
    
    # Filter out fuzzy matches with low field overlap (likely false positives)
    validated_fuzzy = []
    for repo_name, spec_name in fuzzy_matches:
        repo_s = repo_structs[repo_name]
        spec_s = spec_structs[spec_name]
        
        # If either is a tuple struct, keep the match (can't validate by fields)
        if repo_s.is_tuple_struct or spec_s.is_tuple_struct:
            validated_fuzzy.append((repo_name, spec_name))
            continue
        
        # Check field overlap
        repo_fields = {f.normalized_name for f in repo_s.fields}
        spec_fields = {f.normalized_name for f in spec_s.fields}
        
        overlap = len(repo_fields & spec_fields)
        total_repo = len(repo_fields)
        total_spec = len(spec_fields)
        
        # Keep if names are case-insensitive match OR there's significant overlap
        if repo_name.lower() == spec_name.lower():
            validated_fuzzy.append((repo_name, spec_name))
        elif total_repo > 0 and total_spec > 0:
            # Require at least 50% overlap on the smaller struct
            min_fields = min(total_repo, total_spec)
            if min_fields > 0 and overlap / min_fields >= 0.5:
                validated_fuzzy.append((repo_name, spec_name))
    
    if validated_fuzzy:
        print(f"\n🔗 Fuzzy name matches ({len(validated_fuzzy)}):")
        for repo_name, spec_name in sorted(validated_fuzzy):
            print(f"  {repo_name} ↔ {spec_name}")
    
    print("\n" + "=" * 70)


def main():
    import argparse
    ap = argparse.ArgumentParser(description="Compare struct definitions between files")
    ap.add_argument("--repo", default="flattened.rs", help="File with repo struct definitions")
    ap.add_argument("--spec", default="generated.rs", help="File with spec struct definitions")
    ap.add_argument("--all", action="store_true", help="Show all differences (including noise)")
    args = ap.parse_args()
    
    repo_text = Path(args.repo).read_text(encoding="utf-8")
    spec_text = Path(args.spec).read_text(encoding="utf-8")
    
    repo_structs = parse_structs(repo_text)
    spec_structs = parse_structs(spec_text)
    
    compare_structs(repo_structs, spec_structs, show_all=args.all)
    return 0


if __name__ == "__main__":
    sys.exit(main())
