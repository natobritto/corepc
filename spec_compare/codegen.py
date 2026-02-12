#!/usr/bin/env python3
"""
Generate Rust type definitions from a Bitcoin Core OpenRPC specification.

Usage (matches the old Rust CLI):
    python3 codegen.py --input specs/v30_2_0_openrpc.json \
                       --output output \
                       --core-version 30 \
                       --single-file
"""
from __future__ import annotations

import argparse
import json
import os
import sys
from collections import OrderedDict
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any, Dict, List, Optional, Tuple


# ============================================================================
# PascalCase conversion for RPC method names
# ============================================================================

# Longest words first so "addresses" beats "address", etc.
_METHOD_WORDS: list[str] = sorted([
    "abandon", "abort", "add", "address", "addresses", "addrman", "all",
    "analyze", "ancestors", "api", "backup", "balance", "balances", "banned",
    "best", "bip125", "blockchain", "block", "blocks", "bump", "chain",
    "change", "clear", "combine", "connection", "control", "convert", "count",
    "create", "decode", "delete", "deployment", "derive", "descendants", "descriptor",
    "descriptors", "difficulty", "dir", "disconnect", "display", "dump",
    "echo", "encrypt", "entry", "enumerate", "estimate", "fee", "filter",
    "finalize", "from", "fund", "funded", "generate", "get", "group",
    "groupings", "hash", "hashps", "hd", "header", "height", "help", "hex",
    "import", "index", "info", "ipc", "join", "json", "key", "keys", "label", "labels",
    "list", "load", "loaded", "lock", "logging", "mempool", "memory", "message",
    "mining", "multisig", "net", "network", "new", "node", "orphan", "outset",
    "out", "package", "passphrase", "peer", "peers", "pool", "precious",
    "prioritise", "prioritised", "priv", "process", "proof", "prune", "psbts",
    "psbt", "raw", "received", "recipient", "rescan", "restore", "rpc",
    "save", "scan", "script", "send", "set", "sign", "signer", "signers",
    "simulate", "since", "smart", "spending", "stats", "status",
    "stop", "submit", "test", "tips", "to", "totals", "transactions",
    "transaction", "txout", "txs", "tx", "unload", "unlock", "unspent",
    "update", "upgrade", "uptime", "utxo", "validate", "verify", "wait",
    "wallet", "wallets", "zmq", "for", "activity", "descriptor",
], key=lambda w: -len(w))


def method_name_to_pascal_case(name: str) -> str:
    """Convert an RPC method name like ``getblockchaininfo`` to ``GetBlockchainInfo``."""
    result: list[str] = []
    remaining = name.lower()

    while remaining:
        best: str | None = None
        for word in _METHOD_WORDS:
            if remaining.startswith(word):
                if best is None or len(word) > len(best):
                    best = word
        if best is not None:
            result.append(best[0].upper() + best[1:])
            remaining = remaining[len(best):]
        else:
            ch = remaining[0]
            if not result:
                result.append(ch.upper())
            else:
                result.append(ch)
            remaining = remaining[1:]

    return "".join(result)


# ============================================================================
# Snake-case conversion for JSON field names → Rust field names
# ============================================================================

