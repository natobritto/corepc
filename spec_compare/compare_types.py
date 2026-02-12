#!/usr/bin/env python3
"""
Compare struct definitions between codegen output (from OpenRPC spec) and
the corepc-types source code.

Reports:
  1. Structs that exist only in one side
  2. Field differences between matched structs

Matching rules:
  - Exact name match, or explicit TYPE_BRIDGE mapping.  No fuzzy matching.
  - SPEC_ONLY_IGNORE / REPO_ONLY_IGNORE suppress known false positives.
  - Version-aware: pass --version N to use per-version ignore sets.

Typical invocation (called by run.sh):
  python3 compare_types.py --all --version 30 \\
      --repo output/v30_flattened.rs \\
      --spec output/v30_generated.rs
"""
import re
import sys
from pathlib import Path
from dataclasses import dataclass, field
from typing import Dict, List, Optional, Set, Tuple


# ============================================================================
# TYPE_BRIDGE: spec struct name - repo struct name  (or "IGNORE")
#
# "IGNORE" means the spec type has no repo counterpart on purpose (e.g. it
# wraps a bitcoin crate type, is a placeholder, or simply doesn't exist).
# ============================================================================
TYPE_BRIDGE: Dict[str, str] = {
    # ── Case-sensitivity fixes ──────────────────────────────────────────────
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
    "GetTxspendingprevOut": "GetTxSpendingPrevout",
    "ListReceivedbyAddress": "ListReceivedByAddress",
    "ListReceivedbyLabel": "ListReceivedByLabel",
    "LoadTxoutSet": "LoadTxOutSet",
    "SetNetworkactive": "SetNetworkActive",
    "SetWalletflag": "SetWalletFlag",
    "SignMessagewithPrivKey": "SignMessageWithPrivKey",
    "TestMempoolaccept": "TestMempoolAccept",
    "VerifyTxoutProof": "VerifyTxOutProof",
    # ── Array-item type renames (spec FooItem - repo shared type) ───────────
    "GetAddedNodeInfoItem": "AddedNode",
    "GetAddedNodeInfoItemAddressesItem": "AddedNodeAddress",
    "GetChainTipsItem": "ChainTips",
    "GetPeerInfoItem": "PeerInfo",
    "ListBannedItem": "Banned",
    "GetNodeAddressesItem": "NodeAddress",
    "GetTxspendingprevOutItem": "GetTxSpendingPrevoutItem",
    "TestMempoolacceptItem": "MempoolAcceptance",
    "TestMempoolacceptItemFees": "MempoolAcceptanceFees",
    "ImportDescriptorsItem": "ImportDescriptorsResult",
    "ImportDescriptorsItemError": "IGNORE",  # empty placeholder; repo uses serde_json::Value
    "ListReceivedbyAddressItem": "ListReceivedByAddressItem",
    "ListReceivedbyLabelItem": "ListReceivedByLabelItem",
    "ListTransactionsItem": "TransactionItem",
    "GetHdKeysItem": "HdKey",
    "GetHdKeysItemDescriptorsItem": "HdKeyDescriptor",
    # ── Wallet types ────────────────────────────────────────────────────────
    "DeriveAddressesVerboseZero": "DeriveAddresses",
    "DeriveAddressesVerboseOne": "DeriveAddressesMultipath",
    "UnloadWallet": "UnloadWallet",
    # ── Mempool types ───────────────────────────────────────────────────────
    "GetMempoolEntryFees": "MempoolEntryFees",
    "GetMempoolAncestorsVerboseOneEntry": "MempoolEntry",
    "GetMempoolDescendantsVerboseOneEntry": "MempoolEntry",
    "GetRawMempoolVerboseOneEntry": "MempoolEntry",
    "GetMempoolAncestorsVerboseOneEntryFees": "MempoolEntryFees",
    "GetMempoolDescendantsVerboseOneEntryFees": "MempoolEntryFees",
    "GetRawMempoolVerboseOneEntryFees": "MempoolEntryFees",
    "GetMempoolAncestorsVerboseZero": "GetMempoolAncestors",
    "GetMempoolDescendantsVerboseZero": "GetMempoolDescendants",
    "GetRawMempoolVerboseZero": "GetRawMempool",
    "GetMempoolAncestorsVerboseOne": "GetMempoolAncestorsVerbose",
    "GetMempoolDescendantsVerboseOne": "GetMempoolDescendantsVerbose",
    "GetRawMempoolVerboseOne": "GetRawMempoolVerbose",
    "GetRawMempoolVerboseTwo": "GetRawMempoolSequence",
    "GetOrphanTxsVerboseZero": "GetOrphanTxs",
    "GetOrphanTxsVerboseOneItem": "GetOrphanTxsVerboseOneEntry",
    "GetOrphanTxsVerboseTwoItem": "GetOrphanTxsVerboseTwoEntry",
    # ── Deployment / softfork types ─────────────────────────────────────────
    "GetDeploymentInfoDeployments": "DeploymentInfo",
    "GetDeploymentInfoDeploymentsBip9": "Bip9Info",
    "GetDeploymentInfoDeploymentsBip9Statistics": "Bip9Statistics",
    # ── Block-verbose nested types ──────────────────────────────────────────
    "GetBlockVerboseTwoTxItem": "GetBlockVerboseTwoTransaction",
    "GetBlockVerboseThreeTxItem": "GetBlockVerboseThreeTransaction",
    "GetBlockVerboseThreeTxItemVinItem": "RawTransactionInputWithPrevout",
    "GetBlockVerboseThreeTxItemVinItemPrevout": "GetBlockVerboseThreePrevout",
    "GetBlockVerboseThreeTxItemVinItemPrevoutScriptPubKey": "IGNORE",  # repo uses ScriptPubkey from types/src/lib.rs
    # ── Block template types ───────────────────────────────────────────────
    "GetBlocktemplateVerboseTwo": "GetBlockTemplate",
    "GetBlocktemplateVerboseTwoTransactionsItem": "BlockTemplateTransaction",
    # ── Estimate raw fee types ─────────────────────────────────────────────
    "EstimateRawFeeShort": "RawFeeDetail",
    "EstimateRawFeeMedium": "IGNORE",  # spec generates empty struct, not used in repo
    "EstimateRawFeeLong": "IGNORE",  # spec generates empty struct, not used in repo
    "EstimateRawFeeShortPass": "RawFeeRange",
    "EstimateRawFeeShortFail": "IGNORE",  # spec generates empty struct for error case
    # ── Raw transaction types ──────────────────────────────────────────────
    "GetBlockHeaderVerboseZero": "GetBlockHeader",
    "GetBlockHeaderVerboseOne": "GetBlockHeaderVerbose",
    "GetRawTransactionVerboseZero": "GetRawTransaction",
    "GetRawTransactionVerboseOne": "GetRawTransactionVerbose",
    "GetRawTransactionVerboseTwo": "GetRawTransactionVerboseWithPrevout",
    "GetRawTransactionVerboseOneVinItem": "IGNORE",  # repo uses RawTransactionInput from psbt module
    "GetRawTransactionVerboseOneVinItemScriptSig": "IGNORE",  # repo uses ScriptSig from types/src/lib.rs
    "GetRawTransactionVerboseOneVoutItem": "IGNORE",  # repo uses RawTransactionOutput from psbt module
    "GetRawTransactionVerboseOneVoutItemScriptPubKey": "IGNORE",  # repo uses ScriptPubkey from types/src/lib.rs
    "GetRawTransactionVerboseTwoVinItem": "IGNORE",  # repo uses RawTransactionInput from psbt module
    "GetRawTransactionVerboseTwoVinItemPrevout": "GetBlockVerboseThreePrevout",
    "GetRawTransactionVerboseTwoVinItemPrevoutScriptPubKey": "IGNORE",  # repo uses ScriptPubkey
    "GetTxoutVerboseOne": "GetTxOut",
    "GetTxoutVerboseOneScriptPubKey": "IGNORE",  # repo uses ScriptPubkey from types/src/lib.rs
    # ── Scan types ─────────────────────────────────────────────────────────
    "ScanTxoutSetVerboseZero": "ScanTxOutSetStart",
    "ScanTxoutSetVerboseOne": "ScanTxOutSetStatus",
    "ScanTxoutSetVerboseTwo": "ScanTxOutSetAbort",
    "ScanTxoutSetVerboseZeroUnspentsItem": "ScanTxOutSetUnspent",
    "ScanBlocksVerboseOne": "ScanBlocksStart",
    "ScanBlocksVerboseTwo": "ScanBlocksStatus",
    "ScanBlocksVerboseThree": "ScanBlocksAbort",
    # ── Memory info types ──────────────────────────────────────────────────
    "GetMemoryInfoVerboseZero": "GetMemoryInfoStats",
    "GetMemoryInfoVerboseOne": "IGNORE",  # spec wrapper for verbose=true; repo uses same struct
    "GetMemoryInfoVerboseZeroLocked": "Locked",
    # ── Send types ─────────────────────────────────────────────────────────
    "SendToAddressVerboseZero": "SendToAddress",
    "SendToAddressVerboseOne": "SendManyVerbose",
    "SendmanyVerboseZero": "SendMany",
    "SendmanyVerboseOne": "SendManyVerbose",
    # ── Sign raw transaction types ─────────────────────────────────────────
    "SignRawTransactionwithKey": "SignRawTransaction",
    "SignRawTransactionwithKeyErrorsItem": "SignFail",
    "SignRawTransactionwithWallet": "SignRawTransaction",
    "SignRawTransactionwithWalletErrorsItem": "SignFail",
    # ── PSBT types (repo uses shared types from psbt module) ───────────────
    "DecodePsbtInputsItem": "PsbtInput",
    "DecodePsbtInputsItemBip32DerivsItem": "IGNORE",  # repo uses Bip32Deriv from psbt module
    "DecodePsbtInputsItemFinalScriptSig": "IGNORE",  # repo uses ScriptSig from types/src/lib.rs
    "DecodePsbtInputsItemNonWitnessUtxo": "IGNORE",  # repo uses RawTransaction from psbt module
    "DecodePsbtInputsItemProprietaryItem": "Proprietary",
    "DecodePsbtInputsItemRedeemScript": "IGNORE",  # repo uses PsbtScript from psbt module
    "DecodePsbtInputsItemTaprootBip32DerivsItem": "TaprootBip32Deriv",
    "DecodePsbtInputsItemWitnessScript": "IGNORE",  # repo uses PsbtScript from psbt module
    "DecodePsbtInputsItemWitnessUtxo": "IGNORE",  # repo uses WitnessUtxo from psbt module
    "DecodePsbtInputsItemWitnessUtxoScriptPubKey": "IGNORE",  # repo uses ScriptPubkey
    "DecodePsbtInputsItemTaprootScriptsItem": "TaprootScript",
    "DecodePsbtInputsItemTaprootScriptPathSigsItem": "TaprootScriptPathSig",
    "DecodePsbtInputsItemMusig2PartialSigsItem": "Musig2PartialSig",
    "DecodePsbtInputsItemMusig2ParticipantPubkeysItem": "Musig2ParticipantPubkeys",
    "DecodePsbtInputsItemMusig2PubnoncesItem": "Musig2Pubnonce",
    "DecodePsbtOutputsItem": "PsbtOutput",
    "DecodePsbtOutputsItemBip32DerivsItem": "IGNORE",  # repo uses Bip32Deriv from psbt module
    "DecodePsbtOutputsItemMusig2ParticipantPubkeysItem": "Musig2ParticipantPubkeys",
    "DecodePsbtOutputsItemRedeemScript": "IGNORE",  # repo uses PsbtScript from psbt module
    "DecodePsbtOutputsItemTaprootTreeItem": "TaprootLeaf",
    "DecodePsbtOutputsItemWitnessScript": "IGNORE",  # repo uses PsbtScript from psbt module
    "DecodePsbtOutputsItemProprietaryItem": "Proprietary",
    "DecodePsbtOutputsItemTaprootBip32DerivsItem": "TaprootBip32Deriv",
    "DecodePsbtProprietaryItem": "Proprietary",
    "DecodePsbtGlobalXpubsItem": "GlobalXpub",
    "AnalyzePsbtInputsItem": "AnalyzePsbtInput",
    "AnalyzePsbtInputsItemMissing": "AnalyzePsbtInputMissing",
    # ── Decode raw transaction (repo uses shared psbt/lib types) ───────────
    "DecodeRawTransactionVinItem": "IGNORE",  # repo uses RawTransactionInput
    "DecodeRawTransactionVinItemScriptSig": "IGNORE",  # repo uses ScriptSig
    "DecodeRawTransactionVoutItem": "IGNORE",  # repo uses RawTransactionOutput
    "DecodeRawTransactionVoutItemScriptPubKey": "IGNORE",  # repo uses ScriptPubkey
    # ── Nested - shared-type renames ───────────────────────────────────────
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
    "GetWalletInfoScanning": "IGNORE",  # repo uses serde_json::Value for polymorphic bool|object
    "GetAddressesbyLabelEntry": "AddressInformation",
    "ListWalletDirWalletsItem": "ListWalletDirWallet",
    "ListDescriptorsDescriptorsItem": "DescriptorInfo",
    "ListSinceBlockTransactionsItem": "TransactionItem",
    "GetPrioritisedTransactionsEntry": "PrioritisedTransaction",
    "EnumerateSignersSignersItem": "Signers",
    "GetTxoutSetInfoBlockInfoUnspendables": "GetTxOutSetInfoUnspendables",
    "SubmitPackageTxResultsFees": "SubmitPackageTxResultssFees",
    # ── Simple / non-struct RPC returns ────────────────────────────────────
    "GetOpenRPC": "IGNORE",  # OpenRPC patch artefact
    "GetBlockFromPeer": "IGNORE",  # returns null
    "SubmitBlockVerboseOne": "IGNORE",  # returns string
}


