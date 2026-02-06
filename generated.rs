// SPDX-License-Identifier: CC0-1.0

//! # JSON-RPC types for Bitcoin Core `30`
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

//! <details>
//! <summary> Methods from the == Blockchain == section </summary>
//!
//! | JSON-RPC Method Name               | Returns         | Notes                                  |
//! |:-----------------------------------|:---------------:|:--------------------------------------:|
//! | dumptxoutset                         | version (DumpTxoutSet) |                                        |
//! | getbestblockhash                     | returns string  |                                        |
//! | getblock                             | version (GetBlock) |                                        |
//! | getblockchaininfo                    | version (GetBlockchainInfo) |                                        |
//! | getblockcount                        | returns number  |                                        |
//! | getblockfilter                       | version (GetBlockFilter) |                                        |
//! | getblockfrompeer                     | version (GetBlockFromPeer) |                                        |
//! | getblockhash                         | returns string  |                                        |
//! | getblockheader                       | version (GetBlockHeader) |                                        |
//! | getblockstats                        | version (GetBlockstats) |                                        |
//! | getchainstates                       | version (GetChainstates) |                                        |
//! | getchaintips                         | version (GetChainTips) |                                        |
//! | getchaintxstats                      | version (GetChainTxstats) |                                        |
//! | getdeploymentinfo                    | version (GetDeploymentInfo) |                                        |
//! | getdescriptoractivity                | version (GetDescriptorActivity) |                                        |
//! | getdifficulty                        | returns number  |                                        |
//! | getmempoolancestors                  | version (GetMempoolAncestors) |                                        |
//! | getmempooldescendants                | version (GetMempoolDescendants) |                                        |
//! | getmempoolentry                      | version (GetMempoolEntry) |                                        |
//! | getmempoolinfo                       | version (GetMempoolInfo) |                                        |
//! | getrawmempool                        | version (GetRawMempool) |                                        |
//! | gettxout                             | version (GetTxout) |                                        |
//! | gettxoutproof                        | returns string  |                                        |
//! | gettxoutsetinfo                      | version (GetTxoutSetInfo) |                                        |
//! | gettxspendingprevout                 | version (GetTxspendingprevOut) |                                        |
//! | importmempool                        | version (ImportMempool) |                                        |
//! | loadtxoutset                         | version (LoadTxoutSet) |                                        |
//! | preciousblock                        | returns nothing |                                        |
//! | pruneblockchain                      | returns number  |                                        |
//! | savemempool                          | version (SaveMempool) |                                        |
//! | scanblocks                           | version (ScanBlocks) |                                        |
//! | scantxoutset                         | version (ScanTxoutSet) |                                        |
//! | verifychain                          | returns boolean |                                        |
//! | verifytxoutproof                     | version (VerifyTxoutProof) |                                        |
//! | waitforblock                         | version (WaitForBlock) |                                        |
//! | waitforblockheight                   | version (WaitForBlockHeight) |                                        |
//! | waitfornewblock                      | version (WaitForNewBlock) |                                        |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Control == section </summary>
//!
//! | JSON-RPC Method Name               | Returns         | Notes                                  |
//! |:-----------------------------------|:---------------:|:--------------------------------------:|
//! | api                                  | version (Api)   |                                        |
//! | getmemoryinfo                        | version (GetMemoryInfo) |                                        |
//! | getrpcinfo                           | version (GetRpcInfo) |                                        |
//! | help                                 | returns string  |                                        |
//! | logging                              | version (Logging) |                                        |
//! | stop                                 | returns string  |                                        |
//! | uptime                               | returns number  |                                        |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Hidden == section </summary>
//!
//! | JSON-RPC Method Name               | Returns         | Notes                                  |
//! |:-----------------------------------|:---------------:|:--------------------------------------:|
//! | addconnection                        | version (AddConnection) |                                        |
//! | addpeeraddress                       | version (AddPeerAddress) |                                        |
//! | echo                                 | version (Echo)  |                                        |
//! | echoipc                              | returns string  |                                        |
//! | echojson                             | version (EchoJson) |                                        |
//! | estimaterawfee                       | version (EstimateRawFee) |                                        |
//! | generate                             | version (Generate) |                                        |
//! | generateblock                        | version (GenerateBlock) |                                        |
//! | generatetoaddress                    | version (GenerateToAddress) |                                        |
//! | generatetodescriptor                 | version (GenerateToDescriptor) |                                        |
//! | getorphantxs                         | version (GetOrphanTxs) |                                        |
//! | getrawaddrman                        | version (GetRawAddrman) |                                        |
//! | invalidateblock                      | returns nothing |                                        |
//! | mockscheduler                        | returns nothing |                                        |
//! | reconsiderblock                      | returns nothing |                                        |
//! | sendmsgtopeer                        | version (SendmsgToPeer) |                                        |
//! | setmocktime                          | returns nothing |                                        |
//! | syncwithvalidationinterfacequeue     | returns nothing |                                        |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Mining == section </summary>
//!
//! | JSON-RPC Method Name               | Returns         | Notes                                  |
//! |:-----------------------------------|:---------------:|:--------------------------------------:|
//! | getblocktemplate                     | version (GetBlocktemplate) |                                        |
//! | getmininginfo                        | version (GetMiningInfo) |                                        |
//! | getnetworkhashps                     | returns number  |                                        |
//! | getprioritisedtransactions           | version (GetPrioritisedTransactions) |                                        |
//! | prioritisetransaction                | returns boolean |                                        |
//! | submitblock                          | version (SubmitBlock) |                                        |
//! | submitheader                         | returns nothing |                                        |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Network == section </summary>
//!
//! | JSON-RPC Method Name               | Returns         | Notes                                  |
//! |:-----------------------------------|:---------------:|:--------------------------------------:|
//! | addnode                              | returns nothing |                                        |
//! | clearbanned                          | returns nothing |                                        |
//! | disconnectnode                       | returns nothing |                                        |
//! | getaddednodeinfo                     | version (GetAddedNodeInfo) |                                        |
//! | getaddrmaninfo                       | version (GetAddrmanInfo) |                                        |
//! | getconnectioncount                   | returns number  |                                        |
//! | getnettotals                         | version (GetNetTotals) |                                        |
//! | getnetworkinfo                       | version (GetNetworkInfo) |                                        |
//! | getnodeaddresses                     | version (GetNodeAddresses) |                                        |
//! | getpeerinfo                          | version (GetPeerInfo) |                                        |
//! | listbanned                           | version (ListBanned) |                                        |
//! | ping                                 | returns nothing |                                        |
//! | setban                               | returns nothing |                                        |
//! | setnetworkactive                     | returns boolean |                                        |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == RawTransactions == section </summary>
//!
//! | JSON-RPC Method Name               | Returns         | Notes                                  |
//! |:-----------------------------------|:---------------:|:--------------------------------------:|
//! | analyzepsbt                          | version (AnalyzePsbt) |                                        |
//! | combinepsbt                          | returns string  |                                        |
//! | combinerawtransaction                | returns string  |                                        |
//! | converttopsbt                        | returns string  |                                        |
//! | createpsbt                           | returns string  |                                        |
//! | createrawtransaction                 | returns string  |                                        |
//! | decodepsbt                           | version (DecodePsbt) |                                        |
//! | decoderawtransaction                 | version (DecodeRawTransaction) |                                        |
//! | decodescript                         | version (DecodeScript) |                                        |
//! | descriptorprocesspsbt                | version (DescriptorProcessPsbt) |                                        |
//! | finalizepsbt                         | version (FinalizePsbt) |                                        |
//! | getrawtransaction                    | version (GetRawTransaction) |                                        |
//! | joinpsbts                            | returns string  |                                        |
//! | sendrawtransaction                   | returns string  |                                        |
//! | signrawtransactionwithkey            | version (SignRawTransactionwithKey) |                                        |
//! | submitpackage                        | version (SubmitPackage) |                                        |
//! | testmempoolaccept                    | version (TestMempoolaccept) |                                        |
//! | utxoupdatepsbt                       | returns string  |                                        |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Signer == section </summary>
//!
//! | JSON-RPC Method Name               | Returns         | Notes                                  |
//! |:-----------------------------------|:---------------:|:--------------------------------------:|
//! | enumeratesigners                     | version (EnumerateSigners) |                                        |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Util == section </summary>
//!
//! | JSON-RPC Method Name               | Returns         | Notes                                  |
//! |:-----------------------------------|:---------------:|:--------------------------------------:|
//! | createmultisig                       | version (CreateMultisig) |                                        |
//! | deriveaddresses                      | version (DeriveAddresses) |                                        |
//! | estimatesmartfee                     | version (EstimateSmartFee) |                                        |
//! | getdescriptorinfo                    | version (GetDescriptorInfo) |                                        |
//! | getindexinfo                         | version (GetIndexInfo) |                                        |
//! | signmessagewithprivkey               | returns string  |                                        |
//! | validateaddress                      | version (ValidateAddress) |                                        |
//! | verifymessage                        | returns boolean |                                        |
//!
//! </details>
//!

pub mod blockchain;
pub mod control;
pub mod hidden;
pub mod mining;
pub mod network;
pub mod raw_transactions;
pub mod signer;
pub mod util;

pub use self::blockchain::{DumpTxoutSet, GetBestBlockHash, GetBlockVerboseZero, GetBlockVerboseOne, GetBlockVerboseTwoTxItem, GetBlockVerboseTwo, GetBlockVerboseThreeTxItem, GetBlockVerboseThreeTxItemVinItem, GetBlockVerboseThreeTxItemVinItemPrevoutScriptPubKey, GetBlockVerboseThreeTxItemVinItemPrevout, GetBlockVerboseThree, GetBlockchainInfo, GetBlockCount, GetBlockFilter, GetBlockFromPeer, GetBlockHash, GetBlockHeaderVerboseZero, GetBlockHeaderVerboseOne, GetBlockstats, GetChainstates, GetChainstatesChainstatesItem, GetChainTips, GetChainTxstats, GetDeploymentInfo, GetDeploymentInfoDeployments, GetDeploymentInfoDeploymentsBip9Statistics, GetDeploymentInfoDeploymentsBip9, GetDescriptorActivity, GetDifficulty, GetMempoolAncestorsVerboseOne, GetMempoolDescendantsVerboseOne, GetMempoolEntry, GetMempoolEntryFees, GetMempoolInfo, GetRawMempoolVerboseOne, GetRawMempoolVerboseTwo, GetTxoutVerboseOne, GetTxoutVerboseOneScriptPubKey, GetTxoutProof, GetTxoutSetInfo, GetTxoutSetInfoBlockInfoUnspendables, GetTxoutSetInfoBlockInfo, GetTxspendingprevOut, ImportMempool, LoadTxoutSet, PruneBlockchain, SaveMempool, ScanBlocksVerboseOne, ScanBlocksVerboseTwo, ScanBlocksVerboseThree, ScanTxoutSetVerboseZero, ScanTxoutSetVerboseZeroUnspentsItem, ScanTxoutSetVerboseOne, ScanTxoutSetVerboseTwo, VerifyChain, VerifyTxoutProof, WaitForBlock, WaitForBlockHeight, WaitForNewBlock};
pub use self::control::{Api, GetMemoryInfoVerboseZero, GetMemoryInfoVerboseZeroLocked, GetMemoryInfoVerboseOne, GetRpcInfo, GetRpcInfoActiveCommandsItem, Help, Logging, Stop, Uptime};
pub use self::hidden::{AddConnection, AddPeerAddress, Echo, EchoIpc, EchoJson, EstimateRawFee, EstimateRawFeeLong, EstimateRawFeeMedium, EstimateRawFeeShortFail, EstimateRawFeeShortPass, EstimateRawFeeShort, Generate, GenerateBlock, GenerateToAddress, GenerateToDescriptor, GetRawAddrman, GetRawAddrmanEntryEntry, SendmsgToPeer};
pub use self::mining::{GetBlocktemplateVerboseOne, GetBlocktemplateVerboseTwoTransactionsItem, GetBlocktemplateVerboseTwo, GetMiningInfo, GetMiningInfoNext, GetNetworkHashps, GetPrioritisedTransactions, GetPrioritisedTransactionsEntry, PrioritiseTransaction, SubmitBlockVerboseOne};
pub use self::network::{GetAddedNodeInfo, GetAddrmanInfo, GetAddrmanInfoEntry, GetConnectionCount, GetNetTotals, GetNetTotalsUploadtarget, GetNetworkInfo, GetNetworkInfoLocaladdressesItem, GetNetworkInfoNetworksItem, GetNodeAddresses, GetPeerInfo, ListBanned, SetNetworkactive};
pub use self::raw_transactions::{AnalyzePsbt, AnalyzePsbtInputsItem, AnalyzePsbtInputsItemMissing, CombinePsbt, CombineRawTransaction, ConvertToPsbt, CreatePsbt, CreateRawTransaction, DecodePsbt, DecodePsbtGlobalXpubsItem, DecodePsbtInputsItem, DecodePsbtInputsItemBip32DerivsItem, DecodePsbtInputsItemFinalScriptSig, DecodePsbtInputsItemMusig2PartialSigsItem, DecodePsbtInputsItemMusig2ParticipantPubkeysItem, DecodePsbtInputsItemMusig2PubnoncesItem, DecodePsbtInputsItemNonWitnessUtxo, DecodePsbtInputsItemProprietaryItem, DecodePsbtInputsItemRedeemScript, DecodePsbtInputsItemTaprootBip32DerivsItem, DecodePsbtInputsItemTaprootScriptPathSigsItem, DecodePsbtInputsItemTaprootScriptsItem, DecodePsbtInputsItemWitnessScript, DecodePsbtInputsItemWitnessUtxoScriptPubKey, DecodePsbtInputsItemWitnessUtxo, DecodePsbtOutputsItem, DecodePsbtOutputsItemBip32DerivsItem, DecodePsbtOutputsItemMusig2ParticipantPubkeysItem, DecodePsbtOutputsItemProprietaryItem, DecodePsbtOutputsItemRedeemScript, DecodePsbtOutputsItemTaprootBip32DerivsItem, DecodePsbtOutputsItemTaprootTreeItem, DecodePsbtOutputsItemWitnessScript, DecodePsbtProprietaryItem, DecodePsbtTx, DecodeRawTransaction, DecodeRawTransactionVinItem, DecodeRawTransactionVinItemScriptSig, DecodeRawTransactionVoutItem, DecodeRawTransactionVoutItemScriptPubKey, DecodeScript, DecodeScriptSegwit, DescriptorProcessPsbt, FinalizePsbt, GetRawTransactionVerboseZero, GetRawTransactionVerboseOneVinItem, GetRawTransactionVerboseOneVinItemScriptSig, GetRawTransactionVerboseOneVoutItem, GetRawTransactionVerboseOneVoutItemScriptPubKey, GetRawTransactionVerboseOne, GetRawTransactionVerboseTwoVinItem, GetRawTransactionVerboseTwoVinItemPrevoutScriptPubKey, GetRawTransactionVerboseTwoVinItemPrevout, GetRawTransactionVerboseTwo, JoinPsbts, SendRawTransaction, SignRawTransactionwithKey, SignRawTransactionwithKeyErrorsItem, SubmitPackage, SubmitPackageTxResults, SubmitPackageTxResultsFees, TestMempoolaccept, UtxoUpdatePsbt};
pub use self::signer::{EnumerateSigners, EnumerateSignersSignersItem};
pub use self::util::{CreateMultisig, EstimateSmartFee, GetDescriptorInfo, GetIndexInfo, GetIndexInfoEntry, SignMessagewithPrivKey, ValidateAddress, VerifyMessage};

