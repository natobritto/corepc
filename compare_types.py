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
- TYPE_BRIDGE maps spec types to repo types or marks them as IGNORE
- NO fuzzy matching - all mappings are explicit
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
    # CASE SENSITIVITY FIXES (spec uses lowercase where repo uses PascalCase)
    # ========================================================================
    "DumpTxoutSet": "DumpTxOutSet",
    "GetAddrmanInfo": "GetAddrManInfo",
    "GetAddressesbyLabel": "GetAddressesByLabel",
    "GetBlockstats": "GetBlockStats",
    "GetChainstates": "GetChainStates",
    "GetChainTxstats": "GetChainTxStats",
    "GetRawAddrman": "GetRawAddrMan",
    "GetReceivedbyAddress": "GetReceivedByAddress",
    "GetReceivedbyLabel": "GetReceivedByLabel",
    "GetTxoutSetInfo": "GetTxOutSetInfo",
    "GetTxoutSetInfoBlockInfo": "GetTxOutSetInfoBlockInfo",
    "GetTxspendingprevOut": "GetTxSpendingPrevOut",
    "ListReceivedbyAddress": "ListReceivedByAddress",
    "ListReceivedbyLabel": "ListReceivedByLabel",
    "LoadTxoutSet": "LoadTxOutSet",
    "SetNetworkactive": "SetNetworkActive",
    "SetWalletflag": "SetWalletFlag",
    "SignMessagewithPrivKey": "SignMessageWithPrivKey",
    "TestMempoolaccept": "TestMempoolAccept",
    "VerifyTxoutProof": "VerifyTxOutProof",
    # ========================================================================
    # ARRAY ITEM TYPE MAPPINGS (spec generates FooItem, repo uses different names)
    # ========================================================================
    # GetAddedNodeInfo: spec generates wrapper + item
    "GetAddedNodeInfoItem": "AddedNode",
    "GetAddedNodeInfoItemAddressesItem": "AddedNodeAddress",
    # GetChainTips: spec generates wrapper + item
    "GetChainTipsItem": "ChainTips",
    # GetPeerInfo: spec generates wrapper + item
    "GetPeerInfoItem": "PeerInfo",
    # ListBanned: spec generates wrapper + item
    "ListBannedItem": "Banned",
    # GetNodeAddresses: spec generates wrapper + item
    "GetNodeAddressesItem": "NodeAddress",
    # GetTxSpendingPrevOut: spec generates wrapper + item
    "GetTxspendingprevOutItem": "GetTxSpendingPrevOutItem",
    # TestMempoolAccept: spec generates wrapper + item
    "TestMempoolacceptItem": "MempoolAcceptance",
    "TestMempoolacceptItemFees": "MempoolAcceptanceFees",
    # ImportDescriptors: spec generates wrapper + item
    "ImportDescriptorsItem": "ImportDescriptorsResult",
    "ImportDescriptorsItemError": "IGNORE",  # spec emits an empty placeholder; repo keeps generic error JSON
    # ListReceivedByAddress/Label: spec generates wrapper + item
    "ListReceivedbyAddressItem": "ListReceivedByAddressItem",
    "ListReceivedbyLabelItem": "ListReceivedByLabelItem",
    # ListTransactions: spec generates wrapper + item
    "ListTransactionsItem": "TransactionItem",
    # GetHdKeys: spec generates wrapper + item
    "GetHdKeysItem": "HdKey",
    "GetHdKeysItemDescriptorsItem": "HdKeyDescriptor",
    # ========================================================================
    # WALLET TYPES
    # ========================================================================
    "DeriveAddressesVerboseZero": "DeriveAddresses",
    "DeriveAddressesVerboseOne": "DeriveAddressesMultipath",
    "UnloadWallet": "UnloadWallet",
    # ========================================================================
    # MEMPOOL TYPES
    # ========================================================================
    # Mempool entry is a shared type used by GetMempoolEntry, GetRawMempoolVerbose, etc.
    "GetMempoolEntryFees": "MempoolEntryFees",
    # The *Entry structs inside the map types all map to the repo's shared MempoolEntry
    "GetMempoolAncestorsVerboseOneEntry": "MempoolEntry",
    "GetMempoolDescendantsVerboseOneEntry": "MempoolEntry",
    "GetRawMempoolVerboseOneEntry": "MempoolEntry",
    # Fees inside those entries are all MempoolEntryFees
    "GetMempoolAncestorsVerboseOneEntryFees": "MempoolEntryFees",
    "GetMempoolDescendantsVerboseOneEntryFees": "MempoolEntryFees",
    "GetRawMempoolVerboseOneEntryFees": "MempoolEntryFees",
    # Mempool verbose=false -> Vec<String> wrappers
    "GetMempoolAncestorsVerboseZero": "GetMempoolAncestors",
    "GetMempoolDescendantsVerboseZero": "GetMempoolDescendants",
    "GetRawMempoolVerboseZero": "GetRawMempool",
    # Mempool verbose=true -> BTreeMap wrappers
    "GetMempoolAncestorsVerboseOne": "GetMempoolAncestorsVerbose",
    "GetMempoolDescendantsVerboseOne": "GetMempoolDescendantsVerbose",
    "GetRawMempoolVerboseOne": "GetRawMempoolVerbose",
    # GetRawMempool sequence variant
    "GetRawMempoolVerboseTwo": "GetRawMempoolSequence",
    # Orphan transaction types
    "GetOrphanTxsVerboseZero": "GetOrphanTxs",
    "GetOrphanTxsVerboseOneItem": "GetOrphanTxsVerboseOneEntry",
    "GetOrphanTxsVerboseTwoItem": "GetOrphanTxsVerboseTwoEntry",
    # ========================================================================
    # DEPLOYMENT/SOFTFORK TYPES
    # ========================================================================
    "GetDeploymentInfoDeployments": "DeploymentInfo",
    "GetDeploymentInfoDeploymentsBip9": "Bip9Info",
    "GetDeploymentInfoDeploymentsBip9Statistics": "Bip9Statistics",
    # ========================================================================
    # BLOCK VERBOSE NESTED TYPES
    # ========================================================================
    "GetBlockVerboseTwoTxItem": "GetBlockVerboseTwoTransaction",
    "GetBlockVerboseThreeTxItem": "GetBlockVerboseThreeTransaction",
    "GetBlockVerboseThreeTxItemVinItem": "RawTransactionInputWithPrevout",
    "GetBlockVerboseThreeTxItemVinItemPrevout": "GetBlockVerboseThreePrevout",
    "GetBlockVerboseThreeTxItemVinItemPrevoutScriptPubKey": "IGNORE",
    # ========================================================================
    # BLOCK TEMPLATE TYPES
    # ========================================================================
    "GetBlocktemplateVerboseOne": "IGNORE",  # proposal mode
    "GetBlocktemplateVerboseTwo": "GetBlockTemplate",
    "GetBlocktemplateVerboseTwoTransactionsItem": "BlockTemplateTransaction",
    # ========================================================================
    # ESTIMATE RAW FEE TYPES
    # ========================================================================
    "EstimateRawFeeShort": "RawFeeDetail",
    "EstimateRawFeeMedium": "IGNORE",  # spec emits an empty placeholder object
    "EstimateRawFeeLong": "IGNORE",  # spec emits an empty placeholder object
    "EstimateRawFeeShortPass": "RawFeeRange",
    "EstimateRawFeeShortFail": "IGNORE",  # spec emits an empty placeholder object
    # ========================================================================
    # RAW TRANSACTION TYPES
    # ========================================================================
    "GetBlockHeaderVerboseZero": "GetBlockHeader",
    "GetBlockHeaderVerboseOne": "GetBlockHeaderVerbose",
    "GetRawTransactionVerboseZero": "GetRawTransaction",
    "GetRawTransactionVerboseOne": "GetRawTransactionVerbose",
    "GetRawTransactionVerboseTwo": "GetRawTransactionVerboseWithPrevout",
    "GetRawTransactionVerboseOneVinItem": "IGNORE",
    "GetRawTransactionVerboseOneVinItemScriptSig": "IGNORE",
    "GetRawTransactionVerboseOneVoutItem": "IGNORE",
    "GetRawTransactionVerboseOneVoutItemScriptPubKey": "IGNORE",
    "GetRawTransactionVerboseTwoVinItem": "IGNORE",
    "GetRawTransactionVerboseTwoVinItemPrevout": "GetBlockVerboseThreePrevout",
    "GetRawTransactionVerboseTwoVinItemPrevoutScriptPubKey": "IGNORE",
    # GetTxOut
    "GetTxoutVerboseOne": "GetTxOut",
    "GetTxoutVerboseOneScriptPubKey": "IGNORE",
    # ========================================================================
    # SCAN TYPES
    # ========================================================================
    "ScanTxoutSetVerboseZero": "ScanTxOutSetStart",
    "ScanTxoutSetVerboseOne": "ScanTxOutSetStatus",
    "ScanTxoutSetVerboseTwo": "ScanTxOutSetAbort",
    "ScanTxoutSetVerboseZeroUnspentsItem": "ScanTxOutSetUnspent",
    "ScanBlocksVerboseOne": "ScanBlocksStart",
    "ScanBlocksVerboseTwo": "ScanBlocksStatus",
    "ScanBlocksVerboseThree": "ScanBlocksAbort",
    # ========================================================================
    # MEMORY INFO TYPES
    # ========================================================================
    "GetMemoryInfoVerboseZero": "GetMemoryInfoStats",
    "GetMemoryInfoVerboseOne": "IGNORE",
    "GetMemoryInfoVerboseZeroLocked": "Locked",
    # ========================================================================
    # SEND TYPES
    # ========================================================================
    "SendToAddressVerboseZero": "SendToAddress",
    "SendToAddressVerboseOne": "SendManyVerbose",
    "SendmanyVerboseZero": "SendMany",
    "SendmanyVerboseOne": "SendManyVerbose",
    # ========================================================================
    # SIGN RAW TRANSACTION TYPES
    # ========================================================================
    "SignRawTransactionwithKey": "SignRawTransaction",
    "SignRawTransactionwithKeyErrorsItem": "SignFail",
    "SignRawTransactionwithWallet": "SignRawTransaction",
    "SignRawTransactionwithWalletErrorsItem": "SignFail",
    # ========================================================================
    # PSBT TYPES - repo uses shared types
    # ========================================================================
    "DecodePsbtInputsItem": "PsbtInput",
    "DecodePsbtInputsItemBip32DerivsItem": "IGNORE",
    "DecodePsbtInputsItemFinalScriptSig": "IGNORE",
    "DecodePsbtInputsItemNonWitnessUtxo": "IGNORE",
    "DecodePsbtInputsItemProprietaryItem": "Proprietary",
    "DecodePsbtInputsItemRedeemScript": "IGNORE",
    "DecodePsbtInputsItemTaprootBip32DerivsItem": "TaprootBip32Deriv",
    "DecodePsbtInputsItemWitnessScript": "IGNORE",
    "DecodePsbtInputsItemWitnessUtxo": "IGNORE",
    "DecodePsbtInputsItemWitnessUtxoScriptPubKey": "IGNORE",
    "DecodePsbtInputsItemTaprootScriptsItem": "TaprootScript",
    "DecodePsbtInputsItemTaprootScriptPathSigsItem": "TaprootScriptPathSig",
    "DecodePsbtInputsItemMusig2PartialSigsItem": "Musig2PartialSig",
    "DecodePsbtInputsItemMusig2ParticipantPubkeysItem": "Musig2ParticipantPubkeys",
    "DecodePsbtInputsItemMusig2PubnoncesItem": "Musig2Pubnonce",
    "DecodePsbtOutputsItem": "PsbtOutput",
    "DecodePsbtOutputsItemBip32DerivsItem": "IGNORE",
    "DecodePsbtOutputsItemMusig2ParticipantPubkeysItem": "Musig2ParticipantPubkeys",
    "DecodePsbtOutputsItemRedeemScript": "IGNORE",
    "DecodePsbtOutputsItemTaprootTreeItem": "TaprootLeaf",
    "DecodePsbtOutputsItemWitnessScript": "IGNORE",
    "DecodePsbtOutputsItemProprietaryItem": "Proprietary",
    "DecodePsbtOutputsItemTaprootBip32DerivsItem": "TaprootBip32Deriv",
    "DecodePsbtProprietaryItem": "Proprietary",
    "DecodePsbtGlobalXpubsItem": "GlobalXpub",
    "AnalyzePsbtInputsItem": "AnalyzePsbtInput",
    "AnalyzePsbtInputsItemMissing": "AnalyzePsbtInputMissing",
    # ========================================================================
    # DECODE RAW TRANSACTION TYPES
    # ========================================================================
    "DecodeRawTransactionVinItem": "IGNORE",
    "DecodeRawTransactionVinItemScriptSig": "IGNORE",
    "DecodeRawTransactionVoutItem": "IGNORE",
    "DecodeRawTransactionVoutItemScriptPubKey": "IGNORE",
    # ========================================================================
    # NESTED TYPE MAPPINGS (spec nested items -> repo shared types)
    # ========================================================================
    "GetRpcInfoActiveCommandsItem": "ActiveCommand",
    "GetChainstatesChainstatesItem": "ChainState",
    "GetAddrmanInfoEntry": "AddrManInfoNetwork",
    "GetRawAddrmanEntryEntry": "RawAddrManEntry",
    "GetIndexInfoEntry": "GetIndexInfoName",
    "GetMiningInfoNext": "NextBlockInfo",
    "GetNetworkInfoLocaladdressesItem": "GetNetworkInfoAddress",
    "GetNetworkInfoNetworksItem": "GetNetworkInfoNetwork",
    "GetNetTotalsUploadtarget": "UploadTarget",
    "GetTransactionLastprocessedblock": "LastProcessedBlock",
    "GetTransactionDetailsItem": "GetTransactionDetail",
    "GetBalancesLastprocessedblock": "LastProcessedBlock",
    "GetWalletInfoLastprocessedblock": "LastProcessedBlock",
    "GetWalletInfoScanning": "IGNORE",
    "GetAddressesbyLabelEntry": "AddressInformation",
    "ListWalletDirWalletsItem": "ListWalletDirWallet",
    "ListDescriptorsDescriptorsItem": "DescriptorInfo",
    "ListSinceBlockTransactionsItem": "TransactionItem",
    "GetPrioritisedTransactionsEntry": "PrioritisedTransaction",
    "EnumerateSignersSignersItem": "Signers",
    "GetTxoutSetInfoBlockInfoUnspendables": "GetTxOutSetInfoUnspendables",
    "SubmitPackageTxResultsFees": "SubmitPackageTxResultssFees",
    # ========================================================================
    # SIMPLE RPC RETURN TYPES (primitives, not structs in repo)
    # ========================================================================
    "Api": "IGNORE",
    "Echo": "IGNORE",
    "EchoIpc": "IGNORE",
    "EchoJson": "IGNORE",
    "Help": "IGNORE",
    "Stop": "IGNORE",
    "Uptime": "IGNORE",
    "GetTxoutProof": "IGNORE",
    "GetNetworkHashps": "IGNORE",
    "ImportMempool": "IGNORE",
    "SendmsgToPeer": "IGNORE",
    "GetBlockFromPeer": "IGNORE",
    "PrioritiseTransaction": "IGNORE",
    "SubmitBlockVerboseOne": "IGNORE",
}

