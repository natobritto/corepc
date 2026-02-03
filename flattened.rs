// SPDX-License-Identifier: CC0-1.0

//! # JSON-RPC types for Bitcoin Core `v30`
//!
//! These structs are shaped for the JSON data returned by the JSON-RPC API. They use stdlib types
//! (or custom types) and where necessary implement an `into_model` function to convert the type to
//! a [`crate::model`] type of the same name. The types in this module are version specific. The
//! types in the `model` module are version nonspecific and are strongly typed using `rust-bitcoin`.
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
//!
//! If a method has UNTESTED then there is no integration test yet for it.
//!
//! <details>
//! <summary> Methods from the == Blockchain == section </summary>
//!
//! | JSON-RPC Method Name               | Returns         | Notes                                  |
//! |:-----------------------------------|:---------------:|:--------------------------------------:|
//! | dumptxoutset                       | version + model |                                        |
//! | getbestblockhash                   | version + model |                                        |
//! | getblock                           | version + model | Includes additional 'verbose' type     |
//! | getblockchaininfo                  | version + model |                                        |
//! | getblockcount                      | version + model |                                        |
//! | getblockfilter                     | version + model |                                        |
//! | getblockfrompeer                   | returns nothing |                                        |
//! | getblockhash                       | version + model |                                        |
//! | getblockheader                     | version + model | Includes additional 'verbose' type     |
//! | getblockstats                      | version + model |                                        |
//! | getchainstates                     | version + model |                                        |
//! | getchaintips                       | version + model |                                        |
//! | getchaintxstats                    | version + model |                                        |
//! | getdeploymentinfo                  | version + model |                                        |
//! | getdescriptoractivity              | version + model |                                        |
//! | getdifficulty                      | version + model |                                        |
//! | getmempoolancestors                | version + model |                                        |
//! | getmempooldescendants              | version + model |                                        |
//! | getmempoolentry                    | version + model |                                        |
//! | getmempoolinfo                     | version + model |                                        |
//! | getrawmempool                      | version + model | Includes additional 'verbose' type     |
//! | gettxout                           | version + model |                                        |
//! | gettxoutproof                      | returns string  |                                        |
//! | gettxoutsetinfo                    | version + model |                                        |
//! | gettxspendingprevout               | version + model |                                        |
//! | importmempool                      | returns nothing |                                        |
//! | loadtxoutset                       | version + model | UNTESTED                               |
//! | preciousblock                      | returns nothing |                                        |
//! | pruneblockchain                    | version         |                                        |
//! | savemempool                        | version         |                                        |
//! | scanblocks                         | version + model |                                        |
//! | scantxoutset                       | version + model | API marked as experimental             |
//! | verifychain                        | version         |                                        |
//! | verifytxoutproof                   | version + model |                                        |
//! | waitforblock                       | version + model |                                        |
//! | waitforblockheight                 | version + model |                                        |
//! | waitfornewblock                    | version + model |                                        |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Control == section </summary>
//!
//! | JSON-RPC Method Name               | Returns         | Notes                                  |
//! |:-----------------------------------|:---------------:|:--------------------------------------:|
//! | getmemoryinfo                      | version         |                                        |
//! | getrpcinfo                         | version         |                                        |
//! | help                               | returns string  |                                        |
//! | logging                            | version         |                                        |
//! | stop                               | returns string  |                                        |
//! | uptime                             | returns numeric |                                        |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Mining == section </summary>
//!
//! | JSON-RPC Method Name               | Returns         | Notes                                  |
//! |:-----------------------------------|:---------------:|:--------------------------------------:|
//! | getblocktemplate                   | version + model |                                        |
//! | getmininginfo                      | version + model |                                        |
//! | getnetworkhashps                   | returns boolean |                                        |
//! | getprioritisedtransactions         | version + model |                                        |
//! | prioritisetransaction              | returns boolean |                                        |
//! | submitblock                        | returns nothing |                                        |
//! | submitheader                       | returns nothing |                                        |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Network == section </summary>
//!
//! | JSON-RPC Method Name               | Returns         | Notes                                  |
//! |:-----------------------------------|:---------------:|:--------------------------------------:|
//! | addnode                            | returns nothing |                                        |
//! | clearbanned                        | returns nothing |                                        |
//! | disconnectnode                     | returns nothing |                                        |
//! | getaddednodeinfo                   | version         |                                        |
//! | getaddrmaninfo                     | version         |                                        |
//! | getconnectioncount                 | version         |                                        |
//! | getnettotals                       | version         |                                        |
//! | getnetworkinfo                     | version + model |                                        |
//! | getnodeaddresses                   | version         |                                        |
//! | getpeerinfo                        | version         |                                        |
//! | listbanned                         | version         |                                        |
//! | ping                               | returns nothing |                                        |
//! | setban                             | returns nothing |                                        |
//! | setnetworkactive                   | version         |                                        |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Rawtransactions == section </summary>
//!
//! | JSON-RPC Method Name               | Returns         | Notes                                  |
//! |:-----------------------------------|:---------------:|:--------------------------------------:|
//! | analyzepsbt                        | version + model |                                        |
//! | combinepsbt                        | version + model |                                        |
//! | combinerawtransaction              | version + model |                                        |
//! | converttopsbt                      | version + model |                                        |
//! | createpsbt                         | version + model |                                        |
//! | createrawtransaction               | version + model |                                        |
//! | decodepsbt                         | version + model | Musig not modelled: not in rust-bitcoin|
//! | descriptorprocesspsbt              | returns boolean |                                        |
//! | decoderawtransaction               | version + model |                                        |
//! | decodescript                       | version + model |                                        |
//! | finalizepsbt                       | version + model |                                        |
//! | fundrawtransaction                 | version + model |                                        |
//! | getrawtransaction                  | version + model | Includes additional 'verbose' type     |
//! | joinpsbts                          | version + model |                                        |
//! | sendrawtransaction                 | version + model |                                        |
//! | signrawtransactionwithkey          | version + model |                                        |
//! | submitpackage                      | version + model |                                        |
//! | testmempoolaccept                  | version + model |                                        |
//! | utxoupdatepsbt                     | version + model |                                        |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Signer == section </summary>
//!
//! | JSON-RPC Method Name               | Returns         | Notes                                  |
//! |:-----------------------------------|:---------------:|:--------------------------------------:|
//! | enumeratesigners                   | version         |                                        |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Util == section </summary>
//!
//! | JSON-RPC Method Name               | Returns         | Notes                                  |
//! |:-----------------------------------|:---------------:|:--------------------------------------:|
//! | createmultisig                     | version + model |                                        |
//! | deriveaddresses                    | version + model |                                        |
//! | estimatesmartfee                   | version + model |                                        |
//! | getdescriptorinfo                  | version         |                                        |
//! | getindexinfo                       | version         |                                        |
//! | signmessagewithprivkey             | version + model |                                        |
//! | validateaddress                    | version + model |                                        |
//! | verifymessage                      | version         |                                        |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Wallet == section </summary>
//!
//! | JSON-RPC Method Name               | Returns         | Notes                                  |
//! |:-----------------------------------|:---------------:|:--------------------------------------:|
//! | abandontransaction                 | returns nothing |                                        |
//! | abortrescan                        | version         |                                        |
//! | backupwallet                       | returns nothing |                                        |
//! | bumpfee                            | version + model |                                        |
//! | createwallet                       | version + model |                                        |
//! | createwalletdescriptor             | version         |                                        |
//! | encryptwallet                      | version         |                                        |
//! | getaddressesbylabel                | version + model |                                        |
//! | getaddressinfo                     | version + model |                                        |
//! | getbalance                         | version + model |                                        |
//! | getbalances                        | version + model |                                        |
//! | gethdkeys                          | version + model |                                        |
//! | getnewaddress                      | version + model |                                        |
//! | getrawchangeaddress                | version + model |                                        |
//! | getreceivedbyaddress               | version + model |                                        |
//! | getreceivedbylabel                 | version + model |                                        |
//! | gettransaction                     | version + model |                                        |
//! | getwalletinfo                      | version + model |                                        |
//! | importdescriptors                  | version         |                                        |
//! | importprunedfunds                  | returns nothing |                                        |
//! | keypoolrefill                      | returns nothing |                                        |
//! | listaddressgroupings               | version + model |                                        |
//! | listdescriptors                    | version         |                                        |
//! | listlabels                         | version         |                                        |
//! | listlockunspent                    | version + model |                                        |
//! | migratewallet                      | version         | Untested in v30, unchanged from v29    |
//! | psbtbumpfee                        | version + model |                                        |
//! | listreceivedbyaddress              | version + model |                                        |
//! | listreceivedbylabel                | version + model |                                        |
//! | listsinceblock                     | version + model |                                        |
//! | listtransactions                   | version + model |                                        |
//! | listunspent                        | version + model |                                        |
//! | listwalletdir                      | version         |                                        |
//! | listwallets                        | version + model |                                        |
//! | loadwallet                         | version + model |                                        |
//! | lockunspent                        | version         |                                        |
//! | removeprunedfunds                  | returns nothing |                                        |
//! | rescanblockchain                   | version + model |                                        |
//! | restorewallet                      | version         |                                        |
//! | send                               | version + model |                                        |
//! | sendall                            | version + model |                                        |
//! | sendmany                           | version + model |                                        |
//! | sendtoaddress                      | version + model |                                        |
//! | setlabel                           | returns nothing |                                        |
//! | settxfee                           | version         |                                        |
//! | setwalletflag                      | version         |                                        |
//! | signmessage                        | version + model |                                        |
//! | signrawtransactionwithwallet       | version + model |                                        |
//! | simulaterawtransaction             | version + model |                                        |
//! | unloadwallet                       | returns nothing |                                        |
//! | walletcreatefundedpsbt             | version + model |                                        |
//! | walletdisplayaddress               | version + model | UNTESTED                               |
//! | walletlock                         | returns nothing |                                        |
//! | walletpassphrase                   | returns nothing |                                        |
//! | walletpassphrasechange             | returns nothing |                                        |
//! | walletprocesspsbt                  | version + model |                                        |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Zmq == section </summary>
//!
//! | JSON-RPC Method Name               | Returns         | Notes                                  |
//! |:-----------------------------------|:---------------:|:--------------------------------------:|
//! | getzmqnotifications                | version         |                                        |
//!
//! </details>

mod blockchain {
    // SPDX-License-Identifier: CC0-1.0

    //! The JSON-RPC API for Bitcoin Core `v30` - blockchain.
    //!
    //! Types for methods found under the `== Blockchain ==` section of the API docs.

    mod into {
        // SPDX-License-Identifier: CC0-1.0

        use super::{GetMempoolInfo, GetMempoolInfoError};
        use crate::model;

        impl GetMempoolInfo {
            /// Converts version specific type to a version nonspecific, more strongly typed type.
            pub fn into_model(self) -> Result<model::GetMempoolInfo, GetMempoolInfoError> {
                let size = crate::to_u32(self.size, "size")?;
                let bytes = crate::to_u32(self.bytes, "bytes")?;
                let usage = crate::to_u32(self.usage, "usage")?;
                let max_mempool = crate::to_u32(self.max_mempool, "max_mempool")?;
                let mempool_min_fee = crate::btc_per_kb(self.mempool_min_fee)?;
                let min_relay_tx_fee = crate::btc_per_kb(self.min_relay_tx_fee)?;
                let incremental_relay_fee = crate::btc_per_kb(self.incremental_relay_fee)?;
                let unbroadcast_count = Some(crate::to_u32(self.unbroadcast_count, "unbroadcast_count")?);

                Ok(model::GetMempoolInfo {
                    loaded: Some(self.loaded),
                    size,
                    bytes,
                    usage,
                    total_fee: Some(self.total_fee),
                    max_mempool,
                    mempool_min_fee,
                    min_relay_tx_fee,
                    incremental_relay_fee,
                    unbroadcast_count,
                    full_rbf: Some(self.full_rbf),
                    permit_bare_multisig: Some(self.permit_bare_multisig),
                    max_data_carrier_size: Some(self.max_data_carrier_size),
                })
            }
        }
    }

    use serde::{Deserialize, Serialize};

    pub use super::GetMempoolInfoError;

