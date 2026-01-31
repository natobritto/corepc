// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v30` - mining.
//!
//! Types for methods found under the `== Mining ==` section of the API docs.

mod error;
mod into;

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

pub use self::error::{BlockTemplateTransactionError, GetBlockTemplateError, GetMiningInfoError};
pub use super::{NextBlockInfo, NextBlockInfoError};

/// Result of the JSON-RPC method `getblocktemplate`.
///
/// > getblocktemplate
/// >
/// > If the request parameters include a 'mode' key, that is used to explicitly select between the
/// > default 'template' request or a 'proposal'.
///
/// v30 makes `weight_limit` optional.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockTemplate {
    /// The preferred block version.
    pub version: i32,
    /// Specific block rules that are to be enforced.
    pub rules: Vec<String>,
    /// Set of pending, supported versionbit (BIP 9) softfork deployments.
    #[serde(rename = "vbavailable")]
    pub version_bits_available: BTreeMap<String, u32>,
    /// Client side supported features.
    pub capabilities: Vec<String>,
    /// Bit mask of versionbits the server requires set in submissions.
    #[serde(rename = "vbrequired")]
    pub version_bits_required: i64,
    /// The hash of current highest block.
    #[serde(rename = "previousblockhash")]
    pub previous_block_hash: String,
    /// Contents of non-coinbase transactions that should be included in the next block.
    pub transactions: Vec<BlockTemplateTransaction>,
    /// Data that should be included in the coinbase's scriptSig content.
    #[serde(rename = "coinbaseaux")]
    pub coinbase_aux: BTreeMap<String, String>,
    /// Maximum allowable input to coinbase transaction (in satoshis).
    #[serde(rename = "coinbasevalue")]
    pub coinbase_value: i64,
    /// An id to include with a request to longpoll on an update to this template.
    #[serde(rename = "longpollid")]
    pub long_poll_id: Option<String>,
    /// The hash target.
    pub target: String,
    /// The minimum timestamp appropriate for next block time (UNIX epoch).
    #[serde(rename = "mintime")]
    pub min_time: u32,
    /// List of ways the block template may be changed.
    pub mutable: Vec<String>,
    /// A range of valid nonces.
    #[serde(rename = "noncerange")]
    pub nonce_range: String,
    /// Limit of sigops in blocks.
    #[serde(rename = "sigoplimit")]
    pub sigop_limit: i64,
    /// Limit of block size.
    #[serde(rename = "sizelimit")]
    pub size_limit: i64,
    /// Limit of block weight (optional in v30).
    #[serde(rename = "weightlimit")]
    pub weight_limit: Option<i64>,
    /// Current timestamp in seconds since epoch.
    #[serde(rename = "curtime")]
    pub current_time: u64,
    /// Compressed target of next block.
    pub bits: String,
    /// The height of the next block.
    pub height: i64,
    /// Optional signet challenge.
    pub signet_challenge: Option<String>,
    /// A valid witness commitment for the unmodified block template.
    pub default_witness_commitment: Option<String>,
}

/// Transaction contents. Part of `getblocktemplate`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct BlockTemplateTransaction {
    /// Transaction data encoded in hexadecimal (byte-for-byte).
    pub data: String,
    /// Transaction id encoded in little-endian hexadecimal.
    pub txid: String,
    /// Hash encoded in little-endian hexadecimal (including witness data).
    pub hash: String,
    /// Transactions before this one that must be present in the final block.
    pub depends: Vec<i64>,
    /// Difference in value between transaction inputs and outputs (in satoshis).
    pub fee: i64,
    /// Total SigOps cost.
    pub sigops: i64,
    /// Total transaction weight.
    pub weight: u64,
}

/// Result of the JSON-RPC method `getmininginfo`.
///
/// > getmininginfo
/// >
/// > Returns a json object containing mining-related information.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMiningInfo {
    /// The current block.
    pub blocks: u64,
    /// The block weight (including reserved weight for block header, txs count and coinbase tx) of
    /// the last assembled block (only present if a block was ever assembled).
    #[serde(rename = "currentblockweight")]
    pub current_block_weight: Option<u64>,
    /// The number of block transactions (excluding coinbase) of the last assembled block (only
    /// present if a block was ever assembled).
    #[serde(rename = "currentblocktx")]
    pub current_block_tx: Option<i64>,
    /// The current nBits, compact representation of the block difficulty target.
    pub bits: String,
    /// The current difficulty.
    pub difficulty: f64,
    /// The current target.
    pub target: String,
    /// The network hashes per second.
    #[serde(rename = "networkhashps")]
    pub network_hash_ps: f64,
    /// The size of the mempool.
    #[serde(rename = "pooledtx")]
    pub pooled_tx: i64,
    /// Minimum feerate of packages selected for block inclusion in BTC/kvB.
    #[serde(rename = "blockmintxfee")]
    pub block_min_tx_fee: f64,
    /// Current network name as defined in BIP70 (main, test, regtest).
    pub chain: String,
    /// The block challenge (aka. block script), in hexadecimal (only present if the current network
    /// is a signet).
    pub signet_challenge: Option<String>,
    /// The next block.
    pub next: NextBlockInfo,
    /// Any network and blockchain warnings.
    pub warnings: Vec<String>,
}