_FIELD_WORDS: list[str] = sorted([
    "chainwork", "mediantime", "nchaintx",
    "addresses", "address", "amounts", "amount", "ancestors", "ancestor",
    "automatic", "balances", "balance", "banned", "bare", "bestblock",
    "blocks", "block", "bytes", "carrier", "chainstates", "chainstate",
    "chains", "chain", "challenge", "changes", "change", "coins", "coin",
    "confirmations", "conf", "connections", "counts", "count", "current",
    "data", "deltas", "delta", "depths", "depth", "descendants", "descendant",
    "descriptors", "descriptor", "desc", "difficulty", "disk", "download",
    "effective", "entries", "entry", "errors", "error", "estimated",
    "feerates", "feerate", "fees", "fee", "filters", "filter", "final",
    "first", "full", "hashps", "hash", "hd", "headers", "header", "heights",
    "height", "hex", "included", "incremental", "index", "info", "initial",
    "internal", "keys", "key", "labels", "label", "last", "limits", "limit",
    "loaded", "locktimes", "locktime", "locks", "lock", "log", "max",
    "mempool", "messages", "message", "min", "mining", "modified", "multisig",
    "names", "name", "networks", "network", "new", "next", "nodes", "node",
    "nonce", "num", "only", "orphans", "orphan", "outputs", "output", "outset",
    "paths", "path", "peers", "peer", "permit", "pooled", "previous", "prev",
    "prioritised", "priority", "processed", "progress", "proofs", "proof",
    "pruned", "pruneheight", "pruning", "prune", "pubkeys", "pubkey", "raw",
    "rbf", "reachable", "received", "relay", "required", "results", "result",
    "scanning", "scan", "scripts", "script", "sequences", "sequence", "signet",
    "sizes", "size", "spending", "start", "states", "state", "stats", "status",
    "stripped", "success", "targets", "target", "times", "time", "tips",
    "totals", "total", "transactions", "transaction", "txids", "txid",
    "txout", "txs", "tx", "types", "type_", "type", "unbroadcast",
    "unconfirmed", "unlock", "unspent", "used", "utxos", "utxo", "values",
    "value", "verification", "versions", "version", "vsize", "wallets",
    "wallet", "warnings", "watch", "weights", "weight", "witness", "work",
    "written", "out", "spending", "accept",
], key=lambda w: -len(w))

_RUST_KEYWORDS = {
    "type": "type_", "match": "match_", "ref": "ref_", "self": "self_",
    "mod": "mod_", "async": "async_", "await": "await_", "use": "use_",
}


def _split_into_snake_case(name: str) -> str:
    """Split a lowercase name into snake_case using known word boundaries."""
    # Handle camelCase first
    with_underscores: list[str] = []
    prev_lower = False
    for ch in name:
        if ch.isupper() and prev_lower:
            with_underscores.append("_")
        with_underscores.append(ch.lower())
        prev_lower = ch.islower()
    joined = "".join(with_underscores)

    if "_" in joined and joined != name.lower():
        return joined

    # All-lowercase: greedily match longest known word
    lower = name.lower()
    parts: list[str] = []
    remaining = lower
    while remaining:
        best: str | None = None
        for word in _FIELD_WORDS:
            if remaining.startswith(word):
                if best is None or len(word) > len(best):
                    best = word
        if best is not None:
            parts.append(best)
            remaining = remaining[len(best):]
        else:
            if not parts:
                return lower
            parts.append(remaining)
            break
    return "_".join(parts)


def to_rust_field_name(name: str) -> str:
    """Convert a JSON property name to a valid Rust field name (snake_case)."""
    name = name.replace("-", "_")
    if "_" in name:
        lower = name.lower()
        return _RUST_KEYWORDS.get(lower, lower)
    snake = _split_into_snake_case(name)
    return _RUST_KEYWORDS.get(snake, snake)


def _to_pascal_case(s: str) -> str:
    """Convert snake_case / camelCase / hyphenated names to PascalCase.

    Mimics the behaviour of the Rust ``heck`` crate's ``to_pascal_case``:
    split on underscores, hyphens, and camelCase boundaries then capitalise
    each word.
    """
    import re as _re
    if not s:
        return s
    # Replace hyphens/underscores with a separator we can split on
    s = s.replace("-", "_")
    # Insert underscore before uppercase letters that follow lowercase (camelCase)
    s = _re.sub(r"([a-z])([A-Z])", r"\1_\2", s)
    return "".join(part.capitalize() for part in s.split("_") if part)


# ============================================================================
# Data model for generated types / modules
# ============================================================================

@dataclass
class GeneratedType:
    name: str
    definition: str
    nested_types: list["GeneratedType"] = field(default_factory=list)


@dataclass
class MethodInfo:
    name: str
    struct_name: str | None
    notes: str


@dataclass
class GeneratedModule:
    name: str
    code: str
    exports: list[str]
    methods: list[MethodInfo]


# ============================================================================
# Category → module mapping
# ============================================================================

