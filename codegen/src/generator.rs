// SPDX-License-Identifier: CC0-1.0

//! Rust code generator for Bitcoin Core RPC types.
//!
//! Generates Rust structs from OpenRPC schemas with proper serde attributes.

use std::collections::{BTreeMap, HashSet};

use heck::ToPascalCase;

use crate::schema::{AdditionalProperties, Method, Schema, SchemaOrArray};

/// Configuration for the code generator.
#[derive(Debug, Clone)]
pub struct GeneratorConfig {
    /// Whether to add `deny_unknown_fields` attribute.
    pub deny_unknown_fields: bool,
    /// Whether to generate into_model conversions (reserved for future use).
    #[allow(dead_code)]
    pub generate_model_conversions: bool,
    /// Whether to generate wrapper types for simple return values (string, bool, number).
    pub generate_simple_wrappers: bool,
}

impl Default for GeneratorConfig {
    fn default() -> Self {
        Self {
            deny_unknown_fields: true,
            generate_model_conversions: false, // Model types require manual work
            generate_simple_wrappers: true,    // Generate AbortRescan(bool), etc.
        }
    }
}

/// Represents a generated Rust type.
#[derive(Debug, Clone)]
pub struct GeneratedType {
    /// The Rust struct/enum definition.
    pub definition: String,
    /// Nested types that need to be generated alongside.
    pub nested_types: Vec<GeneratedType>,
    /// The name of this type.
    pub name: String,
}

/// Code generator for creating Rust types from OpenRPC schemas.
pub struct Generator {
    config: GeneratorConfig,
    /// Types that have already been generated (to avoid duplicates).
    generated_names: HashSet<String>,
}

impl Generator {
    /// Create a new generator with the given configuration.
    pub fn new(config: GeneratorConfig) -> Self {
        Self {
            config,
            generated_names: HashSet::new(),
        }
    }

    /// Generate a Rust type for a method's result.
    pub fn generate_method_result(&mut self, method: &Method) -> Option<GeneratedType> {
        // Skip methods that return null
        if method.returns_null() {
            return None;
        }

        let schema = &method.result.schema;
        let struct_name = method.struct_name();

        // For simple types, generate a wrapper struct if configured
        if method.returns_simple_type() && !schema.is_dynamic_object() {
            if self.config.generate_simple_wrappers {
                return self.generate_simple_wrapper(method);
            }
            return None;
        }

        // Handle oneOf/anyOf schemas - generate enum or pick the object variant
        if let Some(ref one_of) = schema.one_of {
            return self.generate_one_of_type(&struct_name, one_of, Some(&method.description));
        }
        if let Some(ref any_of) = schema.any_of {
            return self.generate_one_of_type(&struct_name, any_of, Some(&method.description));
        }

        // For dynamic objects (maps), generate a newtype wrapper
        if schema.is_dynamic_object() {
            return self.generate_map_type(method, schema);
        }

        self.generate_struct_type(&struct_name, schema, Some(&method.description))
    }

    /// Generate a simple wrapper type like `pub struct AbortRescan(pub bool);`
    fn generate_simple_wrapper(&mut self, method: &Method) -> Option<GeneratedType> {
        let struct_name = method.struct_name();
        
        if self.generated_names.contains(&struct_name) {
            return None;
        }
        self.generated_names.insert(struct_name.clone());

        let inner_type = match method.result.schema.type_.as_deref() {
            Some("string") => "String",
            Some("boolean") => "bool",
            Some("number") | Some("integer") => "i64",
            _ => return None,
        };

        let doc = format!(
            "/// Result of the JSON-RPC method `{}`.\n///\n/// > {}\n/// >\n/// > {}",
            method.name,
            method.name,
            method.summary.lines().collect::<Vec<_>>().join("\n/// > ")
        );

        let deny_unknown = if self.config.deny_unknown_fields {
            "\n#[cfg_attr(feature = \"serde-deny-unknown-fields\", serde(deny_unknown_fields))]"
        } else {
            ""
        };

        let definition = format!(
            r#"{doc}
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]{deny_unknown}
pub struct {struct_name}(pub {inner_type});
"#,
        );