// ============ Generated Types ============


// --- blockchain ---

// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core - blockchain.
//!
//! Types for methods found under the `== Blockchain ==` section of the API docs.
//!
//! This file is auto-generated from OpenRPC specification.

use serde::{Deserialize, Serialize};

/// Write the serialized UTXO set to a file. This can be used in loadtxoutset afterwards if this snapshot height is supported in the chainparams as well.
/// 
/// Unless the "latest" type is requested, the node will roll back to the requested height and network activity will be suspended during this process. Because of this it is discouraged to interact with the node in any other way during the execution of this call to avoid inconsistent results and race conditions, particularly RPCs that interact with blockstorage.
/// 
/// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DumpTxoutSet {
    /// the hash of the base of the snapshot
    pub base_hash: String,
    /// the height of the base of the snapshot
    pub base_height: i64,
    /// the number of coins written in the snapshot
    pub coins_written: i64,
    /// the number of transactions in the chain up to and including the base block
    pub nchaintx: i64,
    /// the absolute path that the snapshot was written to
    pub path: String,
    /// the hash of the UTXO set contents
    pub txoutset_hash: String,
}

/// Result of the JSON-RPC method `getbestblockhash`.
///
/// > getbestblockhash
/// >
/// > Returns the hash of the best (tip) block in the most-work fully-validated chain.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBestBlockHash(pub String);

/// Result of the JSON-RPC method `getblockcount`.
///
/// > getblockcount
/// >
/// > Returns the height of the most-work fully-validated chain.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockCount(pub i64);

/// Retrieve a BIP 157 content filter for a particular block.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockFilter {
    /// the hex-encoded filter data
    pub filter: String,
    /// the hex-encoded filter header
    pub header: String,
}

/// Attempt to fetch block from a given peer.
/// 
/// We must have the header for this block, e.g. using submitheader.
/// The block will not have any undo data which can limit the usage of the block data in a context where the undo data is needed.
/// Subsequent calls for the same block may cause the response from the previous peer to be ignored.
/// Peers generally ignore requests for a stale block that they never fully verified, or one that is more than a month old.
/// When a peer does not respond with a block, we will disconnect.
/// Note: The block could be re-pruned as soon as it is received.
/// 
/// Returns an empty JSON object if the request was successfully scheduled.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockFromPeer {

}

/// Result of the JSON-RPC method `getblockhash`.
///
/// > getblockhash
/// >
/// > Returns hash of block in best-block-chain at height provided.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockHash(pub String);

/// If verbose is false, returns a string that is serialized, hex-encoded data for blockheader 'hash'.
/// If verbose is true, returns an Object with information about blockheader <hash>.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockHeaderVerboseOne(pub String);

/// If verbose is false, returns a string that is serialized, hex-encoded data for blockheader 'hash'.
/// If verbose is true, returns an Object with information about blockheader <hash>.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockHeaderVerboseZero {
    /// nBits: compact representation of the block difficulty target
    pub bits: String,
    /// Expected number of hashes required to produce the current chain
    pub chainwork: String,
    /// The number of confirmations, or -1 if the block is not on the main chain
    pub confirmations: i64,
    /// The difficulty
    pub difficulty: i64,
    /// the block hash (same as provided)
    pub hash: String,
    /// The block height or index
    pub height: i64,
    /// The median block time expressed in UNIX epoch time
    pub mediantime: i64,
    /// The merkle root
    pub merkleroot: String,
    /// The number of transactions in the block
    #[serde(rename = "nTx")]
    pub n_tx: i64,
    /// The hash of the next block (if available)
    #[serde(rename = "nextblockhash")]
    pub next_block_hash: Option<String>,
    /// The nonce
    pub nonce: i64,
    /// The hash of the previous block (if available)
    #[serde(rename = "previousblockhash")]
    pub previous_block_hash: Option<String>,
    /// The difficulty target
    pub target: String,
    /// The block time expressed in UNIX epoch time
    pub time: i64,
    /// The block version
    pub version: i64,
    /// The block version formatted in hexadecimal
    #[serde(rename = "versionHex")]
    pub version_hex: String,
}

/// If verbosity is 0, returns a string that is serialized, hex-encoded data for block 'hash'.
/// If verbosity is 1, returns an Object with information about block <hash>.
/// If verbosity is 2, returns an Object with information about block <hash> and information about each transaction.
/// If verbosity is 3, returns an Object with information about block <hash> and information about each transaction, including prevout information for inputs (only for unpruned blocks in the current best chain).
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockVerboseOne {
    /// nBits: compact representation of the block difficulty target
    pub bits: String,
    /// Expected number of hashes required to produce the chain up to this block (in hex)
    pub chainwork: String,
    /// The number of confirmations, or -1 if the block is not on the main chain
    pub confirmations: i64,
    /// The difficulty
    pub difficulty: i64,
    /// the block hash (same as provided)
    pub hash: String,
    /// The block height or index
    pub height: i64,
    /// The median block time expressed in UNIX epoch time
    pub mediantime: i64,
    /// The merkle root
    pub merkleroot: String,
    /// The number of transactions in the block
    #[serde(rename = "nTx")]
    pub n_tx: i64,
    /// The hash of the next block (if available)
    #[serde(rename = "nextblockhash")]
    pub next_block_hash: Option<String>,
    /// The nonce
    pub nonce: i64,
    /// The hash of the previous block (if available)
    #[serde(rename = "previousblockhash")]
    pub previous_block_hash: Option<String>,
    /// The block size
    pub size: i64,
    /// The block size excluding witness data
    #[serde(rename = "strippedsize")]
    pub stripped_size: i64,
    /// The difficulty target
    pub target: String,
    /// The block time expressed in UNIX epoch time
    pub time: i64,
    /// The transaction ids
    pub tx: Vec<String>,
    /// The block version
    pub version: i64,
    /// The block version formatted in hexadecimal
    #[serde(rename = "versionHex")]
    pub version_hex: String,
    /// The block weight as defined in BIP 141
    pub weight: i64,
}

/// If verbosity is 0, returns a string that is serialized, hex-encoded data for block 'hash'.
/// If verbosity is 1, returns an Object with information about block <hash>.
/// If verbosity is 2, returns an Object with information about block <hash> and information about each transaction.
/// If verbosity is 3, returns an Object with information about block <hash> and information about each transaction, including prevout information for inputs (only for unpruned blocks in the current best chain).
/// [TODO] this is a commentary from documentation explaining what this field is supposed be: 
/// commentary: Same output as verbosity = 2
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockVerboseThree {
    pub tx: Vec<GetBlockVerboseThreeTxItem>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockVerboseThreeTxItem {
    pub vin: Vec<GetBlockVerboseThreeTxItemVinItem>,
}

/// [TODO] this is a commentary from documentation explaining what this field is supposed be: 
/// commentary: The same output as verbosity = 2
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockVerboseThreeTxItemVinItem {
    /// (Only if undo information is available)
    #[serde(rename = "prevout")]
    pub prev_out: GetBlockVerboseThreeTxItemVinItemPrevout,
}

/// (Only if undo information is available)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockVerboseThreeTxItemVinItemPrevout {
    /// Coinbase or not
    pub generated: bool,
    /// The height of the prevout
    pub height: i64,
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: GetBlockVerboseThreeTxItemVinItemPrevoutScriptPubKey,
    /// The value in BTC
    pub value: f64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockVerboseThreeTxItemVinItemPrevoutScriptPubKey {
    /// The Bitcoin address (only if a well-defined address exists)
    pub address: Option<String>,
    /// Disassembly of the output script
    pub asm: String,
    /// Inferred descriptor for the output
    pub desc: String,
    /// The raw output script bytes, hex-encoded
    pub hex: String,
    /// The type (one of: nonstandard, anchor, pubkey, pubkeyhash, scripthash, multisig, nulldata, witness_v0_scripthash, witness_v0_keyhash, witness_v1_taproot, witness_unknown)
    #[serde(rename = "type")]
    pub type_: String,
}

/// If verbosity is 0, returns a string that is serialized, hex-encoded data for block 'hash'.
/// If verbosity is 1, returns an Object with information about block <hash>.
/// If verbosity is 2, returns an Object with information about block <hash> and information about each transaction.
/// If verbosity is 3, returns an Object with information about block <hash> and information about each transaction, including prevout information for inputs (only for unpruned blocks in the current best chain).
/// [TODO] this is a commentary from documentation explaining what this field is supposed be: 
/// commentary: Same output as verbosity = 1
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockVerboseTwo {
    pub tx: Vec<GetBlockVerboseTwoTxItem>,
}

/// [TODO] this is a commentary from documentation explaining what this field is supposed be: 
/// commentary: The transactions in the format of the getrawtransaction RPC. Different from verbosity = 1 "tx" result
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockVerboseTwoTxItem {
    /// The transaction fee in BTC, omitted if block undo data is not available
    pub fee: i64,
}

/// If verbosity is 0, returns a string that is serialized, hex-encoded data for block 'hash'.
/// If verbosity is 1, returns an Object with information about block <hash>.
/// If verbosity is 2, returns an Object with information about block <hash> and information about each transaction.
/// If verbosity is 3, returns an Object with information about block <hash> and information about each transaction, including prevout information for inputs (only for unpruned blocks in the current best chain).
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockVerboseZero(pub String);

/// Returns an object containing various state info regarding blockchain processing.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockchainInfo {
    /// whether automatic pruning is enabled (only present if pruning is enabled)
    pub automatic_pruning: Option<bool>,
    /// the hash of the currently best block
    #[serde(rename = "bestblockhash")]
    pub bestblock_hash: String,
    /// nBits: compact representation of the block difficulty target
    pub bits: String,
    /// the height of the most-work fully-validated chain. The genesis block has height 0
    pub blocks: i64,
    /// current network name (main, test, testnet4, signet, regtest)
    pub chain: String,
    /// total amount of work in active chain, in hexadecimal
    pub chainwork: String,
    /// the current difficulty
    pub difficulty: i64,
    /// the current number of headers we have validated
    pub headers: i64,
    /// (debug information) estimate of whether this node is in Initial Block Download mode
    #[serde(rename = "initialblockdownload")]
    pub initial_block_download: bool,
    /// The median block time expressed in UNIX epoch time
    pub mediantime: i64,
    /// the target size used by pruning (only present if automatic pruning is enabled)
    pub prune_target_size: Option<i64>,
    /// if the blocks are subject to pruning
    pub pruned: bool,
    /// height of the last block pruned, plus one (only present if pruning is enabled)
    pub pruneheight: Option<i64>,
    /// the block challenge (aka. block script), in hexadecimal (only present if the current network is a signet)
    pub signet_challenge: Option<String>,
    /// the estimated size of the block and undo files on disk
    pub size_on_disk: i64,
    /// The difficulty target
    pub target: String,
    /// The block time expressed in UNIX epoch time
    pub time: i64,
    /// estimate of verification progress [0..1]
    #[serde(rename = "verificationprogress")]
    pub verification_progress: i64,
    /// any network and blockchain warnings (run with `-deprecatedrpc=warnings` to return the latest warning as a single string)
    pub warnings: Vec<String>,
}