    /// Result of JSON-RPC method `getmempoolinfo` with verbose set to `true`.
    ///
    /// > getmempoolinfo
    /// >
    /// > Returns details on the active state of the TX memory pool.
    #[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
    #[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
    pub struct GetMempoolInfo {
        /// True if the initial load attempt of the persisted mempool finished.
        pub loaded: bool,
        /// Current tx count.
        pub size: i64,
        /// Sum of all virtual transaction sizes as defined in BIP 141.
        ///
        /// Differs from actual serialized size because witness data is discounted.
        pub bytes: i64,
        /// Total memory usage for the mempool.
        pub usage: i64,
        /// Total fees for the mempool in BTC, ignoring modified fees through prioritisetransaction.
        pub total_fee: f64,
        /// Maximum memory usage for the mempool.
        #[serde(rename = "maxmempool")]
        pub max_mempool: i64,
        /// Minimum fee rate in BTC/kB for a transaction to be accepted.
        ///
        /// This is the maximum of `minrelaytxfee` and the minimum mempool fee.
        #[serde(rename = "mempoolminfee")]
        pub mempool_min_fee: f64,
        /// Current minimum relay fee for transactions.
        #[serde(rename = "minrelaytxfee")]
        pub min_relay_tx_fee: f64,
        /// Minimum fee rate increment for mempool limiting or replacement in BTC/kvB.
        #[serde(rename = "incrementalrelayfee")]
        pub incremental_relay_fee: f64,
        /// Current number of transactions that haven't passed initial broadcast yet.
        #[serde(rename = "unbroadcastcount")]
        pub unbroadcast_count: i64,
        /// True if the mempool accepts RBF without replaceability signaling inspection.
        #[serde(rename = "fullrbf")]
        pub full_rbf: bool,
        /// True if the mempool accepts transactions with bare multisig outputs.
        #[serde(rename = "permitbaremultisig")]
        pub permit_bare_multisig: bool,
        /// Maximum number of bytes that can be used by OP_RETURN outputs in the mempool.
        #[serde(rename = "maxdatacarriersize")]
        pub max_data_carrier_size: u64,
    }
}
mod hidden {
    // SPDX-License-Identifier: CC0-1.0

    //! The JSON-RPC API for Bitcoin Core `v29` - hidden.
    //!
    //! Types for methods that are excluded from the API docs by default.

    mod error {
        // SPDX-License-Identifier: CC0-1.0

        use core::fmt;

        use bitcoin::consensus::encode;
        use bitcoin::hex;

        use crate::error::write_err;

        /// Error when converting a `GetOrphanTxsVerboseOneEntry` type into the model type.
        #[derive(Debug)]
        pub enum GetOrphanTxsVerboseOneEntryError {
            /// Conversion of the transaction `txid` field failed.
            Txid(hex::HexToArrayError),
            /// Conversion of the transaction `wtxid` field failed.
            Wtxid(hex::HexToArrayError),
        }

        impl fmt::Display for GetOrphanTxsVerboseOneEntryError {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match *self {
                    Self::Txid(ref e) => write_err!(f, "conversion of the `txid` field failed"; e),
                    Self::Wtxid(ref e) => write_err!(f, "conversion of the `wtxid` field failed"; e),
                }
            }
        }

        #[cfg(feature = "std")]
        impl std::error::Error for GetOrphanTxsVerboseOneEntryError {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                match *self {
                    Self::Txid(ref e) => Some(e),
                    Self::Wtxid(ref e) => Some(e),
                }
            }
        }

        /// Error when converting a `GetOrphanTxsVerboseTwoEntry` type into the model type.
        #[derive(Debug)]
        pub enum GetOrphanTxsVerboseTwoEntryError {
            /// Conversion of the transaction `txid` field failed.
            Txid(hex::HexToArrayError),
            /// Conversion of the transaction `wtxid` field failed.
            Wtxid(hex::HexToArrayError),
            /// Conversion of hex data to bytes failed.
            Hex(hex::HexToBytesError),
            /// Consensus decoding of `hex` to transaction failed.
            Consensus(encode::Error),
        }

        impl fmt::Display for GetOrphanTxsVerboseTwoEntryError {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match *self {
                    Self::Txid(ref e) => write_err!(f, "conversion of the `txid` field failed"; e),
                    Self::Wtxid(ref e) => write_err!(f, "conversion of the `wtxid` field failed"; e),
                    Self::Hex(ref e) => write_err!(f, "conversion of hex data to bytes failed"; e),
                    Self::Consensus(ref e) =>
                        write_err!(f, "consensus decoding of `hex` to transaction failed"; e),
                }
            }
        }

        #[cfg(feature = "std")]
        impl std::error::Error for GetOrphanTxsVerboseTwoEntryError {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                match *self {
                    Self::Txid(ref e) => Some(e),
                    Self::Wtxid(ref e) => Some(e),
                    Self::Hex(ref e) => Some(e),
                    Self::Consensus(ref e) => Some(e),
                }
            }
        }
    }
    mod into {
        // SPDX-License-Identifier: CC0-1.0

        use bitcoin::consensus::encode;
        use bitcoin::hashes::hex::FromHex;
        use bitcoin::{Transaction, Txid, Wtxid};

        use super::{
            GetOrphanTxs, GetOrphanTxsVerboseOne, GetOrphanTxsVerboseOneEntry,
            GetOrphanTxsVerboseOneEntryError, GetOrphanTxsVerboseTwo, GetOrphanTxsVerboseTwoEntry,
            GetOrphanTxsVerboseTwoEntryError,
        };
        use crate::model;

        impl GetOrphanTxs {
            /// Converts version specific type to a version nonspecific, more strongly typed type.
            pub fn into_model(self) -> model::GetOrphanTxs { model::GetOrphanTxs(self.0) }
        }

        impl GetOrphanTxsVerboseOneEntry {
            /// Converts version specific type to a version nonspecific, more strongly typed type.
            pub fn into_model(
                self,
            ) -> Result<model::GetOrphanTxsVerboseOneEntry, GetOrphanTxsVerboseOneEntryError> {
                use GetOrphanTxsVerboseOneEntryError as E;

                let txid = self.txid.parse::<Txid>().map_err(E::Txid)?;
                let wtxid = self.wtxid.parse::<Wtxid>().map_err(E::Wtxid)?;

                Ok(model::GetOrphanTxsVerboseOneEntry {
                    txid,
                    wtxid,
                    bytes: self.bytes,
                    vsize: self.vsize,
                    weight: self.weight,
                    from: self.from,
                    entry_time: None,
                    expiration_time: None,
                })
            }
        }

        impl GetOrphanTxsVerboseTwoEntry {
            /// Converts version specific type to a version nonspecific, more strongly typed type.
            pub fn into_model(
                self,
            ) -> Result<model::GetOrphanTxsVerboseTwoEntry, GetOrphanTxsVerboseTwoEntryError> {
                use GetOrphanTxsVerboseTwoEntryError as E;

                let txid = self.txid.parse::<Txid>().map_err(E::Txid)?;
                let wtxid = self.wtxid.parse::<Wtxid>().map_err(E::Wtxid)?;
                let v = Vec::from_hex(&self.hex).map_err(E::Hex)?;
                let transaction = encode::deserialize::<Transaction>(&v).map_err(E::Consensus)?;

                Ok(model::GetOrphanTxsVerboseTwoEntry {
                    txid,
                    wtxid,
                    bytes: self.bytes,
                    vsize: self.vsize,
                    weight: self.weight,
                    from: self.from,
                    entry_time: None,
                    expiration_time: None,
                    transaction,
                })
            }
        }

        impl GetOrphanTxsVerboseOne {
            /// Converts version specific type to a version nonspecific, more strongly typed type.
            pub fn into_model(
                self,
            ) -> Result<model::GetOrphanTxsVerboseOne, GetOrphanTxsVerboseOneEntryError> {
                let v = self.0.into_iter().map(|e| e.into_model()).collect::<Result<Vec<_>, _>>()?;

                Ok(model::GetOrphanTxsVerboseOne(v))
            }
        }

        impl GetOrphanTxsVerboseTwo {
            /// Converts version specific type to a version nonspecific, more strongly typed type.
            pub fn into_model(
                self,
            ) -> Result<model::GetOrphanTxsVerboseTwo, GetOrphanTxsVerboseTwoEntryError> {
                let v = self.0.into_iter().map(|e| e.into_model()).collect::<Result<Vec<_>, _>>()?;

                Ok(model::GetOrphanTxsVerboseTwo(v))
            }
        }
    }

    use bitcoin::Txid;
    use serde::{Deserialize, Serialize};

    pub use self::error::{GetOrphanTxsVerboseOneEntryError, GetOrphanTxsVerboseTwoEntryError};

    /// Result of JSON-RPC method `getorphantxs` verbosity 0.
    ///
    /// > getorphantxs ( verbosity )
    /// >
    /// > Shows transactions in the tx orphanage.
    #[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
    #[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
    pub struct GetOrphanTxs(pub Vec<Txid>);

    /// Result of JSON-RPC method `getorphantxs` verbosity 1.
    ///
    /// > getorphantxs ( verbosity )
    /// >
    /// > Shows transactions in the tx orphanage.
    #[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
    #[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
    pub struct GetOrphanTxsVerboseOne(pub Vec<GetOrphanTxsVerboseOneEntry>);

    #[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
    #[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
    pub struct GetOrphanTxsVerboseOneEntry {
        /// The transaction hash in hex
        pub txid: String,
        /// The transaction witness hash in hex
        pub wtxid: String,
        /// The serialized transaction size in bytes
        pub bytes: u64,
        /// The virtual transaction size as defined in BIP 141. This is different from actual serialized size for witness transactions as witness data is discounted.
        pub vsize: u64,
        /// The transaction weight as defined in BIP 141.
        pub weight: u64,
        /// The entry time into the orphanage expressed in UNIX epoch time
        /// Only present in v29.
        #[serde(rename = "entry")]
        pub entry_time: Option<u32>,
        /// The orphan expiration time expressed in UNIX epoch time
        /// Only present in v29.
        #[serde(rename = "expiration")]
        pub expiration_time: Option<u32>,
        /// List of peer ids that we store this transaction for.
        pub from: Vec<u64>,
    }

    /// Result of JSON-RPC method `getorphantxs` verbosity 2.
    ///
    /// > getorphantxs ( verbosity )
    /// >
    /// > Shows transactions in the tx orphanage.
    #[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
    #[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
    pub struct GetOrphanTxsVerboseTwo(pub Vec<GetOrphanTxsVerboseTwoEntry>);

    #[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
    #[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
    pub struct GetOrphanTxsVerboseTwoEntry {
        /// The transaction hash in hex
        pub txid: String,
        /// The transaction witness hash in hex
        pub wtxid: String,
        /// The serialized transaction size in bytes
        pub bytes: u64,
        /// The virtual transaction size as defined in BIP 141. This is different from actual serialized size for witness transactions as witness data is discounted.
        pub vsize: u64,
        /// The transaction weight as defined in BIP 141.
        pub weight: u64,
        /// List of peer ids that we store this transaction for.
        pub from: Vec<u64>,
        /// The entry time into the orphanage expressed in UNIX epoch time
        /// Only present in v29.
        pub entry_time: Option<u32>,
        /// The orphan expiration time expressed in UNIX epoch time
        /// Only present in v29.
        pub expiration_time: Option<u32>,
        /// The serialized, hex-encoded transaction data.
        pub hex: String,
    }
}
mod mining {
    // SPDX-License-Identifier: CC0-1.0

    //! The JSON-RPC API for Bitcoin Core `v30` - mining.
    //!
    //! Types for methods found under the `== Mining ==` section of the API docs.

    mod error {
        // SPDX-License-Identifier: CC0-1.0

        use core::fmt;

        use bitcoin::amount::ParseAmountError;
        use bitcoin::error::UnprefixedHexError;
        use bitcoin::hex::HexToBytesError;
        use bitcoin::{consensus, hex};

        use super::NextBlockInfoError;
        use crate::error::write_err;
        use crate::NumericError;

        /// Error when converting a `GetBlockTemplate` type into the model type.
        #[derive(Debug)]
        pub enum GetBlockTemplateError {
            /// Conversion of numeric type to expected type failed.
            Numeric(NumericError),
            /// Conversion of the `previous_block_hash` field failed.
            PreviousBlockHash(hex::HexToArrayError),
            /// Conversion of the `transactions` field failed.
            Transactions(BlockTemplateTransactionError),
            /// Conversion of the `target` field failed.
            Target(HexToBytesError),
            /// Conversion of the `bits` field failed.
            Bits(UnprefixedHexError),
        }

        impl fmt::Display for GetBlockTemplateError {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match *self {
                    Self::Numeric(ref e) => write_err!(f, "numeric"; e),
                    Self::PreviousBlockHash(ref e) =>
                        write_err!(f, "conversion of the `previous_block_hash` field failed"; e),
                    Self::Transactions(ref e) =>
                        write_err!(f, "conversion of the `transactions` field failed"; e),
                    Self::Target(ref e) => write_err!(f, "conversion of the `target` field failed"; e),
                    Self::Bits(ref e) => write_err!(f, "conversion of the `bits` field failed"; e),
                }
            }
        }