        Some(GeneratedType {
            definition,
            nested_types: vec![],
            name: struct_name,
        })
    }

    /// Generate a simple wrapper type from a schema (used for oneOf/anyOf variants).
    fn generate_simple_wrapper_from_schema(
        &mut self,
        name: &str,
        schema: &Schema,
        doc: Option<&str>,
    ) -> Option<GeneratedType> {
        if self.generated_names.contains(name) {
            return None;
        }
        self.generated_names.insert(name.to_string());

        let inner_type = match schema.type_.as_deref() {
            Some("string") => "String",
            Some("boolean") => "bool",
            Some("number") | Some("integer") => "i64",
            _ => return None,
        };

        let doc_str = doc
            .map(|d| format!("/// {}\n", escape_doc(d)))
            .unwrap_or_default();

        let deny_unknown = if self.config.deny_unknown_fields {
            "\n#[cfg_attr(feature = \"serde-deny-unknown-fields\", serde(deny_unknown_fields))]"
        } else {
            ""
        };

        let definition = format!(
            r#"{doc_str}#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]{deny_unknown}
pub struct {name}(pub {inner_type});
"#,
        );

        Some(GeneratedType {
            definition,
            nested_types: vec![],
            name: name.to_string(),
        })
    }

    /// Generate types for oneOf/anyOf schemas.
    /// For Bitcoin Core, these typically represent different verbosity levels.
    fn generate_one_of_type(
        &mut self,
        name: &str,
        variants: &[Schema],
        doc: Option<&str>,
    ) -> Option<GeneratedType> {
        if variants.len() == 1 {
            if variants[0].type_.as_deref() == Some("object") {
                return self.generate_struct_type(name, &variants[0], doc);
            }
            return self.generate_simple_wrapper_from_schema(name, &variants[0], doc);
        }

        // If there are multiple variants (e.g., different verbosity levels),
        // generate a struct or wrapper for each with a suffix
        let mut all_types = Vec::new();
        let mut all_names = Vec::new();

        for (i, variant) in variants.iter().enumerate() {
            let variant_name = if variants.len() > 1 {
                // Try to use x-bitcoin-condition for naming
                let suffix = variant
                    .description
                    .as_ref()
                    .and_then(|d| {
                        if d.contains("verbosity = 0") || d.contains("verbosity=0") {
                            Some("Verbose0")
                        } else if d.contains("verbosity = 1") || d.contains("verbosity=1") {
                            Some("Verbose1")
                        } else if d.contains("verbosity = 2") || d.contains("verbosity=2") {
                            Some("Verbose2")
                        } else if d.contains("verbosity = 3") || d.contains("verbosity=3") {
                            Some("Verbose3")
                        } else {
                            None
                        }
                    })
                    .unwrap_or_else(|| match i {
                        0 => "VerboseZero",
                        1 => "VerboseOne",
                        2 => "VerboseTwo",
                        3 => "VerboseThree",
                        4 => "VerboseFour",
                        _ => "---------ERROR_badtype",
                    });
                format!("{}{}", name, suffix)
            } else {
                name.to_string()
            };

            if variant_name.contains("GetRawTran") {
                println!("Debug: Generating variant '{}' for method '{}'", variant_name, name);
            }

            let generated = if variant.type_.as_deref() == Some("object") {
                self.generate_struct_type(&variant_name, variant, doc)
            } else {
                self.generate_simple_wrapper_from_schema(&variant_name, variant, doc)
            };

            if let Some(generated) = generated {
                all_names.push(generated.name.clone());
                all_types.push(generated);
            }
        }

        // Also check for array variants
        for variant in variants.iter() {
            if variant.type_.as_deref() == Some("array") {
                // This is typically a simple array return - we can skip generating a type for it
                // since it would just be Vec<T>
            }
        }

        if all_types.is_empty() {
            return None;
        }

        // Return the first/primary type with all others as nested.
        // Make sure to also include each variant's nested_types (like array item structs).
        let mut primary = all_types.remove(0);
        for variant in all_types {
            // First add the variant's own nested types
            primary.nested_types.extend(variant.nested_types.clone());
            // Then add the variant itself
            primary.nested_types.push(variant);
        }
        Some(primary)
    }

    /// Generate a map/newtype wrapper for dynamic object types.
    fn generate_map_type(&mut self, method: &Method, schema: &Schema) -> Option<GeneratedType> {
        let struct_name = method.struct_name();

        if self.generated_names.contains(&struct_name) {
            return None;
        }
        self.generated_names.insert(struct_name.clone());

        let value_type = match &schema.additional_properties {
            Some(AdditionalProperties::Schema(inner_schema)) => {
                // Generate the inner type
                // Avoid double Entry suffix (e.g., GetRawAddrmanEntryEntry)
                let inner_name = if struct_name.ends_with("Entry") {
                    format!("{}Item", struct_name)
                } else {
                    format!("{}Entry", struct_name)
                };
                self.generate_inner_type(&inner_name, inner_schema)
            }
            _ => ("serde_json::Value".to_string(), vec![]),
        };

        let doc = format!(
            "/// Result of the JSON-RPC method `{}`.\n///\n/// > {}\n/// >\n/// > {}",
            method.name,
            method.name,
            method.summary.lines().collect::<Vec<_>>().join("\n/// > ")
        );

        let deny_unknown = if self.config.deny_unknown_fields {
            "\n#[cfg_attr(feature = \"serde-deny-unknown-fields\", serde(deny_unknown_fields))]"
        } else {
            ""
        };

        let definition = format!(
            r#"{doc}
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]{deny_unknown}
pub struct {struct_name}(
    /// {desc}
    pub std::collections::BTreeMap<String, {value_type}>,
);
"#,
            doc = doc,
            deny_unknown = deny_unknown,
            struct_name = struct_name,
            desc = schema.description.as_deref().unwrap_or("Map entries"),
            value_type = value_type.0,
        );

        Some(GeneratedType {
            definition,
            nested_types: value_type.1,
            name: struct_name,
        })
    }

    /// Generate a struct type from a schema.
    fn generate_struct_type(
        &mut self,
        name: &str,
        schema: &Schema,
        doc: Option<&str>,
    ) -> Option<GeneratedType> {
        if self.generated_names.contains(name) {
            return None;
        }
        self.generated_names.insert(name.to_string());

        let empty_props = std::collections::HashMap::new();
        let properties = schema.properties.as_ref().unwrap_or(&empty_props);
        let required = schema.required.as_ref().map(|v| v.iter().collect::<HashSet<_>>());

        let mut fields = Vec::new();
        let mut nested_types = Vec::new();

        // Sort properties for consistent output
        let sorted_props: BTreeMap<_, _> = properties.iter().collect();

        for (prop_name, prop_schema) in sorted_props {
            let is_required = required
                .as_ref()
                .map(|r| r.contains(prop_name))
                .unwrap_or(false);
            let is_optional = !is_required || prop_schema.is_optional();

            let (field_type, field_nested) =
                self.schema_to_rust_type(prop_schema, name, prop_name);
            nested_types.extend(field_nested);

            let rust_field_name = to_rust_field_name(prop_name);
            let serde_rename = if rust_field_name != *prop_name {
                format!("    #[serde(rename = \"{}\")]\n", prop_name)
            } else {
                String::new()
            };

            let final_type = if is_optional {
                format!("Option<{}>", field_type)
            } else {
                field_type
            };

            let field_doc = prop_schema
                .description
                .as_ref()
                .map(|d| format!("    /// {}\n", escape_doc(d)))
                .unwrap_or_default();

            fields.push(format!(
                "{field_doc}{serde_rename}    pub {rust_field_name}: {final_type},"
            ));
        }

        let doc_str = doc
            .map(|d| format!("/// {}\n", escape_doc(d)))
            .unwrap_or_default();

        let deny_unknown = if self.config.deny_unknown_fields {
            "\n#[cfg_attr(feature = \"serde-deny-unknown-fields\", serde(deny_unknown_fields))]"
        } else {
            ""
        };

        let definition = format!(
            r#"{doc_str}#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]{deny_unknown}
pub struct {name} {{
{fields}
}}
"#,
            doc_str = doc_str,
            deny_unknown = deny_unknown,
            name = name,
            fields = fields.join("\n"),
        );

        Some(GeneratedType {
            definition,
            nested_types,
            name: name.to_string(),
        })
    }

    /// Generate an inner/nested type and return its name and definitions.
    fn generate_inner_type(&mut self, name: &str, schema: &Schema) -> (String, Vec<GeneratedType>) {
        // If it's an object with properties, generate a struct
        if schema.properties.is_some() {
            if let Some(generated) = self.generate_struct_type(name, schema, schema.description.as_deref()) {
                let nested = std::iter::once(generated.clone())
                    .chain(generated.nested_types.clone())
                    .collect();
                return (name.to_string(), nested);
            }
        }

        // If it's a dynamic object (map), handle the nested additionalProperties
        if schema.is_dynamic_object() {
            if let Some(AdditionalProperties::Schema(inner)) = &schema.additional_properties {
                let inner_name = format!("{}Entry", name);
                let (inner_type, inner_nested) = self.generate_inner_type(&inner_name, inner);
                let map_type = format!("std::collections::BTreeMap<String, {}>", inner_type);
                return (map_type, inner_nested);
            }
        }

        // Otherwise, determine the inline type, using the name for context
        let (type_str, nested) = self.schema_to_rust_type(schema, name, "");
        (type_str, nested)
    }

    /// Convert a schema to a Rust type string.
    fn schema_to_rust_type(
        &mut self,
        schema: &Schema,
        parent_name: &str,
        field_name: &str,
    ) -> (String, Vec<GeneratedType>) {
        // Handle oneOf/anyOf
        if schema.one_of.is_some() || schema.any_of.is_some() {
            return ("serde_json::Value".to_string(), vec![]);
        }

        // Handle special bitcoin types
        if schema.is_hex() {
            return ("String".to_string(), vec![]);
        }
        if schema.is_amount() {
            return ("f64".to_string(), vec![]); // Could be String in some contexts
        }

        match schema.type_.as_deref() {
            Some("string") => ("String".to_string(), vec![]),
            Some("boolean") => ("bool".to_string(), vec![]),
            Some("number") => {
                // Use i64 for general numbers, f64 for decimals
                if schema.bitcoin_type.as_deref() == Some("amount") {
                    ("f64".to_string(), vec![])
                } else {
                    ("i64".to_string(), vec![])
                }
            }
            Some("integer") => ("i64".to_string(), vec![]),
            Some("null") => ("()".to_string(), vec![]),
            Some("array") => {
                let (item_type, nested) = match &schema.items {
                    Some(items) => match items.as_ref() {
                        SchemaOrArray::Schema(item_schema) => {
                            // If the item is an object, generate a nested struct
                            if item_schema.type_.as_deref() == Some("object")
                                && item_schema.properties.is_some()
                            {
                                let inner_name = format!(
                                    "{}{}Item",
                                    parent_name,
                                    field_name.to_pascal_case()
                                );
                                let (type_name, nested) =
                                    self.generate_inner_type(&inner_name, item_schema);
                                (type_name, nested)
                            } else {
                                self.schema_to_rust_type(item_schema, parent_name, field_name)
                            }
                        }
                        SchemaOrArray::Array(schemas) => {
                            // Tuple type for fixed arrays
                            if schemas.is_empty() {
                                ("serde_json::Value".to_string(), vec![])
                            } else {
                                // Just use Value for complex tuples
                                ("serde_json::Value".to_string(), vec![])
                            }
                        }
                    },
                    None => ("serde_json::Value".to_string(), vec![]),
                };
                (format!("Vec<{}>", item_type), nested)
            }
            Some("object") => {
                // Check if it's a dynamic object (map)
                if schema.is_dynamic_object() {
                    let (value_type, nested) = match &schema.additional_properties {
                        Some(AdditionalProperties::Schema(inner)) => {
                            // If the inner schema is an object, generate a nested struct
                            if inner.type_.as_deref() == Some("object") && inner.properties.is_some() {
                                let inner_name = format!(
                                    "{}{}",
                                    parent_name,
                                    field_name.to_pascal_case()
                                );
                                self.generate_inner_type(&inner_name, inner)
                            } else {
                                self.schema_to_rust_type(inner, parent_name, field_name)
                            }
                        }
                        _ => ("serde_json::Value".to_string(), vec![]),
                    };
                    return (
                        format!("std::collections::BTreeMap<String, {}>", value_type),
                        nested,
                    );
                }

                // Check for additionalProperties with schema (also a map)
                if let Some(AdditionalProperties::Schema(inner)) = &schema.additional_properties {
                    if schema.properties.is_none() {
                        let (value_type, nested) = if inner.type_.as_deref() == Some("object") && inner.properties.is_some() {
                            let inner_name = format!(
                                "{}{}",
                                parent_name,
                                field_name.to_pascal_case()
                            );
                            self.generate_inner_type(&inner_name, inner)
                        } else {
                            self.schema_to_rust_type(inner, parent_name, field_name)
                        };
                        return (
                            format!("std::collections::BTreeMap<String, {}>", value_type),
                            nested,
                        );
                    }
                }

                // Named nested object - generate a struct
                if schema.properties.is_some() {
                    let nested_name = format!(
                        "{}{}",
                        parent_name,
                        field_name.to_pascal_case()
                    );
                    if let Some(generated) = self.generate_struct_type(
                        &nested_name,
                        schema,
                        schema.description.as_deref(),
                    ) {
                        let mut nested = generated.nested_types.clone();
                        nested.push(generated);
                        return (nested_name, nested);
                    }
                }

                // Fallback for unknown objects
                ("serde_json::Value".to_string(), vec![])
            }
            _ => ("serde_json::Value".to_string(), vec![]),
        }
    }

    /// Reset the generator state for a new module.
    pub fn reset(&mut self) {
        self.generated_names.clear();
    }
}

