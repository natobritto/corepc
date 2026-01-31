// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core - network.
//!
//! Types for methods found under the `== Network ==` section of the API docs.
//!
//! This file is auto-generated from OpenRPC specification.

use serde::{Deserialize, Serialize};

/// Result of the JSON-RPC method `getaddrmaninfo`.
///
/// > getaddrmaninfo
/// >
/// > Provides information about the node's address manager by returning the number of addresses in the `new` and `tried` tables and their sum for all networks.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetAddrmanInfo(
    /// json object with network type as keys
    pub std::collections::BTreeMap<String, GetAddrmanInfoEntry>,
);

/// the network (ipv4, ipv6, onion, i2p, cjdns, all_networks)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetAddrmanInfoEntry {
    /// number of addresses in the new table, which represent potential peers the node has discovered but hasn't yet successfully connected to.
    pub new: i64,
    /// total number of addresses in both new/tried tables
    pub total: i64,
    /// number of addresses in the tried table, which represent peers the node has successfully connected to in the past.
    pub tried: i64,
}

/// Returns information about network traffic, including bytes in, bytes out,
/// and current system time.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetNetTotals {
    /// Current system UNIX epoch time in milliseconds
    #[serde(rename = "timemillis")]
    pub time_millis: i64,
    /// Total bytes received
    #[serde(rename = "totalbytesrecv")]
    pub total_bytes_recv: i64,
    /// Total bytes sent
    #[serde(rename = "totalbytessent")]
    pub total_bytes_sent: i64,
    pub uploadtarget: GetNetTotalsUploadtarget,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetNetTotalsUploadtarget {
    /// Bytes left in current time cycle
    pub bytes_left_in_cycle: i64,
    /// True if serving historical blocks
    pub serve_historical_blocks: bool,
    /// Target in bytes
    pub target: i64,
    /// True if target is reached
    pub target_reached: bool,
    /// Seconds left in current time cycle
    pub time_left_in_cycle: i64,
    /// Length of the measuring timeframe in seconds
    #[serde(rename = "timeframe")]
    pub time_frame: i64,
}

/// Returns an object containing various state info regarding P2P networking.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetNetworkInfo {
    /// the total number of connections
    pub connections: i64,
    /// the number of inbound connections
    pub connections_in: i64,
    /// the number of outbound connections
    pub connections_out: i64,
    /// minimum fee rate increment for mempool limiting or replacement in BTC/kvB
    #[serde(rename = "incrementalfee")]
    pub incremental_fee: i64,
    /// list of local addresses
    pub localaddresses: Vec<GetNetworkInfoLocaladdressesItem>,
    /// true if transaction relay is requested from peers
    pub localrelay: bool,
    /// the services we offer to the network
    pub localservices: String,
    /// the services we offer to the network, in human-readable form
    pub localservicesnames: Vec<String>,
    /// whether p2p networking is enabled
    #[serde(rename = "networkactive")]
    pub network_active: bool,
    /// information per network
    pub networks: Vec<GetNetworkInfoNetworksItem>,
    /// the protocol version
    pub protocolversion: i64,
    /// minimum relay fee rate for transactions in BTC/kvB
    #[serde(rename = "relayfee")]
    pub relay_fee: i64,
    /// the server subversion string
    pub subversion: String,
    /// the time offset
    #[serde(rename = "timeoffset")]
    pub time_offset: i64,
    /// the server version
    pub version: i64,
    /// any network and blockchain warnings (run with `-deprecatedrpc=warnings` to return the latest warning as a single string)
    pub warnings: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetNetworkInfoLocaladdressesItem {
    /// network address
    pub address: String,
    /// network port
    pub port: i64,
    /// relative score
    pub score: i64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetNetworkInfoNetworksItem {
    /// is the network limited using -onlynet?
    #[serde(rename = "limited")]
    pub limit_ed: bool,
    /// network (ipv4, ipv6, onion, i2p, cjdns)
    pub name: String,
    /// ("host:port") the proxy that is used for this network, or empty if none
    pub proxy: String,
    /// Whether randomized credentials are used
    pub proxy_randomize_credentials: bool,
    /// is the network reachable?
    pub reachable: bool,
}

