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