/// Convert a JSON property name to a valid Rust field name (snake_case).
fn to_rust_field_name(name: &str) -> String {
    // First, replace hyphens with underscores (JSON allows hyphens, Rust doesn't)
    let name = name.replace('-', "_");
    
    // If already has underscores, just convert to lowercase and handle keywords
    if name.contains('_') {
        let lower = name.to_lowercase();
        return handle_rust_keyword(&lower);
    }

    // For all-lowercase names without word boundaries, try to split by known words
    let snake = split_into_snake_case(&name);
    handle_rust_keyword(&snake)
}

fn handle_rust_keyword(name: &str) -> String {
    match name {
        "type" => "type_".to_string(),
        "match" => "match_".to_string(),
        "ref" => "ref_".to_string(),
        "self" => "self_".to_string(),
        "mod" => "mod_".to_string(),
        "async" => "async_".to_string(),
        "await" => "await_".to_string(),
        "use" => "use_".to_string(),
        _ => name.to_string(),
    }
}

/// Split a lowercase name into snake_case using known words.
fn split_into_snake_case(name: &str) -> String {
    // First, handle camelCase by inserting underscores
    let mut with_underscores = String::new();
    let mut prev_lower = false;

    for c in name.chars() {
        if c.is_uppercase() && prev_lower {
            with_underscores.push('_');
        }
        with_underscores.push(c.to_ascii_lowercase());
        prev_lower = c.is_lowercase();
    }

    // If it contains underscores now or has mixed case originally, we're done
    if with_underscores.contains('_') && with_underscores != name.to_lowercase() {
        return with_underscores;
    }

    // For all-lowercase strings, try to split by known words
    let name_lower = name.to_lowercase();
    
    // Known word boundaries for Bitcoin Core RPC field names
    // Words are sorted with longer forms before shorter to ensure longest match
    const FIELD_WORDS: &[&str] = &[
        // Only include true compounds that should never be split
        "chainwork", "mediantime", "nchaintx",
        // Regular words (alphabetically sorted, longer forms before shorter)
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
        "written", "out", "stats", "spending",  "accept"
    ];

    let mut result = Vec::new();
    let mut remaining = name_lower.as_str();

    while !remaining.is_empty() {
        // Find the longest matching word at the start
        let mut best_match = None;
        for word in FIELD_WORDS {
            if remaining.starts_with(word) {
                match best_match {
                    None => best_match = Some(*word),
                    Some(current) if word.len() > current.len() => best_match = Some(*word),
                    _ => {}
                }
            }
        }

        if let Some(word) = best_match {
            result.push(word);
            remaining = &remaining[word.len()..];
        } else {
            // No word match - if there are already words, append remaining as is
            // Otherwise, just use the original
            if result.is_empty() {
                return name_lower;
            }
            result.push(remaining);
            break;
        }
    }

    result.join("_")
}