_CATEGORY_TO_MODULE: dict[str, str] = {
    "blockchain": "blockchain",
    "control": "control",
    "generating": "generating",
    "hidden": "hidden",
    "mining": "mining",
    "network": "network",
    "rawtransactions": "raw_transactions",
    "signer": "signer",
    "util": "util",
    "wallet": "wallet",
    "zmq": "zmq",
}


# ============================================================================
# Code generator
# ============================================================================

def _escape_doc(s: str) -> str:
    return s.replace("\n", "\n/// ")


class Generator:
    """Generates Rust type definitions from OpenRPC schemas."""

    def __init__(self) -> None:
        self._generated: set[str] = set()

    def reset(self) -> None:
        self._generated.clear()

    # ------------------------------------------------------------------
    # Public entry point
    # ------------------------------------------------------------------

    def generate_method_result(self, method: dict) -> GeneratedType | None:
        schema = method["result"]["schema"]
        struct_name = method_name_to_pascal_case(method["name"])

        if _returns_null(method):
            return None

        if _returns_simple_type(method) and not _is_dynamic_object(schema):
            return self._simple_wrapper_for_method(method)

        # oneOf / anyOf
        for key in ("oneOf", "anyOf"):
            if key in schema:
                return self._one_of_type(struct_name, schema[key], method.get("description"))

        if _is_dynamic_object(schema):
            return self._map_type_for_method(method, schema)

        if schema.get("type") == "array":
            return self._array_wrapper_for_method(method, schema)

        return self._struct_type(struct_name, schema, method.get("description"))

    # ------------------------------------------------------------------
    # Simple wrapper  (e.g. ``pub struct AbortRescan(pub bool);``)
    # ------------------------------------------------------------------

    def _simple_wrapper_for_method(self, method: dict) -> GeneratedType | None:
        name = method_name_to_pascal_case(method["name"])
        if name in self._generated:
            return None
        self._generated.add(name)
        inner = _simple_rust_type(method["result"]["schema"])
        if inner is None:
            return None
        doc = (
            f'/// Result of the JSON-RPC method `{method["name"]}`.\n'
            f'///\n/// > {method["name"]}\n/// >\n'
            f'/// > {_escape_doc(method["summary"])}'
        )
        defn = (
            f"{doc}\n"
            f"#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]\n"
            f'#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]\n'
            f"pub struct {name}(pub {inner});\n"
        )
        return GeneratedType(name=name, definition=defn)

    def _simple_wrapper_from_schema(
        self, name: str, schema: dict, doc: str | None
    ) -> GeneratedType | None:
        if name in self._generated:
            return None
        self._generated.add(name)
        inner = _simple_rust_type(schema)
        if inner is None:
            return None
        doc_str = f"/// {_escape_doc(doc)}\n" if doc else ""
        defn = (
            f"{doc_str}"
            f"#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]\n"
            f'#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]\n'
            f"pub struct {name}(pub {inner});\n"
        )
        return GeneratedType(name=name, definition=defn)

    # ------------------------------------------------------------------
    # oneOf / anyOf
    # ------------------------------------------------------------------

    def _one_of_type(
        self, name: str, variants: list[dict], doc: str | None
    ) -> GeneratedType | None:
        if len(variants) == 1:
            v = variants[0]
            if v.get("type") == "object":
                if _is_dynamic_object(v):
                    return self._map_type_from_schema(name, v, doc)
                return self._struct_type(name, v, doc)
            if v.get("type") == "array":
                return self._array_wrapper_from_schema(name, v, doc)
            return self._simple_wrapper_from_schema(name, v, doc)

        all_types: list[GeneratedType] = []
        for i, variant in enumerate(variants):
            variant_name = self._variant_name(name, variant, i, len(variants))
            gen = self._generate_variant(variant_name, variant, doc)
            if gen is not None:
                all_types.append(gen)

        if not all_types:
            return None

        primary = all_types[0]
        for extra in all_types[1:]:
            primary.nested_types.extend(extra.nested_types)
            primary.nested_types.append(extra)
        return primary

    @staticmethod
    def _variant_name(base: str, variant: dict, index: int, total: int) -> str:
        if total <= 1:
            return base
        condition = variant.get("x-bitcoin-condition", "") or variant.get("description", "") or ""
        is_simple = " and " not in condition

        suffix: str | None = None
        if is_simple:
            cond_lower = condition.lower()
            for pattern, sfx in [
                ("verbose = false", "VerboseZero"), ("verbose=false", "VerboseZero"),
                ("verbose is not set or set to false", "VerboseZero"),
                ("verbose is not set or set to 0", "VerboseZero"),
                ("verbosity = 0", "VerboseZero"), ("verbosity=0", "VerboseZero"),
                ("verbose = 0", "VerboseZero"), ("verbose=0", "VerboseZero"),
                ("verbosity = 1", "VerboseOne"), ("verbosity=1", "VerboseOne"),
                ("verbose = true", "VerboseOne"), ("verbose=true", "VerboseOne"),
                ("verbose = 1", "VerboseOne"), ("verbose=1", "VerboseOne"),
                ("verbose is set to 1", "VerboseOne"),
                ("verbose is set to true", "VerboseOne"),
                ("verbosity = 2", "VerboseTwo"), ("verbosity=2", "VerboseTwo"),
                ("verbose = 2", "VerboseTwo"), ("verbose=2", "VerboseTwo"),
                ("verbosity = 3", "VerboseThree"), ("verbosity=3", "VerboseThree"),
                ("verbose = 3", "VerboseThree"), ("verbose=3", "VerboseThree"),
            ]:
                if pattern in cond_lower:
                    suffix = sfx
                    break

        if suffix is None:
            suffix = ["VerboseZero", "VerboseOne", "VerboseTwo", "VerboseThree", "VerboseFour"][index] if index < 5 else "---------ERROR_badtype"
        return f"{base}{suffix}"

    def _generate_variant(
        self, name: str, variant: dict, doc: str | None
    ) -> GeneratedType | None:
        t = variant.get("type")
        if t == "object":
            if _is_dynamic_object(variant):
                return self._map_type_from_schema(name, variant, doc)
            return self._struct_type(name, variant, doc)
        if t == "array":
            return self._array_wrapper_from_schema(name, variant, doc)
        return self._simple_wrapper_from_schema(name, variant, doc)

    # ------------------------------------------------------------------
    # Array wrapper  (e.g. ``pub struct GetChainTips(pub Vec<…>);``)
    # ------------------------------------------------------------------

    def _array_wrapper_for_method(self, method: dict, schema: dict) -> GeneratedType | None:
        name = method_name_to_pascal_case(method["name"])
        doc = (
            f'/// Result of the JSON-RPC method `{method["name"]}`.\n'
            f'///\n/// > {method["name"]}\n/// >\n'
            f'/// > {_escape_doc(method["summary"])}'
        )
        return self._array_wrapper_from_schema(name, schema, doc)

    def _array_wrapper_from_schema(
        self, name: str, schema: dict, doc: str | None
    ) -> GeneratedType | None:
        if name in self._generated:
            return None

        item_type, nested = self._array_item_type(schema, name)
        self._generated.add(name)

        if doc:
            doc_str = f"{doc}\n" if doc.startswith("///") else f"/// {_escape_doc(doc)}\n"
        else:
            doc_str = ""

        defn = (
            f"{doc_str}"
            f"#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]\n"
            f'#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]\n'
            f"pub struct {name}(pub Vec<{item_type}>);\n"
        )
        return GeneratedType(name=name, definition=defn, nested_types=nested)

    def _array_item_type(self, schema: dict, parent: str) -> Tuple[str, list[GeneratedType]]:
        items = schema.get("items")
        if items is None:
            return ("String", [])
        if isinstance(items, list):
            return ("serde_json::Value", [])
        # items is a schema dict
        if items.get("type") == "object" and "properties" in items:
            inner_name = f"{parent}Item"
            return self._generate_inner_type(inner_name, items)
        return self._schema_to_rust_type(items, parent, "")

    # ------------------------------------------------------------------
    # Map/dynamic-object wrapper
    # ------------------------------------------------------------------

    def _map_type_for_method(self, method: dict, schema: dict) -> GeneratedType | None:
        name = method_name_to_pascal_case(method["name"])
        if name in self._generated:
            return None
        self._generated.add(name)
        value_type, nested = self._map_value_type(schema, name)
        doc = (
            f'/// Result of the JSON-RPC method `{method["name"]}`.\n'
            f'///\n/// > {method["name"]}\n/// >\n'
            f'/// > {_escape_doc(method["summary"])}'
        )
        desc = schema.get("description", "Map entries")
        defn = (
            f"{doc}\n"
            f"#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]\n"
            f'#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]\n'
            f"pub struct {name}(\n"
            f"    /// {desc}\n"
            f"    pub std::collections::BTreeMap<String, {value_type}>,\n"
            f");\n"
        )
        return GeneratedType(name=name, definition=defn, nested_types=nested)

    def _map_type_from_schema(
        self, name: str, schema: dict, doc: str | None
    ) -> GeneratedType | None:
        if name in self._generated:
            return None
        self._generated.add(name)
        value_type, nested = self._map_value_type(schema, name)
        if doc:
            doc_str = f"{doc}\n" if doc.startswith("///") else f"/// {_escape_doc(doc)}\n"
        else:
            doc_str = ""
        desc = schema.get("description", "Map entries")
        defn = (
            f"{doc_str}"
            f"#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]\n"
            f'#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]\n'
            f"pub struct {name}(\n"
            f"    /// {desc}\n"
            f"    pub std::collections::BTreeMap<String, {value_type}>,\n"
            f");\n"
        )
        return GeneratedType(name=name, definition=defn, nested_types=nested)

    def _map_value_type(self, schema: dict, parent: str) -> Tuple[str, list[GeneratedType]]:
        ap = schema.get("additionalProperties")
        if isinstance(ap, dict):
            inner_name = f"{parent}Item" if parent.endswith("Entry") else f"{parent}Entry"
            return self._generate_inner_type(inner_name, ap)
        return ("serde_json::Value", [])

    # ------------------------------------------------------------------
    # Struct type
    # ------------------------------------------------------------------

    def _struct_type(
        self, name: str, schema: dict, doc: str | None
    ) -> GeneratedType | None:
        if name in self._generated:
            return None
        self._generated.add(name)

        properties: dict[str, Any] = schema.get("properties") or {}
        required_set: set[str] = set(schema.get("required") or [])

        fields: list[str] = []
        nested_types: list[GeneratedType] = []

        commentary: list[str] = []
        commentary_raw: list[str] = []

        # Separate schema properties from commentary strings
        sorted_props: list[tuple[str, dict]] = []
        for prop_name in sorted(properties.keys()):
            prop_value = properties[prop_name]
            if isinstance(prop_value, str):
                commentary_raw.append(prop_value)
                commentary.append(f"{prop_name}: {_escape_doc(prop_value)}")
            else:
                sorted_props.append((prop_name, prop_value))

        for prop_name, prop_schema in sorted_props:
            is_required = prop_name in required_set
            is_optional = not is_required or prop_schema.get("x-bitcoin-optional", False)

            field_type, field_nested = self._schema_to_rust_type(prop_schema, name, prop_name)
            nested_types.extend(field_nested)

            rust_name = to_rust_field_name(prop_name)
            serde_rename = (
                f'    #[serde(rename = "{prop_name}")]\n' if rust_name != prop_name else ""
            )
            final_type = f"Option<{field_type}>" if is_optional else field_type
            field_doc = (
                f"    /// {_escape_doc(prop_schema['description'])}\n"
                if prop_schema.get("description")
                else ""
            )
            fields.append(f"{field_doc}{serde_rename}    pub {rust_name}: {final_type},")

        # Build doc comment
        doc_str = f"/// {_escape_doc(doc)}\n" if doc else ""
        if commentary:
            doc_str += "/// [TODO] this is a commentary from documentation explaining what this field is supposed be: \n"
            for line in commentary:
                doc_str += f"/// {line}\n"

        # Check for alias (DecodeRawTransaction commentary shortcut)
        alias_target: str | None = None
        for text in commentary_raw:
            if "decoderawtransaction" in text.lower():
                alias_target = "DecodeRawTransaction"
                break

        if not fields and alias_target:
            defn = f"{doc_str}pub type {name} = {alias_target};\n"
        else:
            fields_str = "\n".join(fields)
            defn = (
                f"{doc_str}"
                f"#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]\n"
                f'#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]\n'
                f"pub struct {name} {{\n"
                f"{fields_str}\n"
                f"}}\n"
            )

        return GeneratedType(name=name, definition=defn, nested_types=nested_types)

    # ------------------------------------------------------------------
    # Inner / nested type helper
    # ------------------------------------------------------------------

    def _generate_inner_type(
        self, name: str, schema: dict
    ) -> Tuple[str, list[GeneratedType]]:
        if "properties" in schema:
            gen = self._struct_type(name, schema, schema.get("description"))
            if gen:
                nested = [gen] + list(gen.nested_types)
                return (name, nested)

        if _is_dynamic_object(schema):
            ap = schema.get("additionalProperties")
            if isinstance(ap, dict):
                inner_name = f"{name}Entry"
                inner_type, inner_nested = self._generate_inner_type(inner_name, ap)
                return (f"std::collections::BTreeMap<String, {inner_type}>", inner_nested)

        type_str, nested = self._schema_to_rust_type(schema, name, "")
        return (type_str, nested)

    # ------------------------------------------------------------------
    # Schema → Rust type string
    # ------------------------------------------------------------------

    def _schema_to_rust_type(
        self, schema: dict, parent_name: str, field_name: str
    ) -> Tuple[str, list[GeneratedType]]:
        # oneOf/anyOf → fall back to Value
        if "oneOf" in schema or "anyOf" in schema:
            return ("serde_json::Value", [])

        if schema.get("x-bitcoin-type") == "hex":
            return ("String", [])
        if schema.get("x-bitcoin-type") == "amount":
            return ("f64", [])

        t = schema.get("type")
        if t == "string":
            return ("String", [])
        if t == "boolean":
            return ("bool", [])
        if t == "number":
            if schema.get("x-bitcoin-type") == "amount":
                return ("f64", [])
            return ("i64", [])
        if t == "integer":
            return ("i64", [])
        if t == "null":
            return ("()", [])

        if t == "array":
            items = schema.get("items")
            if items is None:
                return ("Vec<serde_json::Value>", [])
            if isinstance(items, list):
                return ("Vec<serde_json::Value>", [])
            if items.get("type") == "object" and "properties" in items:
                inner_name = f"{parent_name}{_to_pascal_case(field_name)}Item"
                type_name, nested = self._generate_inner_type(inner_name, items)
                return (f"Vec<{type_name}>", nested)
            inner_type, nested = self._schema_to_rust_type(items, parent_name, field_name)
            return (f"Vec<{inner_type}>", nested)

        if t == "object":
            if _is_dynamic_object(schema):
                ap = schema.get("additionalProperties")
                if isinstance(ap, dict):
                    if ap.get("type") == "object" and "properties" in ap:
                        inner_name = f"{parent_name}{_to_pascal_case(field_name)}"
                        vt, nested = self._generate_inner_type(inner_name, ap)
                    else:
                        vt, nested = self._schema_to_rust_type(ap, parent_name, field_name)
                    return (f"std::collections::BTreeMap<String, {vt}>", nested)
                return ("std::collections::BTreeMap<String, serde_json::Value>", [])

            ap = schema.get("additionalProperties")
            if isinstance(ap, dict) and not _has_schema_properties(schema):
                if ap.get("type") == "object" and "properties" in ap:
                    inner_name = f"{parent_name}{_to_pascal_case(field_name)}"
                    vt, nested = self._generate_inner_type(inner_name, ap)
                else:
                    vt, nested = self._schema_to_rust_type(ap, parent_name, field_name)
                return (f"std::collections::BTreeMap<String, {vt}>", nested)

            if "properties" in schema:
                nested_name = f"{parent_name}{_to_pascal_case(field_name)}"
                gen = self._struct_type(nested_name, schema, schema.get("description"))
                if gen:
                    nested = list(gen.nested_types) + [gen]
                    return (nested_name, nested)

            return ("serde_json::Value", [])

        return ("serde_json::Value", [])