/// Compute per block statistics for a given window. All amounts are in satoshis.
/// It won't work for some heights with pruning.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockstats {
    /// Average fee in the block
    pub avgfee: Option<i64>,
    /// Average feerate (in satoshis per virtual byte)
    pub avgfeerate: Option<i64>,
    /// Average transaction size
    pub avgtxsize: Option<i64>,
    /// The block hash (to check for potential reorgs)
    #[serde(rename = "blockhash")]
    pub block_hash: Option<String>,
    /// Feerates at the 10th, 25th, 50th, 75th, and 90th percentile weight unit (in satoshis per virtual byte)
    pub feerate_percentiles: Option<Vec<serde_json::Value>>,
    /// The height of the block
    pub height: Option<i64>,
    /// The number of inputs (excluding coinbase)
    pub ins: Option<i64>,
    /// Maximum fee in the block
    #[serde(rename = "maxfee")]
    pub max_fee: Option<i64>,
    /// Maximum feerate (in satoshis per virtual byte)
    #[serde(rename = "maxfeerate")]
    pub max_feerate: Option<i64>,
    /// Maximum transaction size
    #[serde(rename = "maxtxsize")]
    pub max_txs_ize: Option<i64>,
    /// Truncated median fee in the block
    pub medianfee: Option<i64>,
    /// The block median time past
    pub mediantime: Option<i64>,
    /// Truncated median transaction size
    pub mediantxsize: Option<i64>,
    /// Minimum fee in the block
    #[serde(rename = "minfee")]
    pub min_fee: Option<i64>,
    /// Minimum feerate (in satoshis per virtual byte)
    #[serde(rename = "minfeerate")]
    pub min_feerate: Option<i64>,
    /// Minimum transaction size
    #[serde(rename = "mintxsize")]
    pub min_txs_ize: Option<i64>,
    /// The number of outputs
    #[serde(rename = "outs")]
    pub out_s: Option<i64>,
    /// The block subsidy
    pub subsidy: Option<i64>,
    /// Total size of all segwit transactions
    pub swtotal_size: Option<i64>,
    /// Total weight of all segwit transactions
    pub swtotal_weight: Option<i64>,
    /// The number of segwit transactions
    pub swtxs: Option<i64>,
    /// The block time
    pub time: Option<i64>,
    /// Total amount in all outputs (excluding coinbase and thus reward [ie subsidy + totalfee])
    pub total_out: Option<i64>,
    /// Total size of all non-coinbase transactions
    pub total_size: Option<i64>,
    /// Total weight of all non-coinbase transactions
    pub total_weight: Option<i64>,
    /// The fee total
    #[serde(rename = "totalfee")]
    pub total_fee: Option<i64>,
    /// The number of transactions (including coinbase)
    pub txs: Option<i64>,
    /// The increase/decrease in the number of unspent outputs (not discounting op_return and similar)
    pub utxo_increase: Option<i64>,
    /// The increase/decrease in the number of unspent outputs, not counting unspendables
    pub utxo_increase_actual: Option<i64>,
    /// The increase/decrease in size for the utxo index (not discounting op_return and similar)
    pub utxo_size_inc: Option<i64>,
    /// The increase/decrease in size for the utxo index, not counting unspendables
    pub utxo_size_inc_actual: Option<i64>,
}

/// Return information about all known tips in the block tree, including the main chain as well as orphaned branches.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetChainTips {

}

/// Compute statistics about the total number and rate of transactions in the chain.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetChainTxstats {
    /// The timestamp for the final block in the window, expressed in UNIX epoch time
    pub time: i64,
    /// The total number of transactions in the chain up to that point, if known. It may be unknown when using assumeutxo.
    #[serde(rename = "txcount")]
    pub tx_count: Option<i64>,
    /// The average rate of transactions per second in the window. Only returned if "window_interval" is > 0 and if window_tx_count exists.
    #[serde(rename = "txrate")]
    pub tx_rate: Option<i64>,
    /// Size of the window in number of blocks
    pub window_block_count: i64,
    /// The hash of the final block in the window
    pub window_final_block_hash: String,
    /// The height of the final block in the window.
    pub window_final_block_height: i64,
    /// The elapsed time in the window in seconds. Only returned if "window_block_count" is > 0
    pub window_interval: Option<i64>,
    /// The number of transactions in the window. Only returned if "window_block_count" is > 0 and if txcount exists for the start and end of the window.
    pub window_tx_count: Option<i64>,
}

/// Return information about chainstates.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetChainstates {
    /// list of the chainstates ordered by work, with the most-work (active) chainstate last
    pub chainstates: Vec<GetChainstatesChainstatesItem>,
    /// the number of headers seen so far
    pub headers: i64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetChainstatesChainstatesItem {
    /// blockhash of the tip
    #[serde(rename = "bestblockhash")]
    pub bestblock_hash: String,
    /// nBits: compact representation of the block difficulty target
    pub bits: String,
    /// number of blocks in this chainstate
    pub blocks: i64,
    /// size of the coinsdb cache
    pub coins_db_cache_bytes: i64,
    /// size of the coinstip cache
    pub coins_tip_cache_bytes: i64,
    /// difficulty of the tip
    pub difficulty: i64,
    /// the base block of the snapshot this chainstate is based on, if any
    pub snapshot_blockhash: Option<String>,
    /// The difficulty target
    pub target: String,
    /// whether the chainstate is fully validated. True if all blocks in the chainstate were validated, false if the chain is based on a snapshot and the snapshot has not yet been validated.
    pub validated: bool,
    /// progress towards the network tip
    #[serde(rename = "verificationprogress")]
    pub verification_progress: i64,
}

/// Returns an object containing various state info regarding deployments of consensus changes.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetDeploymentInfo {
    pub deployments: std::collections::BTreeMap<String, GetDeploymentInfoDeployments>,
    /// requested block hash (or tip)
    pub hash: String,
    /// requested block height (or tip)
    pub height: i64,
}

/// name of the deployment
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetDeploymentInfoDeployments {
    /// true if the rules are enforced for the mempool and the next block
    pub active: bool,
    /// status of bip9 softforks (only for "bip9" type)
    pub bip9: Option<GetDeploymentInfoDeploymentsBip9>,
    /// height of the first block which the rules are or will be enforced (only for "buried" type, or "bip9" type with "active" status)
    pub height: Option<i64>,
    /// one of "buried", "bip9"
    #[serde(rename = "type")]
    pub type_: String,
}

/// status of bip9 softforks (only for "bip9" type)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetDeploymentInfoDeploymentsBip9 {
    /// the bit (0-28) in the block version field used to signal this softfork (only for "started" and "locked_in" status)
    pub bit: Option<i64>,
    /// minimum height of blocks for which the rules may be enforced
    pub min_activation_height: i64,
    /// indicates blocks that signalled with a # and blocks that did not with a -
    pub signalling: Option<String>,
    /// height of the first block to which the status applies
    pub since: i64,
    /// the minimum median time past of a block at which the bit gains its meaning
    pub start_time: i64,
    /// numeric statistics about signalling for a softfork (only for "started" and "locked_in" status)
    pub statistics: Option<GetDeploymentInfoDeploymentsBip9Statistics>,
    /// status of deployment at specified block (one of "defined", "started", "locked_in", "active", "failed")
    pub status: String,
    /// status of deployment at the next block
    pub status_next: String,
    /// the median time past of a block at which the deployment is considered failed if not yet locked in
    #[serde(rename = "timeout")]
    pub time_out: i64,
}

/// numeric statistics about signalling for a softfork (only for "started" and "locked_in" status)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetDeploymentInfoDeploymentsBip9Statistics {
    /// the number of blocks with the version bit set in the current period
    pub count: i64,
    /// the number of blocks elapsed since the beginning of the current period
    pub elapsed: i64,
    /// the length in blocks of the signalling period
    pub period: i64,
    /// returns false if there are not enough blocks left in this period to pass activation threshold (only for "started" status)
    pub possible: Option<bool>,
    /// the number of blocks with the version bit set required to activate the feature (only for "started" status)
    pub threshold: Option<i64>,
}

/// Get spend and receive activity associated with a set of descriptors for a set of blocks. This command pairs well with the `relevant_blocks` output of `scanblocks()`.
/// This call may take several minutes. If you encounter timeouts, try specifying no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetDescriptorActivity {
    /// events
    pub activity: Vec<serde_json::Value>,
}

/// Result of the JSON-RPC method `getdifficulty`.
///
/// > getdifficulty
/// >
/// > Returns the proof-of-work difficulty as a multiple of the minimum difficulty.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetDifficulty(pub i64);

/// If txid is in the mempool, returns all in-mempool ancestors.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolAncestorsVerboseOne {

}

/// If txid is in the mempool, returns all in-mempool descendants.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolDescendantsVerboseOne {

}

/// Returns mempool data for given transaction
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolEntry {
    /// number of in-mempool ancestor transactions (including this one)
    #[serde(rename = "ancestorcount")]
    pub ancestor_count: i64,
    /// virtual transaction size of in-mempool ancestors (including this one)
    #[serde(rename = "ancestorsize")]
    pub ancestors_ize: i64,
    /// Whether this transaction signals BIP125 replaceability or has an unconfirmed ancestor signaling BIP125 replaceability. (DEPRECATED)
/// 
    #[serde(rename = "bip125-replaceable")]
    pub bip125_replaceable: bool,
    /// unconfirmed transactions used as inputs for this transaction
    pub depends: Vec<String>,
    /// number of in-mempool descendant transactions (including this one)
    #[serde(rename = "descendantcount")]
    pub descendant_count: i64,
    /// virtual transaction size of in-mempool descendants (including this one)
    #[serde(rename = "descendantsize")]
    pub descendants_ize: i64,
    pub fees: GetMempoolEntryFees,
    /// block height when transaction entered pool
    pub height: i64,
    /// unconfirmed transactions spending outputs from this transaction
    pub spentby: Vec<String>,
    /// local time transaction entered pool in seconds since 1 Jan 1970 GMT
    pub time: i64,
    /// Whether this transaction is currently unbroadcast (initial broadcast not yet acknowledged by any peers)
    pub unbroadcast: bool,
    /// virtual transaction size as defined in BIP 141. This is different from actual serialized size for witness transactions as witness data is discounted.
    pub vsize: i64,
    /// transaction weight as defined in BIP 141.
    pub weight: i64,
    /// hash of serialized transaction, including witness data
    pub wtxid: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolEntryFees {
    /// transaction fees of in-mempool ancestors (including this one) with fee deltas used for mining priority, denominated in BTC
    pub ancestor: f64,
    /// transaction fee, denominated in BTC
    pub base: f64,
    /// transaction fees of in-mempool descendants (including this one) with fee deltas used for mining priority, denominated in BTC
    pub descendant: f64,
    /// transaction fee with fee deltas used for mining priority, denominated in BTC
    pub modified: f64,
}

/// Returns details on the active state of the TX memory pool.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolInfo {
    /// Sum of all virtual transaction sizes as defined in BIP 141. Differs from actual serialized size because witness data is discounted
    pub bytes: i64,
    /// True if the mempool accepts RBF without replaceability signaling inspection (DEPRECATED)
    #[serde(rename = "fullrbf")]
    pub full_rbf: bool,
    /// minimum fee rate increment for mempool limiting or replacement in BTC/kvB
    #[serde(rename = "incrementalrelayfee")]
    pub incremental_relay_fee: i64,
    /// True if the initial load attempt of the persisted mempool finished
    pub loaded: bool,
    /// Maximum number of bytes that can be used by OP_RETURN outputs in the mempool
    #[serde(rename = "maxdatacarriersize")]
    pub max_data_carrier_size: i64,
    /// Maximum memory usage for the mempool
    #[serde(rename = "maxmempool")]
    pub max_mempool: i64,
    /// Minimum fee rate in BTC/kvB for tx to be accepted. Is the maximum of minrelaytxfee and minimum mempool fee
    #[serde(rename = "mempoolminfee")]
    pub mempool_min_fee: f64,
    /// Current minimum relay fee for transactions
    #[serde(rename = "minrelaytxfee")]
    pub min_relay_tx_fee: f64,
    /// True if the mempool accepts transactions with bare multisig outputs
    #[serde(rename = "permitbaremultisig")]
    pub permit_bare_multisig: bool,
    /// Current tx count
    pub size: i64,
    /// Total fees for the mempool in BTC, ignoring modified fees through prioritisetransaction
    pub total_fee: f64,
    /// Current number of transactions that haven't passed initial broadcast yet
    #[serde(rename = "unbroadcastcount")]
    pub unbroadcast_count: i64,
    /// Total memory usage for the mempool
    pub usage: i64,
}

