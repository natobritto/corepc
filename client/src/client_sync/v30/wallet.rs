// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Wallet ==` section of the
//! API docs of Bitcoin Core `v30`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_bitreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `createwallet` for v30.
///
/// In v30, legacy wallets are no longer supported, so `create_legacy_wallet` is not available.
#[macro_export]
macro_rules! impl_client_v30__create_wallet {
    () => {
        impl Client {
            /// Calls `createwallet` with `wallet` as the only argument.
            ///
            /// In v30, this creates a descriptor wallet. Legacy wallets are no longer supported.
            pub fn create_wallet(&self, wallet: &str) -> Result<CreateWallet> {
                self.call("createwallet", &[wallet.into()])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `restorewallet` for v30.
#[macro_export]
macro_rules! impl_client_v30__restore_wallet {
    () => {
        impl Client {
            /// Calls `restorewallet` with required and optional arguments.
            ///
            /// > restorewallet "wallet_name" "backup_file" ( load_on_startup )
            pub fn restore_wallet(
                &self,
                wallet_name: &str,
                backup_file: &Path,
            ) -> Result<RestoreWallet> {
                self.call("restorewallet", &[wallet_name.into(), into_json(backup_file)?])
            }
        }
    };
}
