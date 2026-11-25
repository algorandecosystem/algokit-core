#![allow(dead_code)]

use seed_vault::memory_store::MemoryStore;
use seed_vault::{import_account, KeyAlgorithm, HUMAN_SIGN_INDEX};
use seed_vault_wasm::manifest::{Capabilities, Manifest, Revocations, Signed};
use seed_vault_wasm::policy::{dev_publisher_seed, publisher_keypair, DEV_KEY_ID};
use seed_vault_wasm::protocol::{request, ErrorCode, ListedProgram};
use seed_vault_wasm::service::handle_request;
use seed_vault_wasm::{
    MemoryWasmStore, ProgramId, PublisherPolicy, VaultConfig, WasmPolicy, WasmVault,
};

/// The TA's IPC message size.
pub const MAX_MSG_LEN: usize = 4096;

/// Stands in for `seed_vault_ta`: the vault plus both of its stores, driven
/// over the wire protocol.
pub struct Ta<P: WasmPolicy = PublisherPolicy> {
    pub vault: WasmVault<P>,
    pub accounts: MemoryStore,
    pub wasm_store: MemoryWasmStore,
}

impl Ta<PublisherPolicy> {
    /// Dev mode: trusts the dev publisher key and unsigned modules.
    pub fn new() -> Self {
        Self::with(VaultConfig::default(), PublisherPolicy::dev())
    }

    /// Production mode: signed modules only, from the dev publisher key.
    pub fn signed_only() -> Self {
        Self::with(
            VaultConfig::default(),
            PublisherPolicy::new(vec![seed_vault_wasm::dev_publisher_key()]),
        )
    }
}

/// Signs `body` with the dev publisher key.
pub fn dev_sign(body: Vec<u8>) -> Signed {
    let (_, private_key) = publisher_keypair(&dev_publisher_seed());
    Signed::sign(body, &private_key)
}

/// A dev-signed manifest for `module`.
pub fn manifest_for(module: &[u8], name: &str, version: u32, capabilities: Capabilities) -> Signed {
    dev_sign(
        Manifest {
            key_id: DEV_KEY_ID,
            program_id: ProgramId::of(module),
            name: name.into(),
            version,
            capabilities,
        }
        .encode(),
    )
}

/// A dev-signed revocation list.
pub fn revocations(sequence: u32, keys: &[u8], programs: &[ProgramId]) -> Signed {
    dev_sign(
        Revocations {
            key_id: DEV_KEY_ID,
            sequence,
            revoked_keys: keys.to_vec(),
            revoked_programs: programs.to_vec(),
        }
        .encode(),
    )
}

impl<P: WasmPolicy> Ta<P> {
    pub fn with(config: VaultConfig, policy: P) -> Self {
        Ta {
            vault: WasmVault::new(config, policy),
            accounts: MemoryStore::default(),
            wasm_store: MemoryWasmStore::default(),
        }
    }

    /// The TA restarting: memory is lost, storage isn't.
    pub fn restart(self, config: VaultConfig, policy: P) -> Self {
        Ta {
            vault: WasmVault::new(config, policy),
            ..self
        }
    }

    pub fn vault_policy(&self) -> &P {
        self.vault.policy()
    }

    /// Imports a seed with entropy `[fill; 32]` and returns its index-0
    /// address.
    pub fn seed(&mut self, algorithm: KeyAlgorithm, fill: u8) -> String {
        let mnemonic = algokit_crypto::algo25::mnemonic_from_seed(&[fill; 32]).unwrap();
        import_account(&mut self.accounts, algorithm, HUMAN_SIGN_INDEX, &mnemonic)
            .unwrap()
            .address
    }

    /// Sends `request` and returns the raw response.
    pub fn raw(&mut self, request: &[u8]) -> Vec<u8> {
        assert!(
            request.len() <= MAX_MSG_LEN,
            "request exceeds one IPC message"
        );
        handle_request(
            &mut self.vault,
            &mut self.accounts,
            &mut self.wasm_store,
            request,
        )
    }

