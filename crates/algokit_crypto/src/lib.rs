//! `no_std` by default when the `std` feature (enabled by default) is
//! disabled. See the `std` feature doc-comment in `Cargo.toml` for the
//! precise story around the `xhd` module, which always needs a target with a
//! real `std` regardless of this crate's own feature flags.
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod address;
pub mod algo25;
pub mod ed25519;
pub mod xhd;
pub use signature::{Keypair, Signer};