/// Returns all transaction ids in memory pool as a json array of string transaction ids.
/// 
/// Hint: use getmempoolentry to fetch a specific transaction from the mempool.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawMempoolVerboseOne {

}

/// Returns all transaction ids in memory pool as a json array of string transaction ids.
/// 
/// Hint: use getmempoolentry to fetch a specific transaction from the mempool.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawMempoolVerboseTwo {
    /// The mempool sequence value.
    pub mempool_sequence: i64,
    pub txids: Vec<String>,
}

/// Result of the JSON-RPC method `gettxoutproof`.
///
/// > gettxoutproof
/// >
/// > Returns a hex-encoded proof that "txid" was included in a block.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTxoutProof(pub String);

/// Returns statistics about the unspent transaction output set.
/// Note this call may take some time if you are not using coinstatsindex.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTxoutSetInfo {
    /// The hash of the block at which these statistics are calculated
    pub bestblock: String,
    /// Info on amounts in the block at this block height (only available if coinstatsindex is used)
    pub block_info: Option<GetTxoutSetInfoBlockInfo>,
    /// Database-independent, meaningless metric indicating the UTXO set size
    pub bogosize: i64,
    /// The estimated size of the chainstate on disk (not available when coinstatsindex is used)
    pub disk_size: Option<i64>,
    /// The serialized hash (only present if 'hash_serialized_3' hash_type is chosen)
    pub hash_serialized_3: Option<String>,
    /// The block height (index) of the returned statistics
    pub height: i64,
    /// The serialized hash (only present if 'muhash' hash_type is chosen)
    pub muhash: Option<String>,
    /// The total amount of coins in the UTXO set
    pub total_amount: f64,
    /// The total amount of coins permanently excluded from the UTXO set (only available if coinstatsindex is used)
    pub total_unspendable_amount: Option<f64>,
    /// The number of transactions with unspent outputs (not available when coinstatsindex is used)
    pub transactions: Option<i64>,
    /// The number of unspent transaction outputs
    #[serde(rename = "txouts")]
    pub txout_s: i64,
}

/// Info on amounts in the block at this block height (only available if coinstatsindex is used)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTxoutSetInfoBlockInfo {
    /// Coinbase subsidy amount of this block
    #[serde(rename = "coinbase")]
    pub coin_base: f64,
    /// Total amount of new outputs created by this block
    pub new_outputs_ex_coinbase: f64,
    /// Total amount of all prevouts spent in this block
    pub prevout_spent: f64,
    /// Total amount of unspendable outputs created in this block
    pub unspendable: f64,
    /// Detailed view of the unspendable categories
    pub unspendables: GetTxoutSetInfoBlockInfoUnspendables,
}

/// Detailed view of the unspendable categories
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTxoutSetInfoBlockInfoUnspendables {
    /// Transactions overridden by duplicates (no longer possible with BIP30)
    pub bip30: f64,
    /// The unspendable amount of the Genesis block subsidy
    pub genesis_block: f64,
    /// Amounts sent to scripts that are unspendable (for example OP_RETURN outputs)
    pub scripts: f64,
    /// Fee rewards that miners did not claim in their coinbase transaction
    pub unclaimed_rewards: f64,
}

/// Returns details about an unspent transaction output.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTxoutVerboseOne {
    /// The hash of the block at the tip of the chain
    pub bestblock: String,
    /// Coinbase or not
    #[serde(rename = "coinbase")]
    pub coin_base: bool,
    /// The number of confirmations
    pub confirmations: i64,
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: GetTxoutVerboseOneScriptPubKey,
    /// The transaction value in BTC
    pub value: f64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTxoutVerboseOneScriptPubKey {
    /// The Bitcoin address (only if a well-defined address exists)
    pub address: Option<String>,
    /// Disassembly of the output script
    pub asm: String,
    /// Inferred descriptor for the output
    pub desc: String,
    /// The raw output script bytes, hex-encoded
    pub hex: String,
    /// The type, eg pubkeyhash
    #[serde(rename = "type")]
    pub type_: String,
}

/// Scans the mempool to find transactions spending any of the given outputs
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTxspendingprevOut {

}

/// Import a mempool.dat file and attempt to add its contents to the mempool.
/// Warning: Importing untrusted files is dangerous, especially if metadata from the file is taken over.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ImportMempool {

}

/// Load the serialized UTXO set from a file.
/// Once this snapshot is loaded, its contents will be deserialized into a second chainstate data structure, which is then used to sync to the network's tip. Meanwhile, the original chainstate will complete the initial block download process in the background, eventually validating up to the block that the snapshot is based upon.
/// 
/// The result is a usable bitcoind instance that is current with the network tip in a matter of minutes rather than hours. UTXO snapshot are typically obtained from third-party sources (HTTP, torrent, etc.) which is reasonable since their contents are always checked by hash.
/// 
/// You can find more information on this process in the `assumeutxo` design document (<https://github.com/bitcoin/bitcoin/blob/master/doc/design/assumeutxo.md>).
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct LoadTxoutSet {
    /// the height of the base of the snapshot
    pub base_height: i64,
    /// the number of coins loaded from the snapshot
    pub coins_loaded: i64,
    /// the absolute path that the snapshot was loaded from
    pub path: String,
    /// the hash of the base of the snapshot
    pub tip_hash: String,
}

/// Result of the JSON-RPC method `pruneblockchain`.
///
/// > pruneblockchain
/// >
/// > Attempts to delete block and undo data up to a specified height or timestamp, if eligible for pruning.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct PruneBlockchain(pub i64);

/// Dumps the mempool to disk. It will fail until the previous dump is fully loaded.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SaveMempool {
    /// the directory and file where the mempool was saved
    pub filename: String,
}

/// Return relevant blockhashes for given descriptors (requires blockfilterindex).
/// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ScanBlocksVerboseOne {
    /// true if the scan process was not aborted
    pub completed: bool,
    /// The height we started the scan from
    pub from_height: i64,
    /// Blocks that may have matched a scanobject.
    pub relevant_blocks: Vec<String>,
    /// The height we ended the scan at
    pub to_height: i64,
}

/// Return relevant blockhashes for given descriptors (requires blockfilterindex).
/// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ScanBlocksVerboseThree(pub bool);

/// Return relevant blockhashes for given descriptors (requires blockfilterindex).
/// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ScanBlocksVerboseTwo {
    /// Height of the block currently being scanned
    pub current_height: i64,
    /// Approximate percent complete
    pub progress: i64,
}

/// Scans the unspent transaction output set for entries that match certain output descriptors.
/// Examples of output descriptors are:
///     addr(<address>)                      Outputs whose output script corresponds to the specified address (does not include P2PK)
///     raw(<hex script>)                    Outputs whose output script equals the specified hex-encoded bytes
///     combo(<pubkey>)                      P2PK, P2PKH, P2WPKH, and P2SH-P2WPKH outputs for the given pubkey
///     pkh(<pubkey>)                        P2PKH outputs for the given pubkey
///     sh(multi(<n>,<pubkey>,<pubkey>,...)) P2SH-multisig outputs for the given threshold and pubkeys
///     tr(<pubkey>)                         P2TR
///     tr(<pubkey>,{pk(<pubkey>)})          P2TR with single fallback pubkey in tapscript
///     rawtr(<pubkey>)                      P2TR with the specified key as output key rather than inner
///     wsh(and_v(v:pk(<pubkey>),after(2)))  P2WSH miniscript with mandatory pubkey and a timelock
/// 
/// In the above, <pubkey> either refers to a fixed public key in hexadecimal notation, or to an xpub/xprv optionally followed by one
/// or more path elements separated by "/", and optionally ending in "/*" (unhardened), or "/*'" or "/*h" (hardened) to specify all
/// unhardened or hardened child keys.
/// In the latter case, a range needs to be specified by below if different from 1000.
/// For more information on output descriptors, see the documentation in the doc/descriptors.md file.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ScanTxoutSetVerboseOne(pub bool);

/// Scans the unspent transaction output set for entries that match certain output descriptors.
/// Examples of output descriptors are:
///     addr(<address>)                      Outputs whose output script corresponds to the specified address (does not include P2PK)
///     raw(<hex script>)                    Outputs whose output script equals the specified hex-encoded bytes
///     combo(<pubkey>)                      P2PK, P2PKH, P2WPKH, and P2SH-P2WPKH outputs for the given pubkey
///     pkh(<pubkey>)                        P2PKH outputs for the given pubkey
///     sh(multi(<n>,<pubkey>,<pubkey>,...)) P2SH-multisig outputs for the given threshold and pubkeys
///     tr(<pubkey>)                         P2TR
///     tr(<pubkey>,{pk(<pubkey>)})          P2TR with single fallback pubkey in tapscript
///     rawtr(<pubkey>)                      P2TR with the specified key as output key rather than inner
///     wsh(and_v(v:pk(<pubkey>),after(2)))  P2WSH miniscript with mandatory pubkey and a timelock
/// 
/// In the above, <pubkey> either refers to a fixed public key in hexadecimal notation, or to an xpub/xprv optionally followed by one
/// or more path elements separated by "/", and optionally ending in "/*" (unhardened), or "/*'" or "/*h" (hardened) to specify all
/// unhardened or hardened child keys.
/// In the latter case, a range needs to be specified by below if different from 1000.
/// For more information on output descriptors, see the documentation in the doc/descriptors.md file.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ScanTxoutSetVerboseTwo {
    /// Approximate percent complete
    pub progress: i64,
}

/// Scans the unspent transaction output set for entries that match certain output descriptors.
/// Examples of output descriptors are:
///     addr(<address>)                      Outputs whose output script corresponds to the specified address (does not include P2PK)
///     raw(<hex script>)                    Outputs whose output script equals the specified hex-encoded bytes
///     combo(<pubkey>)                      P2PK, P2PKH, P2WPKH, and P2SH-P2WPKH outputs for the given pubkey
///     pkh(<pubkey>)                        P2PKH outputs for the given pubkey
///     sh(multi(<n>,<pubkey>,<pubkey>,...)) P2SH-multisig outputs for the given threshold and pubkeys
///     tr(<pubkey>)                         P2TR
///     tr(<pubkey>,{pk(<pubkey>)})          P2TR with single fallback pubkey in tapscript
///     rawtr(<pubkey>)                      P2TR with the specified key as output key rather than inner
///     wsh(and_v(v:pk(<pubkey>),after(2)))  P2WSH miniscript with mandatory pubkey and a timelock
/// 
/// In the above, <pubkey> either refers to a fixed public key in hexadecimal notation, or to an xpub/xprv optionally followed by one
/// or more path elements separated by "/", and optionally ending in "/*" (unhardened), or "/*'" or "/*h" (hardened) to specify all
/// unhardened or hardened child keys.
/// In the latter case, a range needs to be specified by below if different from 1000.
/// For more information on output descriptors, see the documentation in the doc/descriptors.md file.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ScanTxoutSetVerboseZero {
    /// The hash of the block at the tip of the chain
    pub bestblock: String,
    /// The block height at which the scan was done
    pub height: i64,
    /// Whether the scan was completed
    pub success: bool,
    /// The total amount of all found unspent outputs in BTC
    pub total_amount: f64,
    /// The number of unspent transaction outputs scanned
    #[serde(rename = "txouts")]
    pub txout_s: i64,
    #[serde(rename = "unspents")]
    pub unspent_s: Vec<ScanTxoutSetVerboseZeroUnspentsItem>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ScanTxoutSetVerboseZeroUnspentsItem {
    /// The total amount in BTC of the unspent output
    pub amount: f64,
    /// Blockhash of the unspent transaction output
    #[serde(rename = "blockhash")]
    pub block_hash: String,
    /// Whether this is a coinbase output
    #[serde(rename = "coinbase")]
    pub coin_base: bool,
    /// Number of confirmations of the unspent transaction output when the scan was done
    pub confirmations: i64,
    /// A specialized descriptor for the matched output script
    pub desc: String,
    /// Height of the unspent transaction output
    pub height: i64,
    /// The output script
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: String,
    /// The transaction id
    pub txid: String,
    /// The vout value
    pub vout: i64,
}

