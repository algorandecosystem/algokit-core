//! `AccountStore` implementation backed by Trusty's secure storage service.
//!
//! This is the one piece of `seed_vault_ta` that genuinely cannot be
//! exercised outside a real Trusty build/boot (unlike `seed_vault`'s
//! protocol and account-creation logic, which is proven today via
//! `cargo test -p seed_vault` against
//! `seed_vault::memory_store::MemoryStore`). It is modeled directly on
//! `trusty/app/secretkeeper/store.rs`
//! (https://android.googlesource.com/trusty/app/secretkeeper/+/refs/heads/main/store.rs),
//! which is Google's own reference implementation of a `KeyValueStore` on
//! top of this exact `storage` crate -- `Session::new(Port::TamperDetect, true)`,
//! `session.open_file(name, OpenMode::Open | OpenMode::Create)`,
//! `session.write_all` / `session.read_all` / `session.get_size`,
//! `session.remove(name)`, and `session.list_files()` returning
//! `(filename, state)` pairs are all taken from that file.
//!
//! ## Storage layout
//!
//! Each account is one file: `seed_vault_v1_<index:08x>`, holding
//! `[algorithm_tag: u8][key material bytes]` and nothing else. There is no
//! separate index/metadata file -- `next_index`/`all_records` both derive
//! everything they need by listing files and parsing the `<index>` suffix,
//! so the set of files *is* the database. Deleting an account removes its
//! file, which may leave a gap in the index sequence; that's fine, since
//! nothing assumes indices are contiguous.
//!
//! What "key material" holds depends on the algorithm (see
//! `seed_vault::store`): an `Algo25Ed25519` record is just its 32-byte
//! seed (the address is re-derived, which is cheap), while a `Falcon1024`
//! record is the 32-byte entropy followed by a cache of its keygen output
//! (public key, private key) and of both of its accounts' salt + address
//! (see `seed_vault::store::falcon_record`), so Falcon keygen runs only at
//! create/import time.
//!
//! ## `Port::TamperDetect` vs `Port::TamperProof`
//!
//! `TamperDetect` storage is wiped on factory reset and can only be read
//! once Android has finished booting -- the same tradeoff secretkeeper
//! accepts for its secrets. That is almost certainly the right choice here
//! too (a seed vault account is exactly the kind of secret that should not
//! survive a factory reset), but this is a product decision worth
//! confirming, not a technical constraint of this file.
//!
//! ## Honest caveat
//!
//! The exact signature of `Session::new` (specifically its second, boolean
//! argument) and the precise `Result`/error types returned by each method
//! should be checked against whatever version of the `storage` crate is
//! actually vendored at `trusty/user/base/lib/storage/rust` in your synced
//! Trusty tree -- this file is written to match the shape of the real
//! `secretkeeper` reference implementation as closely as can be verified
//! without a live checkout, but Trusty's Rust libraries are not published to
//! crates.io/docs.rs, so this could not be compiled against the real crate
//! from this environment.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use seed_vault::{AccountRecord, AccountStore, KeyAlgorithm, StoreError};
use storage::{OpenMode, Port, Session};
use zeroize::Zeroize;

const FILE_PREFIX: &str = "seed_vault_v1_";

fn filename_for_index(index: u32) -> String {
    format!("{FILE_PREFIX}{index:08x}")
}

fn index_from_filename(name: &str) -> Option<u32> {
    u32::from_str_radix(name.strip_prefix(FILE_PREFIX)?, 16).ok()
}

fn open_session() -> Result<Session, StoreError> {
    Session::new(Port::TamperDetect, true).map_err(|_| StoreError::Backend)
}

/// Every existing `seed_vault_v1_*` file's parsed index, sorted ascending.
fn indexed_filenames(session: &mut Session) -> Result<Vec<(u32, String)>, StoreError> {
    let mut indexed: Vec<(u32, String)> = session
        .list_files()
        .map_err(|_| StoreError::Backend)?
        .filter_map(|entry| entry.ok())
        .filter_map(|(filename, _state)| {
            index_from_filename(&filename).map(|index| (index, filename))
        })
        .collect();
    indexed.sort_by_key(|(index, _)| *index);
    Ok(indexed)
}

/// [`AccountStore`] backed by Trusty's on-device secure storage service, so
/// seed vault accounts created by this TA survive reboots (subject to the
/// `TamperDetect` factory-reset semantics described above).
#[derive(Default)]
pub struct TrustySecureStore;

impl AccountStore for TrustySecureStore {
    fn next_index(&mut self) -> Result<u32, StoreError> {
        let mut session = open_session()?;
        let indexed = indexed_filenames(&mut session)?;
        Ok(indexed.last().map_or(0, |(max, _)| max + 1))
    }

    fn persist_record(&mut self, index: u32, record: &AccountRecord) -> Result<(), StoreError> {
        let mut session = open_session()?;
        let filename = filename_for_index(index);
        let mut file = session
            .open_file(&filename, OpenMode::Create)
            .map_err(|_| StoreError::Backend)?;
        let mut bytes = Vec::with_capacity(1 + record.key_material.len());
        bytes.push(record.algorithm as u8);
        bytes.extend_from_slice(&record.key_material);
        let result = session
            .write_all(&mut file, &bytes)
            .map_err(|_| StoreError::Backend);
        bytes.zeroize();
        result
    }

    fn delete_record(&mut self, index: u32) -> Result<(), StoreError> {
        let mut session = open_session()?;
        // Distinguishing "not found" would need `trusty_sys::Error::NotFound`
        // (as secretkeeper does); callers only delete indices they just
        // listed, so any failure here is a genuine backend error.
        session
            .remove(&filename_for_index(index))
            .map_err(|_| StoreError::Backend)
    }

    fn all_indexed_records(&mut self) -> Result<Vec<(u32, AccountRecord)>, StoreError> {
        let mut session = open_session()?;
        let indexed = indexed_filenames(&mut session)?;

        let mut records = Vec::with_capacity(indexed.len());
        for (index, filename) in indexed {
            let file = session
                .open_file(&filename, OpenMode::Open)
                .map_err(|_| StoreError::Backend)?;
            let size = session.get_size(&file).map_err(|_| StoreError::Backend)?;
            if size == 0 {
                return Err(StoreError::CorruptRecord);
            }
            let mut buf = alloc::vec![0u8; size as usize];
            if session.read_all(&file, &mut buf).is_err() {
                buf.zeroize();
                return Err(StoreError::Backend);
            }

            // Split in place rather than copying, so (up to ~4 KB per
            // Falcon record) key material is never duplicated on the heap.
            let mut key_material = buf.split_off(1);
            let Some(algorithm) = KeyAlgorithm::from_tag(buf[0]) else {
                key_material.zeroize();
                return Err(StoreError::CorruptRecord);
            };
            records.push((
                index,
                AccountRecord {
                    algorithm,
                    key_material,
                },
            ));
        }
        Ok(records)
    }
}