# ============================================================================
# REPO_ONLY_IGNORE: repo types with no spec counterpart (all versions).
# ============================================================================
REPO_ONLY_IGNORE: set = {
    # Deprecated / removed RPCs
    "AddMultisigAddress",
    "DumpWallet",
    "GetZmqNotifications",
    "ImportMulti",
    "ImportMultiEntry",
    "JsonRpcError",
    "UpgradeWallet",
    # Legacy softfork types (pre-v21 restructuring)
    "Bip9Softfork",
    "Bip9SoftforkInfo",
    "Bip9SoftforkStatistics",
    "Softfork",
    "SoftforkReject",
    # Deprecated wallet types
    "GetAddressInfoLabel",
    "GetBalancesWatchOnly",
    # Hand-crafted typed representations (spec uses Vec<serde_json::Value>)
    "ReceiveActivity",
    "SpendActivity",
}

# Version-specific repo-only ignores.
# key = type name, value = set of version numbers where it should be ignored.
REPO_ONLY_IGNORE_BY_VERSION: Dict[str, Set[int]] = {
    "GetTxSpendingPrevout": {24, 25, 26, 27, 28, 29, 30},
    "GetTxSpendingPrevoutItem": {24, 25, 26, 27, 28, 29, 30},
    "GetUnconfirmedBalance": {24, 25, 26, 27, 28, 29, 30},
    "SubmitPackageTxResult": {24, 25, 26, 27, 28, 29, 30},
    "SubmitPackageTxResultFees": {24, 25, 26, 27, 28, 29, 30},
    "DumpPrivKey": {30},
}


