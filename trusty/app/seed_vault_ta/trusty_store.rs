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
//! `session.write_all` / `session.read_all` / `session.get_size`, and
//! `session.list_files()` returning `(filename, state)` pairs are all taken
//! from that file.
//!
//! ## Storage layout
//!
//! Each account is one file: `seed_vault_v1_<index:08x>`, holding
//! `[algorithm_tag: u8][key material bytes]` and nothing else. There is no
//! separate index/metadata file -- `next_index`/`all_records` both derive
//! everything they need by listing files and parsing the `<index>` suffix,
//! so the set of files *is* the database. An account's Algorand address is
//! never stored; it is always re-derived from the key material on demand
//! (see `seed_vault::store`), so there is exactly one source of truth
//! per account and no way for a stored address to drift out of sync with
//! its key material.
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
        session
            .write_all(&mut file, &bytes)
            .map_err(|_| StoreError::Backend)
    }

    fn all_records(&mut self) -> Result<Vec<AccountRecord>, StoreError> {
        let mut session = open_session()?;
        let indexed = indexed_filenames(&mut session)?;

        let mut records = Vec::with_capacity(indexed.len());
        for (_, filename) in indexed {
            let file = session
                .open_file(&filename, OpenMode::Open)
                .map_err(|_| StoreError::Backend)?;
            let size = session.get_size(&file).map_err(|_| StoreError::Backend)?;
            if size == 0 {
                return Err(StoreError::CorruptRecord);
            }
            let mut buf = alloc::vec![0u8; size as usize];
            session
                .read_all(&file, &mut buf)
                .map_err(|_| StoreError::Backend)?;

            let (&tag, key_material) = buf.split_first().ok_or(StoreError::CorruptRecord)?;
            let algorithm = KeyAlgorithm::from_tag(tag).ok_or(StoreError::CorruptRecord)?;
            records.push(AccountRecord {
                algorithm,
                key_material: key_material.to_vec(),
            });
        }
        Ok(records)
    }
}