/// Result of the JSON-RPC method `verifychain`.
///
/// > verifychain
/// >
/// > Verifies blockchain database.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct VerifyChain(pub bool);

/// Verifies that a proof points to a transaction in a block, returning the transaction it commits to
/// and throwing an RPC error if the block is not in our best chain
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct VerifyTxoutProof {

}

/// Waits for a specific new block and returns useful info about it.
/// 
/// Returns the current block on timeout or exit.
/// 
/// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct WaitForBlock {
    /// The blockhash
    pub hash: String,
    /// Block height
    pub height: i64,
}

/// Waits for (at least) block height and returns the height and hash
/// of the current tip.
/// 
/// Returns the current block on timeout or exit.
/// 
/// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct WaitForBlockHeight {
    /// The blockhash
    pub hash: String,
    /// Block height
    pub height: i64,
}

/// Waits for any new block and returns useful info about it.
/// 
/// Returns the current block on timeout or exit.
/// 
/// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct WaitForNewBlock {
    /// The blockhash
    pub hash: String,
    /// Block height
    pub height: i64,
}


// --- control ---

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
pub struct GetMemoryInfoVerboseOne(pub String);

/// Returns an object containing information about memory usage.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMemoryInfoVerboseZero {
    /// Information about locked memory manager
    #[serde(rename = "locked")]
    pub lock_ed: GetMemoryInfoVerboseZeroLocked,
}

/// Information about locked memory manager
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMemoryInfoVerboseZeroLocked {
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

/// Result of the JSON-RPC method `help`.
///
/// > help
/// >
/// > List all commands, or get help for a specified command.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Help(pub String);

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

/// Result of the JSON-RPC method `stop`.
///
/// > stop
/// >
/// > Request a graceful shutdown of Bitcoin Core.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Stop(pub String);

/// Result of the JSON-RPC method `uptime`.
///
/// > uptime
/// >
/// > Returns the total uptime of the server.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Uptime(pub i64);


// --- hidden ---

// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core - hidden.
//!
//! Types for methods found under the `== Hidden ==` section of the API docs.
//!
//! This file is auto-generated from OpenRPC specification.

use serde::{Deserialize, Serialize};

/// Open an outbound connection to a specified node. This RPC is for testing only.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct AddConnection {
    /// Address of newly added connection.
    pub address: String,
    /// Type of connection opened.
    pub connection_type: String,
}

/// Add the address of a potential peer to an address manager table. This RPC is for testing only.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct AddPeerAddress {
    /// error description, if the address could not be added
    pub error: Option<String>,
    /// whether the peer address was successfully added to the address manager table
    pub success: bool,
}

/// Simply echo back the input arguments. This command is for testing.
/// 
/// It will return an internal bug report when arg9='trigger_internal_bug' is passed.
/// 
/// The difference between echo and echojson is that echojson has argument conversion enabled in the client-side table in bitcoin-cli and the GUI. There is no server-side difference.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Echo {

}

/// Result of the JSON-RPC method `echoipc`.
///
/// > echoipc
/// >
/// > Echo back the input argument, passing it through a spawned process in a multiprocess build.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct EchoIpc(pub String);

/// Simply echo back the input arguments. This command is for testing.
/// 
/// It will return an internal bug report when arg9='trigger_internal_bug' is passed.
/// 
/// The difference between echo and echojson is that echojson has argument conversion enabled in the client-side table in bitcoin-cli and the GUI. There is no server-side difference.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct EchoJson {

}

/// WARNING: This interface is unstable and may disappear or change!
/// 
/// WARNING: This is an advanced API call that is tightly coupled to the specific
/// implementation of fee estimation. The parameters it can be called with
/// and the results it returns will change if the internal implementation changes.
/// 
/// Estimates the approximate fee per kilobyte needed for a transaction to begin
/// confirmation within conf_target blocks if possible. Uses virtual transaction size as
/// defined in BIP 141 (witness data is discounted).
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct EstimateRawFee {
    /// estimate for long time horizon
    pub long: Option<EstimateRawFeeLong>,
    /// estimate for medium time horizon
    pub medium: Option<EstimateRawFeeMedium>,
    /// estimate for short time horizon
    pub short: Option<EstimateRawFeeShort>,
}

/// estimate for long time horizon
/// [TODO] this is a commentary from documentation explaining what this field is supposed be: 
/// commentary: 
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct EstimateRawFeeLong {

}

/// estimate for medium time horizon
/// [TODO] this is a commentary from documentation explaining what this field is supposed be: 
/// commentary: 
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct EstimateRawFeeMedium {

}

/// estimate for short time horizon
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct EstimateRawFeeShort {
    /// exponential decay (per block) for historical moving average of confirmation data
    pub decay: i64,
    /// Errors encountered during processing (if there are any)
    pub errors: Option<Vec<String>>,
    /// information about the highest range of feerates to fail to meet the threshold
    pub fail: Option<EstimateRawFeeShortFail>,
    /// estimate fee rate in BTC/kvB
    pub feerate: Option<i64>,
    /// information about the lowest range of feerates to succeed in meeting the threshold
    pub pass: Option<EstimateRawFeeShortPass>,
    /// The resolution of confirmation targets at this time horizon
    pub scale: i64,
}

/// information about the highest range of feerates to fail to meet the threshold
/// [TODO] this is a commentary from documentation explaining what this field is supposed be: 
/// commentary: 
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct EstimateRawFeeShortFail {

}

/// information about the lowest range of feerates to succeed in meeting the threshold
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct EstimateRawFeeShortPass {
    /// end of feerate range
    pub endrange: i64,
    /// current number of txs in mempool in the feerate range unconfirmed for at least target blocks
    pub inmempool: i64,
    /// number of txs over history horizon in the feerate range that left mempool unconfirmed after target
    pub leftmempool: i64,
    /// start of feerate range
    #[serde(rename = "startrange")]
    pub start_range: i64,
    /// number of txs over history horizon in the feerate range that were confirmed at any point
    #[serde(rename = "totalconfirmed")]
    pub total_conf_irmed: i64,
    /// number of txs over history horizon in the feerate range that were confirmed within target
    pub withintarget: i64,
}

/// has been replaced by the -generate cli option. Refer to -help for more information.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Generate {

}

/// Mine a set of ordered transactions to a specified address or descriptor and return the block hash.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GenerateBlock {
    /// hash of generated block
    pub hash: String,
    /// hex of generated block, only present when submit=false
    pub hex: Option<String>,
}

/// Mine to a specified address and return the block hashes.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GenerateToAddress {

}

/// Mine to a specified descriptor and return the block hashes.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GenerateToDescriptor {

}

/// Result of the JSON-RPC method `getrawaddrman`.
///
/// > getrawaddrman
/// >
/// > EXPERIMENTAL warning: this call may be changed in future releases.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawAddrman(
    /// Map entries
    pub std::collections::BTreeMap<String, std::collections::BTreeMap<String, GetRawAddrmanEntryEntry>>,
);

/// the location in the address manager table (<bucket>/<position>)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawAddrmanEntryEntry {
    /// The address of the node
    pub address: String,
    /// Mapped AS (Autonomous System) number at the end of the BGP route to the peer, used for diversifying peer selection (only displayed if the -asmap config option is set)
    pub mapped_as: Option<i64>,
    /// The network (ipv4, ipv6, onion, i2p, cjdns) of the address
    pub network: String,
    /// The port number of the node
    pub port: i64,
    /// The services offered by the node
    pub services: i64,
    /// The address that relayed the address to us
    pub source: String,
    /// Mapped AS (Autonomous System) number at the end of the BGP route to the source, used for diversifying peer selection (only displayed if the -asmap config option is set)
    pub source_mapped_as: Option<i64>,
    /// The network (ipv4, ipv6, onion, i2p, cjdns) of the source address
    pub source_network: String,
    /// The UNIX epoch time when the node was last seen
    pub time: i64,
}

/// Send a p2p message to a peer specified by id.
/// The message type and body must be provided, the message header will be generated.
/// This RPC is for testing only.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SendmsgToPeer {

}


// --- mining ---

// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core - mining.
//!
//! Types for methods found under the `== Mining ==` section of the API docs.
//!
//! This file is auto-generated from OpenRPC specification.

use serde::{Deserialize, Serialize};

/// If the request parameters include a 'mode' key, that is used to explicitly select between the default 'template' request or a 'proposal'.
/// It returns data needed to construct a block to work on.
/// For full specification, see BIPs 22, 23, 9, and 145:
///     https://github.com/bitcoin/bips/blob/master/bip-0022.mediawiki
///     https://github.com/bitcoin/bips/blob/master/bip-0023.mediawiki
///     https://github.com/bitcoin/bips/blob/master/bip-0009.mediawiki#getblocktemplate_changes
///     https://github.com/bitcoin/bips/blob/master/bip-0145.mediawiki
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlocktemplateVerboseOne(pub String);