# ============================================================================
# SPEC_ONLY_IGNORE: spec types with no repo counterpart (all versions).
# ============================================================================
SPEC_ONLY_IGNORE: set = {
    "Echo",
    "EchoIpc",
    "EchoJson",  # string returns
    "GetNetworkHashps",  # number
    "GetTxoutProof",  # hex string
    "Help",  # string
    "PrioritiseTransaction",  # bool
    "Stop",  # string
    "Uptime",  # number
    "ImportMempool",  # null
    "SendmsgToPeer",  # bool
    "GetBlocktemplateVerboseOne",  # proposal mode returns null
    "GetTxspendingprevOut",  # bridge handles casing
    "GetTxspendingprevOutItem",  # bridge handles casing
    "SubmitPackageTxResults",  # different naming than repo
    "SubmitPackageTxResultsFees",  # different naming than repo
}

# Version-specific spec-only ignores.
SPEC_ONLY_IGNORE_BY_VERSION: Dict[str, Set[int]] = {
    "DecodePsbtTx": {24, 25, 26, 27, 28, 29},
    "DumpTxoutSet": {24, 25},
    "GetBalancesWatchonly": {24, 25, 26, 27, 28, 29},
    "GetBlockVerboseTwo": {24, 25, 26, 27, 28},
    "GetBlockVerboseTwoTxItem": {24, 25, 26, 27, 28},
    "GetBlockVerboseThree": {24, 25, 26, 27, 28},
    "GetBlockVerboseThreeTxItem": {24, 25, 26, 27, 28},
    "GetBlockVerboseThreeTxItemVinItem": {24, 25, 26, 27, 28},
    "GetBlockVerboseThreeTxItemVinItemPrevout": {24, 25, 26, 27, 28},
    "GetRawTransactionVerboseTwo": {25, 26, 27, 28},
    "GetRawTransactionVerboseTwoVinItemPrevout": {25, 26, 27, 28},
    "GetTransactionDecoded": {24, 25, 26, 27, 28, 29},
    "GetTxoutSetInfoBlockInfo": {24, 25},
    "GetTxoutSetInfoBlockInfoUnspendables": {24, 25},
    "GetZmqnotifications": {24, 25, 26, 27, 28},
    "GetZmqnotificationsItem": {24, 25, 26, 27, 28},
    "GetunconfirmedBalance": {24, 25, 26, 27, 28, 29},
    "Importmulti": {24, 25, 26, 27, 28, 29},
    "ImportmultiItem": {24, 25, 26, 27, 28, 29},
    "ImportmultiItemError": {24, 25, 26, 27, 28, 29},
    "SubmitPackage": {24, 25},
}