# ============================================================================
# Module generation
# ============================================================================

def _returns_null(method: dict) -> bool:
    return method["result"]["schema"].get("type") == "null"


def _returns_simple_type(method: dict) -> bool:
    s = method["result"]["schema"]
    return s.get("type") in ("string", "boolean", "number", "integer") and not _has_schema_properties(s)


def _is_dynamic_object(schema: dict) -> bool:
    return schema.get("x-bitcoin-object-dynamic", False)


def _has_schema_properties(schema: dict) -> bool:
    props = schema.get("properties")
    if not props:
        return False
    return any(isinstance(v, dict) for v in props.values())


def _simple_rust_type(schema: dict) -> str | None:
    t = schema.get("type")
    if t == "string":
        return "String"
    if t == "boolean":
        return "bool"
    if t in ("number", "integer"):
        return "i64"
    return None


def generate_modules(methods: list[dict], version: str) -> list[GeneratedModule]:
    """Group methods by category and generate a ``GeneratedModule`` for each."""
    by_category: dict[str, list[dict]] = {}
    for m in methods:
        cat = m.get("x-bitcoin-category", "misc")
        by_category.setdefault(cat, []).append(m)

    generator = Generator()
    modules: list[GeneratedModule] = []

    for category in sorted(by_category):
        generator.reset()
        mod = _generate_module(category, by_category[category], generator)
        modules.append(mod)

    return modules


