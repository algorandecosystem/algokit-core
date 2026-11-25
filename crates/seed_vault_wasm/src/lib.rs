//! Runs programs delivered as WebAssembly inside the `seed_vault_ta` Trusty
//! applet: signing algorithms, using keys derived from the seeds the vault
//! already holds, and general computation with persistent state.
//!
//! The point is to add functionality without shipping new firmware: a
//! client uploads a WASM module (see [`protocol`] for the chunked upload,
//! needed because a Trusty IPC message is capped at about 4 KB), and can
//! then ask it for a public key or a signature for any seed in the vault
//! (`guest/ed25519`, `guest/falcon1024`), or `RUN` it on any input -- e.g.
//! `guest/light_client`, an Algorand light client that verifies state
//! proofs and transactions inside the TEE.
//!
//! # Security model
//!
//! - **Modules are signed.** A module ships with a [`Manifest`] signed
//!   (Falcon-1024) by a publisher key built into the TA, pinning its exact
//!   bytes and granting it specific [`Capabilities`]. New algorithms need
//!   no firmware update, but only ones you signed run. Rollback to a lower
//!   version and revoked modules or publisher keys are refused
//!   ([`Revocations`]). See [`PublisherPolicy`]; its dev mode also admits
//!   unsigned modules and trusts a public dev key -- never ship that.
//! - **By default a program never sees a seed.** It gets one 32-byte key per
//!   (seed, namespace, `address_index`), [`derive_program_key`]:
//!   `SHA512/256("SeedVaultWasm" || namespace || address_index || entropy)`,
//!   where the namespace is the algorithm name for signed modules (so new
//!   versions keep their accounts) and the module hash for unsigned ones.
//!   A malicious or buggy program can leak at most its *own* keys.
//! - **Native accounts need a capability.** A module granted
//!   [`Capabilities::account_entropy`] gets the native key material for the
//!   requested account (never the root seed beyond that), so it can
//!   reproduce the vault's own accounts. Index 0's key material *is* the
//!   seed, so index 0 additionally needs [`Capabilities::human_sign`] (and,
//!   still TODO, a fingerprint/PIN check). Nothing technical stops a module
//!   that has key material from leaking it: the signature is the control.
//! - **Each call is sandboxed.** A fresh instance per call (no state between
//!   calls), capped fuel and memory, and the guest memory and keys are wiped
//!   afterwards.
//! - **Persistence.** Programs, manifests, the keys programs cache (so e.g.
//!   Falcon keygen runs once per key), rollback versions and revocations are
//!   kept in a [`WasmStore`] -- Trusty secure storage in the TA -- and
//!   re-verified after a restart. Cached keys are deleted with their program
//!   or seed.
//!
//! Not covered: wasmi is an interpreter, so an algorithm's constant-time
//! guarantees don't necessarily survive (a timing side channel inside the
//! TEE), and it runs roughly 10-50x slower than native code.
//!
//! Features: `vault` (default) is the TA side -- the runtime, storage,
//! policy and request handling. Without it, only what a client needs
//! remains: [`protocol`], [`manifest`], [`client`] and [`light_client`]
//! (e.g. for the Android FFI, which shouldn't link wasmi).
#![no_std]

extern crate alloc;

#[cfg(feature = "vault")]
pub mod abi;
pub mod client;
pub mod light_client;
pub mod manifest;
#[cfg(feature = "vault")]
pub mod policy;
pub mod protocol;
#[cfg(feature = "vault")]
pub mod service;
#[cfg(feature = "vault")]
pub mod storage;
#[cfg(feature = "vault")]
pub mod vault;

pub use manifest::{Capabilities, Manifest, PublisherKey, Revocations, Signed};
#[cfg(feature = "vault")]
pub use policy::{dev_publisher_key, Admission, PublisherPolicy, WasmPolicy};
#[cfg(feature = "vault")]
pub use storage::{MemoryWasmStore, SeedSource, WasmStore};
#[cfg(feature = "vault")]
pub use vault::{derive_program_key, KeyNamespace, ProgramInfo, VaultConfig, WasmError, WasmVault};
pub use wasm_runtime::ProgramId;
