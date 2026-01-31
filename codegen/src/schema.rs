// SPDX-License-Identifier: CC0-1.0

//! OpenRPC schema types for Bitcoin Core RPC API.
//!
//! This module contains structures that represent the OpenRPC document format
//! used by Bitcoin Core to describe its JSON-RPC API.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The root OpenRPC document.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpenRpcDocument {
    /// The OpenRPC specification version.
    pub openrpc: String,
    /// Metadata about the API.
    pub info: Info,
    /// The list of RPC methods.
    pub methods: Vec<Method>,
}

/// API metadata.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Info {
    /// The title of the API.
    pub title: String,
    /// The version of the API.
    pub version: String,
}

/// An RPC method definition.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Method {
    /// The method name (e.g., "getblockchaininfo").
    pub name: String,
    /// A short summary of what the method does.
    pub summary: String,
    /// A detailed description of the method.
    pub description: String,
    /// How parameters are structured.
    #[serde(rename = "paramStructure")]
    pub param_structure: String,
    /// The method parameters.
    pub params: Vec<Param>,
    /// The method result.
    pub result: MethodResult,
    /// The Bitcoin Core category for this method.
    #[serde(rename = "x-bitcoin-category")]
    pub category: Option<String>,
    /// Example usage.
    #[serde(rename = "x-bitcoin-examples")]
    pub examples: Option<String>,
}

/// A method parameter.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Param {
    /// The parameter name.
    pub name: String,
    /// The parameter schema.
    pub schema: Schema,
    /// Whether the parameter is required.
    pub required: bool,
    /// Parameter description.
    pub description: Option<String>,
}

/// The result of a method call.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MethodResult {
    /// The result name.
    pub name: String,
    /// The result schema.
    pub schema: Schema,
}

/// A JSON schema definition.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Schema {
    /// The JSON type.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// Description of this field.
    pub description: Option<String>,
    /// For objects: the properties.
    pub properties: Option<HashMap<String, Schema>>,
    /// For objects: required properties.
    pub required: Option<Vec<String>>,
    /// For objects: whether additional properties are allowed.
    #[serde(rename = "additionalProperties")]
    pub additional_properties: Option<AdditionalProperties>,
    /// For arrays: the item schema.
    pub items: Option<Box<SchemaOrArray>>,
    /// For strings: pattern validation.
    pub pattern: Option<String>,
    /// Default value.
    pub default: Option<serde_json::Value>,
    /// Minimum value for numbers.
    pub minimum: Option<f64>,
    /// Maximum value for numbers.
    pub maximum: Option<f64>,
    /// Bitcoin-specific type hint.
    #[serde(rename = "x-bitcoin-type")]
    pub bitcoin_type: Option<String>,
    /// Whether this field is optional.
    #[serde(rename = "x-bitcoin-optional")]
    pub bitcoin_optional: Option<bool>,
    /// Default hint for documentation.
    #[serde(rename = "x-bitcoin-default-hint")]
    pub bitcoin_default_hint: Option<String>,
    /// For oneOf schemas.
    #[serde(rename = "oneOf")]
    pub one_of: Option<Vec<Schema>>,
    /// For allOf schemas.
    #[serde(rename = "allOf")]
    pub all_of: Option<Vec<Schema>>,
    /// For anyOf schemas.
    #[serde(rename = "anyOf")]
    pub any_of: Option<Vec<Schema>>,
    /// Whether this is an object with dynamic keys.
    #[serde(rename = "x-bitcoin-object-dynamic")]
    pub object_dynamic: Option<bool>,
    /// Minimum items for arrays.
    #[serde(rename = "minItems")]
    pub min_items: Option<i32>,
    /// Maximum items for arrays.
    #[serde(rename = "maxItems")]
    pub max_items: Option<i32>,
    /// Enum values.
    #[serde(rename = "enum")]
    pub enum_values: Option<Vec<serde_json::Value>>,
}

/// Additional properties can be a boolean or a schema.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum AdditionalProperties {
    /// Boolean indicating whether additional properties are allowed.
    Bool(bool),
    /// Schema defining the shape of additional properties.
    Schema(Box<Schema>),
}

/// Schema items can be a single schema or an array (for tuple validation).
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SchemaOrArray {
    /// A single schema.
    Schema(Schema),
    /// An array of schemas (for tuple validation).
    Array(Vec<Schema>),
}

impl Schema {
    /// Returns true if this field should be optional in the Rust type.
    pub fn is_optional(&self) -> bool {
        self.bitcoin_optional.unwrap_or(false)
    }

    /// Returns true if this is a hex string type.
    pub fn is_hex(&self) -> bool {
        self.bitcoin_type.as_deref() == Some("hex")
    }

    /// Returns true if this is an amount type.
    pub fn is_amount(&self) -> bool {
        self.bitcoin_type.as_deref() == Some("amount")
    }

    /// Returns true if this schema represents an object with dynamic keys (like a map).
    pub fn is_dynamic_object(&self) -> bool {
        self.object_dynamic.unwrap_or(false)
    }
}

