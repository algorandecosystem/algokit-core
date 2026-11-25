//! A `no_std` WebAssembly sandbox (on [wasmi], an interpreter) with
//! content-addressed program identity and per-call resource limits.
//!
//! Adapted from <https://github.com/joe-p/wasm-crypto-eval>; see this
//! crate's `Cargo.toml` for what changed and why.
//!
//! - [`ProgramId`]: SHA-256 of a module's bytes. The host decides what a
//!   program may reach based on this, and a guest can't change it: any
//!   change to the module is a different ID.
//! - [`WasmRuntime`]: an engine plus the host functions every program can
//!   import. [`WasmRuntime::compile`] validates a module once;
//!   [`WasmRuntime::instantiate`] gives each call a fresh instance (fresh
//!   linear memory), so nothing a program computed in one call survives
//!   into the next.
//! - [`Limits`]: fuel (bounds execution time) and linear-memory caps, so a
//!   program can't hang or exhaust the host.
//! - [`guest_memory`]: bounds-checked access to a guest's memory from
//!   host functions.
//!
//! Everything but [`ProgramId`] needs the default `runtime` feature: clients
//! that only name programs (e.g. the Android FFI) turn it off and don't
//! link wasmi.
#![no_std]

extern crate alloc;

use core::fmt;

#[cfg(feature = "runtime")]
mod error;
#[cfg(feature = "runtime")]
pub mod guest_memory;
#[cfg(feature = "runtime")]
mod runtime;

#[cfg(feature = "runtime")]
pub use error::{Error, Result};
#[cfg(feature = "runtime")]
pub use runtime::*;
// So host-function code can name `Caller`, `Linker`, ... without its own
// direct wasmi dependency (and version to keep in sync).
#[cfg(feature = "runtime")]
pub use wasmi;

/// A program's identity: the SHA-256 of its module bytes.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProgramId(pub [u8; 32]);

impl ProgramId {
    /// Computes the identity of `module_bytes`.
    pub fn of(module_bytes: &[u8]) -> Self {
        ProgramId(cryptoxide::hashing::sha256(module_bytes))
    }
}

impl fmt::Debug for ProgramId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ProgramId(")?;
        for byte in self.0 {
            write!(f, "{byte:02x}")?;
        }
        f.write_str(")")
    }
}

#[cfg(all(test, feature = "runtime"))]
mod tests;