        #[cfg(feature = "std")]
        impl std::error::Error for GetBlockTemplateError {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                match *self {
                    Self::Numeric(ref e) => Some(e),
                    Self::PreviousBlockHash(ref e) => Some(e),
                    Self::Transactions(ref e) => Some(e),
                    Self::Target(ref e) => Some(e),
                    Self::Bits(ref e) => Some(e),
                }
            }
        }

        impl From<NumericError> for GetBlockTemplateError {
            fn from(e: NumericError) -> Self { Self::Numeric(e) }
        }

        /// Error when converting a `BlockTemplateTransaction` type into the model type.
        #[derive(Debug)]
        pub enum BlockTemplateTransactionError {
            /// Conversion of numeric type to expected type failed.
            Numeric(NumericError),
            /// Conversion of the `data` field failed.
            Data(consensus::encode::FromHexError),
            /// Conversion of the `txid` field failed.
            Txid(hex::HexToArrayError),
            /// Conversion of the `hash` field failed.
            Hash(hex::HexToArrayError),
            /// Conversion of the `fee` field failed.
            Fee(ParseAmountError),
        }

        impl fmt::Display for BlockTemplateTransactionError {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match *self {
                    Self::Numeric(ref e) => write_err!(f, "numeric"; e),
                    Self::Data(ref e) => write_err!(f, "conversion of the `data` field failed"; e),
                    Self::Txid(ref e) => write_err!(f, "conversion of the `txid` field failed"; e),
                    Self::Hash(ref e) => write_err!(f, "conversion of the `hash` field failed"; e),
                    Self::Fee(ref e) => write_err!(f, "conversion of the `fee` field failed"; e),
                }
            }
        }

        #[cfg(feature = "std")]
        impl std::error::Error for BlockTemplateTransactionError {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                match *self {
                    Self::Numeric(ref e) => Some(e),
                    Self::Data(ref e) => Some(e),
                    Self::Txid(ref e) => Some(e),
                    Self::Hash(ref e) => Some(e),
                    Self::Fee(ref e) => Some(e),
                }
            }
        }

        impl From<NumericError> for BlockTemplateTransactionError {
            fn from(e: NumericError) -> Self { Self::Numeric(e) }
        }

        /// Error when converting a `GetMiningInfo` type into the model type.
        #[derive(Debug)]
        pub enum GetMiningInfoError {
            /// Conversion of the `bits` field failed.
            Bits(UnprefixedHexError),
            /// Conversion of the `target` field failed.
            Target(UnprefixedHexError),
            /// Conversion of the `block_min_tx_fee` field failed.
            BlockMinTxFee(ParseAmountError),
            /// Conversion of one of the items in field `next` failed.
            Next(NextBlockInfoError),
        }

        impl fmt::Display for GetMiningInfoError {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match *self {
                    Self::Bits(ref e) => write_err!(f, "conversion of the `bits` field failed"; e),
                    Self::Target(ref e) => write_err!(f, "conversion of the `target` field failed"; e),
                    Self::BlockMinTxFee(ref e) =>
                        write_err!(f, "conversion of the `block_min_tx_fee` field failed"; e),
                    Self::Next(ref e) =>
                        write_err!(f, "conversion of one of the items in field `next` failed"; e),
                }
            }
        }

        #[cfg(feature = "std")]
        impl std::error::Error for GetMiningInfoError {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                match *self {
                    Self::Bits(ref e) => Some(e),
                    Self::Target(ref e) => Some(e),
                    Self::BlockMinTxFee(ref e) => Some(e),
                    Self::Next(ref e) => Some(e),
                }
            }
        }
    }
    mod into {
        // SPDX-License-Identifier: CC0-1.0

        use bitcoin::{
            block, consensus, BlockHash, CompactTarget, SignedAmount, Target, Transaction, Txid, Weight,
            Wtxid,
        };

        use super::{
            BlockTemplateTransaction, BlockTemplateTransactionError, GetBlockTemplate,
            GetBlockTemplateError, GetMiningInfo, GetMiningInfoError,
        };
        use crate::model;

        impl GetBlockTemplate {
            /// Converts version specific type to a version nonspecific, more strongly typed type.
            pub fn into_model(self) -> Result<model::GetBlockTemplate, GetBlockTemplateError> {
                use GetBlockTemplateError as E;

                let version = block::Version::from_consensus(self.version);
                let version_bits_required =
                    crate::to_u32(self.version_bits_required, "version_bits_required")?;
                let previous_block_hash =
                    self.previous_block_hash.parse::<BlockHash>().map_err(E::PreviousBlockHash)?;
                let transactions = self
                    .transactions
                    .into_iter()
                    .map(|t| t.into_model())
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(E::Transactions)?;
                let coinbase_value = SignedAmount::from_sat(self.coinbase_value);
                let target = bitcoin::hex::FromHex::from_hex(&self.target).map_err(E::Target)?;
                let sigop_limit = crate::to_u32(self.sigop_limit, "sigop_limit")?;
                let weight_limit = self
                    .weight_limit
                    .map(|w| crate::to_u32(w, "weight_limit"))
                    .transpose()?;
                let size_limit = crate::to_u32(self.size_limit, "size_limit")?;
                let bits = CompactTarget::from_unprefixed_hex(&self.bits).map_err(E::Bits)?;
                let height = crate::to_u32(self.height, "height")?;

                Ok(model::GetBlockTemplate {
                    version,
                    rules: self.rules,
                    version_bits_available: self.version_bits_available,
                    capabilities: self.capabilities,
                    version_bits_required,
                    previous_block_hash,
                    transactions,
                    coinbase_aux: self.coinbase_aux,
                    coinbase_value,
                    long_poll_id: self.long_poll_id,
                    target,
                    min_time: self.min_time,
                    mutable: self.mutable,
                    nonce_range: self.nonce_range,
                    sigop_limit,
                    weight_limit,
                    size_limit,
                    current_time: self.current_time,
                    bits,
                    height,
                    signet_challenge: self.signet_challenge,
                    default_witness_commitment: self.default_witness_commitment,
                })
            }
        }

        impl BlockTemplateTransaction {
            /// Converts version specific type to a version nonspecific, more strongly typed type.
            pub fn into_model(
                self,
            ) -> Result<model::BlockTemplateTransaction, BlockTemplateTransactionError> {
                use BlockTemplateTransactionError as E;

                let data =
                    consensus::encode::deserialize_hex::<Transaction>(&self.data).map_err(E::Data)?;
                let txid = self.txid.parse::<Txid>().map_err(E::Txid)?;
                let wtxid = self.hash.parse::<Wtxid>().map_err(E::Hash)?;
                let depends = self
                    .depends
                    .iter()
                    .map(|x| crate::to_u32(*x, "depend"))
                    .collect::<Result<Vec<_>, _>>()?;
                let fee = SignedAmount::from_sat(self.fee);
                let sigops = crate::to_u32(self.sigops, "sigops")?;
                let weight = Weight::from_wu(self.weight);

                Ok(model::BlockTemplateTransaction { data, txid, wtxid, depends, fee, sigops, weight })
            }
        }

        impl GetMiningInfo {
            /// Converts version specific type to a version nonspecific, more strongly typed type.
            pub fn into_model(self) -> Result<model::GetMiningInfo, GetMiningInfoError> {
                use GetMiningInfoError as E;

                let current_block_weight = self.current_block_weight.map(Weight::from_wu);
                let bits = CompactTarget::from_unprefixed_hex(&self.bits).map_err(E::Bits)?;
                let target = Target::from_unprefixed_hex(self.target.as_ref()).map_err(E::Target)?;
                let block_min_tx_fee =
                    crate::btc_per_kb(self.block_min_tx_fee).map_err(E::BlockMinTxFee)?;

                let next = self.next.into_model().map_err(E::Next)?;

                Ok(model::GetMiningInfo {
                    blocks: self.blocks,
                    current_block_weight,
                    current_block_tx: self.current_block_tx,
                    bits: Some(bits),
                    difficulty: self.difficulty,
                    target: Some(target),
                    network_hash_ps: self.network_hash_ps,
                    pooled_tx: self.pooled_tx,
                    block_min_tx_fee,
                    chain: self.chain,
                    signet_challenge: self.signet_challenge,
                    next: Some(next),
                    warnings: self.warnings,
                })
            }
        }
    }

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
}
mod raw_transactions {
    // SPDX-License-Identifier: CC0-1.0

    //! The JSON-RPC API for Bitcoin Core `v30` - raw transactions.
    //!
    //! Types for methods found under the `== Rawtransactions ==` section of the API docs.

    mod error {
        // SPDX-License-Identifier: CC0-1.0

        use core::fmt;

        use bitcoin::amount::ParseAmountError;
        use bitcoin::taproot::{IncompleteBuilderError, TaprootBuilderError, TaprootError};
        use bitcoin::{bip32, hex, secp256k1, sighash};

        use super::{Bip32DerivError, PartialSignatureError, RawTransactionError, WitnessUtxoError};
        use crate::error::write_err;

        /// Error when converting a `DecodePsbt` type into the model type.
        #[derive(Debug)]
        pub enum DecodePsbtError {
            /// Conversion of the `tx` field failed.
            Tx(RawTransactionError),
            /// Conversion of the `global_xpubs` field failed.
            GlobalXpubs(GlobalXpubError),
            /// Conversion of the `proprietary` field failed.
            Proprietary(hex::HexToBytesError),
            /// Conversion of one the map items in the `unknown` field failed.
            Unknown(hex::HexToBytesError),
            /// Conversion of one of the PSBT inputs failed.
            Inputs(PsbtInputError),
            /// Conversion of one of the PSBT outputs failed.
            Outputs(PsbtOutputError),
            /// Conversion of the `fee` field failed.
            Fee(ParseAmountError),
        }

        impl fmt::Display for DecodePsbtError {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match *self {
                    Self::Tx(ref e) => write_err!(f, "conversion of raw transaction data field failed"; e),
                    Self::GlobalXpubs(ref e) =>
                        write_err!(f, "conversion of one the map items in the `global_xbubs` field failed"; e),
                    Self::Proprietary(ref e) =>
                        write_err!(f, "conversion of one the map items in the `proprietray` field failed"; e),
                    Self::Unknown(ref e) =>
                        write_err!(f, "conversion of one the map items in the `unknown` field failed"; e),
                    Self::Inputs(ref e) => write_err!(f, "conversion of one of the PSBT inputs failed"; e),
                    Self::Outputs(ref e) =>
                        write_err!(f, "conversion of one of the PSBT outputs failed"; e),
                    Self::Fee(ref e) => write_err!(f, "conversion of the `fee` field failed"; e),
                }
            }
        }

