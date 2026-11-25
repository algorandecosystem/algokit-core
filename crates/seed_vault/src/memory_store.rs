//! An in-memory [`AccountStore`] implementation.
//!
//! **Not** a production backend -- nothing here is persisted. Its purpose
//! is to let `seed_vault`'s account creation/listing/protocol logic be
//! exercised with a plain `cargo test -p seed_vault`, no Trusty/AOSP
//! checkout required. The real Trusty applet (`trusty/app/seed_vault_ta`)
//! implements `AccountStore` against Trusty's secure storage service
//! instead.

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use zeroize::Zeroize;

use crate::store::{AccountRecord, AccountStore, StoreError};

/// Keyed by index so deletions leave gaps, exactly like the Trusty backend's
/// one-file-per-index layout.
#[derive(Default)]
pub struct MemoryStore {
    records: BTreeMap<u32, AccountRecord>,
}

impl AccountStore for MemoryStore {
    fn next_index(&mut self) -> Result<u32, StoreError> {
        Ok(self
            .records
            .last_key_value()
            .map_or(0, |(&index, _)| index + 1))
    }

    fn persist_record(&mut self, index: u32, record: &AccountRecord) -> Result<(), StoreError> {
        if self.records.contains_key(&index) {
            // Enforce the same "no overwriting an existing index" contract
            // a real backend must uphold.
            return Err(StoreError::Backend);
        }
        self.records.insert(index, record.clone());
        Ok(())
    }

    fn replace_record(&mut self, index: u32, record: &AccountRecord) -> Result<(), StoreError> {
        let existing = self.records.get_mut(&index).ok_or(StoreError::NotFound)?;
        existing.key_material.zeroize();
        *existing = record.clone();
        Ok(())
    }

    fn all_indexed_records(&mut self) -> Result<Vec<(u32, AccountRecord)>, StoreError> {
        Ok(self
            .records
            .iter()
            .map(|(&index, record)| (index, record.clone()))
            .collect())
    }

    fn delete_record(&mut self, index: u32) -> Result<(), StoreError> {
        let mut record = self.records.remove(&index).ok_or(StoreError::NotFound)?;
        record.key_material.zeroize();
        Ok(())
    }
}