# ============================================================================
# Data types
# ============================================================================


@dataclass
class StructField:
    name: str
    type_: str
    serde_rename: Optional[str] = None
    is_optional: bool = False

    @property
    def json_name(self) -> str:
        return self.serde_rename or self.name

    @property
    def normalized_name(self) -> str:
        return self.json_name.lower().replace("_", "")


@dataclass
class StructDef:
    name: str
    fields: List[StructField] = field(default_factory=list)
    is_tuple_struct: bool = False
    tuple_type: Optional[str] = None
    line: int = 0  # 1-based line where `pub struct` appears
    source_file: str = ""  # path of the file it was parsed from

    def get_field_by_json_name(self, json_name: str) -> Optional[StructField]:
        target = json_name.lower().replace("_", "")
        for f in self.fields:
            if f.normalized_name == target:
                return f
        return None


# ============================================================================
# Parsing
# ============================================================================


def parse_structs(text: str, source_file: str = "") -> Dict[str, StructDef]:
    """Parse `pub struct` definitions from Rust source text."""
    structs: Dict[str, StructDef] = {}
    lines = text.splitlines()
    i = 0

    while i < len(lines):
        line = lines[i]
        stripped = line.strip()

        if not stripped or stripped.startswith("//") or stripped.startswith("#["):
            i += 1
            continue

        line_nc = re.sub(r"//.*$", "", line)  # strip inline comments

        # ── Tuple struct (single line) ──
        m = re.match(r"^pub struct (\w+)\s*\((.*)\)\s*;?\s*$", line_nc)
        if m:
            structs[m.group(1)] = StructDef(
                name=m.group(1),
                is_tuple_struct=True,
                tuple_type=m.group(2).strip(),
                line=i + 1,
                source_file=source_file,
            )
            i += 1
            continue

        # ── Tuple struct (multi-line) ──
        m = re.match(r"^pub struct (\w+)\s*\(\s*$", line)
        if m:
            name = m.group(1)
            struct_line = i + 1
            types: list[str] = []
            i += 1
            while i < len(lines):
                l = lines[i].strip()
                if l == ");" or l.endswith(");"):
                    inline = l.rstrip(");").strip()
                    if inline:
                        fm = re.match(
                            r"(?:pub(?:\([^)]*\))?\s+)?(.+?)\s*,?\s*$", inline
                        )
                        if fm:
                            types.append(fm.group(1).strip())
                    break
                if not l or l.startswith("///") or l.startswith("#["):
                    i += 1
                    continue
                fm = re.match(r"(?:pub(?:\([^)]*\))?\s+)?(.+?)\s*,?\s*$", l)
                if fm:
                    types.append(fm.group(1).strip())
                i += 1
            structs[name] = StructDef(
                name=name,
                is_tuple_struct=True,
                tuple_type=", ".join(types) or None,
                line=struct_line,
                source_file=source_file,
            )
            i += 1
            continue

        # ── Regular struct ──
        m = re.match(r"^pub struct (\w+)\s*\{", line)
        if m:
            name = m.group(1)
            struct_line = i + 1
            fields: List[StructField] = []
            serde_rename: Optional[str] = None
            i += 1
            while i < len(lines):
                cur = lines[i].strip()
                if cur == "}":
                    break
                if cur.startswith("pub struct ") and "{" in cur:
                    break  # malformed – let outer loop pick it up

                if not cur or cur.startswith("///"):
                    i += 1
                    continue
                if cur.startswith("#[") and "serde(rename" not in cur:
                    i += 1
                    continue

                rm = re.search(r'#\[serde\(rename\s*=\s*"([^"]+)"', cur)
                if rm:
                    serde_rename = rm.group(1)
                    i += 1
                    continue

                cur_nc = re.sub(r"//.*$", "", cur)
                fm = re.match(r"pub\s+(\w+):\s*(.+?),?\s*$", cur_nc)
                if fm:
                    ftype = fm.group(2).rstrip(",")
                    fields.append(
                        StructField(
                            name=fm.group(1),
                            type_=ftype,
                            serde_rename=serde_rename,
                            is_optional=ftype.startswith("Option<"),
                        )
                    )
                    serde_rename = None
                i += 1

            if i < len(lines) and lines[i].strip() == "}":
                i += 1
            structs[name] = StructDef(
                name=name, fields=fields, line=struct_line, source_file=source_file
            )
            continue

        i += 1
    return structs