impl Method {
    /// Returns the category, defaulting to "misc" if not specified.
    pub fn category(&self) -> &str {
        self.category.as_deref().unwrap_or("misc")
    }

    /// Returns true if the method returns null/nothing.
    pub fn returns_null(&self) -> bool {
        self.result.schema.type_.as_deref() == Some("null")
    }

    /// Returns true if the method returns a simple type (string, boolean, number).
    pub fn returns_simple_type(&self) -> bool {
        matches!(
            self.result.schema.type_.as_deref(),
            Some("string") | Some("boolean") | Some("number") | Some("integer")
        ) && self.result.schema.properties.is_none()
    }

    /// Converts the method name to a Rust struct name (PascalCase).
    pub fn struct_name(&self) -> String {
        method_name_to_pascal_case(&self.name)
    }
}

/// Convert a Bitcoin Core RPC method name to PascalCase.
///
/// Since method names like "getblockchaininfo" don't have natural word boundaries,
/// we use a list of known words to split them properly into "GetBlockchainInfo".
fn method_name_to_pascal_case(name: &str) -> String {
    // Known word boundaries for Bitcoin Core RPC method names
    // Note: longer words should come before shorter ones with the same prefix
    const KNOWN_WORDS: &[&str] = &[
        "abandon", "abort", "add", "address", "addresses", "addrman", "all",
        "analyze", "ancestors", "api", "backup", "balance", "balances", "banned",
        "best", "bip125", "blockchain", "block", "blocks", "bump", "chain",
        "change", "clear", "combine", "connection", "control", "convert", "count",
        "create", "decode", "delete", "derive", "descendants", "descriptor",
        "descriptors", "difficulty", "dir", "disconnect", "display", "dump",
        "echo", "encrypt", "entry", "enumerate", "estimate", "fee", "filter",
        "finalize", "from", "fund", "funded", "generate", "get", "group",
        "groupings", "hash", "hashps", "hd", "header", "height", "help", "hex",
        "import", "info", "ipc", "join", "json", "key", "keys", "label", "labels",
        "list", "load", "loaded", "lock", "logging", "mempool", "memory", "message",
        "mining", "multisig", "net", "network", "new", "node", "orphan", "outset",
        "out", "package", "passphrase", "peer", "peers", "pool", "precious",
        "prioritise", "prioritised", "priv", "process", "proof", "prune", "psbts",
        "psbt", "raw", "received", "recipient", "rescan", "restore", "rpc",
        "save", "scan", "script", "send", "set", "sign", "signer", "signers",
        "simulate", "since", "smart", "spending", "spending", "stats", "status",
        "stop", "submit", "test", "tips", "to", "totals", "transactions",
        "transaction", "txout", "txs", "tx", "unload", "unlock", "unspent",
        "update", "upgrade", "uptime", "utxo", "validate", "verify", "wait",
        "wallet", "wallets", "zmq", "for",
    ];

    let mut result = String::new();
    let mut remaining = name.to_lowercase();

    while !remaining.is_empty() {
        // Find the longest matching word at the start
        let mut best_match = None;
        for word in KNOWN_WORDS {
            if remaining.starts_with(word) {
                match best_match {
                    None => best_match = Some(*word),
                    Some(current) if word.len() > current.len() => best_match = Some(*word),
                    _ => {}
                }
            }
        }

        if let Some(word) = best_match {
            // Capitalize first letter of the word
            let mut chars = word.chars();
            if let Some(first) = chars.next() {
                result.push(first.to_ascii_uppercase());
                result.extend(chars);
            }
            remaining = remaining[word.len()..].to_string();
        } else {
            // No word match, take one character
            if let Some(c) = remaining.chars().next() {
                if result.is_empty() {
                    result.push(c.to_ascii_uppercase());
                } else {
                    result.push(c);
                }
                remaining = remaining[1..].to_string();
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method_name_to_pascal_case() {
        assert_eq!(method_name_to_pascal_case("getblockchaininfo"), "GetBlockchainInfo");
        assert_eq!(method_name_to_pascal_case("getmininginfo"), "GetMiningInfo");
        assert_eq!(method_name_to_pascal_case("getprioritisedtransactions"), "GetPrioritisedTransactions");
        assert_eq!(method_name_to_pascal_case("abandontransaction"), "AbandonTransaction");
        assert_eq!(method_name_to_pascal_case("getmempoolinfo"), "GetMempoolInfo");
        assert_eq!(method_name_to_pascal_case("createwallet"), "CreateWallet");
        assert_eq!(method_name_to_pascal_case("sendtoaddress"), "SendToAddress");
        assert_eq!(method_name_to_pascal_case("dumptxoutset"), "DumpTxoutSet");
        assert_eq!(method_name_to_pascal_case("waitforblock"), "WaitForBlock");
        assert_eq!(method_name_to_pascal_case("gettxout"), "GetTxout");
        assert_eq!(method_name_to_pascal_case("scantxoutset"), "ScanTxoutSet");
    }
}
