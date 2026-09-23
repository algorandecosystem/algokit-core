//! An in-memory [`AccountStore`] implementation.
//!
//! **Not** a production backend -- nothing here is persisted. Its purpose
//! is to let `seed_vault`'s account creation/listing/protocol logic be
//! exercised with a plain `cargo test -p seed_vault`, no Trusty/AOSP
//! checkout required. The real Trusty applet (`trusty/app/seed_vault_ta`)
//! implements `AccountStore` against Trusty's secure storage service
//! instead.

use alloc::vec::Vec;

use crate::store::{AccountRecord, AccountStore, StoreError};

#[derive(Default)]
pub struct MemoryStore {
    records: Vec<AccountRecord>,
}

impl AccountStore for MemoryStore {
    fn next_index(&mut self) -> Result<u32, StoreError> {
        Ok(self.records.len() as u32)
    }

    fn persist_record(&mut self, index: u32, record: &AccountRecord) -> Result<(), StoreError> {
        if index as usize != self.records.len() {
            // Enforce the same "no overwriting an existing index" contract
            // a real backend must uphold.
            return Err(StoreError::Backend);
        }
        self.records.push(record.clone());
        Ok(())
    }

    fn all_records(&mut self) -> Result<Vec<AccountRecord>, StoreError> {
        Ok(self.records.clone())
    }
}