        #[cfg(feature = "std")]
        impl std::error::Error for DecodePsbtError {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                match *self {
                    Self::Tx(ref e) => Some(e),
                    Self::GlobalXpubs(ref e) => Some(e),
                    Self::Proprietary(ref e) => Some(e),
                    Self::Unknown(ref e) => Some(e),
                    Self::Inputs(ref e) => Some(e),
                    Self::Outputs(ref e) => Some(e),
                    Self::Fee(ref e) => Some(e),
                }
            }
        }

        /// Error when converting one of the global xpubs failed.
        #[derive(Debug)]
        pub enum GlobalXpubError {
            /// Conversion of the `xpub` field failed.
            Xpub(bip32::Error),
            /// Conversion of the `master_fingerprint` field failed.
            MasterFingerprint(hex::HexToArrayError),
            /// Conversion of the `path` field failed.
            Path(bip32::Error),
        }

        impl fmt::Display for GlobalXpubError {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match *self {
                    Self::Xpub(ref e) => write_err!(f, "conversion of the xpub failed"; e),
                    Self::MasterFingerprint(ref e) =>
                        write_err!(f, "conversion of the `master_fingerprint` field failed"; e),
                    Self::Path(ref e) => write_err!(f, "conversion of the `path` field failed"; e),
                }
            }
        }

        #[cfg(feature = "std")]
        impl std::error::Error for GlobalXpubError {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                match *self {
                    Self::Xpub(ref e) => Some(e),
                    Self::MasterFingerprint(ref e) => Some(e),
                    Self::Path(ref e) => Some(e),
                }
            }
        }

        /// Error when converting one of the `DecodePsbt` inputs failed.
        #[derive(Debug)]
        pub enum PsbtInputError {
            /// Conversion of the `non_witness_utxo` field failed.
            NonWitnessUtxo(RawTransactionError),
            /// Conversion of the `witness_utxo` field failed.
            WitnessUtxo(WitnessUtxoError),
            /// Conversion of the `partial_signatures` field failed.
            PartialSignatures(PartialSignatureError),
            /// Conversion of the `sighash` field failed.
            Sighash(sighash::SighashTypeParseError),
            /// Conversion of the `redeem_script` field failed.
            RedeemScript(hex::HexToBytesError),
            /// Conversion of the `witness_script` field failed.
            WitnessScript(hex::HexToBytesError),
            /// Conversion of the `bip32_derivs` field failed.
            Bip32Derivs(Bip32DerivError),
            /// Conversion of the `final_script_sig` field failed.
            FinalScriptSig(hex::HexToBytesError),
            /// Conversion of the `final_script_witness` field failed.
            FinalScriptWitness(hex::HexToBytesError),
            /// Conversion of the `ripemd160` hash failed.
            Ripemd160(hex::HexToArrayError),
            /// Conversion of the `ripemd160` preimage failed.
            Ripemd160Preimage(hex::HexToBytesError),
            /// Conversion of the `sha256` hash failed.
            Sha256(hex::HexToArrayError),
            /// Conversion of the `sha256` preimage failed.
            Sha256Preimage(hex::HexToBytesError),
            /// Conversion of the `hash160` hash failed.
            Hash160(hex::HexToArrayError),
            /// Conversion of the `hash160` preimage failed.
            Hash160Preimage(hex::HexToBytesError),
            /// Conversion of the `hash256` hash failed.
            Hash256(hex::HexToArrayError),
            /// Conversion of the `hash256` preimage failed.
            Hash256Preimage(hex::HexToBytesError),
            /// Conversion of the `taproot_key_path_sig` field failed.
            TaprootKeyPathSig(super::taproot::Error),
            /// Conversion of the `taproot_script_path_sigs` field failed.
            TaprootScriptPathSigs(TaprootScriptPathSigError),
            /// Conversion of the `taproot_scripts` field failed.
            TaprootScripts(TaprootScriptError),
            /// Conversion of the `taproot_bip32_derives` field failed.
            TaprootBip32Derivs(TaprootBip32DerivsError),
            /// Conversion of the `taproot_internal_key` field failed.
            TaprootInternalKey(secp256k1::Error),
            /// Conversion of the `taproot_merkle_root` field failed.
            TaprootMerkleRoot(hex::HexToArrayError),
            /// Conversion of the `proprietary` field failed.
            Proprietary(hex::HexToBytesError),
            /// Conversion of the `unknown` field failed.
            Unknown(hex::HexToBytesError),
        }

        impl fmt::Display for PsbtInputError {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match *self {
                    Self::NonWitnessUtxo(ref e) =>
                        write_err!(f, "conversion of the `non_witness_utxo` field failed"; e),
                    Self::WitnessUtxo(ref e) =>
                        write_err!(f, "conversion of the `witness_utxo` field failed"; e),
                    Self::PartialSignatures(ref e) =>
                        write_err!(f, "conversion of the `partial_signatures` field failed"; e),
                    Self::Sighash(ref e) => write_err!(f, "conversion of the `sighash` field failed"; e),
                    Self::RedeemScript(ref e) =>
                        write_err!(f, "conversion of the `redeem_script` field failed"; e),
                    Self::WitnessScript(ref e) =>
                        write_err!(f, "conversion of the `witness_script` field failed"; e),
                    Self::Bip32Derivs(ref e) =>
                        write_err!(f, "conversion of the `bip32_derivs` field failed"; e),
                    Self::FinalScriptSig(ref e) =>
                        write_err!(f, "conversion of the `final_script_sig` field failed"; e),
                    Self::FinalScriptWitness(ref e) =>
                        write_err!(f, "conversion of the `final_script_witness` field failed"; e),
                    Self::Ripemd160(ref e) => write_err!(f, "conversion of the `ripemd160` hash failed"; e),
                    Self::Ripemd160Preimage(ref e) =>
                        write_err!(f, "conversion of the `ripemd160` preimage failed"; e),
                    Self::Sha256(ref e) => write_err!(f, "conversion of the `sha256` hash failed"; e),
                    Self::Sha256Preimage(ref e) =>
                        write_err!(f, "conversion of the `sha256` preimage failed"; e),
                    Self::Hash160(ref e) => write_err!(f, "conversion of the `hash160` hash failed"; e),
                    Self::Hash160Preimage(ref e) =>
                        write_err!(f, "conversion of the `hash160` preimage failed"; e),
                    Self::Hash256(ref e) => write_err!(f, "conversion of the `hash256` hash failed"; e),
                    Self::Hash256Preimage(ref e) =>
                        write_err!(f, "conversion of the `hash256` preimage failed"; e),
                    Self::TaprootKeyPathSig(ref e) =>
                        write_err!(f, "conversion of the `taproot_key_path_sig` field failed"; e),
                    Self::TaprootScriptPathSigs(ref e) =>
                        write_err!(f, "conversion of the `taproot_script_path_sigs` field failed"; e),
                    Self::TaprootScripts(ref e) =>
                        write_err!(f, "conversion of the `taproot_scripts` field failed"; e),
                    Self::TaprootBip32Derivs(ref e) =>
                        write_err!(f, "conversion of the `taproot_bip32_derivs` field failed"; e),
                    Self::TaprootInternalKey(ref e) =>
                        write_err!(f, "conversion of the `taproot_internal_key` field failed"; e),
                    Self::TaprootMerkleRoot(ref e) =>
                        write_err!(f, "conversion of the `taproot_merkle_root` field failed"; e),
                    Self::Proprietary(ref e) =>
                        write_err!(f, "conversion of one the map items in the `proprietray` field failed"; e),
                    Self::Unknown(ref e) => write_err!(f, "conversion of the `unknown` field failed"; e),
                }
            }
        }

        #[cfg(feature = "std")]
        impl std::error::Error for PsbtInputError {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                match *self {
                    Self::NonWitnessUtxo(ref e) => Some(e),
                    Self::WitnessUtxo(ref e) => Some(e),
                    Self::PartialSignatures(ref e) => Some(e),
                    Self::Sighash(ref e) => Some(e),
                    Self::RedeemScript(ref e) => Some(e),
                    Self::WitnessScript(ref e) => Some(e),
                    Self::Bip32Derivs(ref e) => Some(e),
                    Self::FinalScriptSig(ref e) => Some(e),
                    Self::FinalScriptWitness(ref e) => Some(e),
                    Self::Ripemd160(ref e) => Some(e),
                    Self::Ripemd160Preimage(ref e) => Some(e),
                    Self::Sha256(ref e) => Some(e),
                    Self::Sha256Preimage(ref e) => Some(e),
                    Self::Hash160(ref e) => Some(e),
                    Self::Hash160Preimage(ref e) => Some(e),
                    Self::Hash256(ref e) => Some(e),
                    Self::Hash256Preimage(ref e) => Some(e),
                    Self::TaprootKeyPathSig(ref e) => Some(e),
                    Self::TaprootScriptPathSigs(ref e) => Some(e),
                    Self::TaprootScripts(ref e) => Some(e),
                    Self::TaprootBip32Derivs(ref e) => Some(e),
                    Self::TaprootInternalKey(ref e) => Some(e),
                    Self::TaprootMerkleRoot(ref e) => Some(e),
                    Self::Proprietary(ref e) => Some(e),
                    Self::Unknown(ref e) => Some(e),
                }
            }
        }

        /// Error when converting one of the `DecodePsbt` outputs failed.
        #[derive(Debug)]
        pub enum PsbtOutputError {
            /// Conversion of the `redeem_script` field failed.
            RedeemScript(hex::HexToBytesError),
            /// Conversion of the `witness_script` field failed.
            WitnessScript(hex::HexToBytesError),
            /// Conversion of the `bip32_derivs` field failed.
            Bip32Derivs(Bip32DerivError),
            /// Conversion of the `taproot_internal_key` field failed.
            TaprootInternalKey(secp256k1::Error),
            /// Conversion of the `taproot_tree` field failed.
            TaprootTree(TaprootLeafError),
            /// Conversion of the `taproot_bip32_derives` field failed.
            TaprootBip32Derivs(TaprootBip32DerivsError),
            /// Conversion of the `proprietary` field failed.
            Proprietary(hex::HexToBytesError),
            /// Conversion of the `unknown` field failed.
            Unknown(hex::HexToBytesError),
        }

        impl fmt::Display for PsbtOutputError {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match *self {
                    Self::RedeemScript(ref e) =>
                        write_err!(f, "conversion of the `redeem_script` field failed"; e),
                    Self::WitnessScript(ref e) =>
                        write_err!(f, "conversion of the `witness_script` field failed"; e),
                    Self::Bip32Derivs(ref e) =>
                        write_err!(f, "conversion of the `bip32_derivs` field failed"; e),
                    Self::TaprootInternalKey(ref e) =>
                        write_err!(f, "conversion of the `taproot_internal_key` field failed"; e),
                    Self::TaprootTree(ref e) =>
                        write_err!(f, "conversion of the `taproot_tree` field failed"; e),
                    Self::TaprootBip32Derivs(ref e) =>
                        write_err!(f, "conversion of the `taproot_bip32_derivs` field failed"; e),
                    Self::Proprietary(ref e) =>
                        write_err!(f, "conversion of one the map items in the `proprietray` field failed"; e),
                    Self::Unknown(ref e) => write_err!(f, "conversion of the `unknown` field failed"; e),
                }
            }
        }

        #[cfg(feature = "std")]
        impl std::error::Error for PsbtOutputError {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                match *self {
                    Self::RedeemScript(ref e) => Some(e),
                    Self::WitnessScript(ref e) => Some(e),
                    Self::Bip32Derivs(ref e) => Some(e),
                    Self::TaprootInternalKey(ref e) => Some(e),
                    Self::TaprootTree(ref e) => Some(e),
                    Self::TaprootBip32Derivs(ref e) => Some(e),
                    Self::Proprietary(ref e) => Some(e),
                    Self::Unknown(ref e) => Some(e),
                }
            }
        }

        /// Error when converting a taproot script path sig.
        #[derive(Debug)]
        pub enum TaprootScriptPathSigError {
            /// Conversion of the `pubkey` field failed.
            Pubkey(secp256k1::Error),
            /// Conversion of the `leaf_hash` field failed.
            LeafHash(hex::HexToArrayError),
            /// Conversion of the `sig` field failed.
            Sig(super::taproot::Error),
        }

        impl fmt::Display for TaprootScriptPathSigError {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match *self {
                    Self::Pubkey(ref e) => write_err!(f, "conversion of the `pubkey` field failed"; e),
                    Self::LeafHash(ref e) => write_err!(f, "conversion of the `leaf_hash` field failed"; e),
                    Self::Sig(ref e) => write_err!(f, "conversion of the `sig` field failed"; e),
                }
            }
        }

        #[cfg(feature = "std")]
        impl std::error::Error for TaprootScriptPathSigError {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                match *self {
                    Self::Pubkey(ref e) => Some(e),
                    Self::LeafHash(ref e) => Some(e),
                    Self::Sig(ref e) => Some(e),
                }
            }
        }

        /// Error when converting a taproot script.
        #[derive(Debug)]
        pub enum TaprootScriptError {
            /// Conversion of the `script` field failed.
            Script(hex::HexToBytesError),
            /// Conversion of the `leaf_ver` field failed.
            LeafVer(TaprootError),
            /// Conversion of the `control_blocks` field failed.
            ControlBlocks(ControlBlocksError),
        }

        impl fmt::Display for TaprootScriptError {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match *self {
                    Self::Script(ref e) => write_err!(f, "conversion of the `script` field failed"; e),
                    Self::LeafVer(ref e) => write_err!(f, "conversion of the `leaf_ver` field failed"; e),
                    Self::ControlBlocks(ref e) =>
                        write_err!(f, "conversion of the `control_blocks` field failed"; e),
                }
            }
        }

        #[cfg(feature = "std")]
        impl std::error::Error for TaprootScriptError {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                match *self {
                    Self::Script(ref e) => Some(e),
                    Self::LeafVer(ref e) => Some(e),
                    Self::ControlBlocks(ref e) => Some(e),
                }
            }
        }

        /// Error when converting the control blocks vector.
        #[derive(Debug)]
        pub enum ControlBlocksError {
            /// No control block returned by Core for this script.
            Missing,
            /// Multiple control blocks returned by Core for this script.
            Multiple(usize),
            /// Failed to parse control block hex string.
            Parse(hex::HexToBytesError),
            /// Failed to decode parsed bytes.
            Decode(TaprootError),
        }

        impl fmt::Display for ControlBlocksError {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match *self {
                    Self::Missing => write!(f, "no control block returned by Core for this script"),
                    Self::Multiple(n) =>
                        write!(f, "multiple control blocks returned by Core for this script: {}", n),
                    Self::Parse(ref e) => write_err!(f, "failed to parse control block hex"; e),
                    Self::Decode(ref e) => write_err!(f, "failed to decode control block from bytes"; e),
                }
            }
        }

        #[cfg(feature = "std")]
        impl std::error::Error for ControlBlocksError {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                match *self {
                    Self::Missing => None,
                    Self::Multiple(_) => None,
                    Self::Parse(ref e) => Some(e),
                    Self::Decode(ref e) => Some(e),
                }
            }
        }

        /// Error when converting a taproot BIP-32 derivation.
        #[derive(Debug)]
        pub enum TaprootBip32DerivsError {
            /// Conversion of the `pubkey` field failed.
            Pubkey(secp256k1::Error),
            /// Conversion of the `master_fingerprint` field failed.
            MasterFingerprint(hex::HexToArrayError),
            /// Conversion of the `path` field failed.
            Path(bip32::Error),
            /// Conversion of one of the leaf hashes failed.
            LeafHashes(hex::HexToArrayError),
        }

        impl fmt::Display for TaprootBip32DerivsError {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match *self {
                    Self::Pubkey(ref e) => write_err!(f, "conversion of the `pubkey` field failed"; e),
                    Self::MasterFingerprint(ref e) =>
                        write_err!(f, "conversion of the `master_fingerprint` field failed"; e),
                    Self::Path(ref e) => write_err!(f, "conversion of the `path` field failed"; e),
                    Self::LeafHashes(ref e) =>
                        write_err!(f, "conversion of the `leaf_hashes` field failed"; e),
                }
            }
        }

        #[cfg(feature = "std")]
        impl std::error::Error for TaprootBip32DerivsError {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                match *self {
                    Self::Pubkey(ref e) => Some(e),
                    Self::MasterFingerprint(ref e) => Some(e),
                    Self::Path(ref e) => Some(e),
                    Self::LeafHashes(ref e) => Some(e),
                }
            }
        }

        /// Error when converting a taproot script.
        #[derive(Debug)]
        pub enum TaprootLeafError {
            /// Conversion of the `leaf_ver` field failed.
            LeafVer(TaprootError),
            /// Conversion of the `script` field failed.
            Script(hex::HexToBytesError),
            /// Failed to add leaf to builder.
            TaprootBuilder(TaprootBuilderError),
            /// Failed to convert builder into a tap tree.
            IncompleteBuilder(IncompleteBuilderError),
        }

        impl fmt::Display for TaprootLeafError {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match *self {
                    Self::LeafVer(ref e) => write_err!(f, "conversion of the `leaf_ver` field failed"; e),
                    Self::Script(ref e) => write_err!(f, "conversion of the `script` field failed"; e),
                    Self::TaprootBuilder(ref e) => write_err!(f, "failed to add leaf to builder"; e),
                    Self::IncompleteBuilder(ref e) =>
                        write_err!(f, "failed to convert builder into a tap tree"; e),
                }
            }
        }

        #[cfg(feature = "std")]
        impl std::error::Error for TaprootLeafError {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                match *self {
                    Self::Script(ref e) => Some(e),
                    Self::LeafVer(ref e) => Some(e),
                    Self::TaprootBuilder(ref e) => Some(e),
                    Self::IncompleteBuilder(ref e) => Some(e),
                }
            }
        }
    }
    mod into {
        // SPDX-License-Identifier: CC0-1.0

        use std::collections::BTreeMap;

        use bitcoin::bip32::{DerivationPath, Fingerprint, KeySource, Xpub};
        use bitcoin::hashes::{hash160, ripemd160, sha256, sha256d};
        use bitcoin::hex::{self, FromHex as _};
        use bitcoin::psbt::{self, raw, PsbtSighashType};
        use bitcoin::taproot::{
            ControlBlock, LeafVersion, TapLeafHash, TapNodeHash, TapTree, TaprootBuilder,
        };
        use bitcoin::{Amount, ScriptBuf, XOnlyPublicKey};

        use super::{
            taproot, ControlBlocksError, DecodePsbt, DecodePsbtError, GlobalXpub, GlobalXpubError,
            Proprietary, PsbtInput, PsbtInputError, PsbtOutput, PsbtOutputError, TaprootBip32Deriv,
            TaprootBip32DerivsError, TaprootLeaf, TaprootLeafError, TaprootScript, TaprootScriptError,
            TaprootScriptPathSig, TaprootScriptPathSigError,
        };
        use crate::model;

        impl DecodePsbt {
            /// Converts version specific type to a version nonspecific, more strongly typed type.
            pub fn into_model(self) -> Result<model::DecodePsbt, DecodePsbtError> {
                use DecodePsbtError as E;

                let unsigned_tx = self.tx.to_transaction().map_err(E::Tx)?;
                let version = self.psbt_version;

                let mut xpubs = BTreeMap::default();
                for g in self.global_xpubs {
                    let (xpub, key_source) = g.to_key_value_pair().map_err(E::GlobalXpubs)?;
                    xpubs.insert(xpub, key_source);
                }

                let proprietary = match self.proprietary {
                    Some(props) => {
                        let mut map = BTreeMap::default();
                        for prop in props {
                            let (key, vec) = prop.to_key_value_pair().map_err(E::Proprietary)?;
                            map.insert(key, vec);
                        }
                        map
                    }
                    None => BTreeMap::default(),
                };

                let unknown = match self.unknown {
                    Some(map) => crate::psbt::into_unknown(map).map_err(E::Unknown)?,
                    None => BTreeMap::default(),
                };

                let inputs = self
                    .inputs
                    .into_iter()
                    .map(|input| input.into_input())
                    .collect::<Result<_, _>>()
                    .map_err(E::Inputs)?;
                let outputs = self
                    .outputs
                    .into_iter()
                    .map(|output| output.into_output())
                    .collect::<Result<_, _>>()
                    .map_err(E::Outputs)?;

                let psbt = bitcoin::Psbt {
                    unsigned_tx,
                    version,
                    xpub: xpubs,
                    proprietary,
                    unknown,
                    inputs,
                    outputs,
                };
                let fee = self.fee.map(Amount::from_btc).transpose().map_err(E::Fee)?;

                Ok(model::DecodePsbt { psbt, fee })
            }
        }

        impl GlobalXpub {
            /// Converts this global xpub list element to a map entry suitable to use in `bitcoin::Psbt`.
            pub fn to_key_value_pair(&self) -> Result<(Xpub, KeySource), GlobalXpubError> {
                use GlobalXpubError as E;

                let xpub = self.xpub.parse::<Xpub>().map_err(E::Xpub)?;
                let fp = Fingerprint::from_hex(&self.master_fingerprint).map_err(E::MasterFingerprint)?;
                let path = self.path.parse::<DerivationPath>().map_err(E::Path)?;
                Ok((xpub, (fp, path)))
            }
        }

        impl Proprietary {
            /// Converts this proprietary list element to a map entry suitable to use in `bitcoin::Psbt`.
            pub fn to_key_value_pair(
                &self,
            ) -> Result<(raw::ProprietaryKey, Vec<u8>), hex::HexToBytesError> {
                // FIXME: Remove cast once rust-bitcoin 0.33 is out.
                //
                // This is changed to a u64 in the upcoming rust-bitcoin
                // release, until then just ignore any additional bits.
                let subtype = self.subtype as u8;

                let prefix = Vec::from_hex(&self.identifier)?;
                let key = Vec::from_hex(&self.key)?;
                let value = Vec::from_hex(&self.value)?;

                Ok((raw::ProprietaryKey { prefix, subtype, key }, value))
            }
        }

        impl PsbtInput {
            /// Converts this PSBT data into a PSBT input.
            pub fn into_input(self) -> Result<psbt::Input, PsbtInputError> {
                use PsbtInputError as E;

                let non_witness_utxo = self
                    .non_witness_utxo
                    .map(|raw| raw.to_transaction())
                    .transpose()
                    .map_err(E::NonWitnessUtxo)?;
                let witness_utxo =
                    self.witness_utxo.map(|utxo| utxo.to_tx_out()).transpose().map_err(E::WitnessUtxo)?;
                let partial_sigs = match self.partial_signatures {
                    Some(map) => crate::psbt::into_partial_signatures(map).map_err(E::PartialSignatures)?,
                    None => BTreeMap::default(),
                };
                let sighash_type = self
                    .sighash
                    .map(|partial| partial.parse::<PsbtSighashType>())
                    .transpose()
                    .map_err(E::Sighash)?;
                let redeem_script = self
                    .redeem_script
                    .map(|script| script.script_buf())
                    .transpose()
                    .map_err(E::RedeemScript)?;
                let witness_script = self
                    .witness_script
                    .map(|script| script.script_buf())
                    .transpose()
                    .map_err(E::WitnessScript)?;
                let bip32_derivation = match self.bip32_derivs {
                    Some(derivs) =>
                        crate::psbt::vec_into_bip32_derivation(derivs).map_err(E::Bip32Derivs)?,
                    None => BTreeMap::default(),
                };
                let final_script_sig = self
                    .final_script_sig
                    .map(|script| script.script_buf())
                    .transpose()
                    .map_err(E::FinalScriptSig)?;
                let final_script_witness = self
                    .final_script_witness
                    .map(|v| crate::witness_from_hex_slice(&v))
                    .transpose()
                    .map_err(E::FinalScriptWitness)?;

                let ripemd160_preimages = match self.ripemd160_preimages {
                    Some(map) => {
                        let mut preimages = BTreeMap::default();
                        for (hash, preimage) in map.iter() {
                            let hash = hash.parse::<ripemd160::Hash>().map_err(E::Ripemd160)?;
                            let preimage = Vec::from_hex(preimage).map_err(E::Ripemd160Preimage)?;
                            preimages.insert(hash, preimage);
                        }
                        preimages
                    }
                    None => BTreeMap::default(),
                };
                let sha256_preimages = match self.sha256_preimages {
                    Some(map) => {
                        let mut preimages = BTreeMap::default();
                        for (hash, preimage) in map.iter() {
                            let hash = hash.parse::<sha256::Hash>().map_err(E::Sha256)?;
                            let preimage = Vec::from_hex(preimage).map_err(E::Sha256Preimage)?;
                            preimages.insert(hash, preimage);
                        }
                        preimages
                    }
                    None => BTreeMap::default(),
                };
                let hash160_preimages = match self.hash160_preimages {
                    Some(map) => {
                        let mut preimages = BTreeMap::default();
                        for (hash, preimage) in map.iter() {
                            let hash = hash.parse::<hash160::Hash>().map_err(E::Hash160)?;
                            let preimage = Vec::from_hex(preimage).map_err(E::Hash160Preimage)?;
                            preimages.insert(hash, preimage);
                        }
                        preimages
                    }
                    None => BTreeMap::default(),
                };
                let hash256_preimages = match self.hash256_preimages {
                    Some(map) => {
                        let mut preimages = BTreeMap::default();
                        for (hash, preimage) in map.iter() {
                            let hash = hash.parse::<sha256d::Hash>().map_err(E::Hash256)?;
                            let preimage = Vec::from_hex(preimage).map_err(E::Hash256Preimage)?;
                            preimages.insert(hash, preimage);
                        }
                        preimages
                    }
                    None => BTreeMap::default(),
                };

                let tap_key_sig = self
                    .taproot_key_path_sig
                    .map(|s| taproot::signature_from_str(&s))
                    .transpose()
                    .map_err(E::TaprootKeyPathSig)?;
                let tap_script_sigs = match self.taproot_script_path_sigs {
                    Some(vec) => {
                        let mut map = BTreeMap::default();
                        for elem in vec.iter() {
                            let ((pubkey, hash), sig) =
                                elem.to_key_value_pair().map_err(E::TaprootScriptPathSigs)?;
                            map.insert((pubkey, hash), sig);
                        }
                        map
                    }
                    None => BTreeMap::default(),
                };
                let tap_scripts = match self.taproot_scripts {
                    Some(vec) => {
                        let mut map = BTreeMap::default();
                        for elem in vec.iter() {
                            let (control_block, (script, key_source)) =
                                elem.to_key_value_pair().map_err(E::TaprootScripts)?;
                            map.insert(control_block, (script, key_source));
                        }
                        map
                    }
                    None => BTreeMap::default(),
                };
                let tap_key_origins = match self.taproot_bip32_derivs {
                    Some(vec) => {
                        let mut map = BTreeMap::default();
                        for elem in vec.iter() {
                            let (pubkey, (leaves, key_source)) =
                                elem.to_key_value_pair().map_err(E::TaprootBip32Derivs)?;
                            map.insert(pubkey, (leaves, key_source));
                        }
                        map
                    }
                    None => BTreeMap::default(),
                };
                let tap_internal_key = self
                    .taproot_internal_key
                    .map(|key| key.parse::<XOnlyPublicKey>())
                    .transpose()
                    .map_err(E::TaprootInternalKey)?;
                let tap_merkle_root = self
                    .taproot_merkle_root
                    .map(|root| root.parse::<TapNodeHash>())
                    .transpose()
                    .map_err(E::TaprootMerkleRoot)?;

                let proprietary = match self.proprietary {
                    Some(props) => {
                        let mut map = BTreeMap::default();
                        for prop in props {
                            let (key, vec) = prop.to_key_value_pair().map_err(E::Proprietary)?;
                            map.insert(key, vec);
                        }
                        map
                    }
                    None => BTreeMap::default(),
                };

                let unknown = match self.unknown {
                    Some(map) => crate::psbt::into_unknown(map).map_err(E::Unknown)?,
                    None => BTreeMap::default(),
                };

                Ok(psbt::Input {
                    non_witness_utxo,
                    witness_utxo,
                    partial_sigs,
                    sighash_type,
                    redeem_script,
                    witness_script,
                    bip32_derivation,
                    final_script_sig,
                    final_script_witness,
                    ripemd160_preimages,
                    sha256_preimages,
                    hash160_preimages,
                    hash256_preimages,
                    tap_key_sig,
                    tap_script_sigs,
                    tap_scripts,
                    tap_key_origins,
                    tap_internal_key,
                    tap_merkle_root,
                    proprietary,
                    unknown,
                })
            }
        }

        impl PsbtOutput {
            /// Converts this PSBT data into a PSBT output.
            pub fn into_output(self) -> Result<psbt::Output, PsbtOutputError> {
                use PsbtOutputError as E;

                let redeem_script = self
                    .redeem_script
                    .map(|script| script.script_buf())
                    .transpose()
                    .map_err(E::RedeemScript)?;
                let witness_script = self
                    .witness_script
                    .map(|script| script.script_buf())
                    .transpose()
                    .map_err(E::WitnessScript)?;
                let bip32_derivation = match self.bip32_derivs {
                    Some(derivs) =>
                        crate::psbt::vec_into_bip32_derivation(derivs).map_err(E::Bip32Derivs)?,
                    None => BTreeMap::default(),
                };

                let tap_internal_key = self
                    .taproot_internal_key
                    .map(|key| key.parse::<XOnlyPublicKey>())
                    .transpose()
                    .map_err(E::TaprootInternalKey)?;

                let tap_tree =
                    self.taproot_tree.map(build_taproot_tree).transpose().map_err(E::TaprootTree)?;
                let tap_key_origins = match self.taproot_bip32_derivs {
                    Some(vec) => {
                        let mut map = BTreeMap::default();
                        for elem in vec.iter() {
                            let (pubkey, (leaves, key_source)) =
                                elem.to_key_value_pair().map_err(E::TaprootBip32Derivs)?;
                            map.insert(pubkey, (leaves, key_source));
                        }
                        map
                    }
                    None => BTreeMap::default(),
                };

                let proprietary = match self.proprietary {
                    Some(props) => {
                        let mut map = BTreeMap::default();
                        for prop in props {
                            let (key, vec) = prop.to_key_value_pair().map_err(E::Proprietary)?;
                            map.insert(key, vec);
                        }
                        map
                    }
                    None => BTreeMap::default(),
                };

                let unknown = match self.unknown {
                    Some(map) => crate::psbt::into_unknown(map).map_err(E::Unknown)?,
                    None => BTreeMap::default(),
                };

                Ok(psbt::Output {
                    redeem_script,
                    witness_script,
                    bip32_derivation,
                    tap_internal_key,
                    tap_tree,
                    tap_key_origins,
                    proprietary,
                    unknown,
                })
            }
        }

        impl TaprootScriptPathSig {
            /// Converts list element to a map entry suitable to use in `bitcoin::psbt::Input`.
            pub fn to_key_value_pair(
                &self,
            ) -> Result<((XOnlyPublicKey, TapLeafHash), taproot::Signature), TaprootScriptPathSigError>
            {
                use TaprootScriptPathSigError as E;

                let pubkey = self.pubkey.parse::<XOnlyPublicKey>().map_err(E::Pubkey)?;
                let hash = self.leaf_hash.parse::<TapLeafHash>().map_err(E::LeafHash)?;
                let sig = super::taproot::signature_from_str(&self.sig).map_err(E::Sig)?;

                Ok(((pubkey, hash), sig))
            }
        }

        impl TaprootScript {
            /// Converts list element to a map entry suitable to use in `bitcoin::psbt::Input`.
            pub fn to_key_value_pair(
                &self,
            ) -> Result<(ControlBlock, (ScriptBuf, LeafVersion)), TaprootScriptError> {
                use TaprootScriptError as E;

                let script = ScriptBuf::from_hex(&self.script).map_err(E::Script)?;

                let leaf_version = self.leaf_version as u8; // FIXME: Is this cast ok?
                let version = LeafVersion::from_consensus(leaf_version).map_err(E::LeafVer)?;

                let control_block = control_block(&self.control_blocks).map_err(E::ControlBlocks)?;

                Ok((control_block, (script, version)))
            }
        }

        // FIXME: I (Tobin) cannot work out why Core returns a vector of control blocks. From my
        // reading of rust-bitcoin code and also BIP-341 there is exactly one control block per script?
        fn control_block(control_blocks: &[String]) -> Result<ControlBlock, ControlBlocksError> {
            use ControlBlocksError as E;

            match control_blocks.len() {
                // FIXME: How can this be empty, there would be nothing to key the `tap_scripts` map by?
                0 => Err(E::Missing),
                1 => {
                    let bytes = Vec::from_hex(&control_blocks[0]).map_err(E::Parse)?;
                    Ok(ControlBlock::decode(&bytes).map_err(E::Decode)?)
                }
                n => Err(E::Multiple(n)),
            }
        }

        impl TaprootBip32Deriv {
            /// Converts list element to a map entry suitable to use in `bitcoin::psbt::Input`.
            pub fn to_key_value_pair(
                &self,
            ) -> Result<(XOnlyPublicKey, (Vec<TapLeafHash>, KeySource)), TaprootBip32DerivsError> {
                use TaprootBip32DerivsError as E;

                let pubkey = self.pubkey.parse::<XOnlyPublicKey>().map_err(E::Pubkey)?;
                let fp = Fingerprint::from_hex(&self.master_fingerprint).map_err(E::MasterFingerprint)?;
                let path = self.path.parse::<DerivationPath>().map_err(E::Path)?;
                let hashes = self
                    .leaf_hashes
                    .iter()
                    .map(|leaf| leaf.parse::<TapLeafHash>())
                    .collect::<Result<_, _>>()
                    .map_err(E::LeafHashes)?;

                Ok((pubkey, (hashes, (fp, path))))
            }
        }

        fn build_taproot_tree(leaves: Vec<TaprootLeaf>) -> Result<TapTree, TaprootLeafError> {
            use TaprootLeafError as E;

            let mut builder = TaprootBuilder::with_capacity(leaves.len());

            for leaf in leaves.iter() {
                // Cast ok because depth can never exceed 128.
                let depth = leaf.depth as u8;

                let leaf_version = leaf.leaf_version as u8; // FIXME: Is this cast ok?
                let version = LeafVersion::from_consensus(leaf_version).map_err(E::LeafVer)?;

                let script = ScriptBuf::from_hex(&leaf.script).map_err(E::Script)?;

                builder = builder.add_leaf_with_ver(depth, script, version).map_err(E::TaprootBuilder)?;
            }
            let tree = builder.try_into_taptree().map_err(E::IncompleteBuilder)?;
            Ok(tree)
        }
    }

    use std::collections::HashMap;

    use serde::{Deserialize, Serialize};

    use crate::ScriptSig;

    #[rustfmt::skip]                // Keep public re-exports separate.
    pub use self::error::{
        DecodePsbtError, GlobalXpubError, PsbtInputError, PsbtOutputError, TaprootScriptPathSigError,
        TaprootScriptError, TaprootBip32DerivsError, ControlBlocksError, TaprootLeafError
    };
    // Re-export types that appear in the public API of this module.
    pub use super::{Bip32DerivError, PartialSignatureError, RawTransactionError, WitnessUtxoError};
    pub use crate::psbt::{Bip32Deriv, PsbtScript, RawTransaction, WitnessUtxo};

    /// Result of JSON-RPC method `decodepsbt`.
    ///
    /// > decodepsbt "psbt"
    /// >
    /// > Return a JSON object representing the serialized, base64-encoded partially signed Bitcoin transaction.
    /// >
    /// > Arguments:
    /// > 1. "psbt"            (string, required) The PSBT base64 string
    #[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
    #[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
    pub struct DecodePsbt {
        /// The decoded network-serialized unsigned transaction.
        pub tx: RawTransaction,
        /// The global xpubs.
        pub global_xpubs: Vec<GlobalXpub>,
        /// The PSBT version number. Not to be confused with the unsigned transaction version.
        pub psbt_version: u32,
        /// The global proprietary map.
        pub proprietary: Option<Vec<Proprietary>>,
        /// The unknown global fields.
        pub unknown: Option<HashMap<String, String>>,
        /// Array of transaction inputs.
        pub inputs: Vec<PsbtInput>,
        /// Array of transaction outputs.
        pub outputs: Vec<PsbtOutput>,
        /// The transaction fee paid if all UTXOs slots in the PSBT have been filled.
        pub fee: Option<f64>,
    }

    /// An item from the global xpubs list. Part of `decodepsbt`.
    #[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
    #[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
    pub struct GlobalXpub {
        /// The extended public key this path corresponds to.
        pub xpub: String,
        /// The fingerprint of the master key.
        pub master_fingerprint: String,
        /// The path.
        pub path: String,
    }

    /// An item from the global proprietary list. Part of `decodepsbt`.
    #[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
    #[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
    pub struct Proprietary {
        /// The hex string for the proprietary identifier.
        identifier: String,
        /// The number for the subtype.
        subtype: i64,
        /// The hex for the key.
        key: String,
        /// The hex for the value.
        value: String,
    }

    /// An input in a partially signed Bitcoin transaction. Part of `decodepsbt`.
    ///
    /// TODO: Update model once Musig is supported in rust-bitcoin.
    #[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
    #[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
    pub struct PsbtInput {
        /// Decoded network transaction for non-witness UTXOs.
        pub non_witness_utxo: Option<RawTransaction>,
        /// Transaction output for witness UTXOs.
        pub witness_utxo: Option<WitnessUtxo>,
        /// The public key and signature that corresponds to it.
        pub partial_signatures: Option<HashMap<String, String>>,
        /// The sighash type to be used.
        pub sighash: Option<String>,
        /// The redeem script.
        pub redeem_script: Option<PsbtScript>,
        /// The witness script.
        pub witness_script: Option<PsbtScript>,
        /// The public key with the derivation path as the value.
        pub bip32_derivs: Option<Vec<Bip32Deriv>>,
        /// The final scriptsig.
        #[serde(rename = "final_scriptsig")]
        pub final_script_sig: Option<ScriptSig>,
        /// Hex-encoded witness data (if any).
        #[serde(rename = "final_scriptwitness")]
        pub final_script_witness: Option<Vec<String>>,
        /// The hash and preimage that corresponds to it.
        pub ripemd160_preimages: Option<HashMap<String, String>>,
        /// The hash and preimage that corresponds to it.
        pub sha256_preimages: Option<HashMap<String, String>>,
        /// The hash and preimage that corresponds to it.
        pub hash160_preimages: Option<HashMap<String, String>>,
        /// The hash and preimage that corresponds to it.
        pub hash256_preimages: Option<HashMap<String, String>>,
        /// Hex-encoded signature for the Taproot key path spend.
        pub taproot_key_path_sig: Option<String>,
        /// The signature for the pubkey and leaf hash combination.
        pub taproot_script_path_sigs: Option<Vec<TaprootScriptPathSig>>,
        /// Scripts and control blocks for script path spends.
        pub taproot_scripts: Option<Vec<TaprootScript>>,
        /// BIP-32 derivation paths for keys.
        pub taproot_bip32_derivs: Option<Vec<TaprootBip32Deriv>>,
        /// The hex-encoded Taproot x-only internal key.
        pub taproot_internal_key: Option<String>,
        /// The hex-encoded Taproot merkle root.
        pub taproot_merkle_root: Option<String>,
        /// MuSig2 participant public keys.
        pub musig2_participant_pubkeys: Option<Vec<Musig2ParticipantPubkeys>>,
        /// MuSig2 public nonces.
        pub musig2_pubnonces: Option<Vec<Musig2Pubnonce>>,
        /// MuSig2 partial signatures.
        pub musig2_partial_sigs: Option<Vec<Musig2PartialSig>>,
        /// The input proprietary map.
        pub proprietary: Option<Vec<Proprietary>>,
        /// The unknown input fields.
        pub unknown: Option<HashMap<String, String>>,
    }

    /// An output in a partially signed Bitcoin transaction. Part of `decodepsbt`.
    #[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
    #[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
    pub struct PsbtOutput {
        /// The redeem script.
        pub redeem_script: Option<PsbtScript>,
        /// The witness script.
        pub witness_script: Option<PsbtScript>,
        /// The public key with the derivation path as the value.
        pub bip32_derivs: Option<Vec<Bip32Deriv>>,
        /// The hex-encoded Taproot x-only internal key.
        pub taproot_internal_key: Option<String>,
        /// The tuples that make up the Taproot tree, in depth first search order.
        pub taproot_tree: Option<Vec<TaprootLeaf>>,
        /// BIP32 derivation paths for keys.
        pub taproot_bip32_derivs: Option<Vec<TaprootBip32Deriv>>,
        /// MuSig2 participant public keys.
        pub musig2_participant_pubkeys: Option<Vec<Musig2ParticipantPubkeys>>,
        /// The output proprietary map.
        pub proprietary: Option<Vec<Proprietary>>,
        /// The unknown global fields.
        pub unknown: Option<HashMap<String, String>>,
    }

    /// An item from the `taproot_script_path_sigs` list. Part of `decodepsbt`.
    #[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
    #[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
    pub struct TaprootScriptPathSig {
        /// The x-only pubkey for this signature.
        pub pubkey: String,
        /// The leaf hash for this signature.
        pub leaf_hash: String,
        /// The signature itself.
        pub sig: String,
    }

    /// An item from the `taproot_scripts` list. Part of `decodepsbt`.
    #[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
    #[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
    pub struct TaprootScript {
        /// A leaf script.
        pub script: String,
        /// The version number for the leaf script.
        #[serde(rename = "leaf_ver")]
        pub leaf_version: u32,
        /// The control blocks for this script.
        pub control_blocks: Vec<String>,
    }

    /// An item from the `taproot_bip32_derivs` list. Part of `decodepsbt`.
    #[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
    #[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
    pub struct TaprootBip32Deriv {
        /// The x-only public key this path corresponds to.
        pub pubkey: String,
        /// The fingerprint of the master key.
        pub master_fingerprint: String,
        /// The path.
        pub path: String,
        /// The hashes of the leaves this pubkey appears in.
        pub leaf_hashes: Vec<String>,
    }

    /// A Taproot leaf script at depth with version. Part of `decodepsbt`.
    #[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
    #[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
    pub struct TaprootLeaf {
        /// The depth of this element in the tree.
        pub depth: u32,
        /// The version of this leaf.
        #[serde(rename = "leaf_ver")]
        pub leaf_version: u32,
        /// The hex-encoded script itself.
        pub script: String,
    }

    /// MuSig2 participant public keys. Part of `decodepsbt`.
    #[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
    #[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
    pub struct Musig2ParticipantPubkeys {
        /// The compressed aggregate public key for which the participants create.
        pub aggregate_pubkey: String,
        /// The compressed public keys that are aggregated for aggregate_pubkey.
        pub participant_pubkeys: Vec<String>,
    }

    /// MuSig2 public nonce. Part of `decodepsbt`.
    #[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
    #[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
    pub struct Musig2Pubnonce {
        /// The compressed public key of the participant that created this pubnonce.
        pub participant_pubkey: String,
        /// The compressed aggregate public key for which this pubnonce is for.
        pub aggregate_pubkey: String,
        /// The hash of the leaf script that contains the aggregate pubkey being signed for. Omitted when signing for the internal key.
        pub leaf_hash: Option<String>,
        /// The public nonce itself.
        pub pubnonce: String,
    }

    /// MuSig2 partial signature. Part of `decodepsbt`.
    #[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
    #[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
    pub struct Musig2PartialSig {
        /// The compressed public key of the participant that created this partial signature.
        pub participant_pubkey: String,
        /// The compressed aggregate public key for which this partial signature is for.
        pub aggregate_pubkey: String,
        /// The hash of the leaf script that contains the aggregate pubkey being signed for. Omitted when signing for the internal key.
        pub leaf_hash: Option<String>,
        /// The partial signature itself.
        pub partial_sig: String,
    }

    // TODO: Remove all this code once it is implemented and backported to 0.32.x
    // https://github.com/rust-bitcoin/rust-bitcoin/issues/3285
    pub mod taproot {
        use core::fmt;

        use bitcoin::hex::{self, FromHex as _};
        use bitcoin::sighash::InvalidSighashTypeError;
        // Re-export this because this module is named the same as the one from `bitcoin`.
        pub use bitcoin::taproot::Signature;
        use bitcoin::{secp256k1, taproot, TapSighashType};

        use crate::error::write_err;

        /// Parses a Taproot signature from a hex string.
        pub fn signature_from_str(sig: &str) -> Result<taproot::Signature, Error> {
            use Error as E;

            let bytes = Vec::from_hex(sig).map_err(E::Hex)?;
            let (sighash_byte, signature) = bytes.split_last().ok_or(E::EmptySignature)?;
            Ok(Signature {
                signature: secp256k1::schnorr::Signature::from_slice(signature)
                    .map_err(E::Secp256k1)?,
                sighash_type: TapSighashType::from_consensus_u8(*sighash_byte)
                    .map_err(E::SighashType)?,
            })
        }

        /// A Taproot signature-related error.
        #[derive(Debug, Clone, PartialEq, Eq)]
        #[non_exhaustive]
        pub enum Error {
            /// Hex decoding error.
            Hex(hex::HexToBytesError),
            /// Non-standard sighash type.
            SighashType(InvalidSighashTypeError),
            /// Signature was empty.
            EmptySignature,
            /// A secp256k1 error while creating signature from a slice.
            Secp256k1(secp256k1::Error),
        }

        impl fmt::Display for Error {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                use Error::*;

                match *self {
                    Hex(ref e) => write_err!(f, "signature hex decoding error"; e),
                    SighashType(ref e) => write_err!(f, "non-standard signature hash type"; e),
                    EmptySignature => write!(f, "empty Taproot signature"),
                    Secp256k1(ref e) => write_err!(f, "secp256k1"; e),
                }
            }
        }

        #[cfg(feature = "std")]
        impl std::error::Error for Error {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                use Error::*;

                match *self {
                    Hex(ref e) => Some(e),
                    Secp256k1(ref e) => Some(e),
                    SighashType(ref e) => Some(e),
                    EmptySignature => None,
                }
            }
        }
    }
}
mod wallet {
    // SPDX-License-Identifier: CC0-1.0