/// Escape doc comment content.
fn escape_doc(s: &str) -> String {
    s.replace('\n', "\n/// ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_rust_field_name() {
        // Already snake_case
        assert_eq!(to_rust_field_name("tx_count"), "tx_count");
        assert_eq!(to_rust_field_name("fee_delta"), "fee_delta");
        
        // camelCase
        assert_eq!(to_rust_field_name("blockHash"), "block_hash");
        
        // All lowercase - should split on known words
        assert_eq!(to_rust_field_name("blockhash"), "block_hash");
        assert_eq!(to_rust_field_name("networkhashps"), "network_hashps");
        assert_eq!(to_rust_field_name("pooledtx"), "pooled_tx");
        assert_eq!(to_rust_field_name("mempoolminfee"), "mempool_min_fee");
        
        // Compound words that should be split with underscore
        assert_eq!(to_rust_field_name("initialblockdownload"), "initial_block_download");
        assert_eq!(to_rust_field_name("verificationprogress"), "verification_progress");
        assert_eq!(to_rust_field_name("bestblockhash"), "bestblock_hash");
        
        // Headers should not become header_s
        assert_eq!(to_rust_field_name("headers"), "headers");
        
        // Keywords
        assert_eq!(to_rust_field_name("type"), "type_");
    }
}
