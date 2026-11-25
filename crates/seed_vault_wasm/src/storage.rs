//! Persistence for installed programs and their cached keys, so both
//! survive a TA restart.
//!
//! A flat namespace of named blobs, which is all Trusty secure storage is:
//! `seed_vault_ta` implements [`WasmStore`] on its files, and tests use
//! [`MemoryWasmStore`]. Names (at most 93 bytes, under Trusty's 159-byte
//! limit):
//!
//! - `svw1_prog_<program_id hex>`: a module's bytes.
//! - `svw1_mfst_<program_id hex>`: its signed manifest, if it has one
//!   (re-verified on every restore).
//! - `svw1_minv_<SHA-256 of the algorithm name, first 16 bytes, hex>`: the
//!   lowest version of that signed algorithm still allowed (rollback
//!   protection).
//! - `svw1_state_<key namespace hex>`: a program's persistent state (shared
//!   by the signed versions of an algorithm).
//! - `svw1_revocations`: the last signed revocation list applied
//!   (re-verified on every restore).
//! - `svw1_key_<program_id hex>_<seed tag hex>_<address_index hex>`: what
//!   that program cached for that key (see [`crate::abi`]'s `cache_put`).
//!   The seed tag is 8 bytes of `SHA512/256("SeedVaultWasmSeedTag" ||
//!   entropy)`, so entries can be matched to seeds (to drop a deleted
//!   seed's keys) without naming the seed.

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use seed_vault::StoreError;
use wasm_runtime::ProgramId;
use zeroize::Zeroize;

/// Named-blob storage for [`crate::WasmVault`].
pub trait WasmStore {
    /// Names of all stored blobs starting with `prefix`.
    fn list(&mut self, prefix: &str) -> Result<Vec<String>, StoreError>;
    /// The blob `name`, or `None` if there's none.
    fn read(&mut self, name: &str) -> Result<Option<Vec<u8>>, StoreError>;
    /// Creates or replaces the blob `name`.
    fn write(&mut self, name: &str, data: &[u8]) -> Result<(), StoreError>;
    /// Deletes the blob `name`. Deleting a missing blob is not an error.
    fn delete(&mut self, name: &str) -> Result<(), StoreError>;
}

/// In-memory [`WasmStore`] for tests. Nothing is persisted.
#[derive(Default)]
pub struct MemoryWasmStore {
    blobs: BTreeMap<String, Vec<u8>>,
}

impl WasmStore for MemoryWasmStore {
    fn list(&mut self, prefix: &str) -> Result<Vec<String>, StoreError> {
        Ok(self
            .blobs
            .keys()
            .filter(|name| name.starts_with(prefix))
            .cloned()
            .collect())
    }

    fn read(&mut self, name: &str) -> Result<Option<Vec<u8>>, StoreError> {
        Ok(self.blobs.get(name).cloned())
    }

    fn write(&mut self, name: &str, data: &[u8]) -> Result<(), StoreError> {
        if let Some(mut old) = self.blobs.insert(String::from(name), data.to_vec()) {
            old.zeroize();
        }
        Ok(())
    }

    fn delete(&mut self, name: &str) -> Result<(), StoreError> {
        if let Some(mut old) = self.blobs.remove(name) {
            old.zeroize();
        }
        Ok(())
    }
}

pub(crate) const PROGRAM_PREFIX: &str = "svw1_prog_";
pub(crate) const KEY_PREFIX: &str = "svw1_key_";
pub(crate) const REVOCATIONS: &str = "svw1_revocations";

pub(crate) fn manifest_name(program: &ProgramId) -> String {
    format!("svw1_mfst_{}", hex(&program.0))
}

pub(crate) fn state_name(namespace: &[u8; 32]) -> String {
    format!("svw1_state_{}", hex(namespace))
}

pub(crate) fn min_version_name(algorithm: &str) -> String {
    let hash = ProgramId::of(algorithm.as_bytes()).0;
    format!("svw1_minv_{}", hex(&hash[..16]))
}

fn hex(bytes: &[u8]) -> String {
    use core::fmt::Write;
    let mut out = String::with_capacity(2 * bytes.len());
    for byte in bytes {
        let _ = write!(out, "{byte:02x}");
    }
    out
}

fn unhex32(text: &str) -> Option<[u8; 32]> {
    if text.len() != 64 {
        return None;
    }
    let mut out = [0u8; 32];
    for (i, byte) in out.iter_mut().enumerate() {
        *byte = u8::from_str_radix(text.get(2 * i..2 * i + 2)?, 16).ok()?;
    }
    Some(out)
}

pub(crate) fn program_name(program: &ProgramId) -> String {
    format!("{PROGRAM_PREFIX}{}", hex(&program.0))
}

pub(crate) fn program_from_name(name: &str) -> Option<ProgramId> {
    unhex32(name.strip_prefix(PROGRAM_PREFIX)?).map(ProgramId)
}

/// Prefix of every cached key of `program`.
pub(crate) fn program_keys_prefix(program: &ProgramId) -> String {
    format!("{KEY_PREFIX}{}_", hex(&program.0))
}

/// 8 bytes identifying a seed in key-cache names.
pub(crate) fn seed_tag(entropy: &[u8; 32]) -> [u8; 8] {
    let mut input = [0u8; 20 + 32];
    input[..20].copy_from_slice(b"SeedVaultWasmSeedTag");
    input[20..].copy_from_slice(entropy);
    let hash = algokit_crypto::sha512_256(&input);
    input.zeroize();
    hash[..8].try_into().unwrap()
}

pub(crate) fn key_name(program: &ProgramId, seed_tag: &[u8; 8], address_index: u8) -> String {
    format!(
        "{}{}_{address_index:02x}",
        program_keys_prefix(program),
        hex(seed_tag)
    )
}

/// The seed tag in a key-cache name.
pub(crate) fn seed_tag_from_key_name(name: &str) -> Option<String> {
    // svw1_key_<64 hex>_<16 hex>_<2 hex>
    let rest = name.strip_prefix(KEY_PREFIX)?;
    let tag = rest.get(65..81)?;
    Some(String::from(tag))
}

pub(crate) fn seed_tag_hex(tag: &[u8; 8]) -> String {
    hex(tag)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_round_trip_and_fit_trusty_limits() {
        let program = ProgramId([0xAB; 32]);
        let name = program_name(&program);
        assert_eq!(program_from_name(&name), Some(program));
        assert_eq!(program_from_name("svw1_prog_zz"), None);

        let tag = seed_tag(&[1; 32]);
        let key = key_name(&program, &tag, 0xFF);
        assert!(key.starts_with(&program_keys_prefix(&program)));
        assert_eq!(seed_tag_from_key_name(&key), Some(seed_tag_hex(&tag)));
        assert_eq!(key.len(), 93);
        assert!(key.len() <= 159);
        assert_ne!(seed_tag(&[1; 32]), seed_tag(&[2; 32]));
    }
}
