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
//! | getdeploymentinfo                    | version (GetdeploymentInfo) |                                        |
//! | getdescriptoractivity                | version (GetDescriptoractivity) |                                        |
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
//! | fundrawtransaction                   | version (FundRawTransaction) |                                        |
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
//! | getindexinfo                         | version (GetindexInfo) |                                        |
//! | signmessagewithprivkey               | returns string  |                                        |
//! | validateaddress                      | version (ValidateAddress) |                                        |
//! | verifymessage                        | returns boolean |                                        |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Wallet == section </summary>
//!
//! | JSON-RPC Method Name               | Returns         | Notes                                  |
//! |:-----------------------------------|:---------------:|:--------------------------------------:|
//! | abandontransaction                   | returns nothing |                                        |
//! | abortrescan                          | returns boolean |                                        |
//! | backupwallet                         | returns nothing |                                        |
//! | bumpfee                              | version (BumpFee) |                                        |
//! | createwallet                         | version (CreateWallet) |                                        |
//! | createwalletdescriptor               | version (CreateWalletDescriptor) |                                        |
//! | encryptwallet                        | returns string  |                                        |
//! | getaddressesbylabel                  | version (GetAddressesbyLabel) |                                        |
//! | getaddressinfo                       | version (GetAddressInfo) |                                        |
//! | getbalance                           | returns string  |                                        |
//! | getbalances                          | version (GetBalances) |                                        |
//! | gethdkeys                            | version (GetHdKeys) |                                        |
//! | getnewaddress                        | returns string  |                                        |
//! | getrawchangeaddress                  | returns string  |                                        |
//! | getreceivedbyaddress                 | returns string  |                                        |
//! | getreceivedbylabel                   | returns string  |                                        |
//! | gettransaction                       | version (GetTransaction) |                                        |
//! | getwalletinfo                        | version (GetWalletInfo) |                                        |
//! | importdescriptors                    | version (ImportDescriptors) |                                        |
//! | importprunedfunds                    | returns nothing |                                        |
//! | keypoolrefill                        | returns nothing |                                        |
//! | listaddressgroupings                 | version (ListAddressGroupings) |                                        |
//! | listdescriptors                      | version (ListDescriptors) |                                        |
//! | listlabels                           | version (ListLabels) |                                        |
//! | listlockunspent                      | version (ListLockUnspent) |                                        |
//! | listreceivedbyaddress                | version (ListReceivedbyAddress) |                                        |
//! | listreceivedbylabel                  | version (ListReceivedbyLabel) |                                        |
//! | listsinceblock                       | version (ListSinceBlock) |                                        |
//! | listtransactions                     | version (ListTransactions) |                                        |
//! | listunspent                          | version (ListUnspent) |                                        |
//! | listwalletdir                        | version (ListWalletDir) |                                        |
//! | listwallets                          | version (ListWallets) |                                        |
//! | loadwallet                           | version (LoadWallet) |                                        |
//! | lockunspent                          | returns boolean |                                        |
//! | migratewallet                        | version (MigrateWallet) |                                        |
//! | psbtbumpfee                          | version (PsbtBumpFee) |                                        |
//! | removeprunedfunds                    | returns nothing |                                        |
//! | rescanblockchain                     | version (RescanBlockchain) |                                        |
//! | restorewallet                        | version (RestoreWallet) |                                        |
//! | send                                 | version (Send)  |                                        |
//! | sendall                              | version (SendAll) |                                        |
//! | sendmany                             | version (Sendmany) |                                        |
//! | sendtoaddress                        | version (SendToAddress) |                                        |
//! | setlabel                             | returns nothing |                                        |
//! | settxfee                             | returns boolean |                                        |
//! | setwalletflag                        | version (SetWalletflag) |                                        |
//! | signmessage                          | returns string  |                                        |
//! | signrawtransactionwithwallet         | version (SignRawTransactionwithWallet) |                                        |
//! | simulaterawtransaction               | version (SimulateRawTransaction) |                                        |
//! | unloadwallet                         | version (UnloadWallet) |                                        |
//! | walletcreatefundedpsbt               | version (WalletCreateFundedPsbt) |                                        |
//! | walletdisplayaddress                 | version (WalletDisplayAddress) |                                        |
//! | walletlock                           | returns nothing |                                        |
//! | walletpassphrase                     | returns nothing |                                        |
//! | walletpassphrasechange               | returns nothing |                                        |
//! | walletprocesspsbt                    | version (WalletProcessPsbt) |                                        |
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
pub mod wallet;

