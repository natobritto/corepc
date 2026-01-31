// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core - control.
//!
//! Types for methods found under the `== Control ==` section of the API docs.
//!
//! This file is auto-generated from OpenRPC specification.

use serde::{Deserialize, Serialize};

/// Return an OpenRPC document describing the RPC API.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Api {

}

/// Returns an object containing information about memory usage.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMemoryInfo {
    /// Information about locked memory manager
    #[serde(rename = "locked")]
    pub lock_ed: GetMemoryInfoLocked,
}

/// Information about locked memory manager
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMemoryInfoLocked {
    /// Number unused chunks
    pub chunks_free: i64,
    /// Number allocated chunks
    pub chunks_used: i64,
    /// Number of bytes available in current arenas
    pub free: i64,
    /// Amount of bytes that succeeded locking. If this number is smaller than total, locking pages failed at some point and key data could be swapped to disk.
    #[serde(rename = "locked")]
    pub lock_ed: i64,
    /// Total number of bytes managed
    pub total: i64,
    /// Number of bytes used
    pub used: i64,
}

/// Returns details of the RPC server.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRpcInfo {
    /// All active commands
    pub active_commands: Vec<GetRpcInfoActiveCommandsItem>,
    /// The complete file path to the debug log
    #[serde(rename = "logpath")]
    pub log_path: String,
}

/// Information about an active command
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRpcInfoActiveCommandsItem {
    /// The running time in microseconds
    pub duration: i64,
    /// The name of the RPC command
    pub method: String,
}

/// Result of the JSON-RPC method `logging`.
///
/// > logging
/// >
/// > Gets and sets the logging configuration.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Logging(
    /// keys are the logging categories, and values indicates its status
    pub std::collections::BTreeMap<String, bool>,
);

