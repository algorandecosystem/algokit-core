//! Ties [`crate::protocol`] to a [`WasmVault`], independent of the IPC
//! transport, like `seed_vault::service::handle_request`.

use alloc::vec::Vec;

use seed_vault::AccountStore;

use crate::policy::WasmPolicy;
use crate::protocol::{self, Request};
use crate::storage::WasmStore;
use crate::vault::WasmVault;

/// Handles one raw WASM request (see [`protocol::is_wasm_request`]) and
/// returns the raw response. `accounts` is the vault's seed store,
/// `wasm_store` where programs and their cached keys are kept.
pub fn handle_request<P: WasmPolicy>(
    vault: &mut WasmVault<P>,
    accounts: &mut impl AccountStore,
    wasm_store: &mut impl WasmStore,
    request: &[u8],
) -> Vec<u8> {
    let result = match protocol::decode_request(request) {
        Ok(request) => dispatch(vault, accounts, wasm_store, request),
        Err(code) => return protocol::encode_error_response(code),
    };
    result.unwrap_or_else(|error| protocol::encode_error_response(error.into()))
}

fn dispatch<P: WasmPolicy>(
    vault: &mut WasmVault<P>,
    accounts: &mut impl AccountStore,
    wasm_store: &mut impl WasmStore,
    request: Request<'_>,
) -> Result<Vec<u8>, crate::WasmError> {
    Ok(match request {
        Request::LoadBegin { module_len } => {
            protocol::encode_ok(&[vault.begin_upload(module_len)?])
        }
        Request::LoadChunk {
            upload_id,
            offset,
            chunk,
        } => {
            vault.upload_chunk(upload_id, offset, chunk)?;
            protocol::encode_ok(&[])
        }
        Request::LoadFinish {
            upload_id,
            manifest,
        } => protocol::encode_ok(
            &vault
                .finish_upload(wasm_store, upload_id, manifest.as_ref())?
                .0,
        ),
        Request::Revoke { revocations } => protocol::encode_ok(
            &vault
                .apply_revocations(wasm_store, &revocations)?
                .to_le_bytes(),
        ),
        Request::Unload { program } => {
            vault.unload(wasm_store, &program)?;
            protocol::encode_ok(&[])
        }
        Request::ListPrograms => {
            protocol::encode_list_programs_response(&vault.programs(wasm_store)?)
        }
        Request::PublicKey {
            program,
            address_index,
            address,
        } => protocol::encode_bytes_response(&vault.public_key(
            accounts,
            wasm_store,
            &program,
            &address,
            address_index,
        )?),
        Request::Sign {
            program,
            address_index,
            address,
            message,
        } => protocol::encode_bytes_response(&vault.sign(
            accounts,
            wasm_store,
            &program,
            &address,
            address_index,
            message,
        )?),
        Request::GetVersion => protocol::encode_get_version_response(),
        Request::Run { program, input } => protocol::encode_bytes_response(&vault.run_program(
            wasm_store,
            &program,
            input.to_vec(),
        )?),
        Request::InputBegin { input_len } => protocol::encode_ok(&[vault.begin_input(input_len)?]),
        Request::InputChunk {
            input_id,
            offset,
            chunk,
        } => {
            vault.input_chunk(input_id, offset, chunk)?;
            protocol::encode_ok(&[])
        }
        Request::RunInput { program, input_id } => protocol::encode_bytes_response(
            &vault.run_uploaded_input(wasm_store, &program, input_id)?,
        ),
    })
}