# ============================================================================
# Type compatibility helpers
# ============================================================================


def _norm_type(t: str) -> str:
    t = t.replace(" ", "").lower()
    t = re.sub(r"\b(i64|u64|i32|u32|usize|isize)\b", "int", t)
    t = re.sub(r"\b(f64|f32)\b", "float", t)
    t = t.replace("std::collections::", "")
    return t


def types_are_compatible(
    repo_type: str,
    spec_type: str,
    repo_structs: Dict[str, StructDef],
    spec_structs: Dict[str, StructDef],
) -> bool:
    r, s = _norm_type(repo_type), _norm_type(spec_type)
    if r == s:
        return True
    if {r, s} <= {"int", "float"}:
        return True

    # Vec<X> vs Vec<Y>
    vr = re.match(r"vec<(\w+)>", r)
    vs = re.match(r"vec<(\w+)>", s)
    if vr and vs:
        ir, is_ = vr.group(1), vs.group(1)
        for sk, rv in TYPE_BRIDGE.items():
            if sk.lower() == is_ and rv != "IGNORE" and rv.lower() == ir:
                return True
            # IGNORE'd spec type in Vec — check FIELD_TYPE_BRIDGE
            if sk.lower() == is_ and rv == "IGNORE":
                if is_ in FIELD_TYPE_BRIDGE:
                    for compat in FIELD_TYPE_BRIDGE[is_]:
                        if compat == ir:
                            return True
        # Also check FIELD_TYPE_BRIDGE directly for Vec inner types
        if is_ in FIELD_TYPE_BRIDGE:
            for compat in FIELD_TYPE_BRIDGE[is_]:
                if compat == ir:
                    return True
        return _names_compatible(ir, is_)

    # [T; N] vs Vec<serde_json::Value>
    if re.match(r"\[(\w+);\s*\d+\]", r) and "vec<serde_json::value>" in s:
        return True

    # Option wrappers
    or_ = re.match(r"option<(.+)>", r)
    os_ = re.match(r"option<(.+)>", s)
    if or_ and os_:
        return types_are_compatible(
            or_.group(1), os_.group(1), repo_structs, spec_structs
        )
    if or_ and not os_:
        return types_are_compatible(or_.group(1), s, repo_structs, spec_structs)
    if os_ and not or_:
        return types_are_compatible(r, os_.group(1), repo_structs, spec_structs)

    # HashMap / BTreeMap
    if ("hashmap" in r and "btreemap" in s) or ("btreemap" in r and "hashmap" in s):
        return True

    # TYPE_BRIDGE lookup (uses original un-normalized spec_type for case-sensitive names)
    sm = re.search(r"\b([A-Z]\w+)\b", spec_type)
    if sm and sm.group(1) in TYPE_BRIDGE:
        target = TYPE_BRIDGE[sm.group(1)]
        if target != "IGNORE" and target.lower() in r:
            return True
        # IGNORE'd spec type — check FIELD_TYPE_BRIDGE for compatible repo types
        if target == "IGNORE":
            spec_key = sm.group(1).lower()
            if spec_key in FIELD_TYPE_BRIDGE:
                for compat in FIELD_TYPE_BRIDGE[spec_key]:
                    if compat in r:
                        return True

    # Case-insensitive TYPE_BRIDGE lookup (catches post-Option-unwrap lowered names)
    if not sm:
        for sk, rv in TYPE_BRIDGE.items():
            if sk.lower() == s and rv != "IGNORE" and rv.lower() == r:
                return True
            if sk.lower() == s and rv == "IGNORE":
                if s in FIELD_TYPE_BRIDGE:
                    for compat in FIELD_TYPE_BRIDGE[s]:
                        if compat in r:
                            return True

    # FIELD_TYPE_BRIDGE: check by extracting type names from the normalized strings
    # This catches cases after Option<> unwrapping where we only have lowered strings.
    for spec_key, compat_set in FIELD_TYPE_BRIDGE.items():
        if spec_key in s:
            for compat in compat_set:
                if compat in r:
                    return True

    # Enum in repo vs String in spec
    if "string" in s and r not in {"string", "str"}:
        return True

    return _names_compatible(r, s)


def _names_compatible(a: str, b: str) -> bool:
    if a == b:
        return True

    def strip(n: str) -> str:
        for sfx in ("item", "entry", "sitem", "sentry"):
            if n.endswith(sfx):
                n = n[: -len(sfx)]
        return n

    a, b = strip(a), strip(b)
    return a in b or b in a