def _generate_module(
    category: str, methods: list[dict], generator: Generator
) -> GeneratedModule:
    module_name = _CATEGORY_TO_MODULE.get(category, category)

    all_types: list[GeneratedType] = []
    method_infos: list[MethodInfo] = []
    exports: list[str] = []

    for method in methods:
        if _returns_null(method):
            struct_name = None
            notes = "returns nothing"
        elif _returns_simple_type(method):
            struct_name = None
            notes = f"returns {method['result']['schema'].get('type', 'unknown')}"
        else:
            struct_name = method_name_to_pascal_case(method["name"])
            notes = "version + model"

        method_infos.append(MethodInfo(name=method["name"], struct_name=struct_name, notes=notes))

        gen = generator.generate_method_result(method)
        if gen is not None:
            exports.append(gen.name)
            for nested in gen.nested_types:
                exports.append(nested.name)
                all_types.append(nested)
            all_types.append(gen)

    # Build module source code
    code_lines = [
        f"// SPDX-License-Identifier: CC0-1.0\n",
        f"\n",
        f"//! The JSON-RPC API for Bitcoin Core - {category}.\n",
        f"//!\n",
        f"//! Types for methods found under the `== {_to_pascal_case(category)} ==` section of the API docs.\n",
        f"//!\n",
        f"//! This file is auto-generated from OpenRPC specification.\n",
        f"\n",
        f"use serde::{{Deserialize, Serialize}};\n",
        f"\n",
    ]

    for gt in sorted(all_types, key=lambda g: g.name):
        code_lines.append(gt.definition)
        code_lines.append("\n")

    return GeneratedModule(
        name=module_name,
        code="".join(code_lines),
        exports=exports,
        methods=method_infos,
    )