/// If the request parameters include a 'mode' key, that is used to explicitly select between the default 'template' request or a 'proposal'.
/// It returns data needed to construct a block to work on.
/// For full specification, see BIPs 22, 23, 9, and 145:
///     https://github.com/bitcoin/bips/blob/master/bip-0022.mediawiki
///     https://github.com/bitcoin/bips/blob/master/bip-0023.mediawiki
///     https://github.com/bitcoin/bips/blob/master/bip-0009.mediawiki#getblocktemplate_changes
///     https://github.com/bitcoin/bips/blob/master/bip-0145.mediawiki
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlocktemplateVerboseTwo {
    /// compressed target of next block
    pub bits: String,
    pub capabilities: Vec<String>,
    /// data that should be included in the coinbase's scriptSig content
    #[serde(rename = "coinbaseaux")]
    pub coin_baseaux: std::collections::BTreeMap<String, String>,
    /// maximum allowable input to coinbase transaction, including the generation award and transaction fees (in satoshis)
    #[serde(rename = "coinbasevalue")]
    pub coin_basevalue: i64,
    /// current timestamp in UNIX epoch time. Adjusted for the proposed BIP94 timewarp rule.
    pub curtime: i64,
    /// a valid witness commitment for the unmodified block template
    pub default_witness_commitment: Option<String>,
    /// The height of the next block
    pub height: i64,
    /// an id to include with a request to longpoll on an update to this template
    pub longpollid: String,
    /// The minimum timestamp appropriate for the next block time, expressed in UNIX epoch time. Adjusted for the proposed BIP94 timewarp rule.
    #[serde(rename = "mintime")]
    pub min_time: i64,
    /// list of ways the block template may be changed
    pub mutable: Vec<String>,
    /// A range of valid nonces
    #[serde(rename = "noncerange")]
    pub nonce_range: String,
    /// The hash of current highest block
    #[serde(rename = "previousblockhash")]
    pub previous_block_hash: String,
    /// specific block rules that are to be enforced
    pub rules: Vec<String>,
    /// Only on signet
    pub signet_challenge: Option<String>,
    /// limit of sigops in blocks
    pub sigoplimit: i64,
    /// limit of block size
    #[serde(rename = "sizelimit")]
    pub size_limit: i64,
    /// The hash target
    pub target: String,
    /// contents of non-coinbase transactions that should be included in the next block
    pub transactions: Vec<GetBlocktemplateVerboseTwoTransactionsItem>,
    /// set of pending, supported versionbit (BIP 9) softfork deployments
    pub vbavailable: std::collections::BTreeMap<String, i64>,
    /// bit mask of versionbits the server requires set in submissions
    pub vbrequired: i64,
    /// The preferred block version
    pub version: i64,
    /// limit of block weight
    #[serde(rename = "weightlimit")]
    pub weight_limit: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlocktemplateVerboseTwoTransactionsItem {
    /// transaction data encoded in hexadecimal (byte-for-byte)
    pub data: String,
    /// array of numbers
    pub depends: Vec<i64>,
    /// difference in value between transaction inputs and outputs (in satoshis); for coinbase transactions, this is a negative Number of the total collected block fees (ie, not including the block subsidy); if key is not present, fee is unknown and clients MUST NOT assume there isn't one
    pub fee: i64,
    /// transaction hash including witness data, shown in byte-reversed hex
    pub hash: String,
    /// total SigOps cost, as counted for purposes of block limits; if key is not present, sigop cost is unknown and clients MUST NOT assume it is zero
    pub sigops: i64,
    /// transaction hash excluding witness data, shown in byte-reversed hex
    pub txid: String,
    /// total transaction weight, as counted for purposes of block limits
    pub weight: i64,
}

/// Returns a json object containing mining-related information.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMiningInfo {
    /// The current nBits, compact representation of the block difficulty target
    pub bits: String,
    /// Minimum feerate of packages selected for block inclusion in BTC/kvB
    #[serde(rename = "blockmintxfee")]
    pub block_min_tx_fee: f64,
    /// The current block
    pub blocks: i64,
    /// current network name (main, test, testnet4, signet, regtest)
    pub chain: String,
    /// The number of block transactions (excluding coinbase) of the last assembled block (only present if a block was ever assembled)
    #[serde(rename = "currentblocktx")]
    pub current_block_tx: Option<i64>,
    /// The block weight (including reserved weight for block header, txs count and coinbase tx) of the last assembled block (only present if a block was ever assembled)
    #[serde(rename = "currentblockweight")]
    pub current_block_weight: Option<i64>,
    /// The current difficulty
    pub difficulty: i64,
    /// The network hashes per second
    #[serde(rename = "networkhashps")]
    pub network_hashps: i64,
    /// The next block
    pub next: GetMiningInfoNext,
    /// The size of the mempool
    #[serde(rename = "pooledtx")]
    pub pooled_tx: i64,
    /// The block challenge (aka. block script), in hexadecimal (only present if the current network is a signet)
    pub signet_challenge: Option<String>,
    /// The current target
    pub target: String,
    /// any network and blockchain warnings (run with `-deprecatedrpc=warnings` to return the latest warning as a single string)
    pub warnings: Vec<String>,
}

/// The next block
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMiningInfoNext {
    /// The next target nBits
    pub bits: String,
    /// The next difficulty
    pub difficulty: i64,
    /// The next height
    pub height: i64,
    /// The next target
    pub target: String,
}

/// Result of the JSON-RPC method `getnetworkhashps`.
///
/// > getnetworkhashps
/// >
/// > Returns the estimated network hashes per second based on the last n blocks.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetNetworkHashps(pub i64);

/// Result of the JSON-RPC method `getprioritisedtransactions`.
///
/// > getprioritisedtransactions
/// >
/// > Returns a map of all user-created (see prioritisetransaction) fee deltas by txid, and whether the tx is present in mempool.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetPrioritisedTransactions(
    /// prioritisation keyed by txid
    pub std::collections::BTreeMap<String, GetPrioritisedTransactionsEntry>,
);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetPrioritisedTransactionsEntry {
    /// transaction fee delta in satoshis
    pub fee_delta: i64,
    /// whether this transaction is currently in mempool
    pub in_mempool: bool,
    /// modified fee in satoshis. Only returned if in_mempool=true
    pub modified_fee: Option<i64>,
}

/// Result of the JSON-RPC method `prioritisetransaction`.
///
/// > prioritisetransaction
/// >
/// > Accepts the transaction into mined blocks at a higher (or lower) priority
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct PrioritiseTransaction(pub bool);

/// Attempts to submit new block to network.
/// See https://en.bitcoin.it/wiki/BIP_0022 for full specification.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SubmitBlockVerboseOne(pub String);


// --- network ---

// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core - network.
//!
//! Types for methods found under the `== Network ==` section of the API docs.
//!
//! This file is auto-generated from OpenRPC specification.

use serde::{Deserialize, Serialize};

/// Returns information about the given added node, or all added nodes
/// (note that onetry addnodes are not listed here)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetAddedNodeInfo {

}

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

/// Result of the JSON-RPC method `getconnectioncount`.
///
/// > getconnectioncount
/// >
/// > Returns the number of connections to other nodes.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetConnectionCount(pub i64);

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

/// Return known addresses, after filtering for quality and recency.
/// These can potentially be used to find new peers in the network.
/// The total number of addresses known to the node may be higher.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetNodeAddresses {

}

/// Returns data about each connected network peer as a json array of objects.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetPeerInfo {

}

/// List all manually banned IPs/Subnets.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ListBanned {

}

/// Result of the JSON-RPC method `setnetworkactive`.
///
/// > setnetworkactive
/// >
/// > Disable/enable all p2p network activity.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SetNetworkactive(pub bool);


// --- raw_transactions ---

// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core - rawtransactions.
//!
//! Types for methods found under the `== Rawtransactions ==` section of the API docs.
//!
//! This file is auto-generated from OpenRPC specification.

use serde::{Deserialize, Serialize};

