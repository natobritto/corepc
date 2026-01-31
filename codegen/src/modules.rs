// SPDX-License-Identifier: CC0-1.0

//! Module file generator.
//!
//! Organizes generated types into modules matching the existing crate structure.

use std::collections::BTreeMap;

use heck::ToPascalCase;

use crate::generator::{GeneratedType, Generator, GeneratorConfig};
use crate::schema::Method;

/// Categories that map to module names.
const CATEGORY_TO_MODULE: &[(&str, &str)] = &[
    ("blockchain", "blockchain"),
    ("control", "control"),
    ("generating", "generating"),
    ("hidden", "hidden"),
    ("mining", "mining"),
    ("network", "network"),
    ("rawtransactions", "raw_transactions"),
    ("signer", "signer"),
    ("util", "util"),
    ("wallet", "wallet"),
    ("zmq", "zmq"),
];

/// Get the module name for a category.
fn category_to_module(category: &str) -> &str {
    CATEGORY_TO_MODULE
        .iter()
        .find(|(cat, _)| *cat == category)
        .map(|(_, module)| *module)
        .unwrap_or(category)
}

/// Represents a generated module.
#[derive(Debug, Clone)]
pub struct GeneratedModule {
    /// Module name (e.g., "blockchain", "wallet").
    pub name: String,
    /// The full Rust code for this module.
    pub code: String,
    /// Types exported from this module.
    pub exports: Vec<String>,
    /// Method info for documentation.
    pub methods: Vec<MethodInfo>,
}

/// Basic info about a method for documentation.
#[derive(Debug, Clone)]
pub struct MethodInfo {
    /// The RPC method name.
    pub name: String,
    /// The Rust struct name (if any).
    pub struct_name: Option<String>,
    /// Whether it returns a model type (reserved for future use).
    #[allow(dead_code)]
    pub has_model: bool,
    /// Short notes about the method.
    pub notes: String,
}

/// Generate all modules from a list of methods.
pub fn generate_modules(methods: &[Method], config: GeneratorConfig) -> Vec<GeneratedModule> {
    let mut generator = Generator::new(config);

    // Group methods by category
    let mut by_category: BTreeMap<String, Vec<&Method>> = BTreeMap::new();
    for method in methods {
        let category = method.category().to_string();
        by_category.entry(category).or_default().push(method);
    }

    let mut modules = Vec::new();

    for (category, methods) in by_category {
        generator.reset();
        let module = generate_module(&category, &methods, &mut generator);
        modules.push(module);
    }

    modules
}

/// Generate a single module.
fn generate_module(category: &str, methods: &[&Method], generator: &mut Generator) -> GeneratedModule {
    let module_name = category_to_module(category).to_string();

    let mut all_types: Vec<GeneratedType> = Vec::new();
    let mut method_infos = Vec::new();
    let mut exports = Vec::new();

    for method in methods {
        let struct_name = if method.returns_null() {
            None
        } else if method.returns_simple_type() {
            None
        } else {
            Some(method.struct_name())
        };

        let notes = if method.returns_null() {
            "returns nothing".to_string()
        } else if method.returns_simple_type() {
            format!("returns {}", method.result.schema.type_.as_deref().unwrap_or("unknown"))
        } else {
            "version + model".to_string()
        };

        method_infos.push(MethodInfo {
            name: method.name.clone(),
            struct_name: struct_name.clone(),
            has_model: false, // Model types need manual implementation
            notes,
        });

        if let Some(generated) = generator.generate_method_result(method) {
            exports.push(generated.name.clone());
            // Add nested types first
            for nested in &generated.nested_types {
                exports.push(nested.name.clone());
                all_types.push(nested.clone());
            }
            all_types.push(generated);
        }
    }

    // Generate module code
    let mut code = String::new();

    // Module header
    code.push_str(&format!(
        r#"// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core - {category}.
//!
//! Types for methods found under the `== {category_title} ==` section of the API docs.
//!
//! This file is auto-generated from OpenRPC specification.

use serde::{{Deserialize, Serialize}};

"#,
        category = category,
        category_title = category.to_pascal_case(),
    ));

    // Generate all types (sorted by name for consistency)
    let mut sorted_types = all_types.clone();
    sorted_types.sort_by(|a, b| a.name.cmp(&b.name));

    for generated_type in sorted_types {
        code.push_str(&generated_type.definition);
        code.push('\n');
    }

    GeneratedModule {
        name: module_name,
        code,
        exports,
        methods: method_infos,
    }
}

/// Generate the mod.rs file that re-exports all types.
pub fn generate_mod_rs(modules: &[GeneratedModule], version: &str) -> String {
    let mut code = String::new();

    // Header
    code.push_str(&format!(
        r#"// SPDX-License-Identifier: CC0-1.0

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

"#,
        version = version,
    ));

    // Generate documentation tables for each category
    for module in modules {
        code.push_str(&format!(
            "//! <details>\n//! <summary> Methods from the == {} == section </summary>\n//!\n",
            module.name.to_pascal_case()
        ));
        code.push_str("//! | JSON-RPC Method Name               | Returns         | Notes                                  |\n");
        code.push_str("//! |:-----------------------------------|:---------------:|:--------------------------------------:|\n");

        for method in &module.methods {
            let returns = if let Some(ref name) = method.struct_name {
                format!("version ({})", name)
            } else {
                method.notes.clone()
            };
            code.push_str(&format!(
                "//! | {:<36} | {:<15} | {:<38} |\n",
                method.name,
                returns,
                ""
            ));
        }
        code.push_str("//!\n//! </details>\n//!\n");
    }

    code.push('\n');

    // Module declarations
    for module in modules {
        if !module.exports.is_empty() {
            code.push_str(&format!("pub mod {};\n", module.name));
        }
    }

    code.push('\n');

    // Re-exports
    for module in modules {
        if !module.exports.is_empty() {
            let exports = module.exports.join(", ");
            code.push_str(&format!(
                "pub use self::{}::{{{}}};\n",
                module.name, exports
            ));
        }
    }

    code
}