# ============================================================================
# Mapping builder
# ============================================================================


def _build_mapping(
    repo_structs: Dict[str, StructDef],
    spec_structs: Dict[str, StructDef],
    version: Optional[int] = None,
) -> Tuple[Dict[str, str], Set[str]]:
    """Return (repo_name-spec_name mapping, set of ignored spec names)."""
    mapping: Dict[str, str] = {}
    ignored: Set[str] = set()

    for spec_name, target in TYPE_BRIDGE.items():
        if spec_name not in spec_structs:
            continue
        if target == "IGNORE":
            ignored.add(spec_name)
        elif target in repo_structs:
            if target in mapping:
                ignored.add(spec_name)  # duplicate alias
            else:
                mapping[target] = spec_name
        elif version is not None:
            ignored.add(spec_name)  # bridge target missing in this version

    for repo_name in repo_structs:
        if repo_name not in mapping and repo_name in spec_structs:
            mapping[repo_name] = repo_name

    return mapping, ignored


# ============================================================================
# Comparison & reporting
# ============================================================================

# Fields that are known to be extra in the repo (deprecated / removed upstream).
_KNOWN_EXTRA_FIELDS = {
    "watch_only",
    "involves_watch_only",
    "add_node",
    "ban_score",
    "whitelisted",
    "account",
}


# ============================================================================
# COMMENTARY_STUBS: structs generated from x-documentation-commentary entries
# in the OpenRPC spec.  The spec uses "additionalProperties": true + a prose
# comment like "Same output as verbosity = 1" instead of duplicating fields.
# The codegen only emits explicitly declared fields, so these structs are
# partial stubs.  Extra fields in the repo are expected and should not be
# flagged.
# ============================================================================
COMMENTARY_STUBS: Set[str] = {
    # getblock verbosity=2 → "Same output as verbosity = 1"
    "GetBlockVerboseTwo",
    "GetBlockVerboseTwoTxItem",
    # getblock verbosity=3 → "Same output as verbosity = 2"
    "GetBlockVerboseThree",
    "GetBlockVerboseThreeTxItem",
    # getrawtransaction verbosity=2 → "Same output as verbosity = 1"
    "GetRawTransactionVerboseTwo",
    # decodepsbt → "The layout is the same as decoderawtransaction"
    "DecodePsbtTx",
    # getaddressinfo → embedded uses additionalProperties
    "GetAddressInfo",
    # importdescriptors error → "JSONRPC error"
    "ImportDescriptorsItemError",
}


# ============================================================================
# FIELD_TYPE_BRIDGE: maps spec field-level types (IGNOREd in TYPE_BRIDGE)
# to the repo shared types they should be considered compatible with.
# key = (lowered spec type name), value = set of compatible repo type names.
# ============================================================================
FIELD_TYPE_BRIDGE: Dict[str, Set[str]] = {
    # PSBT module shared types (used as field types in PsbtInput/PsbtOutput)
    "decodepsbtinputsitemredeemscript": {"psbtscript"},
    "decodepsbtinputsitemwitnessscript": {"psbtscript"},
    "decodepsbtoutputsitemredeemscript": {"psbtscript"},
    "decodepsbtoutputsitemwitnessscript": {"psbtscript"},
    "decodepsbtinputsitemnonwitnessutxo": {"rawtransaction"},
    "decodepsbtinputsitemwitnessutxo": {"witnessutxo"},
    "decodepsbtinputsitemfinalscriptsig": {"scriptsig"},
    "decodepsbtinputsitembip32derivsitem": {"bip32deriv"},
    "decodepsbtoutputsitembip32derivsitem": {"bip32deriv"},
    # Raw transaction inputs/outputs (used as Vec inner types)
    "getrawtransactionverboseonevinitem": {"rawtransactioninput"},
    "getrawtransactionverboseonevoutitem": {"rawtransactionoutput"},
    "getrawtransactionverbosetwovinitem": {
        "rawtransactioninput",
        "rawtransactioninputwithprevout",
    },
    "decoderawtransactionvinitem": {"rawtransactioninput"},
    "decoderawtransactionvoutitem": {"rawtransactionoutput"},
    # Prevout types
    "getblockverbosethreeprevout": {"getblockverbosethreeprevout"},
    "getblockverbosethreetxitemvinitemprevout": {"getblockverbosethreeprevout"},
    # Fee types
    "testmempoolacceptitemfees": {"mempoolacceptancefees"},
    "estimaterawfeeshort": {"rawfeedetail"},
    "estimaterawfeemedium": {"rawfeedetail"},
    "estimaterawfeelong": {"rawfeedetail"},
    "estimaterawfeeshortpass": {"rawfeerange"},
    "estimaterawfeeshortfail": {"rawfeerange"},
    # AnalyzePsbt
    "analyzepsbtinputsitemmissing": {"analyzepsbtinputmissing"},
    # DecodePsbt tx field — spec generates commentary stub, repo uses RawTransaction
    "decodepsbttx": {"rawtransaction"},
    # Deployment info bip9 field
    "getdeploymentinfodeploymentsbip9": {"bip9info"},
    # ImportDescriptors error field
    "importdescriptorsitemerror": {"serde_json::value"},
}


def _loc(sd: Optional[StructDef]) -> str:
    """Return a ' (file:line)' string for VS Code ctrl+click, or ''."""
    if sd is None or not sd.source_file or not sd.line:
        return ""
    return f"  ({sd.source_file}:{sd.line})"