# ============================================================================
# Repo-only types to exclude from the "only in REPO" report.
# These are types that exist in the repo's versioned modules (v17-v28) but
# have no counterpart in the v30 OpenRPC spec because the RPC was removed,
# deprecated, or the type is a hand-crafted typed representation of untyped
# spec data.
# ============================================================================
REPO_ONLY_IGNORE: set = {
    # Deprecated/removed RPC methods (not in v30 spec)
    "AddMultisigAddress",  # addmultisigaddress removed
    "DumpWallet",  # dumpwallet removed
    "GetZmqNotifications",  # getzmqnotifications not in spec
    "ImportMulti",  # importmulti removed (replaced by importdescriptors)
    "ImportMultiEntry",  # part of importmulti
    "JsonRpcError",  # internal helper type, not an RPC result
    "UpgradeWallet",  # upgradewallet removed
    # Legacy softfork types (pre-v21 getblockchaininfo restructuring)
    "Bip9Softfork",  # replaced by DeploymentInfo/Bip9Info
    "Bip9SoftforkInfo",  # replaced by Bip9Info
    "Bip9SoftforkStatistics",  # replaced by Bip9Statistics
    "Softfork",  # replaced by DeploymentInfo
    "SoftforkReject",  # v17 only
    # Deprecated wallet types
    "GetAddressInfoLabel",  # v17 only (labels changed format)
    "GetBalancesWatchOnly",  # watch-only removed in descriptor wallets
    # Hand-crafted typed representations of untyped spec data
    "ReceiveActivity",  # spec uses Vec<serde_json::Value> for activity
    "SpendActivity",  # spec uses Vec<serde_json::Value> for activity
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
        return self.json_name.lower().replace("_", "")

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
        if n.endswith("Item"):
            n = n[:-4]
        # Strip Entry suffix
        if n.endswith("Entry"):
            n = n[:-5]
        return n.lower()

    def get_field_by_json_name(self, json_name: str) -> Optional[StructField]:
        """Find a field by its JSON name (considering serde rename)."""
        json_name_lower = json_name.lower().replace("_", "")
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

        # Skip empty lines, comments, and attributes
        stripped = line.strip()
        if not stripped or stripped.startswith("//") or stripped.startswith("#["):
            i += 1
            continue

        # Match struct declaration
        # Tuple struct: pub struct Name(pub Type); or pub struct Name(pub Type); // comment
        # Strip inline comments first
        line_no_comment = re.sub(r"//.*$", "", line)
        tuple_match = re.match(
            r"^pub struct (\w+)\s*\((.*)\)\s*;?\s*$", line_no_comment
        )
        if tuple_match:
            name = tuple_match.group(1)
            tuple_type = tuple_match.group(2).strip()
            structs[name] = StructDef(
                name=name, fields=[], is_tuple_struct=True, tuple_type=tuple_type
            )
            i += 1
            continue

        # Multi-line tuple struct:
        # pub struct Name(
        #     /// doc
        #     pub Type,
        # );
        tuple_start = re.match(r"^pub struct (\w+)\s*\(\s*$", line)
        if tuple_start:
            name = tuple_start.group(1)
            tuple_types = []
            i += 1
            while i < len(lines):
                l = lines[i].strip()

                # End of tuple struct
                if l == ");" or l.endswith(");"):
                    # Handle inline type before closing if present: "pub Type);"
                    inline = l[:-2].strip()
                    if inline:
                        field_match = re.match(
                            r"(?:pub(?:\([^)]*\))?\s+)?(.+?)\s*,?\s*$", inline
                        )
                        if field_match:
                            tuple_types.append(field_match.group(1).strip())
                    break

                # Skip docs/attrs/blank lines
                if not l or l.startswith("///") or l.startswith("#["):
                    i += 1
                    continue

                field_match = re.match(r"(?:pub(?:\([^)]*\))?\s+)?(.+?)\s*,?\s*$", l)
                if field_match:
                    tuple_types.append(field_match.group(1).strip())

                i += 1

            tuple_type = ", ".join(t for t in tuple_types if t)
            structs[name] = StructDef(
                name=name,
                fields=[],
                is_tuple_struct=True,
                tuple_type=tuple_type if tuple_type else None,
            )
            i += 1
            continue

        # Regular struct: pub struct Name {
        struct_match = re.match(r"^pub struct (\w+)\s*\{", line)
        if struct_match:
            name = struct_match.group(1)
            fields = []
            serde_rename = None
            i += 1

            # Parse fields until closing brace
            while i < len(lines):
                current_line = lines[i].strip()

                # Check for closing brace first
                if current_line == "}":
                    break

                # Check if we hit a new struct declaration (malformed input - missing closing brace)
                if current_line.startswith("pub struct ") and "{" in current_line:
                    # Don't increment i - let outer loop handle this struct
                    break

                # Skip empty lines, doc comments, and non-serde attributes
                if (
                    not current_line
                    or current_line.startswith("///")
                    or (
                        current_line.startswith("#[")
                        and "serde(rename" not in current_line
                    )
                ):
                    i += 1
                    continue

                # Check for serde rename attribute
                rename_match = re.search(
                    r'#\[serde\(rename\s*=\s*"([^"]+)"', current_line
                )
                if rename_match:
                    serde_rename = rename_match.group(1)
                    i += 1
                    continue

                # Parse field: pub field_name: Type,
                # Strip inline comments first
                field_line_no_comment = re.sub(r"//.*$", "", current_line)
                field_match = re.match(
                    r"pub\s+(\w+):\s*(.+?),?\s*$", field_line_no_comment
                )
                if field_match:
                    field_name = field_match.group(1)
                    field_type = field_match.group(2).rstrip(",")
                    is_optional = field_type.startswith("Option<")

                    fields.append(
                        StructField(
                            name=field_name,
                            type_=field_type,
                            serde_rename=serde_rename,
                            is_optional=is_optional,
                        )
                    )
                    serde_rename = None  # Reset for next field

                i += 1

            # Check if we exited because of closing brace or new struct
            if i < len(lines) and lines[i].strip() == "}":
                i += 1  # Move past the closing brace
            # else: we hit a new struct, don't increment - let outer loop handle it

            structs[name] = StructDef(name=name, fields=fields)
            continue  # Don't increment again at the end of the loop

        i += 1

    return structs


def normalize_type_for_comparison(type_str: str) -> str:
    """Normalize a type string for comparison (ignoring naming convention differences)."""
    # Remove whitespace
    t = type_str.replace(" ", "")
    # Lowercase
    t = t.lower()
    # Normalize integer types (i64, u64, usize, isize -> int)
    t = re.sub(r"\b(i64|u64|i32|u32|usize|isize)\b", "int", t)
    # Normalize float types
    t = re.sub(r"\b(f64|f32)\b", "float", t)
    # Remove std::collections:: prefix
    t = t.replace("std::collections::", "")
    return t


def types_are_compatible(
    repo_type: str,
    spec_type: str,
    repo_structs: Dict[str, StructDef],
    spec_structs: Dict[str, StructDef],
) -> bool:
    """Check if two types are compatible (accounting for naming differences)."""
    # Normalize both types
    r = normalize_type_for_comparison(repo_type)
    s = normalize_type_for_comparison(spec_type)

    if r == s:
        return True

    # f64 and i64 are often intentionally different (repo uses floats for precision)
    # This is an ACCEPTABLE difference, not a bug
    if {r, s} <= {"int", "float"}:
        return True

    # Handle Vec<X> vs Vec<Y> where X and Y might be differently named structs
    vec_match_r = re.match(r"vec<(\w+)>", r)
    vec_match_s = re.match(r"vec<(\w+)>", s)
    if vec_match_r and vec_match_s:
        inner_r = vec_match_r.group(1)
        inner_s = vec_match_s.group(1)
        # Check if inner types are compatible struct names
        return structs_are_compatible(inner_r, inner_s)

    # Handle Option<X> vs Option<Y>
    opt_match_r = re.match(r"option<(.+)>", r)
    opt_match_s = re.match(r"option<(.+)>", s)
    if opt_match_r and opt_match_s:
        return types_are_compatible(
            opt_match_r.group(1), opt_match_s.group(1), repo_structs, spec_structs
        )

    # Handle Option<X> vs X (optionality difference)
    if opt_match_r and not opt_match_s:
        return types_are_compatible(opt_match_r.group(1), s, repo_structs, spec_structs)
    if opt_match_s and not opt_match_r:
        return types_are_compatible(r, opt_match_s.group(1), repo_structs, spec_structs)

    # HashMap vs BTreeMap are compatible
    if "hashmap" in r and "btreemap" in s:
        return True
    if "btreemap" in r and "hashmap" in s:
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
        for suffix in ["item", "entry", "sitem", "sentry"]:
            if n.endswith(suffix):
                n = n[: -len(suffix)]
        return n

    n1 = normalize(name1)
    n2 = normalize(name2)

    # Check if one is a substring of the other (handles nested type naming)
    # e.g., "psbtinput" vs "decodepsbtinputsitem" -> both contain "psbtinput"
    return n1 in n2 or n2 in n1


def build_struct_name_mapping(
    repo_structs: Dict[str, StructDef], spec_structs: Dict[str, StructDef]
) -> Tuple[Dict[str, str], Set[str]]:
    """Build a mapping from repo struct names to spec struct names.

    Uses ONLY explicit TYPE_BRIDGE mappings and exact name matches.
    No fuzzy matching.

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
            if target in mapping:
                # Multiple spec types can intentionally map to one shared repo type.
                # Keep the first canonical match and suppress alias noise.
                ignored_spec.add(spec_name)
            else:
                # Map repo -> spec (we store it this way for compatibility)
                mapping[target] = spec_name

    # Only do exact matches for non-bridged structs
    for repo_name in repo_structs:
        if repo_name in mapping:
            continue  # Already mapped via TYPE_BRIDGE
        if repo_name in spec_structs:
            mapping[repo_name] = repo_name

    # NO fuzzy matching - all mappings must be explicit in TYPE_BRIDGE

    return mapping, ignored_spec


def compare_structs(
    repo_structs: Dict[str, StructDef],
    spec_structs: Dict[str, StructDef],
    show_all: bool = False,
) -> None:
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
        elif repo_name not in REPO_ONLY_IGNORE:
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
    # Count how many repo-only types were filtered by REPO_ONLY_IGNORE
    repo_ignored = sum(
        1 for r in repo_structs if r in REPO_ONLY_IGNORE and r not in name_mapping
    )
    print(f"  Bridged/Ignored:  {len(ignored_spec)}")
    print(f"  Repo deprecated:  {repo_ignored}")
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
            field_diffs.append(
                (repo_name, spec_name, missing_display, extra_display, type_diffs)
            )

    # Show all unmatched structs (no filtering - we want explicit control via TYPE_BRIDGE)
    if only_in_repo:
        print(f"\n🔵 Structs only in REPO ({len(only_in_repo)}):")
        for name in only_in_repo:
            print(f"  - {name}")

    if only_in_spec:
        print(
            f"\n🟡 Structs only in SPEC ({len(only_in_spec)}) - may need implementation:"
        )
        for name in only_in_spec:
            print(f"  + {name}")

    if field_diffs:
        print(f"\n🔄 Structs with FIELD DIFFERENCES ({len(field_diffs)}):")
        for repo_name, spec_name, missing, extra, type_diffs in field_diffs:
            name_display = (
                repo_name if repo_name == spec_name else f"{repo_name} ↔ {spec_name}"
            )
            print(f"\n  {name_display}:")
            if missing:
                print(f"    Missing in repo: {', '.join(sorted(missing))}")
            if extra:
                print(f"    Extra in repo:   {', '.join(sorted(extra))}")
            if type_diffs:
                for fname, rtype, stype in type_diffs:
                    print(f"    Type diff '{fname}': repo={rtype} vs spec={stype}")

    # Show explicit TYPE_BRIDGE mappings that were used (non-exact matches)
    bridged_matches = [(r, s) for r, s in matched_pairs if r != s]
    if bridged_matches:
        print(f"\n🔗 TYPE_BRIDGE mappings used ({len(bridged_matches)}):")
        for repo_name, spec_name in sorted(bridged_matches):
            print(f"  {repo_name} ↔ {spec_name}")

    print(
        f"\n⚠️  Note: {len(ignored_spec)} spec-only structs were intentionally ignored based on TYPE_BRIDGE:"
    )
    for e in TYPE_BRIDGE:
        if TYPE_BRIDGE[e] == "IGNORE":
            print(f"  {e}")

    print("\n" + "=" * 70)


def main():
    import argparse

    ap = argparse.ArgumentParser(description="Compare struct definitions between files")
    ap.add_argument(
        "--repo", default="flattened.rs", help="File with repo struct definitions"
    )
    ap.add_argument(
        "--spec", default="generated.rs", help="File with spec struct definitions"
    )
    ap.add_argument(
        "--all", action="store_true", help="Show all differences (including noise)"
    )
    args = ap.parse_args()

    repo_text = Path(args.repo).read_text(encoding="utf-8")
    spec_text = Path(args.spec).read_text(encoding="utf-8")

    repo_structs = parse_structs(repo_text)
    spec_structs = parse_structs(spec_text)

    compare_structs(repo_structs, spec_structs, show_all=args.all)
    return 0


if __name__ == "__main__":
    sys.exit(main())