    //! The JSON-RPC API for Bitcoin Core `v30` - wallet.
    //!
    //! Types for methods found under the `== Wallet ==` section of the API docs.

    mod error {
        // SPDX-License-Identifier: CC0-1.0

        use core::fmt;

        use bitcoin::address;
        use bitcoin::amount::ParseAmountError;
        use bitcoin::hex;

        use crate::error::write_err;
        use crate::NumericError;

        /// Error when converting a `GetTransactionDetail` type into the model type.
        #[derive(Debug)]
        pub enum GetTransactionDetailError {
            /// Conversion of the `address` field failed.
            Address(address::ParseError),
            /// Conversion of the `amount` field failed.
            Amount(ParseAmountError),
            /// Conversion of the `fee` field failed.
            Fee(ParseAmountError),
        }

        impl fmt::Display for GetTransactionDetailError {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match *self {
                    Self::Address(ref e) => write_err!(f, "conversion of the `address` field failed"; e),
                    Self::Amount(ref e) => write_err!(f, "conversion of the `amount` field failed"; e),
                    Self::Fee(ref e) => write_err!(f, "conversion of the `fee` field failed"; e),
                }
            }
        }

        #[cfg(feature = "std")]
        impl std::error::Error for GetTransactionDetailError {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                match *self {
                    Self::Address(ref e) => Some(e),
                    Self::Amount(ref e) => Some(e),
                    Self::Fee(ref e) => Some(e),
                }
            }
        }

        /// Error when converting a `GetWalletInfo` type into the model type.
        #[derive(Debug)]
        pub enum GetWalletInfoError {
            /// Conversion of numeric type to expected type failed.
            Numeric(NumericError),
            /// Conversion of the `pay_tx_fee` field failed.
            PayTxFee(ParseAmountError),
            /// Conversion of the `last_processed_block` field failed.
            LastProcessedBlock(LastProcessedBlockError),
        }

        impl fmt::Display for GetWalletInfoError {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match *self {
                    Self::Numeric(ref e) => write_err!(f, "numeric"; e),
                    Self::PayTxFee(ref e) =>
                        write_err!(f, "conversion of the `pay_tx_fee` field failed"; e),
                    Self::LastProcessedBlock(ref e) =>
                        write_err!(f, "conversion of the `last_processed_block` field failed"; e),
                }
            }
        }

        #[cfg(feature = "std")]
        impl std::error::Error for GetWalletInfoError {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                match *self {
                    Self::Numeric(ref e) => Some(e),
                    Self::PayTxFee(ref e) => Some(e),
                    Self::LastProcessedBlock(ref e) => Some(e),
                }
            }
        }

        impl From<NumericError> for GetWalletInfoError {
            fn from(e: NumericError) -> Self { Self::Numeric(e) }
        }

        /// Error when converting a `LastProcessedBlock` type into the model type.
        #[derive(Debug)]
        pub enum LastProcessedBlockError {
            /// Conversion of the `hash` field failed.
            Hash(hex::HexToArrayError),
            /// Conversion of the `height` field failed.
            Height(NumericError),
        }

        impl fmt::Display for LastProcessedBlockError {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match *self {
                    Self::Hash(ref e) => write_err!(f, "conversion of the `hash` field failed"; e),
                    Self::Height(ref e) => write_err!(f, "conversion of the `height` field failed"; e),
                }
            }
        }

        #[cfg(feature = "std")]
        impl std::error::Error for LastProcessedBlockError {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                match *self {
                    Self::Hash(ref e) => Some(e),
                    Self::Height(ref e) => Some(e),
                }
            }
        }

        impl From<NumericError> for LastProcessedBlockError {
            fn from(e: NumericError) -> Self { Self::Height(e) }
        }

        /// Error when converting a `ListUnspentItem` type into the model type.
        #[derive(Debug)]
        pub enum ListUnspentItemError {
            /// Conversion of numeric type to expected type failed.
            Numeric(NumericError),
            /// Conversion of the `txid` field failed.
            Txid(hex::HexToArrayError),
            /// Conversion of the `address` field failed.
            Address(address::ParseError),
            /// Conversion of the `script_pubkey` field failed.
            ScriptPubkey(hex::HexToBytesError),
            /// Conversion of the `amount` field failed.
            Amount(ParseAmountError),
            /// Conversion of the `redeem_script` field failed.
            RedeemScript(hex::HexToBytesError),
            /// Conversion of the `witness_script` field failed.
            WitnessScript(hex::HexToBytesError),
            /// Conversion of the `ancestorfees` field failed.
            AncestorFees(ParseAmountError),
        }

        impl fmt::Display for ListUnspentItemError {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match *self {
                    Self::Numeric(ref e) => write_err!(f, "numeric"; e),
                    Self::Txid(ref e) => write_err!(f, "conversion of the `txid` field failed"; e),
                    Self::Address(ref e) => write_err!(f, "conversion of the `address` field failed"; e),
                    Self::ScriptPubkey(ref e) =>
                        write_err!(f, "conversion of the `script_pubkey` field failed"; e),
                    Self::Amount(ref e) => write_err!(f, "conversion of the `amount` field failed"; e),
                    Self::RedeemScript(ref e) =>
                        write_err!(f, "conversion of the `redeem_script` field failed"; e),
                    Self::WitnessScript(ref e) =>
                        write_err!(f, "conversion of the `witness_script` field failed"; e),
                    Self::AncestorFees(ref e) =>
                        write_err!(f, "conversion of the `ancestorfees` field failed"; e),
                }
            }
        }

        #[cfg(feature = "std")]
        impl std::error::Error for ListUnspentItemError {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                match *self {
                    Self::Numeric(ref e) => Some(e),
                    Self::Txid(ref e) => Some(e),
                    Self::Address(ref e) => Some(e),
                    Self::ScriptPubkey(ref e) => Some(e),
                    Self::Amount(ref e) => Some(e),
                    Self::RedeemScript(ref e) => Some(e),
                    Self::WitnessScript(ref e) => Some(e),
                    Self::AncestorFees(ref e) => Some(e),
                }
            }
        }

        impl From<NumericError> for ListUnspentItemError {
            fn from(e: NumericError) -> Self { Self::Numeric(e) }
        }
    }
    mod into {
        // SPDX-License-Identifier: CC0-1.0

        use bitcoin::BlockHash;

        use super::{
            GetWalletInfo, GetWalletInfoError, GetWalletInfoScanning, LastProcessedBlock,
            LastProcessedBlockError,
        };
        use crate::model;

        impl GetWalletInfo {
            /// Converts version specific type to a version nonspecific, more strongly typed type.
            pub fn into_model(self) -> Result<model::GetWalletInfo, GetWalletInfoError> {
                use GetWalletInfoError as E;

                let wallet_version = crate::to_u32(self.wallet_version, "wallet_version")?;
                let tx_count = crate::to_u32(self.tx_count, "tx_count")?;
                let keypool_size = crate::to_u32(self.keypool_size, "keypool_size")?;
                let keypool_size_hd_internal = self
                    .keypool_size_hd_internal
                    .map(|v| crate::to_u32(v, "keypool_size_hd_internal"))
                    .transpose()?;
                let pay_tx_fee = crate::btc_per_kb(self.pay_tx_fee).map_err(E::PayTxFee)?;
                let last_processed_block = self
                    .last_processed_block
                    .map(|l| l.into_model())
                    .transpose()
                    .map_err(E::LastProcessedBlock)?;

                let scanning = match self.scanning {
                    GetWalletInfoScanning::Details { duration, progress } =>
                        Some(model::GetWalletInfoScanning::Details { duration, progress }),
                    GetWalletInfoScanning::NotScanning(b) =>
                        Some(model::GetWalletInfoScanning::NotScanning(b)),
                };

                Ok(model::GetWalletInfo {
                    wallet_name: self.wallet_name,
                    wallet_version,
                    format: Some(self.format),
                    balance: None,
                    unconfirmed_balance: None,
                    immature_balance: None,
                    tx_count,
                    keypool_oldest: None,
                    keypool_size,
                    keypool_size_hd_internal: keypool_size_hd_internal.unwrap_or(0),
                    unlocked_until: self.unlocked_until,
                    pay_tx_fee,
                    hd_seed_id: None,
                    private_keys_enabled: self.private_keys_enabled,
                    avoid_reuse: Some(self.avoid_reuse),
                    scanning,
                    descriptors: Some(self.descriptors),
                    external_signer: Some(self.external_signer),
                    blank: Some(self.blank),
                    birthtime: self.birthtime,
                    flags: Some(self.flags),
                    last_processed_block,
                })
            }
        }

        impl LastProcessedBlock {
            /// Converts version specific type to a version nonspecific, more strongly typed type.
            pub fn into_model(self) -> Result<model::LastProcessedBlock, LastProcessedBlockError> {
                let hash = self.hash.parse::<BlockHash>().map_err(LastProcessedBlockError::Hash)?;
                let height = crate::to_u32(self.height, "height")?;
                Ok(model::LastProcessedBlock { height, hash })
            }
        }
    }

    use serde::{Deserialize, Serialize};

    pub use self::error::{GetWalletInfoError, LastProcessedBlockError};

    /// Result of the JSON-RPC method `getwalletinfo`.
    ///
    /// > getwalletinfo
    /// >
    /// > Returns an object containing various wallet state info.
    #[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
    #[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
    pub struct GetWalletInfo {
        /// the wallet name
        #[serde(rename = "walletname")]
        pub wallet_name: String,
        /// the wallet version
        #[serde(rename = "walletversion")]
        pub wallet_version: i64,
        /// the database format (bdb or sqlite)
        pub format: String,
        /// the total number of transactions in the wallet
        #[serde(rename = "txcount")]
        pub tx_count: i64,
        /// how many new keys are pre-generated (only counts external keys)
        #[serde(rename = "keypoolsize")]
        pub keypool_size: i64,
        /// how many new keys are pre-generated for internal use (used for change outputs, only appears if the wallet is using this feature, otherwise external keys are used)
        #[serde(rename = "keypoolsize_hd_internal")]
        pub keypool_size_hd_internal: Option<i64>,
        /// the UNIX epoch time until which the wallet is unlocked for transfers, or 0 if the wallet is locked (only present for passphrase-encrypted wallets)
        pub unlocked_until: Option<u32>,
        /// the transaction fee configuration, set in BTC/kvB
        #[serde(rename = "paytxfee")]
        pub pay_tx_fee: f64,
        /// false if privatekeys are disabled for this wallet (enforced watch-only wallet)
        pub private_keys_enabled: bool,
        /// whether this wallet tracks clean/dirty coins in terms of reuse
        pub avoid_reuse: bool,
        /// current scanning details, or false if no scan is in progress
        pub scanning: GetWalletInfoScanning,
        /// whether this wallet uses descriptors for scriptPubKey management
        pub descriptors: bool,
        /// whether this wallet is configured to use an external signer such as a hardware wallet
        pub external_signer: bool,
        /// Whether this wallet intentionally does not contain any keys, scripts, or descriptors
        pub blank: bool,
        /// The start time for blocks scanning. It could be modified by (re)importing any descriptor with an earlier timestamp.
        pub birthtime: Option<u32>,
        /// The flags currently set on the wallet.
        pub flags: Vec<String>,
        /// hash and height of the block this information was generated on
        #[serde(rename = "lastprocessedblock")]
        pub last_processed_block: Option<LastProcessedBlock>,
    }

    /// Current scanning details. Part of `getwalletinfo`.
    #[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum GetWalletInfoScanning {
        /// Scanning details.
        Details { duration: u64, progress: f64 },
        /// Not scanning (false).
        NotScanning(bool),
    }

    /// Last processed block item. Part of of `getwalletinfo`.
    #[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
    #[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
    pub struct LastProcessedBlock {
        /// Hash of the block this information was generated on.
        pub hash: String,
        /// Height of the block this information was generated on.
        pub height: i64,
    }

    /// Result of the JSON-RPC method `listwalletdir`.
    ///
    /// > listwalletdir
    /// >
    /// > Returns a list of wallets in the wallet directory.
    #[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
    #[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
    pub struct ListWalletDir {
        /// The list of wallets in the wallet directory.
        pub wallets: Vec<ListWalletDirWallet>,
    }

    /// Wallet entry. Part of `listwalletdir`.
    #[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
    #[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
    pub struct ListWalletDirWallet {
        /// The wallet name.
        pub name: String,
        /// Warning messages, if any, related to loading the wallet.
        pub warnings: Option<Vec<String>>,
    }
}

