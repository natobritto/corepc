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
    pub properties: Option<HashMap<String, PropertySchema>>,
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

/// A property value in a schema can be a nested schema or a commentary string.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PropertySchema {
    /// A nested schema definition.
    Schema(Schema),
    /// A commentary string (non-schema metadata).
    Commentary(String),
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

    /// Returns true if there is at least one real schema property.
    pub fn has_schema_properties(&self) -> bool {
        self.properties
            .as_ref()
            .map(|props| props.values().any(|p| matches!(p, PropertySchema::Schema(_))))
            .unwrap_or(false)
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
        ) && !self.result.schema.has_schema_properties()
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
    // Use proper PascalCase for compound words to match existing codebase conventions:
    // - TxOut (not Txout)
    // - AddrMan (not Addrman)  
    // - ChainStates (not Chainstates)
    // - ChainTxStats (not ChainTxstats)
    const KNOWN_WORDS: &[(&str, &str)] = &[
        // (lowercase pattern, PascalCase output)
        // Compound words with special casing - longer patterns first
        ("txspendingprevout", "TxSpendingPrevOut"),
        ("prevout", "PrevOut"),
        ("uploadtarget", "Uploadtarget"),  // Match existing convention
        ("txoutset", "TxOutSet"),
        ("txoutproof", "TxOutProof"),
        ("txout", "TxOut"),
        ("txstats", "TxStats"),
        ("addrman", "AddrMan"),
        ("chainstates", "ChainStates"),
        ("chaintxstats", "ChainTxStats"),
        ("blockstats", "BlockStats"),
        ("networkactive", "NetworkActive"),
        ("mempoolaccept", "MempoolAccept"),
        // Regular words (alphabetically sorted, longer forms before shorter)
        ("abandon", "Abandon"), ("abort", "Abort"), ("accept", "Accept"),
        ("activity", "Activity"), ("add", "Add"), ("address", "Address"),
        ("addresses", "Addresses"), ("all", "All"), ("analyze", "Analyze"),
        ("ancestors", "Ancestors"), ("api", "Api"), ("backup", "Backup"),
        ("balance", "Balance"), ("balances", "Balances"), ("banned", "Banned"),
        ("best", "Best"), ("bip125", "Bip125"), ("blockchain", "Blockchain"),
        ("block", "Block"), ("blocks", "Blocks"), ("bump", "Bump"),
        ("chain", "Chain"), ("change", "Change"), ("clear", "Clear"),
        ("combine", "Combine"), ("connection", "Connection"), ("control", "Control"),
        ("convert", "Convert"), ("count", "Count"), ("create", "Create"),
        ("decode", "Decode"), ("delete", "Delete"), ("deployment", "Deployment"),
        ("derive", "Derive"), ("descendants", "Descendants"), ("descriptor", "Descriptor"),
        ("descriptors", "Descriptors"), ("difficulty", "Difficulty"), ("dir", "Dir"),
        ("disconnect", "Disconnect"), ("display", "Display"), ("dump", "Dump"),
        ("echo", "Echo"), ("encrypt", "Encrypt"), ("entry", "Entry"),
        ("enumerate", "Enumerate"), ("estimate", "Estimate"), ("fee", "Fee"),
        ("filter", "Filter"), ("finalize", "Finalize"), ("for", "For"),
        ("from", "From"), ("fund", "Fund"), ("funded", "Funded"),
        ("generate", "Generate"), ("get", "Get"), ("group", "Group"),
        ("groupings", "Groupings"), ("hash", "Hash"), ("hashps", "Hashps"),
        ("hd", "Hd"), ("header", "Header"), ("height", "Height"),
        ("help", "Help"), ("hex", "Hex"), ("import", "Import"),
        ("index", "Index"), ("info", "Info"), ("ipc", "Ipc"),
        ("join", "Join"), ("json", "Json"), ("key", "Key"),
        ("keys", "Keys"), ("label", "Label"), ("labels", "Labels"),
        ("list", "List"), ("load", "Load"), ("loaded", "Loaded"),
        ("lock", "Lock"), ("logging", "Logging"), ("mempool", "Mempool"),
        ("memory", "Memory"), ("message", "Message"), ("mining", "Mining"),
        ("msg", "Msg"), ("multisig", "Multisig"), ("net", "Net"),
        ("network", "Network"), ("new", "New"), ("node", "Node"),
        ("orphan", "Orphan"), ("out", "Out"), ("outset", "Outset"),
        ("package", "Package"), ("passphrase", "Passphrase"), ("peer", "Peer"),
        ("peers", "Peers"), ("pool", "Pool"), ("precious", "Precious"),
        ("prioritise", "Prioritise"), ("prioritised", "Prioritised"), ("priv", "Priv"),
        ("process", "Process"), ("proof", "Proof"), ("prune", "Prune"),
        ("psbts", "Psbts"), ("psbt", "Psbt"), ("raw", "Raw"),
        ("received", "Received"), ("recipient", "Recipient"), ("rescan", "Rescan"),
        ("restore", "Restore"), ("rpc", "Rpc"), ("save", "Save"),
        ("scan", "Scan"), ("script", "Script"), ("send", "Send"),
        ("set", "Set"), ("sign", "Sign"), ("signer", "Signer"),
        ("signers", "Signers"), ("simulate", "Simulate"), ("since", "Since"),
        ("smart", "Smart"), ("spending", "Spending"), ("stats", "Stats"),
        ("status", "Status"), ("stop", "Stop"), ("submit", "Submit"),
        ("template", "Template"), ("test", "Test"), ("tips", "Tips"),
        ("to", "To"), ("totals", "Totals"), ("transactions", "Transactions"),
        ("transaction", "Transaction"), ("txs", "Txs"), ("tx", "Tx"),
        ("unload", "Unload"), ("unlock", "Unlock"), ("unspent", "Unspent"),
        ("update", "Update"), ("upgrade", "Upgrade"), ("uptime", "Uptime"),
        ("utxo", "Utxo"), ("validate", "Validate"), ("verify", "Verify"),
        ("wait", "Wait"), ("wallet", "Wallet"), ("wallets", "Wallets"),
        ("with", "With"), ("zmq", "Zmq"),
    ];

    let mut result = String::new();
    let mut remaining = name.to_lowercase();

    while !remaining.is_empty() {
        // Find the longest matching word at the start
        let mut best_match: Option<(&str, &str)> = None;
        for &(pattern, pascal) in KNOWN_WORDS {
            if remaining.starts_with(pattern) {
                match best_match {
                    None => best_match = Some((pattern, pascal)),
                    Some((current_pattern, _)) if pattern.len() > current_pattern.len() => {
                        best_match = Some((pattern, pascal))
                    }
                    _ => {}
                }
            }
        }

        if let Some((pattern, pascal)) = best_match {
            // Use the pre-defined PascalCase version
            result.push_str(pascal);
            remaining = remaining[pattern.len()..].to_string();
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
        // Updated to match existing codebase conventions (TxOut, not Txout)
        assert_eq!(method_name_to_pascal_case("dumptxoutset"), "DumpTxOutSet");
        assert_eq!(method_name_to_pascal_case("waitforblock"), "WaitForBlock");
        assert_eq!(method_name_to_pascal_case("gettxout"), "GetTxOut");
        assert_eq!(method_name_to_pascal_case("scantxoutset"), "ScanTxOutSet");
        assert_eq!(method_name_to_pascal_case("getaddrmaninfo"), "GetAddrManInfo");
        assert_eq!(method_name_to_pascal_case("getchainstates"), "GetChainStates");
        assert_eq!(method_name_to_pascal_case("getchaintxstats"), "GetChainTxStats");
        assert_eq!(method_name_to_pascal_case("getblockstats"), "GetBlockStats");
        assert_eq!(method_name_to_pascal_case("testmempoolaccept"), "TestMempoolAccept");
        assert_eq!(method_name_to_pascal_case("setnetworkactive"), "SetNetworkActive");
        assert_eq!(method_name_to_pascal_case("gettxspendingprevout"), "GetTxSpendingPrevOut");
    }
}
