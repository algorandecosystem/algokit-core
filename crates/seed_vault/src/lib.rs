//! Transport- and storage-agnostic core of the `seed_vault_ta` Trusty
//! applet: creates and enumerates Algorand accounts without their private
//! key material ever leaving the secure world.
//!
//! `trusty/app/seed_vault_ta` implements the [`store::AccountStore`] trait
//! defined here against Trusty's real secure storage service, and drives
//! [`service::handle_request`] from a `tipc` event loop.
#![no_std]

extern crate alloc;

pub mod memory_store;
pub mod protocol;
pub mod service;
pub mod store;

pub use store::{
    ALGO25_SEED_LEN, AccountRecord, AccountStore, AccountSummary, KeyAlgorithm, StoreError,
    create_account, import_account, list_accounts, reveal_mnemonic,
};