def compare_structs(
    repo_structs: Dict[str, StructDef],
    spec_structs: Dict[str, StructDef],
    show_all: bool = False,
    version: Optional[int] = None,
) -> None:
    mapping, ignored_spec = _build_mapping(repo_structs, spec_structs, version)

    # Apply static + version-specific spec ignores
    for name in SPEC_ONLY_IGNORE:
        if name in spec_structs:
            ignored_spec.add(name)
    if version is not None:
        for name, vers in SPEC_ONLY_IGNORE_BY_VERSION.items():
            if version in vers and name in spec_structs:
                ignored_spec.add(name)

    # Build effective repo-ignore set
    eff_repo_ignore = set(REPO_ONLY_IGNORE)
    if version is not None:
        for name, vers in REPO_ONLY_IGNORE_BY_VERSION.items():
            if version in vers:
                eff_repo_ignore.add(name)

    # Categorise
    matched: List[Tuple[str, str]] = []
    only_repo: List[str] = []
    remaining_spec = set(spec_structs.keys()) - ignored_spec

    for repo_name in repo_structs:
        if repo_name in mapping:
            matched.append((repo_name, mapping[repo_name]))
            remaining_spec.discard(mapping[repo_name])
        elif repo_name not in eff_repo_ignore:
            only_repo.append(repo_name)

    only_spec = sorted(remaining_spec)
    only_repo.sort()

    repo_deprecated = sum(
        1 for r in repo_structs if r in eff_repo_ignore and r not in mapping
    )

    # ── Header ──
    print("=" * 70)
    print("STRUCT COMPARISON REPORT")
    print("=" * 70)
    print(f"\n📊 Summary:")
    print(f"  Repo structs:     {len(repo_structs)}")
    print(f"  Spec structs:     {len(spec_structs)}")
    print(f"  Matched pairs:    {len(matched)}")
    print(f"  Bridged/Ignored:  {len(ignored_spec)}")
    print(f"  Repo deprecated:  {repo_deprecated}")
    print(f"  Only in repo:     {len(only_repo)}")
    print(f"  Only in spec:     {len(only_spec)}")

    # # ── Bridge mappings used ──
    bridged = [(r, s) for r, s in matched if r != s]
    if bridged:
        print(f"\n🔗 TYPE_BRIDGE mappings used ({len(bridged)}):")
        for r, s in sorted(bridged)[:5]:
            print(f"  {r} ↔ {s}")
        if len(bridged) > 5:
            print(f"  ... and {len(bridged) - 5} more")

    # ── Unmatched ──
    if only_repo:
        print(f"\n🔵 Structs only in REPO ({len(only_repo)}):")
        for n in only_repo:
            loc = _loc(repo_structs.get(n))
            print(f"  - {n}{loc}")
    if only_spec:
        print(f"\n🟡 Structs only in SPEC ({len(only_spec)}):")
        for n in only_spec:
            loc = _loc(spec_structs.get(n))
            print(f"  + {n}{loc}")

    # ── Field diffs ──
    field_diffs: list = []
    significant: list = []
    commentary_note: list = []  # field diffs suppressed by COMMENTARY_STUBS

    for repo_name, spec_name in matched:
        rs = repo_structs[repo_name]
        ss = spec_structs[spec_name]
        if rs.is_tuple_struct or ss.is_tuple_struct:
            continue

        rf = {f.normalized_name: f for f in rs.fields}
        sf = {f.normalized_name: f for f in ss.fields}
        missing = set(sf) - set(rf)

        # If the spec struct is a commentary stub, suppress extra-field noise
        is_commentary = spec_name in COMMENTARY_STUBS
        extra = {
            n
            for n in (set(rf) - set(sf))
            if rf[n].name not in _KNOWN_EXTRA_FIELDS and not is_commentary
        }

        tdiffs = []
        for fn in set(rf) & set(sf):
            if not types_are_compatible(
                rf[fn].type_, sf[fn].type_, repo_structs, spec_structs
            ):
                tdiffs.append((rf[fn].name, rf[fn].type_, sf[fn].type_))

        if is_commentary and (set(rf) - set(sf)):
            suppressed = sorted(
                rf[n].name
                for n in (set(rf) - set(sf))
                if rf[n].name not in _KNOWN_EXTRA_FIELDS
            )
            if suppressed:
                commentary_note.append((repo_name, spec_name, suppressed))

        if missing or extra or tdiffs:
            md = [sf[n].name for n in missing]
            ed = [rf[n].name for n in extra]
            entry = (repo_name, spec_name, md, ed, tdiffs)
            field_diffs.append(entry)
            if missing or tdiffs:
                significant.append(entry)

    if field_diffs:
        print(f"\n🔄 Structs with FIELD DIFFERENCES ({len(field_diffs)}):")
        print(
            f"   ({len(significant)} significant, {len(field_diffs) - len(significant)} minor)"
        )
        for repo_name, spec_name, miss, extra, tdiffs in field_diffs:
            label = (
                repo_name if repo_name == spec_name else f"{repo_name} ↔ {spec_name}"
            )
            is_sig = (repo_name, spec_name, miss, extra, tdiffs) in significant
            icon = "⚠️ " if is_sig else "ℹ️ "
            rloc = _loc(repo_structs.get(repo_name))
            sloc = _loc(spec_structs.get(spec_name))
            print(f"\n  {icon}{label}:")
            print(f"    repo: {rloc.strip()}")
            print(f"    spec: {sloc.strip()}")
            if miss:
                print(f"    Missing field in repo: {', '.join(sorted(miss))}")
            if extra:
                print(f"    Extra field in repo:   {', '.join(sorted(extra))}")
            for fn, rt, st in tdiffs:
                print(f"    Type diff '{fn}': repo={rt} vs spec={st}")

    # ── Commentary stubs note ──
    if commentary_note:
        print(f"\n📝 x-documentation-commentary stubs ({len(commentary_note)}):")
        print(f"   These spec structs inherit fields from other componenets")
        print(f'   via Bitcoin Core documentation commentary ("Same output as").')
        print(f"   Extra fields in the repo are expected and suppressed:")
        for repo_name, spec_name, suppressed in commentary_note:
            label = (
                repo_name if repo_name == spec_name else f"{repo_name} ↔ {spec_name}"
            )
            print(f"\n    {label}:")
            print(f"      {len(suppressed)} inherited fields: {', '.join(suppressed)}")

    # ── Ignored summary ──
    print(
        f"\n⚠️  Note: {len(ignored_spec)} spec-only structs were intentionally ignored:"
    )
    _print_ignored_breakdown(ignored_spec, repo_structs, version)
    print("\n" + "=" * 70)