pub use self::blockchain::{DumpTxoutSet, GetBlock, GetBlockVerboseTxItem, GetBlockVerbose, GetBlockVerboseAltTxItem, GetBlockVerboseAltTxItemVinItem, GetBlockVerboseAltTxItemVinItemPrevoutScriptPubKey, GetBlockVerboseAltTxItemVinItemPrevout, GetBlockVerboseAlt, GetBlockchainInfo, GetBlockFilter, GetBlockFromPeer, GetBlockHeader, GetBlockstats, GetChainstates, GetChainstatesChainstatesItem, GetChainTxstats, GetdeploymentInfo, GetdeploymentInfoDeployments, GetdeploymentInfoDeploymentsBip9Statistics, GetdeploymentInfoDeploymentsBip9, GetDescriptoractivity, GetMempoolEntry, GetMempoolEntryFees, GetMempoolInfo, GetRawMempool, GetTxout, GetTxoutScriptPubKey, GetTxoutSetInfo, GetTxoutSetInfoBlockInfoUnspendables, GetTxoutSetInfoBlockInfo, ImportMempool, LoadTxoutSet, SaveMempool, ScanBlocks, ScanBlocksVerbose, ScanTxoutSet, ScanTxoutSetUnspentsItem, ScanTxoutSetVerbose, WaitForBlock, WaitForBlockHeight, WaitForNewBlock};
pub use self::control::{Api, GetMemoryInfo, GetMemoryInfoLocked, GetRpcInfo, GetRpcInfoActiveCommandsItem, Logging};
pub use self::hidden::{AddConnection, AddPeerAddress, EstimateRawFee, EstimateRawFeeLong, EstimateRawFeeMedium, EstimateRawFeeShortFail, EstimateRawFeeShortPass, EstimateRawFeeShort, GenerateBlock, GetRawAddrman, GetRawAddrmanEntryEntry, SendmsgToPeer};
pub use self::mining::{GetBlocktemplate, GetBlocktemplateTransactionsItem, GetMiningInfo, GetMiningInfoNext, GetPrioritisedTransactions, GetPrioritisedTransactionsEntry};
pub use self::network::{GetAddrmanInfo, GetAddrmanInfoEntry, GetNetTotals, GetNetTotalsUploadtarget, GetNetworkInfo, GetNetworkInfoLocaladdressesItem, GetNetworkInfoNetworksItem};
pub use self::raw_transactions::{AnalyzePsbt, AnalyzePsbtInputsItem, AnalyzePsbtInputsItemMissing, DecodePsbt, DecodePsbtGlobalXpubsItem, DecodePsbtInputsItem, DecodePsbtInputsItemBip32DerivsItem, DecodePsbtInputsItemFinalScriptSig, DecodePsbtInputsItemMusig2PartialSigsItem, DecodePsbtInputsItemMusig2ParticipantPubkeysItem, DecodePsbtInputsItemMusig2PubnoncesItem, DecodePsbtInputsItemNonWitnessUtxo, DecodePsbtInputsItemProprietaryItem, DecodePsbtInputsItemRedeemScript, DecodePsbtInputsItemTaprootBip32DerivsItem, DecodePsbtInputsItemTaprootScriptPathSigsItem, DecodePsbtInputsItemTaprootScriptsItem, DecodePsbtInputsItemWitnessScript, DecodePsbtInputsItemWitnessUtxoScriptPubKey, DecodePsbtInputsItemWitnessUtxo, DecodePsbtOutputsItem, DecodePsbtOutputsItemBip32DerivsItem, DecodePsbtOutputsItemMusig2ParticipantPubkeysItem, DecodePsbtOutputsItemProprietaryItem, DecodePsbtOutputsItemRedeemScript, DecodePsbtOutputsItemTaprootBip32DerivsItem, DecodePsbtOutputsItemTaprootTreeItem, DecodePsbtOutputsItemWitnessScript, DecodePsbtProprietaryItem, DecodePsbtTx, DecodeRawTransaction, DecodeRawTransactionVinItem, DecodeRawTransactionVinItemScriptSig, DecodeRawTransactionVoutItem, DecodeRawTransactionVoutItemScriptPubKey, DecodeScript, DecodeScriptSegwit, DescriptorProcessPsbt, FinalizePsbt, FundRawTransaction, GetRawTransaction, GetRawTransactionVinItem, GetRawTransactionVinItemScriptSig, GetRawTransactionVoutItem, GetRawTransactionVoutItemScriptPubKey, GetRawTransactionVerboseVinItem, GetRawTransactionVerboseVinItemPrevoutScriptPubKey, GetRawTransactionVerboseVinItemPrevout, GetRawTransactionVerbose, SignRawTransactionwithKey, SignRawTransactionwithKeyErrorsItem, SubmitPackage, SubmitPackageTxResults, SubmitPackageTxResultsFees};
pub use self::signer::{EnumerateSigners, EnumerateSignersSignersItem};
pub use self::util::{CreateMultisig, EstimateSmartFee, GetDescriptorInfo, GetindexInfo, GetindexInfoEntry, ValidateAddress};
pub use self::wallet::{BumpFee, CreateWallet, CreateWalletDescriptor, GetAddressesbyLabel, GetAddressesbyLabelEntry, GetAddressInfo, GetAddressInfoEmbedded, GetBalances, GetBalancesLastprocessedblock, GetBalancesMine, GetTransaction, GetTransactionDecoded, GetTransactionDetailsItem, GetTransactionLastprocessedblock, GetWalletInfo, GetWalletInfoLastprocessedblock, GetWalletInfoScanning, ListDescriptors, ListDescriptorsDescriptorsItem, ListSinceBlock, ListSinceBlockTransactionsItem, ListWalletDir, ListWalletDirWalletsItem, LoadWallet, MigrateWallet, PsbtBumpFee, RescanBlockchain, RestoreWallet, Send, SendAll, Sendmany, SendToAddress, SetWalletflag, SignRawTransactionwithWallet, SignRawTransactionwithWalletErrorsItem, SimulateRawTransaction, UnloadWallet, WalletCreateFundedPsbt, WalletDisplayAddress, WalletProcessPsbt};