    /// Sends `request` and returns the `OK` payload, or the error code.
    pub fn call(&mut self, request: &[u8]) -> Result<Vec<u8>, ErrorCode> {
        match self.raw(request).as_slice() {
            [0x00, payload @ ..] => Ok(payload.to_vec()),
            [0x01, code] => Err(ErrorCode::from_byte(*code).expect("known error code")),
            other => panic!("malformed response {other:?}"),
        }
    }

    /// Uploads `module` over the wire in as many messages as it takes.
    pub fn upload(&mut self, module: &[u8]) -> Result<ProgramId, ErrorCode> {
        self.upload_with(module, None)
    }

    /// Uploads `module` with a signed manifest.
    pub fn upload_signed(
        &mut self,
        module: &[u8],
        manifest: &Signed,
    ) -> Result<ProgramId, ErrorCode> {
        self.upload_with(module, Some(manifest))
    }

    fn upload_with(
        &mut self,
        module: &[u8],
        manifest: Option<&Signed>,
    ) -> Result<ProgramId, ErrorCode> {
        let upload_id = self.call(&request::load_begin(module.len() as u32))?[0];
        let mut offset = 0;
        for chunk in module.chunks(request::max_chunk_len(MAX_MSG_LEN)) {
            self.call(&request::load_chunk(upload_id, offset, chunk))?;
            offset += chunk.len() as u32;
        }
        let finish = match manifest {
            Some(manifest) => request::load_finish_signed(upload_id, manifest),
            None => request::load_finish(upload_id),
        };
        let id = self.call(&finish)?;
        Ok(ProgramId(id.try_into().unwrap()))
    }

    pub fn listed(&mut self) -> Vec<ListedProgram> {
        let payload = self.call(&request::list_programs()).unwrap();
        seed_vault_wasm::protocol::decode_list_programs(&payload).expect("valid list")
    }

    pub fn programs(&mut self) -> Vec<ProgramId> {
        self.listed().into_iter().map(|p| p.id).collect()
    }

    /// Runs `program`'s `run` export on `input`: in one `RUN` message if it
    /// fits, else uploaded in chunks and run with `RUN_INPUT`.
    pub fn run(&mut self, program: &ProgramId, input: &[u8]) -> Result<Vec<u8>, ErrorCode> {
        let single = request::run(program, input);
        let payload = if single.len() <= MAX_MSG_LEN {
            self.call(&single)?
        } else {
            let id = self.call(&request::input_begin(input.len() as u32))?[0];
            let mut offset = 0;
            for chunk in input.chunks(request::max_chunk_len(MAX_MSG_LEN)) {
                self.call(&request::input_chunk(id, offset, chunk))?;
                offset += chunk.len() as u32;
            }
            self.call(&request::run_input(program, id))?
        };
        Ok(bytes_payload(&payload))
    }

    /// Sends a revocation list; returns the applied sequence.
    pub fn revoke(&mut self, list: &Signed) -> Result<u32, ErrorCode> {
        self.call(&request::revoke(list))
            .map(|p| u32::from_le_bytes(p.try_into().unwrap()))
    }

    pub fn public_key(
        &mut self,
        program: &ProgramId,
        address: &str,
        index: u8,
    ) -> Result<Vec<u8>, ErrorCode> {
        self.call(&request::public_key(program, index, address))
            .map(|p| bytes_payload(&p))
    }

    pub fn sign(
        &mut self,
        program: &ProgramId,
        address: &str,
        index: u8,
        message: &[u8],
    ) -> Result<Vec<u8>, ErrorCode> {
        self.call(&request::sign(program, index, address, message))
            .map(|p| bytes_payload(&p))
    }

    /// Names of stored blobs starting with `prefix`.
    pub fn stored(&mut self, prefix: &str) -> Vec<String> {
        use seed_vault_wasm::WasmStore;
        self.wasm_store.list(prefix).unwrap()
    }
}

/// Strips a `[len: u16 LE]` prefix, checking it.
pub fn bytes_payload(payload: &[u8]) -> Vec<u8> {
    let (len, bytes) = payload.split_at(2);
    assert_eq!(
        u16::from_le_bytes(len.try_into().unwrap()) as usize,
        bytes.len()
    );
    bytes.to_vec()
}
