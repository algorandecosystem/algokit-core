//! `seed_vault_wasm::WasmStore` on Trusty secure storage: installed WASM
//! programs and the keys they cache, one file per blob (names from
//! `seed_vault_wasm::storage`, all prefixed `svw1_`, so they never collide
//! with `trusty_store.rs`'s `seed_vault_v1_*` account files).
//!
//! Same `Port::TamperDetect` session as the account store (see
//! `trusty_store.rs`), so these are wiped on factory reset along with the
//! seeds their keys came from. The not-found handling mirrors
//! `trusty/app/secretkeeper/store.rs`.

use alloc::string::String;
use alloc::vec::Vec;

use seed_vault::StoreError;
use seed_vault_wasm::WasmStore;
use storage::{OpenMode, Port, Session};
use zeroize::Zeroize;

fn open_session() -> Result<Session, StoreError> {
    Session::new(Port::TamperDetect, true).map_err(|_| StoreError::Backend)
}

fn is_not_found(error: &storage::Error) -> bool {
    matches!(error, storage::Error::Code(trusty_sys::Error::NotFound))
}

#[derive(Default)]
pub struct TrustyWasmStore;

impl WasmStore for TrustyWasmStore {
    fn list(&mut self, prefix: &str) -> Result<Vec<String>, StoreError> {
        let mut session = open_session()?;
        let names = session
            .list_files()
            .map_err(|_| StoreError::Backend)?
            .filter_map(|entry| entry.ok())
            .map(|(name, _state)| name)
            .filter(|name| name.starts_with(prefix))
            .collect();
        Ok(names)
    }

    fn read(&mut self, name: &str) -> Result<Option<Vec<u8>>, StoreError> {
        let mut session = open_session()?;
        let file = match session.open_file(name, OpenMode::Open) {
            Ok(file) => file,
            Err(e) if is_not_found(&e) => return Ok(None),
            Err(_) => return Err(StoreError::Backend),
        };
        let size = session.get_size(&file).map_err(|_| StoreError::Backend)?;
        let mut buf = alloc::vec![0u8; size];
        if session.read_all(&file, &mut buf).is_err() {
            // Cached blobs are usually private key material.
            buf.zeroize();
            return Err(StoreError::Backend);
        }
        Ok(Some(buf))
    }

    fn write(&mut self, name: &str, data: &[u8]) -> Result<(), StoreError> {
        let mut session = open_session()?;
        let mut file = session
            .open_file(name, OpenMode::TruncateOrCreate)
            .map_err(|_| StoreError::Backend)?;
        session
            .write_all(&mut file, data)
            .map_err(|_| StoreError::Backend)
    }

    fn delete(&mut self, name: &str) -> Result<(), StoreError> {
        let mut session = open_session()?;
        match session.remove(name) {
            Ok(()) => Ok(()),
            Err(e) if is_not_found(&e) => Ok(()),
            Err(_) => Err(StoreError::Backend),
        }
    }
}