def generate_mod_rs(modules: list[GeneratedModule], version: str) -> str:
    """Generate the mod.rs header (documentation tables, module decls, re-exports)."""
    lines: list[str] = []

    lines.append(f"""\
// SPDX-License-Identifier: CC0-1.0

//! # JSON-RPC types for Bitcoin Core `{version}`
//!
//! These structs are shaped for the JSON data returned by the JSON-RPC API. They use stdlib types
//! (or custom types) and where necessary implement an `into_model` function to convert the type to
//! a [`crate::model`] type of the same name. The types in this module are version specific. The
//! types in the `model` module are version nonspecific and are strongly typed using `rust-bitcoin`.
//!
//! This file is auto-generated from OpenRPC specification.
//!
//! ### Method name and implementation status
//!
//! Every JSON-RPC method supported by this version of Bitcoin Core is listed below along with the
//! type it returns and any implementation notes.
//!
//! Key to 'Returns' column:
//!
//! * version: method returns a version specific type but has no model type.
//! * version + model: method returns a version specific type and can be converted to a model type.
//! * returns foo: method returns a foo (e.g. string, boolean, or nothing).
//! * omitted: method intentionally unsupported with no plans of adding support.

""")

    for module in modules:
        lines.append(
            f"//! <details>\n"
            f"//! <summary> Methods from the == {_to_pascal_case(module.name)} == section </summary>\n"
            f"//!\n"
        )
        lines.append("//! | JSON-RPC Method Name               | Returns         | Notes                                  |\n")
        lines.append("//! |:-----------------------------------|:---------------:|:--------------------------------------:|\n")
        for mi in module.methods:
            returns = f"version ({mi.struct_name})" if mi.struct_name else mi.notes
            lines.append(f"//! | {mi.name:<36} | {returns:<15} | {'':<38} |\n")
        lines.append("//!\n//! </details>\n//!\n")

    lines.append("\n")

    for module in modules:
        if module.exports:
            lines.append(f"pub mod {module.name};\n")
    lines.append("\n")

    for module in modules:
        if module.exports:
            exports_str = ", ".join(module.exports)
            lines.append(f"pub use self::{module.name}::{{{exports_str}}};\n")

    return "".join(lines)