/// Analyzes and provides information about the current status of a PSBT and its inputs
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct AnalyzePsbt {
    /// Error message (if there is one)
    pub error: Option<String>,
    /// Estimated feerate of the final signed transaction in BTC/kvB. Shown only if all UTXO slots in the PSBT have been filled
    pub estimated_feerate: Option<f64>,
    /// Estimated vsize of the final signed transaction
    pub estimated_vsize: Option<i64>,
    /// The transaction fee paid. Shown only if all UTXO slots in the PSBT have been filled
    pub fee: Option<f64>,
    pub inputs: Option<Vec<AnalyzePsbtInputsItem>>,
    /// Role of the next person that this psbt needs to go to
    pub next: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct AnalyzePsbtInputsItem {
    /// Whether a UTXO is provided
    pub has_utxo: bool,
    /// Whether the input is finalized
    pub is_final: bool,
    /// Things that are missing that are required to complete this input
    pub missing: Option<AnalyzePsbtInputsItemMissing>,
    /// Role of the next person that this input needs to go to
    pub next: Option<String>,
}

/// Things that are missing that are required to complete this input
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct AnalyzePsbtInputsItemMissing {
    pub pubkeys: Option<Vec<String>>,
    /// Hash160 of the redeem script that is missing
    pub redeemscript: Option<String>,
    pub signatures: Option<Vec<String>>,
    /// SHA256 of the witness script that is missing
    #[serde(rename = "witnessscript")]
    pub witness_script: Option<String>,
}

/// Result of the JSON-RPC method `combinepsbt`.
///
/// > combinepsbt
/// >
/// > Combine multiple partially signed Bitcoin transactions into one transaction.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct CombinePsbt(pub String);

/// Result of the JSON-RPC method `combinerawtransaction`.
///
/// > combinerawtransaction
/// >
/// > Combine multiple partially signed transactions into one transaction.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct CombineRawTransaction(pub String);

/// Result of the JSON-RPC method `converttopsbt`.
///
/// > converttopsbt
/// >
/// > Converts a network serialized transaction to a PSBT. This should be used only with createrawtransaction and fundrawtransaction
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ConvertToPsbt(pub String);

/// Result of the JSON-RPC method `createpsbt`.
///
/// > createpsbt
/// >
/// > Creates a transaction in the Partially Signed Transaction format.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct CreatePsbt(pub String);

/// Result of the JSON-RPC method `createrawtransaction`.
///
/// > createrawtransaction
/// >
/// > Create a transaction spending the given inputs and creating new outputs.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct CreateRawTransaction(pub String);

/// Return a JSON object representing the serialized, base64-encoded partially signed Bitcoin transaction.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbt {
    /// The transaction fee paid if all UTXOs slots in the PSBT have been filled.
    pub fee: Option<f64>,
    pub global_xpubs: Vec<DecodePsbtGlobalXpubsItem>,
    pub inputs: Vec<DecodePsbtInputsItem>,
    pub outputs: Vec<DecodePsbtOutputsItem>,
    /// The global proprietary map
    pub proprietary: Vec<DecodePsbtProprietaryItem>,
    /// The PSBT version number. Not to be confused with the unsigned transaction version
    pub psbt_version: i64,
    /// The decoded network-serialized unsigned transaction.
    pub tx: DecodePsbtTx,
    /// The unknown global fields
    pub unknown: std::collections::BTreeMap<String, String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtGlobalXpubsItem {
    /// The fingerprint of the master key
    pub master_fingerprint: String,
    /// The path
    pub path: String,
    /// The extended public key this path corresponds to
    pub xpub: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtInputsItem {
    pub bip32_derivs: Option<Vec<DecodePsbtInputsItemBip32DerivsItem>>,
    #[serde(rename = "final_scriptSig")]
    pub final_scriptsig: Option<DecodePsbtInputsItemFinalScriptSig>,
    pub final_scriptwitness: Option<Vec<String>>,
    pub hash160_preimages: Option<std::collections::BTreeMap<String, String>>,
    pub hash256_preimages: Option<std::collections::BTreeMap<String, String>>,
    pub musig2_partial_sigs: Option<Vec<DecodePsbtInputsItemMusig2PartialSigsItem>>,
    pub musig2_participant_pubkeys: Option<Vec<DecodePsbtInputsItemMusig2ParticipantPubkeysItem>>,
    pub musig2_pubnonces: Option<Vec<DecodePsbtInputsItemMusig2PubnoncesItem>>,
    /// Decoded network transaction for non-witness UTXOs
    pub non_witness_utxo: Option<DecodePsbtInputsItemNonWitnessUtxo>,
    pub partial_signatures: Option<std::collections::BTreeMap<String, String>>,
    /// The input proprietary map
    pub proprietary: Option<Vec<DecodePsbtInputsItemProprietaryItem>>,
    pub redeem_script: Option<DecodePsbtInputsItemRedeemScript>,
    pub ripemd160_preimages: Option<std::collections::BTreeMap<String, String>>,
    pub sha256_preimages: Option<std::collections::BTreeMap<String, String>>,
    /// The sighash type to be used
    pub sighash: Option<String>,
    pub taproot_bip32_derivs: Option<Vec<DecodePsbtInputsItemTaprootBip32DerivsItem>>,
    /// The hex-encoded Taproot x-only internal key
    pub taproot_internal_key: Option<String>,
    /// hex-encoded signature for the Taproot key path spend
    pub taproot_key_path_sig: Option<String>,
    /// The hex-encoded Taproot merkle root
    pub taproot_merkle_root: Option<String>,
    pub taproot_script_path_sigs: Option<Vec<DecodePsbtInputsItemTaprootScriptPathSigsItem>>,
    pub taproot_scripts: Option<Vec<DecodePsbtInputsItemTaprootScriptsItem>>,
    /// The unknown input fields
    pub unknown: Option<std::collections::BTreeMap<String, String>>,
    pub witness_script: Option<DecodePsbtInputsItemWitnessScript>,
    /// Transaction output for witness UTXOs
    pub witness_utxo: Option<DecodePsbtInputsItemWitnessUtxo>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtInputsItemBip32DerivsItem {
    /// The fingerprint of the master key
    pub master_fingerprint: String,
    /// The path
    pub path: String,
    /// The public key with the derivation path as the value.
    pub pubkey: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtInputsItemFinalScriptSig {
    /// Disassembly of the final signature script
    pub asm: String,
    /// The raw final signature script bytes, hex-encoded
    pub hex: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtInputsItemMusig2PartialSigsItem {
    /// The compressed aggregate public key for which this partial signature is for.
    pub aggregate_pubkey: String,
    /// The hash of the leaf script that contains the aggregate pubkey being signed for. Omitted when signing for the internal key.
    pub leaf_hash: Option<String>,
    /// The partial signature itself.
    pub partial_sig: String,
    /// The compressed public key of the participant that created this partial signature.
    pub participant_pubkey: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtInputsItemMusig2ParticipantPubkeysItem {
    /// The compressed aggregate public key for which the participants create.
    pub aggregate_pubkey: String,
    pub participant_pubkeys: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtInputsItemMusig2PubnoncesItem {
    /// The compressed aggregate public key for which this pubnonce is for.
    pub aggregate_pubkey: String,
    /// The hash of the leaf script that contains the aggregate pubkey being signed for. Omitted when signing for the internal key.
    pub leaf_hash: Option<String>,
    /// The compressed public key of the participant that created this pubnonce.
    pub participant_pubkey: String,
    /// The public nonce itself.
    pub pubnonce: String,
}

/// Decoded network transaction for non-witness UTXOs
/// [TODO] this is a commentary from documentation explaining what this field is supposed be: 
/// commentary: 
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtInputsItemNonWitnessUtxo {

}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtInputsItemProprietaryItem {
    /// The hex string for the proprietary identifier
    pub identifier: String,
    /// The hex for the key
    pub key: String,
    /// The number for the subtype
    pub subtype: i64,
    /// The hex for the value
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtInputsItemRedeemScript {
    /// Disassembly of the redeem script
    pub asm: String,
    /// The raw redeem script bytes, hex-encoded
    pub hex: String,
    /// The type, eg 'pubkeyhash'
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtInputsItemTaprootBip32DerivsItem {
    /// The hashes of the leaves this pubkey appears in
    pub leaf_hashes: Vec<String>,
    /// The fingerprint of the master key
    pub master_fingerprint: String,
    /// The path
    pub path: String,
    /// The x-only public key this path corresponds to
    pub pubkey: String,
}

/// The signature for the pubkey and leaf hash combination
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtInputsItemTaprootScriptPathSigsItem {
    /// The leaf hash for this signature
    pub leaf_hash: String,
    /// The x-only pubkey for this signature
    pub pubkey: String,
    /// The signature itself
    pub sig: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtInputsItemTaprootScriptsItem {
    /// The control blocks for this script
    pub control_blocks: Vec<String>,
    /// The version number for the leaf script
    pub leaf_ver: i64,
    /// A leaf script
    pub script: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtInputsItemWitnessScript {
    /// Disassembly of the witness script
    pub asm: String,
    /// The raw witness script bytes, hex-encoded
    pub hex: String,
    /// The type, eg 'pubkeyhash'
    #[serde(rename = "type")]
    pub type_: String,
}

/// Transaction output for witness UTXOs
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtInputsItemWitnessUtxo {
    /// The value in BTC
    pub amount: i64,
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: DecodePsbtInputsItemWitnessUtxoScriptPubKey,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtInputsItemWitnessUtxoScriptPubKey {
    /// The Bitcoin address (only if a well-defined address exists)
    pub address: Option<String>,
    /// Disassembly of the output script
    pub asm: String,
    /// Inferred descriptor for the output
    pub desc: String,
    /// The raw output script bytes, hex-encoded
    pub hex: String,
    /// The type, eg 'pubkeyhash'
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtOutputsItem {
    pub bip32_derivs: Option<Vec<DecodePsbtOutputsItemBip32DerivsItem>>,
    pub musig2_participant_pubkeys: Option<Vec<DecodePsbtOutputsItemMusig2ParticipantPubkeysItem>>,
    /// The output proprietary map
    pub proprietary: Option<Vec<DecodePsbtOutputsItemProprietaryItem>>,
    pub redeem_script: Option<DecodePsbtOutputsItemRedeemScript>,
    pub taproot_bip32_derivs: Option<Vec<DecodePsbtOutputsItemTaprootBip32DerivsItem>>,
    /// The hex-encoded Taproot x-only internal key
    pub taproot_internal_key: Option<String>,
    /// The tuples that make up the Taproot tree, in depth first search order
    pub taproot_tree: Option<Vec<DecodePsbtOutputsItemTaprootTreeItem>>,
    /// The unknown output fields
    pub unknown: Option<std::collections::BTreeMap<String, String>>,
    pub witness_script: Option<DecodePsbtOutputsItemWitnessScript>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtOutputsItemBip32DerivsItem {
    /// The fingerprint of the master key
    pub master_fingerprint: String,
    /// The path
    pub path: String,
    /// The public key this path corresponds to
    pub pubkey: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtOutputsItemMusig2ParticipantPubkeysItem {
    /// The compressed aggregate public key for which the participants create.
    pub aggregate_pubkey: String,
    pub participant_pubkeys: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtOutputsItemProprietaryItem {
    /// The hex string for the proprietary identifier
    pub identifier: String,
    /// The hex for the key
    pub key: String,
    /// The number for the subtype
    pub subtype: i64,
    /// The hex for the value
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtOutputsItemRedeemScript {
    /// Disassembly of the redeem script
    pub asm: String,
    /// The raw redeem script bytes, hex-encoded
    pub hex: String,
    /// The type, eg 'pubkeyhash'
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtOutputsItemTaprootBip32DerivsItem {
    /// The hashes of the leaves this pubkey appears in
    pub leaf_hashes: Vec<String>,
    /// The fingerprint of the master key
    pub master_fingerprint: String,
    /// The path
    pub path: String,
    /// The x-only public key this path corresponds to
    pub pubkey: String,
}

/// A single leaf script in the taproot tree
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtOutputsItemTaprootTreeItem {
    /// The depth of this element in the tree
    pub depth: i64,
    /// The version of this leaf
    pub leaf_ver: i64,
    /// The hex-encoded script itself
    pub script: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtOutputsItemWitnessScript {
    /// Disassembly of the witness script
    pub asm: String,
    /// The raw witness script bytes, hex-encoded
    pub hex: String,
    /// The type, eg 'pubkeyhash'
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtProprietaryItem {
    /// The hex string for the proprietary identifier
    pub identifier: String,
    /// The hex for the key
    pub key: String,
    /// The number for the subtype
    pub subtype: i64,
    /// The hex for the value
    pub value: String,
}

/// The decoded network-serialized unsigned transaction.
/// [TODO] this is a commentary from documentation explaining what this field is supposed be: 
/// commentary: The layout is the same as the output of decoderawtransaction.
pub type DecodePsbtTx = DecodeRawTransaction;

/// Return a JSON object representing the serialized, hex-encoded transaction.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodeRawTransaction {
    /// The transaction hash (differs from txid for witness transactions)
    pub hash: String,
    /// The lock time
    pub locktime: i64,
    /// The serialized transaction size
    pub size: i64,
    /// The transaction id
    pub txid: String,
    /// The version
    pub version: i64,
    pub vin: Vec<DecodeRawTransactionVinItem>,
    pub vout: Vec<DecodeRawTransactionVoutItem>,
    /// The virtual transaction size (differs from size for witness transactions)
    pub vsize: i64,
    /// The transaction's weight (between vsize*4-3 and vsize*4)
    pub weight: i64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodeRawTransactionVinItem {
    /// The coinbase value (only if coinbase transaction)
    #[serde(rename = "coinbase")]
    pub coin_base: Option<String>,
    /// The script (if not coinbase transaction)
    #[serde(rename = "scriptSig")]
    pub script_sig: Option<DecodeRawTransactionVinItemScriptSig>,
    /// The script sequence number
    pub sequence: i64,
    /// The transaction id (if not coinbase transaction)
    pub txid: Option<String>,
    #[serde(rename = "txinwitness")]
    pub tx_inwitness: Option<Vec<String>>,
    /// The output number (if not coinbase transaction)
    pub vout: Option<i64>,
}

/// The script (if not coinbase transaction)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodeRawTransactionVinItemScriptSig {
    /// Disassembly of the signature script
    pub asm: String,
    /// The raw signature script bytes, hex-encoded
    pub hex: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodeRawTransactionVoutItem {
    /// index
    pub n: i64,
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: DecodeRawTransactionVoutItemScriptPubKey,
    /// The value in BTC
    pub value: f64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodeRawTransactionVoutItemScriptPubKey {
    /// The Bitcoin address (only if a well-defined address exists)
    pub address: Option<String>,
    /// Disassembly of the output script
    pub asm: String,
    /// Inferred descriptor for the output
    pub desc: String,
    /// The raw output script bytes, hex-encoded
    pub hex: String,
    /// The type (one of: nonstandard, anchor, pubkey, pubkeyhash, scripthash, multisig, nulldata, witness_v0_scripthash, witness_v0_keyhash, witness_v1_taproot, witness_unknown)
    #[serde(rename = "type")]
    pub type_: String,
}

/// Decode a hex-encoded script.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodeScript {
    /// The Bitcoin address (only if a well-defined address exists)
    pub address: Option<String>,
    /// Disassembly of the script
    pub asm: String,
    /// Inferred descriptor for the script
    pub desc: String,
    /// address of P2SH script wrapping this redeem script (not returned for types that should not be wrapped)
    pub p2sh: Option<String>,
    /// Result of a witness output script wrapping this redeem script (not returned for types that should not be wrapped)
    pub segwit: Option<DecodeScriptSegwit>,
    /// The output type (e.g. nonstandard, anchor, pubkey, pubkeyhash, scripthash, multisig, nulldata, witness_v0_scripthash, witness_v0_keyhash, witness_v1_taproot, witness_unknown)
    #[serde(rename = "type")]
    pub type_: String,
}

/// Result of a witness output script wrapping this redeem script (not returned for types that should not be wrapped)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodeScriptSegwit {
    /// The Bitcoin address (only if a well-defined address exists)
    pub address: Option<String>,
    /// Disassembly of the output script
    pub asm: String,
    /// Inferred descriptor for the script
    pub desc: String,
    /// The raw output script bytes, hex-encoded
    pub hex: String,
    /// address of the P2SH script wrapping this witness redeem script
    #[serde(rename = "p2sh-segwit")]
    pub p2sh_segwit: String,
    /// The type of the output script (e.g. witness_v0_keyhash or witness_v0_scripthash)
    #[serde(rename = "type")]
    pub type_: String,
}

/// Update all segwit inputs in a PSBT with information from output descriptors, the UTXO set or the mempool. 
/// Then, sign the inputs we are able to with information from the output descriptors.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DescriptorProcessPsbt {
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// The hex-encoded network transaction if complete
    pub hex: Option<String>,
    /// The base64-encoded partially signed transaction
    pub psbt: String,
}

/// Finalize the inputs of a PSBT. If the transaction is fully signed, it will produce a
/// network serialized transaction which can be broadcast with sendrawtransaction. Otherwise a PSBT will be
/// created which has the final_scriptSig and final_scriptwitness fields filled for inputs that are complete.
/// Implements the Finalizer and Extractor roles.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct FinalizePsbt {
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// The hex-encoded network transaction if extracted
    pub hex: Option<String>,
    /// The base64-encoded partially signed transaction if not extracted
    pub psbt: Option<String>,
}

/// By default, this call only returns a transaction if it is in the mempool. If -txindex is enabled
/// and no blockhash argument is passed, it will return the transaction if it is in the mempool or any block.
/// If a blockhash argument is passed, it will return the transaction if
/// the specified block is available and the transaction is in that block.
/// 
/// Hint: Use gettransaction for wallet transactions.
/// 
/// If verbosity is 0 or omitted, returns the serialized transaction as a hex-encoded string.
/// If verbosity is 1, returns a JSON Object with information about the transaction.
/// If verbosity is 2, returns a JSON Object with information about the transaction, including fee and prevout information.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawTransactionVerboseOne {
    /// the block hash
    #[serde(rename = "blockhash")]
    pub block_hash: Option<String>,
    /// The block time expressed in UNIX epoch time
    #[serde(rename = "blocktime")]
    pub block_time: Option<i64>,
    /// The confirmations
    pub confirmations: Option<i64>,
    /// The transaction hash (differs from txid for witness transactions)
    pub hash: String,
    /// The serialized, hex-encoded data for 'txid'
    pub hex: String,
    /// Whether specified block is in the active chain or not (only present with explicit "blockhash" argument)
    pub in_active_chain: Option<bool>,
    /// The lock time
    pub locktime: i64,
    /// The serialized transaction size
    pub size: i64,
    /// Same as "blocktime"
    pub time: Option<i64>,
    /// The transaction id (same as provided)
    pub txid: String,
    /// The version
    pub version: i64,
    pub vin: Vec<GetRawTransactionVerboseOneVinItem>,
    pub vout: Vec<GetRawTransactionVerboseOneVoutItem>,
    /// The virtual transaction size (differs from size for witness transactions)
    pub vsize: i64,
    /// The transaction's weight (between vsize*4-3 and vsize*4)
    pub weight: i64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawTransactionVerboseOneVinItem {
    /// The coinbase value (only if coinbase transaction)
    #[serde(rename = "coinbase")]
    pub coin_base: Option<String>,
    /// The script (if not coinbase transaction)
    #[serde(rename = "scriptSig")]
    pub script_sig: Option<GetRawTransactionVerboseOneVinItemScriptSig>,
    /// The script sequence number
    pub sequence: i64,
    /// The transaction id (if not coinbase transaction)
    pub txid: Option<String>,
    #[serde(rename = "txinwitness")]
    pub tx_inwitness: Option<Vec<String>>,
    /// The output number (if not coinbase transaction)
    pub vout: Option<i64>,
}

/// The script (if not coinbase transaction)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawTransactionVerboseOneVinItemScriptSig {
    /// Disassembly of the signature script
    pub asm: String,
    /// The raw signature script bytes, hex-encoded
    pub hex: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawTransactionVerboseOneVoutItem {
    /// index
    pub n: i64,
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: GetRawTransactionVerboseOneVoutItemScriptPubKey,
    /// The value in BTC
    pub value: f64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawTransactionVerboseOneVoutItemScriptPubKey {
    /// The Bitcoin address (only if a well-defined address exists)
    pub address: Option<String>,
    /// Disassembly of the output script
    pub asm: String,
    /// Inferred descriptor for the output
    pub desc: String,
    /// The raw output script bytes, hex-encoded
    pub hex: String,
    /// The type (one of: nonstandard, anchor, pubkey, pubkeyhash, scripthash, multisig, nulldata, witness_v0_scripthash, witness_v0_keyhash, witness_v1_taproot, witness_unknown)
    #[serde(rename = "type")]
    pub type_: String,
}

/// By default, this call only returns a transaction if it is in the mempool. If -txindex is enabled
/// and no blockhash argument is passed, it will return the transaction if it is in the mempool or any block.
/// If a blockhash argument is passed, it will return the transaction if
/// the specified block is available and the transaction is in that block.
/// 
/// Hint: Use gettransaction for wallet transactions.
/// 
/// If verbosity is 0 or omitted, returns the serialized transaction as a hex-encoded string.
/// If verbosity is 1, returns a JSON Object with information about the transaction.
/// If verbosity is 2, returns a JSON Object with information about the transaction, including fee and prevout information.
/// [TODO] this is a commentary from documentation explaining what this field is supposed be: 
/// commentary: Same output as verbosity = 1
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawTransactionVerboseTwo {
    /// transaction fee in BTC, omitted if block undo data is not available
    pub fee: Option<i64>,
    pub vin: Vec<GetRawTransactionVerboseTwoVinItem>,
}

/// utxo being spent
/// [TODO] this is a commentary from documentation explaining what this field is supposed be: 
/// commentary: Same output as verbosity = 1
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawTransactionVerboseTwoVinItem {
    /// The previous output, omitted if block undo data is not available
    #[serde(rename = "prevout")]
    pub prev_out: Option<GetRawTransactionVerboseTwoVinItemPrevout>,
}

/// The previous output, omitted if block undo data is not available
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawTransactionVerboseTwoVinItemPrevout {
    /// Coinbase or not
    pub generated: bool,
    /// The height of the prevout
    pub height: i64,
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: GetRawTransactionVerboseTwoVinItemPrevoutScriptPubKey,
    /// The value in BTC
    pub value: f64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawTransactionVerboseTwoVinItemPrevoutScriptPubKey {
    /// The Bitcoin address (only if a well-defined address exists)
    pub address: Option<String>,
    /// Disassembly of the output script
    pub asm: String,
    /// Inferred descriptor for the output
    pub desc: String,
    /// The raw output script bytes, hex-encoded
    pub hex: String,
    /// The type (one of: nonstandard, anchor, pubkey, pubkeyhash, scripthash, multisig, nulldata, witness_v0_scripthash, witness_v0_keyhash, witness_v1_taproot, witness_unknown)
    #[serde(rename = "type")]
    pub type_: String,
}

/// By default, this call only returns a transaction if it is in the mempool. If -txindex is enabled
/// and no blockhash argument is passed, it will return the transaction if it is in the mempool or any block.
/// If a blockhash argument is passed, it will return the transaction if
/// the specified block is available and the transaction is in that block.
/// 
/// Hint: Use gettransaction for wallet transactions.
/// 
/// If verbosity is 0 or omitted, returns the serialized transaction as a hex-encoded string.
/// If verbosity is 1, returns a JSON Object with information about the transaction.
/// If verbosity is 2, returns a JSON Object with information about the transaction, including fee and prevout information.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawTransactionVerboseZero(pub String);

/// Result of the JSON-RPC method `joinpsbts`.
///
/// > joinpsbts
/// >
/// > Joins multiple distinct PSBTs with different inputs and outputs into one PSBT with inputs and outputs from all of the PSBTs
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct JoinPsbts(pub String);

/// Result of the JSON-RPC method `sendrawtransaction`.
///
/// > sendrawtransaction
/// >
/// > Submit a raw transaction (serialized, hex-encoded) to local node and network.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SendRawTransaction(pub String);

/// Sign inputs for raw transaction (serialized, hex-encoded).
/// The second argument is an array of base58-encoded private
/// keys that will be the only keys used to sign the transaction.
/// The third optional argument (may be null) is an array of previous transaction outputs that
/// this transaction depends on but may not yet be in the block chain.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SignRawTransactionwithKey {
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// Script verification errors (if there are any)
    pub errors: Option<Vec<SignRawTransactionwithKeyErrorsItem>>,
    /// The hex-encoded raw transaction with signature(s)
    pub hex: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SignRawTransactionwithKeyErrorsItem {
    /// Verification or signing error related to the input
    pub error: String,
    /// The hex-encoded signature script
    #[serde(rename = "scriptSig")]
    pub script_sig: String,
    /// Script sequence number
    pub sequence: i64,
    /// The hash of the referenced, previous transaction
    pub txid: String,
    /// The index of the output to spent and used as input
    pub vout: i64,
    pub witness: Vec<String>,
}

/// Submit a package of raw transactions (serialized, hex-encoded) to local node.
/// The package will be validated according to consensus and mempool policy rules. If any transaction passes, it will be accepted to mempool.
/// This RPC is experimental and the interface may be unstable. Refer to doc/policy/packages.md for documentation on package policies.
/// Warning: successful submission does not mean the transactions will propagate throughout the network.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SubmitPackage {
    /// The transaction package result message. "success" indicates all transactions were accepted into or are already in the mempool.
    pub package_msg: String,
    /// List of txids of replaced transactions
    #[serde(rename = "replaced-transactions")]
    pub replaced_transactions: Option<Vec<String>>,
    /// transaction results keyed by wtxid
    #[serde(rename = "tx-results")]
    pub tx_results: std::collections::BTreeMap<String, SubmitPackageTxResults>,
}

/// transaction wtxid
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SubmitPackageTxResults {
    /// The transaction error string, if it was rejected by the mempool
    pub error: Option<String>,
    /// Transaction fees
    pub fees: Option<SubmitPackageTxResultsFees>,
    /// The wtxid of a different transaction with the same txid but different witness found in the mempool. This means the submitted transaction was ignored.
    #[serde(rename = "other-wtxid")]
    pub other_wtxid: Option<String>,
    /// The transaction hash in hex
    pub txid: String,
    /// Sigops-adjusted virtual transaction size.
    pub vsize: Option<i64>,
}

/// Transaction fees
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SubmitPackageTxResultsFees {
    /// transaction fee in BTC
    pub base: f64,
    /// if the transaction was not already in the mempool, the effective feerate in BTC per KvB. For example, the package feerate and/or feerate with modified fees from prioritisetransaction.
    #[serde(rename = "effective-feerate")]
    pub effective_feerate: Option<f64>,
    /// if effective-feerate is provided, the wtxids of the transactions whose fees and vsizes are included in effective-feerate.
    #[serde(rename = "effective-includes")]
    pub effective_includes: Option<Vec<String>>,
}

/// Returns result of mempool acceptance tests indicating if raw transaction(s) (serialized, hex-encoded) would be accepted by mempool.
/// 
/// If multiple transactions are passed in, parents must come before children and package policies apply: the transactions cannot conflict with any mempool transactions or each other.
/// 
/// If one transaction fails, other transactions may not be fully validated (the 'allowed' key will be blank).
/// 
/// The maximum number of transactions allowed is 25.
/// 
/// This checks if transactions violate the consensus or policy rules.
/// 
/// See sendrawtransaction call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct TestMempoolaccept {

}

/// Result of the JSON-RPC method `utxoupdatepsbt`.
///
/// > utxoupdatepsbt
/// >
/// > Updates all segwit inputs and outputs in a PSBT with data from output descriptors, the UTXO set, txindex, or the mempool.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct UtxoUpdatePsbt(pub String);


// --- signer ---

// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core - signer.
//!
//! Types for methods found under the `== Signer ==` section of the API docs.
//!
//! This file is auto-generated from OpenRPC specification.

use serde::{Deserialize, Serialize};

/// Returns a list of external signers from -signer.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct EnumerateSigners {
    pub signers: Vec<EnumerateSignersSignersItem>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct EnumerateSignersSignersItem {
    /// Master key fingerprint
    pub fingerprint: String,
    /// Device name
    pub name: String,
}


// --- util ---

// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core - util.
//!
//! Types for methods found under the `== Util ==` section of the API docs.
//!
//! This file is auto-generated from OpenRPC specification.

use serde::{Deserialize, Serialize};

/// Creates a multi-signature address with n signatures of m keys required.
/// It returns a json object with the address and redeemScript.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct CreateMultisig {
    /// The value of the new multisig address.
    pub address: String,
    /// The descriptor for this multisig
    pub descriptor: String,
    /// The string value of the hex-encoded redemption script.
    #[serde(rename = "redeemScript")]
    pub redeem_script: String,
    /// Any warnings resulting from the creation of this multisig
    pub warnings: Option<Vec<String>>,
}

/// Estimates the approximate fee per kilobyte needed for a transaction to begin
/// confirmation within conf_target blocks if possible and return the number of blocks
/// for which the estimate is valid. Uses virtual transaction size as defined
/// in BIP 141 (witness data is discounted).
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct EstimateSmartFee {
    /// block number where estimate was found
/// The request target will be clamped between 2 and the highest target
/// fee estimation is able to return based on how long it has been running.
/// An error is returned if not enough transactions and blocks
/// have been observed to make an estimate for any number of blocks.
    pub blocks: i64,
    /// Errors encountered during processing (if there are any)
    pub errors: Option<Vec<String>>,
    /// estimate fee rate in BTC/kvB (only present if no errors were encountered)
    pub feerate: Option<i64>,
}

/// Analyses a descriptor.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetDescriptorInfo {
    /// The checksum for the input descriptor
    pub checksum: String,
    /// The descriptor in canonical form, without private keys. For a multipath descriptor, only the first will be returned.
    pub descriptor: String,
    /// Whether the input descriptor contained at least one private key
    pub hasprivatekeys: bool,
    /// Whether the descriptor is ranged
    pub isrange: bool,
    /// Whether the descriptor is solvable
    pub issolvable: bool,
    /// All descriptors produced by expanding multipath derivation elements. Only if the provided descriptor specifies multipath derivation elements.
    pub multipath_expansion: Option<Vec<String>>,
}

/// Result of the JSON-RPC method `getindexinfo`.
///
/// > getindexinfo
/// >
/// > Returns the status of one or all available indices currently running in the node.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetIndexInfo(
    /// Map entries
    pub std::collections::BTreeMap<String, GetIndexInfoEntry>,
);

/// The name of the index
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetIndexInfoEntry {
    /// The block height to which the index is synced
    pub best_block_height: i64,
    /// Whether the index is synced or not
    pub synced: bool,
}

/// Result of the JSON-RPC method `signmessagewithprivkey`.
///
/// > signmessagewithprivkey
/// >
/// > Sign a message with the private key of an address
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SignMessagewithPrivKey(pub String);

/// Return information about the given bitcoin address.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ValidateAddress {
    /// The bitcoin address validated
    pub address: Option<String>,
    /// Error message, if any
    pub error: Option<String>,
    /// Indices of likely error locations in address, if known (e.g. Bech32 errors)
    pub error_locations: Option<Vec<i64>>,
    /// If the key is a script
    pub isscript: Option<bool>,
    /// If the address is valid or not
    pub isvalid: bool,
    /// If the address is a witness address
    pub iswitness: Option<bool>,
    /// The hex-encoded output script generated by the address
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: Option<String>,
    /// The hex value of the witness program
    pub witness_program: Option<String>,
    /// The version number of the witness program
    pub witness_version: Option<i64>,
}

/// Result of the JSON-RPC method `verifymessage`.
///
/// > verifymessage
/// >
/// > Verify a signed message.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct VerifyMessage(pub bool);