#[doc(inline)]
pub use self::{
    blockchain::GetMempoolInfo,
    hidden::{
        GetOrphanTxs, GetOrphanTxsVerboseOne, GetOrphanTxsVerboseOneEntry,
        GetOrphanTxsVerboseOneEntryError, GetOrphanTxsVerboseTwo, GetOrphanTxsVerboseTwoEntry,
        GetOrphanTxsVerboseTwoEntryError,
    },
    mining::{GetMiningInfo, GetMiningInfoError},
    raw_transactions::{
        ControlBlocksError, DecodePsbt, DecodePsbtError, GlobalXpub, GlobalXpubError,
        Musig2PartialSig, Musig2ParticipantPubkeys, Musig2Pubnonce, Proprietary, PsbtInput,
        PsbtInputError, PsbtOutput, PsbtOutputError, TaprootBip32Deriv, TaprootBip32DerivsError,
        TaprootLeaf, TaprootLeafError, TaprootScript, TaprootScriptError, TaprootScriptPathSig,
        TaprootScriptPathSigError,
    },
    wallet::{
        GetWalletInfo, GetWalletInfoError, GetWalletInfoScanning, LastProcessedBlock,
        LastProcessedBlockError, ListWalletDir, ListWalletDirWallet,
    },
};
#[doc(inline)]
pub use crate::{
    v17::{
        AbortRescan, AddedNode, AddedNodeAddress, AddressInformation, AddressPurpose,
        Bip125Replaceable, Bip32DerivError, BlockTemplateTransaction,
        BlockTemplateTransactionError, BumpFee, BumpFeeError, ChainTips, ChainTipsError,
        ChainTipsStatus, CombinePsbt, CombineRawTransaction, ConvertToPsbt, CreateMultisigError,
        CreatePsbt, CreateRawTransaction, DecodeRawTransaction, EncryptWallet, EstimateRawFee,
        EstimateRawFeeError, EstimateSmartFee, FinalizePsbt, FinalizePsbtError, FundRawTransaction,
        FundRawTransactionError, Generate, GenerateToAddress, GetAddedNodeInfo,
        GetAddressInfoEmbeddedError, GetAddressesByLabel, GetBalance, GetBestBlockHash,
        GetBlockCount, GetBlockHash, GetBlockStatsError, GetBlockTemplate, GetBlockTemplateError,
        GetBlockVerboseZero, GetChainTips, GetChainTxStatsError, GetConnectionCount, GetDifficulty,
        GetMemoryInfoStats, GetMempoolInfoError, GetNetTotals, GetNetworkInfoAddress,
        GetNetworkInfoError, GetNetworkInfoNetwork, GetNewAddress, GetRawChangeAddress,
        GetRawMempool, GetRawTransaction, GetRawTransactionVerbose, GetRawTransactionVerboseError,
        GetReceivedByAddress, GetTransactionDetailError, GetTxOut, GetTxOutError,
        ListAddressGroupings, ListAddressGroupingsError, ListAddressGroupingsItem, ListLabels,
        ListLockUnspent, ListLockUnspentItem, ListLockUnspentItemError, ListReceivedByAddressError,
        ListUnspentItemError, ListWallets, LockUnspent, Locked, NumericError,
        PartialSignatureError, PruneBlockchain, RawFeeDetail, RawFeeRange, RawTransactionError,
        RawTransactionInput, RawTransactionOutput, RescanBlockchain, ScanTxOutSetAbort,
        ScanTxOutSetError, ScanTxOutSetStatus, ScriptType, SendRawTransaction, SendToAddress,
        SetNetworkActive, SetTxFee, SignFail, SignFailError, SignMessage, SignMessageWithPrivKey,
        SignRawTransaction, SignRawTransactionError, SignRawTransactionWithKey,
        SignRawTransactionWithWallet, TransactionCategory, UploadTarget, ValidateAddress,
        ValidateAddressError, VerifyChain, VerifyMessage, VerifyTxOutProof, WaitForBlock,
        WaitForBlockError, WaitForBlockHeight, WaitForBlockHeightError, WaitForNewBlock,
        WaitForNewBlockError, WalletCreateFundedPsbt, WalletCreateFundedPsbtError, WitnessUtxo,
        WitnessUtxoError,
    },
    v18::{
        ActiveCommand, AnalyzePsbt, AnalyzePsbtError, AnalyzePsbtInput, AnalyzePsbtInputMissing,
        AnalyzePsbtInputMissingError, DeriveAddresses, GetAddressInfoError, GetReceivedByLabel,
        GetZmqNotifications, JoinPsbts, JsonRpcError, ListReceivedByAddress,
        ListReceivedByAddressItem, ListReceivedByLabel, ListReceivedByLabelError,
        ListReceivedByLabelItem, UtxoUpdatePsbt,
    },
    v19::{
        Bip9SoftforkInfo, Bip9SoftforkStatistics, Bip9SoftforkStatus, GetBalancesMine,
        GetBalancesWatchOnly, GetBlockFilter, GetBlockFilterError, GetChainTxStats, GetRpcInfo,
        MapMempoolEntryError, MempoolEntryError, MempoolEntryFees, MempoolEntryFeesError,
        SetWalletFlag, Softfork, SoftforkType,
    },
    v20::GenerateToDescriptor,
    v21::{
        AddPeerAddress, GetIndexInfo, GetIndexInfoName, GetRawMempoolSequence, ImportDescriptors,
        ImportDescriptorsResult, PsbtBumpFee, PsbtBumpFeeError, Send, SendError, SendMany,
        SendManyVerbose,
    },
    v22::{
        AddConnection, Banned, EnumerateSigners, GetNodeAddresses, ListBanned, NodeAddress,
        ScriptPubkey, Signers, WalletDisplayAddress,
    },
    v23::{
        Bip9Info, Bip9Statistics, CreateMultisig, DecodeScript, DecodeScriptError,
        DecodeScriptSegwit, DeploymentInfo, GetDeploymentInfo, GetDeploymentInfoError,
        RestoreWallet, SaveMempool,
    },
    v24::{
        GetMempoolAncestors, GetMempoolAncestorsVerbose, GetMempoolDescendants,
        GetMempoolDescendantsVerbose, GetMempoolEntry, GetRawMempoolVerbose, GetTransactionDetail,
        GetTxSpendingPrevout, GetTxSpendingPrevoutError, GetTxSpendingPrevoutItem, ListUnspent,
        ListUnspentItem, MempoolEntry, MigrateWallet, SendAll, SendAllError,
        SimulateRawTransaction,
    },
    v25::{
        DescriptorInfo, GenerateBlock, GenerateBlockError, GetBlockStats, ListDescriptors,
        MempoolAcceptanceError, ScanBlocksAbort, ScanBlocksStart, ScanBlocksStartError,
        ScanBlocksStatus, TestMempoolAcceptError,
    },
    v26::{
        AddrManInfoNetwork, CreateWallet, DescriptorProcessPsbt, DescriptorProcessPsbtError,
        DumpTxOutSet, DumpTxOutSetError, GetAddrManInfo, GetBalances, GetBalancesError,
        GetPeerInfo, GetTransactionError, GetTxOutSetInfo, GetTxOutSetInfoBlockInfo,
        GetTxOutSetInfoError, GetTxOutSetInfoUnspendables, LoadTxOutSet, LoadTxOutSetError,
        LoadWallet, PeerInfo, UnloadWallet, WalletProcessPsbt, WalletProcessPsbtError,
    },
    v27::{GetPrioritisedTransactions, PrioritisedTransaction},
    v28::{
        CreateWalletDescriptor, GetAddressInfo, GetAddressInfoEmbedded, GetHdKeys, GetHdKeysError,
        GetNetworkInfo, GetRawAddrMan, GetTransaction, HdKey, HdKeyDescriptor, ListSinceBlock,
        ListSinceBlockError, ListTransactions, Logging, RawAddrManEntry, ScanTxOutSetStart,
        ScanTxOutSetUnspent, SubmitPackage, SubmitPackageError, SubmitPackageTxResult,
        SubmitPackageTxResultError, SubmitPackageTxResultFees, SubmitPackageTxResultFeesError,
        TransactionItem, TransactionItemError,
    },
    v29::{
        ActivityEntry, ChainState, DeriveAddressesMultipath, GetBlockHeader, GetBlockHeaderError,
        GetBlockHeaderVerbose, GetBlockHeaderVerboseError, GetBlockVerboseOne,
        GetBlockVerboseOneError, GetBlockchainInfo, GetBlockchainInfoError, GetChainStates,
        GetChainStatesError, GetDescriptorActivity, GetDescriptorActivityError, GetDescriptorInfo,
        MempoolAcceptance, MempoolAcceptanceFees, NextBlockInfo, NextBlockInfoError,
        ReceiveActivity, SpendActivity, TestMempoolAccept,
    },
};