# ============================================================================
# CLI
# ============================================================================

def main() -> int:
    ap = argparse.ArgumentParser(description="Generate Rust types from OpenRPC spec")
    ap.add_argument("--input", "-i", required=True, help="Path to OpenRPC JSON file")
    ap.add_argument("--output", "-o", required=True, help="Output directory")
    ap.add_argument("--core-version", "-c", required=True, help="Bitcoin Core version (e.g. 30)")
    ap.add_argument("--single-file", action="store_true", help="Generate a single combined file")
    ap.add_argument("--dry-run", action="store_true", help="Print what would be generated")
    args = ap.parse_args()

    with open(args.input, encoding="utf-8") as f:
        doc = json.load(f)

    print(f"Parsed OpenRPC document: {doc['info']['title']} ({doc['info']['version']})")
    print(f"Found {len(doc['methods'])} methods")

    modules = generate_modules(doc["methods"], args.core_version)

    print(f"Generated {len(modules)} modules:")
    for mod in modules:
        print(f"  - {mod.name}: {len(mod.exports)} types from {len(mod.methods)} methods")

    if args.dry_run:
        print("\n[DRY RUN] Would generate the following files:")
        for mod in modules:
            if mod.exports:
                print(f"  - {args.output}/{mod.name}.rs")
        print(f"  - {args.output}/mod.rs")
        return 0

    os.makedirs(args.output, exist_ok=True)

    if args.single_file:
        combined = generate_mod_rs(modules, args.core_version)
        combined += "\n// ============ Generated Types ============\n\n"
        for mod in modules:
            if mod.exports:
                combined += f"\n// --- {mod.name} ---\n\n"
                combined += mod.code
        out_path = Path(args.output) / "generated.rs"
        out_path.write_text(combined, encoding="utf-8")
        print(f'Wrote: "{out_path}"')
    else:
        for mod in modules:
            if mod.exports:
                out_path = Path(args.output) / f"{mod.name}.rs"
                out_path.write_text(mod.code, encoding="utf-8")
                print(f'Wrote: "{out_path}"')
        mod_rs_path = Path(args.output) / "mod.rs"
        mod_rs_path.write_text(generate_mod_rs(modules, args.core_version), encoding="utf-8")
        print(f'Wrote: "{mod_rs_path}"')

    print("\nCode generation complete!")
    return 0


if __name__ == "__main__":
    sys.exit(main())