def _print_ignored_breakdown(
    ignored_spec: Set[str],
    repo_structs: Dict[str, StructDef],
    version: Optional[int],
) -> None:
    """Categorise and print the ignored spec types by reason."""
    buckets: Dict[str, List[Tuple[str, str]]] = {
        "simple_returns": [],
        "version_specific": [],
        "bridge_missing": [],
        "shared_types": [],
        "empty_structs": [],
        "primitives": [],
        "polymorphic": [],
        "aliases": [],
        "other": [],
    }

    # Pre-load inline comments from TYPE_BRIDGE entries in this file
    bridge_comments: Dict[str, str] = {}
    src = Path(__file__).read_text(encoding="utf-8")
    for m in re.finditer(r'"(\w+)":\s*"IGNORE",?\s*#\s*(.+?)$', src, re.MULTILINE):
        bridge_comments[m.group(1)] = m.group(2).strip()

    for name in sorted(ignored_spec):
        if name in SPEC_ONLY_IGNORE:
            buckets["simple_returns"].append((name, ""))
            continue
        if (
            name in SPEC_ONLY_IGNORE_BY_VERSION
            and version is not None
            and version in SPEC_ONLY_IGNORE_BY_VERSION[name]
        ):
            buckets["version_specific"].append((name, f"not in repo for v{version}"))
            continue
        if name in TYPE_BRIDGE and TYPE_BRIDGE[name] == "IGNORE":
            comment = bridge_comments.get(name, "")
            if "psbt module" in comment or "types/src/lib.rs" in comment:
                buckets["shared_types"].append((name, comment))
            elif "empty" in comment:
                buckets["empty_structs"].append((name, comment))
            elif any(w in comment for w in ("null", "string", "bool")):
                buckets["primitives"].append((name, comment))
            elif "serde_json::Value" in comment or "polymorphic" in comment:
                buckets["polymorphic"].append((name, comment))
            else:
                buckets["other"].append((name, comment or "No reason documented"))
            continue
        if name in TYPE_BRIDGE:
            target = TYPE_BRIDGE[name]
            if target not in repo_structs and version is not None:
                buckets["bridge_missing"].append(
                    (name, f"repo type '{target}' not in v{version}")
                )
            else:
                buckets["aliases"].append((name, f"alias for {target} (deduplicated)"))
            continue
        buckets["other"].append((name, "Unknown reason"))

    labels = {
        "simple_returns": "🔸 RPCs not yet implemented in repo (simple return types)",
        "version_specific": "📅 Not implemented in repo for this version",
        "bridge_missing": "🔧 Bridge target not available in this version",
        "shared_types": "📦 Uses shared types from psbt/lib.rs",
        "empty_structs": "📭 Empty placeholder structs",
        "primitives": "🔤 Primitive return types",
        "polymorphic": "🔀 Polymorphic/dynamic types",
        "aliases": "🔗 Duplicate aliases to same repo type",
        "other": "❓ Other",
    }
    for key, label in labels.items():
        items = buckets[key]
        if not items:
            continue
        print(f"\n  {label} ({len(items)}):")
        for name, reason in items:
            print(f"    • {name}")
            if reason:
                print(f"      - {reason}")


# ============================================================================
# CLI entry point
# ============================================================================


def main() -> int:
    import argparse

    ap = argparse.ArgumentParser(
        description="Compare struct definitions between spec and repo"
    )
    ap.add_argument("--repo", default="flattened.rs", help="Repo structs file")
    ap.add_argument("--spec", default="generated.rs", help="Spec structs file")
    ap.add_argument(
        "--all", action="store_true", help="Show all diffs (including noise)"
    )
    ap.add_argument(
        "--version", type=int, default=None, help="Bitcoin Core version (24-30)"
    )
    args = ap.parse_args()

    repo_structs = parse_structs(
        Path(args.repo).read_text(encoding="utf-8"), source_file=args.repo
    )
    spec_structs = parse_structs(
        Path(args.spec).read_text(encoding="utf-8"), source_file=args.spec
    )
    compare_structs(repo_structs, spec_structs, show_all=args.all, version=args.version)
    return 0


if __name__ == "__main__":
    sys.exit(main())
