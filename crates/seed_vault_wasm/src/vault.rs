//! Installed programs, chunked uploads, and running a program with a key.

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use seed_vault::{AccountStore, StoreError};
use wasm_runtime::{CompiledProgram, Limits, ProgramId, RuntimeConfig, WasmRuntime};
use zeroize::Zeroize;

use crate::abi::{
    self, CallContext, EXPORT_PUBLIC_KEY, EXPORT_RUN, EXPORT_SIGN, IMPORT_ACCOUNT_ENTROPY,
    IMPORT_MODULE, KEY_LEN,
};
use crate::manifest::{Manifest, Revocations, Signed};
use crate::policy::{Admission, WasmPolicy};
use crate::storage::{self, WasmStore};

/// Largest result a program may return: whatever fits in one response
/// (`[OK][len: u16][bytes]`) under `seed_vault`'s message budget.
pub const MAX_OUTPUT_LEN: usize = seed_vault::protocol::MAX_RESPONSE_LEN - 3;

/// Largest blob a program may cache per key (`cache_put`). A Falcon-1024
/// key pair is 4098 bytes.
pub const MAX_CACHE_LEN: usize = 8 * 1024;

/// Largest persistent state a program may keep (`state_put`). The light
/// client's oracle with 1000 commitments is ~32 KB.
pub const MAX_STATE_LEN: usize = 64 * 1024;

/// Domain separator for [`derive_program_key`].
const PROGRAM_KEY_DOMAIN: &[u8] = b"SeedVaultWasm";

/// Which keys a program gets: unsigned programs get keys bound to their
/// exact bytes ([`KeyNamespace::program`]); signed ones get keys bound to
/// their algorithm name ([`KeyNamespace::algorithm`]), so a new version of
/// an algorithm keeps the same accounts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyNamespace(pub [u8; 32]);

impl KeyNamespace {
    /// The namespace of an unsigned program: its `program_id`.
    pub fn program(program: &ProgramId) -> Self {
        KeyNamespace(program.0)
    }

    /// The namespace of every signed version of algorithm `name`:
    /// `SHA-256("SeedVaultWasmAlgorithm" || name)`. (Domain-separated from
    /// program IDs, which hash WASM modules, so the two can't collide.)
    pub fn algorithm(name: &str) -> Self {
        let mut input = Vec::from(&b"SeedVaultWasmAlgorithm"[..]);
        input.extend_from_slice(name.as_bytes());
        KeyNamespace(ProgramId::of(&input).0)
    }

    fn of(program: &ProgramId, admission: &Admission) -> Self {
        match &admission.manifest {
            Some(manifest) => Self::algorithm(&manifest.name),
            None => Self::program(program),
        }
    }
}

/// The key a program in `namespace` gets for key `address_index` of the
/// seed with mnemonic `entropy`:
/// `SHA512/256("SeedVaultWasm" || namespace || address_index as u32 LE || entropy)`.
///
/// One-way, so a program that leaks its key reveals nothing about the seed
/// or any other (namespace, index) key. The caller should zeroize the result.
pub fn derive_program_key(
    entropy: &[u8; 32],
    namespace: &KeyNamespace,
    address_index: u8,
) -> [u8; KEY_LEN] {
    const DOMAIN_LEN: usize = PROGRAM_KEY_DOMAIN.len();
    let mut input = [0u8; DOMAIN_LEN + 32 + 4 + 32];
    input[..DOMAIN_LEN].copy_from_slice(PROGRAM_KEY_DOMAIN);
    input[DOMAIN_LEN..DOMAIN_LEN + 32].copy_from_slice(&namespace.0);
    input[DOMAIN_LEN + 32..DOMAIN_LEN + 36]
        .copy_from_slice(&u32::from(address_index).to_le_bytes());
    input[DOMAIN_LEN + 36..].copy_from_slice(entropy);
    let key = algokit_crypto::sha512_256(&input);
    input.zeroize();
    key
}

/// Resource limits for a [`WasmVault`].
#[derive(Debug, Clone, Copy)]
pub struct VaultConfig {
    /// Largest module that can be uploaded, in bytes.
    pub max_program_len: usize,
    /// Most programs installed at once.
    pub max_programs: usize,
    /// Fuel and linear-memory cap for each `public_key`/`sign` call.
    pub call_limits: Limits,
    /// Most keys one program may cache (each up to [`MAX_CACHE_LEN`] of
    /// secure storage). Past this, `cache_put` is ignored and the program
    /// just re-derives its key on each call.
    pub max_cached_keys_per_program: usize,
    /// Largest `run` input, uploaded in chunks if it doesn't fit in one
    /// message (a state proof is ~16 KB).
    pub max_input_len: usize,
}

impl Default for VaultConfig {
    fn default() -> Self {
        VaultConfig {
            max_program_len: 128 * 1024,
            max_programs: 4,
            call_limits: Limits {
                // Falcon-1024 keygen + signing (guest/falcon1024, which
                // re-runs keygen on every call) needs 250M-500M fuel; this
                // leaves ~4x headroom while bounding how long a runaway
                // program can block the TA (~0.5 s per 500M on an M-series
                // Mac, likely 2-4x that in a phone's TEE).
                fuel: 2_000_000_000,
                memory_limit_bytes: 2 << 20,
            },
            max_cached_keys_per_program: 64,
            max_input_len: 64 * 1024,
        }
    }
}

/// Why a [`WasmVault`] operation failed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WasmError {
    /// The upload is bigger than [`VaultConfig::max_program_len`].
    ProgramTooLarge,
    /// [`VaultConfig::max_programs`] programs are already installed.
    TooManyPrograms,
    /// No upload with that ID is in progress (it finished, or a newer
    /// `begin_upload` replaced it).
    UnknownUpload,
    /// A chunk didn't start where the previous one ended, overran the
    /// declared length, or `finish_upload` came before all bytes arrived.
    BadChunk,
    /// The module isn't valid WASM, or doesn't follow [`crate::abi`].
    InvalidModule,
    /// The policy refused to install this module (unsigned modules when
    /// only signed ones are allowed, or a module importing
    /// `account_entropy` without that capability).
    InstallDenied,
    /// A manifest or revocation list is malformed, signed by an unknown or
    /// revoked key, has a bad signature, or is for a different module.
    BadSignature,
    /// A `run` input bigger than [`VaultConfig::max_input_len`].
    InputTooLarge,
    /// A lower version than one of the same algorithm already installed, or
    /// a revocation list no newer than the one already applied.
    Rollback,
    /// The module, or the key that signed it, has been revoked.
    Revoked,
    /// No program with that ID is installed.
    UnknownProgram,
    /// The program may not use the key it asked for (index 0 without the
    /// `human_sign` capability, or the policy said no).
    KeyUseDenied,
    /// The program trapped, returned non-zero, or set no output.
    ProgramFailed,
    /// The program used up its fuel.
    OutOfFuel,
    /// The seed lookup failed.
    Store(StoreError),
}

/// A module being uploaded in chunks.
struct Upload {
    id: u8,
    expected_len: usize,
    bytes: Vec<u8>,
}

/// An installed program.
struct Installed {
    program: CompiledProgram,
    admission: Admission,
    namespace: KeyNamespace,
}

/// What [`WasmVault::programs`] reports about one installed program.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProgramInfo {
    pub id: ProgramId,
    pub capabilities: crate::manifest::Capabilities,
    /// Exports `public_key` and `sign`: usable with `PUBLIC_KEY`/`SIGN`.
    pub can_sign: bool,
    /// Exports `run`: usable with `RUN`.
    pub can_run: bool,
    /// For signed programs.
    pub manifest: Option<Manifest>,
}

/// The installed WASM programs plus the runtime that runs them. One per TA.
///
/// Programs, their signed manifests, their cached keys, rollback versions
/// and the applied revocation list live in a [`WasmStore`] (Trusty secure
/// storage in the TA), so they survive a restart. They're loaded and
/// re-verified on first use ([`Self::ensure_restored`]), not at
/// construction: the TA starts before Android has booted, and Trusty's
/// `TamperDetect` storage can't be read until then.
pub struct WasmVault<P> {
    runtime: WasmRuntime<CallContext>,
    programs: BTreeMap<ProgramId, Installed>,
    revocations: Revocations,
    restored: bool,
    upload: Option<Upload>,
    /// A `run` input being uploaded in chunks (same shape as a module upload).
    input: Option<Upload>,
    next_upload_id: u8,
    config: VaultConfig,
    policy: P,
}

impl<P: WasmPolicy> WasmVault<P> {
    pub fn new(config: VaultConfig, policy: P) -> Self {
        let runtime = WasmRuntime::new(&RuntimeConfig::default(), abi::define_host_functions)
            .expect("seed_vault host functions are well-formed");
        WasmVault {
            runtime,
            programs: BTreeMap::new(),
            revocations: Revocations::default(),
            restored: false,
            upload: None,
            input: None,
            next_upload_id: 0,
            config,
            policy,
        }
    }

    /// The policy this vault enforces.
    pub fn policy(&self) -> &P {
        &self.policy
    }

    /// Loads (and re-verifies) everything saved in `wasm_store`, once. Every
    /// other operation that takes a `wasm_store` calls this first, and it's
    /// retried until it succeeds (storage may not be readable yet).
    ///
    /// A saved module that no longer passes (its bytes don't hash to its
    /// name, it doesn't compile, it's revoked or rolled back) is deleted.
    /// One the policy no longer admits is skipped but kept, in case the
    /// policy changes back. A saved revocation list that no longer verifies
    /// fails the restore: better no WASM than un-revoking modules.
    pub fn ensure_restored(&mut self, wasm_store: &mut impl WasmStore) -> Result<(), WasmError> {
        if self.restored {
            return Ok(());
        }
        if let Some(bytes) = wasm_store
            .read(storage::REVOCATIONS)
            .map_err(WasmError::Store)?
        {
            let signed =
                Signed::decode(&bytes).ok_or(WasmError::Store(StoreError::CorruptRecord))?;
            self.revocations = self
                .policy
                .verify_revocations(&signed)
                .map_err(|_| WasmError::Store(StoreError::CorruptRecord))?;
        }
        let names = wasm_store
            .list(storage::PROGRAM_PREFIX)
            .map_err(WasmError::Store)?;
        for name in names {
            let Some(id) = storage::program_from_name(&name) else {
                continue;
            };
            let Some(bytes) = wasm_store.read(&name).map_err(WasmError::Store)? else {
                continue;
            };
            let signed = wasm_store
                .read(&storage::manifest_name(&id))
                .map_err(WasmError::Store)?
                .map(|raw| Signed::decode(&raw));
            let result = match signed {
                Some(None) => Err(WasmError::BadSignature),
                Some(Some(signed)) => {
                    self.admit_and_compile(wasm_store, &id, &bytes, Some(&signed))
                }
                None => self.admit_and_compile(wasm_store, &id, &bytes, None),
            };
            match result {
                Ok(installed) => self.insert(installed, id),
                // Kept: the policy may allow it again later.
                Err(WasmError::InstallDenied) | Err(WasmError::TooManyPrograms) => {}
                Err(WasmError::Store(e)) => return Err(WasmError::Store(e)),
                Err(_) => delete_program_files(wasm_store, &id)?,
            }
        }
        self.restored = true;
        Ok(())
    }

    /// Starts uploading a `len`-byte module and returns the upload's ID.
    /// Only one upload runs at a time: this discards any unfinished one, so
    /// a client that died midway can't block everyone else.
    pub fn begin_upload(&mut self, len: u32) -> Result<u8, WasmError> {
        let len = len as usize;
        if len == 0 || len > self.config.max_program_len {
            return Err(WasmError::ProgramTooLarge);
        }
        // `max_programs` is checked in `finish_upload`, once the program's
        // ID is known: re-uploading an installed program must still work
        // when the vault is full.
        let id = self.next_upload_id;
        self.next_upload_id = self.next_upload_id.wrapping_add(1);
        self.upload = Some(Upload {
            id,
            expected_len: len,
            bytes: Vec::new(),
        });
        Ok(id)
    }

    /// Appends `chunk`, which must start at `offset` == bytes received so far.
    pub fn upload_chunk(
        &mut self,
        upload_id: u8,
        offset: u32,
        chunk: &[u8],
    ) -> Result<(), WasmError> {
        let upload = self.upload_mut(upload_id)?;
        let fits = upload.bytes.len() + chunk.len() <= upload.expected_len;
        if offset as usize != upload.bytes.len() || !fits {
            return Err(WasmError::BadChunk);
        }
        upload.bytes.extend_from_slice(chunk);
        Ok(())
    }

    /// Verifies, saves to `wasm_store`, and installs the completed upload,
    /// returning its ID. `signed` is its signed manifest, if any.
    ///
    /// Installing a program that's already installed is a no-op. A signed
    /// version of an algorithm replaces any lower version installed (and
    /// from then on, lower versions are refused).
    pub fn finish_upload(
        &mut self,
        wasm_store: &mut impl WasmStore,
        upload_id: u8,
        signed: Option<&Signed>,
    ) -> Result<ProgramId, WasmError> {
        self.ensure_restored(wasm_store)?;
        let upload = self.upload_mut(upload_id)?;
        if upload.bytes.len() != upload.expected_len {
            return Err(WasmError::BadChunk);
        }
        let Upload { bytes, .. } = self.upload.take().expect("checked above");
        let id = ProgramId::of(&bytes);
        if self.programs.contains_key(&id) {
            return Ok(id);
        }
        let installed = self.admit_and_compile(wasm_store, &id, &bytes, signed)?;
        // Versions of the same algorithm it replaces don't count.
        let replaced = self.older_versions(&installed);
        if self.programs.len() - replaced.len() >= self.config.max_programs {
            return Err(WasmError::TooManyPrograms);
        }

        wasm_store
            .write(&storage::program_name(&id), &bytes)
            .map_err(WasmError::Store)?;
        match signed {
            Some(signed) => wasm_store
                .write(&storage::manifest_name(&id), &signed.encode())
                .map_err(WasmError::Store)?,
            None => wasm_store
                .delete(&storage::manifest_name(&id))
                .map_err(WasmError::Store)?,
        }
        if let Some(manifest) = &installed.admission.manifest {
            // Rollback protection: from now on, nothing lower.
            wasm_store
                .write(
                    &storage::min_version_name(&manifest.name),
                    &manifest.version.to_le_bytes(),
                )
                .map_err(WasmError::Store)?;
        }
        for old in replaced {
            self.remove(wasm_store, &old)?;
        }
        self.insert(installed, id);
        Ok(id)
    }

    /// Applies a signed revocation list: from now on, the modules and
    /// publisher keys it names are refused, and installed ones are removed
    /// (with their cached keys). Only a list with a higher sequence than the
    /// last one applied is accepted; it replaces that one.
    pub fn apply_revocations(
        &mut self,
        wasm_store: &mut impl WasmStore,
        signed: &Signed,
    ) -> Result<u32, WasmError> {
        self.ensure_restored(wasm_store)?;
        let revocations = self.policy.verify_revocations(signed)?;
        // Sequence 0 is "none applied yet", so the first list needs >= 1.
        if revocations.sequence <= self.revocations.sequence {
            return Err(WasmError::Rollback);
        }
        wasm_store
            .write(storage::REVOCATIONS, &signed.encode())
            .map_err(WasmError::Store)?;
        let sequence = revocations.sequence;
        self.revocations = revocations;
        let revoked: Vec<ProgramId> = self
            .programs
            .iter()
            .filter(|(id, installed)| self.is_revoked(id, &installed.admission))
            .map(|(id, _)| *id)
            .collect();
        for id in revoked {
            self.remove(wasm_store, &id)?;
        }
        Ok(sequence)
    }

    /// Removes an installed program, from memory and storage, with every
    /// key it cached.
    pub fn unload(
        &mut self,
        wasm_store: &mut impl WasmStore,
        program: &ProgramId,
    ) -> Result<(), WasmError> {
        self.ensure_restored(wasm_store)?;
        let Some(installed) = self.programs.get(program) else {
            return Err(WasmError::UnknownProgram);
        };
        let namespace = installed.namespace;
        self.remove(wasm_store, program)?;
        // Its state goes too, unless another installed program shares it.
        // (Upgrades and revocations keep it: a later version picks it up.)
        if !self.programs.values().any(|p| p.namespace == namespace) {
            wasm_store
                .delete(&storage::state_name(&namespace.0))
                .map_err(WasmError::Store)?;
        }
        Ok(())
    }

    /// The installed programs, in ascending ID order.
    pub fn programs(
        &mut self,
        wasm_store: &mut impl WasmStore,
    ) -> Result<Vec<ProgramInfo>, WasmError> {
        self.ensure_restored(wasm_store)?;
        Ok(self
            .programs
            .iter()
            .map(|(id, installed)| ProgramInfo {
                id: *id,
                capabilities: installed.admission.capabilities,
                can_sign: installed.program.exports_call(EXPORT_PUBLIC_KEY)
                    && installed.program.exports_call(EXPORT_SIGN),
                can_run: installed.program.exports_call(EXPORT_RUN),
                manifest: installed.admission.manifest.clone(),
            })
            .collect())
    }

    /// Deletes every cached key whose seed is no longer in `accounts`. The
    /// TA calls this after a successful `DELETE_ACCOUNT`, so keys derived
    /// from a deleted seed don't outlive it.
    pub fn purge_orphaned_keys(
        &mut self,
        accounts: &mut impl AccountStore,
        wasm_store: &mut impl WasmStore,
    ) -> Result<(), WasmError> {
        let mut entropies = seed_vault::seed_entropies(accounts).map_err(WasmError::Store)?;
        let live: Vec<_> = entropies
            .iter()
            .map(|entropy| storage::seed_tag_hex(&storage::seed_tag(entropy)))
            .collect();
        entropies.zeroize();
        for name in wasm_store
            .list(storage::KEY_PREFIX)
            .map_err(WasmError::Store)?
        {
            let orphaned =
                storage::seed_tag_from_key_name(&name).is_none_or(|tag| !live.contains(&tag));
            if orphaned {
                wasm_store.delete(&name).map_err(WasmError::Store)?;
            }
        }
        Ok(())
    }

    /// Runs `program`'s `public_key` with its key `address_index` for the
    /// seed whose human-sign address is `seed_address`.
    pub fn public_key(
        &mut self,
        accounts: &mut impl AccountStore,
        wasm_store: &mut impl WasmStore,
        program: &ProgramId,
        seed_address: &str,
        address_index: u8,
    ) -> Result<Vec<u8>, WasmError> {
        let call = Call {
            program,
            seed_address,
            address_index,
            export: EXPORT_PUBLIC_KEY,
        };
        self.run(accounts, wasm_store, call, Vec::new())
    }

    /// Runs `program`'s `sign` over `message`, with the same key as
    /// [`Self::public_key`].
    ///
    /// The bytes are signed as given: it's up to the program what it
    /// signs. That's safe here because the key belongs to this program
    /// alone, unlike `seed_vault`'s native keys, whose arbitrary-data
    /// signing refuses Algorand domain prefixes.
    pub fn sign(
        &mut self,
        accounts: &mut impl AccountStore,
        wasm_store: &mut impl WasmStore,
        program: &ProgramId,
        seed_address: &str,
        address_index: u8,
        message: &[u8],
    ) -> Result<Vec<u8>, WasmError> {
        let call = Call {
            program,
            seed_address,
            address_index,
            export: EXPORT_SIGN,
        };
        self.run(accounts, wasm_store, call, message.to_vec())
    }

    fn run(
        &mut self,
        accounts: &mut impl AccountStore,
        wasm_store: &mut impl WasmStore,
        call: Call<'_>,
        input: Vec<u8>,
    ) -> Result<Vec<u8>, WasmError> {
        self.ensure_restored(wasm_store)?;
        let installed = self
            .programs
            .get(call.program)
            .ok_or(WasmError::UnknownProgram)?;
        let capabilities = installed.admission.capabilities;
        if call.address_index == seed_vault::HUMAN_SIGN_INDEX as u8 && !capabilities.human_sign {
            return Err(WasmError::KeyUseDenied);
        }
        if !self
            .policy
            .allow_use_key(call.program, call.seed_address, call.address_index)
        {
            return Err(WasmError::KeyUseDenied);
        }
        let mut entropy =
            seed_vault::seed_entropy(accounts, call.seed_address).map_err(WasmError::Store)?;
        let key = derive_program_key(&entropy, &installed.namespace, call.address_index);
        // Only for modules granted it (install refuses modules that import
        // `account_entropy` without the capability).
        let account_entropy = if capabilities.account_entropy {
            Some(
                seed_vault::store::falcon_key_entropy(&entropy, u32::from(call.address_index))
                    .map_err(WasmError::Store)?,
            )
        } else {
            None
        };
        let cache_name = storage::key_name(
            call.program,
            &storage::seed_tag(&entropy),
            call.address_index,
        );
        entropy.zeroize();
        let cached = wasm_store.read(&cache_name).map_err(WasmError::Store)?;

        // `CallContext` zeroizes the key, entropy, input and cache data on drop.
        let context = CallContext::new(Some(key), input, MAX_OUTPUT_LEN, cached, MAX_CACHE_LEN)
            .with_key_info(call.address_index, account_entropy);
        let (output, cache_put) = self.execute(wasm_store, call.program, context, call.export)?;
        if let Some(mut blob) = cache_put {
            // Best effort: the result is valid either way, and a program
            // must cope with an empty cache.
            let _ = self.save_cache(wasm_store, call.program, &cache_name, &blob);
            blob.zeroize();
        }
        Ok(output)
    }

    /// Runs `program`'s `run` export on `input` (general computation, not
    /// tied to a seed or key), with its persistent state.
    pub fn run_program(
        &mut self,
        wasm_store: &mut impl WasmStore,
        program: &ProgramId,
        input: Vec<u8>,
    ) -> Result<Vec<u8>, WasmError> {
        self.ensure_restored(wasm_store)?;
        if !self.programs.contains_key(program) {
            return Err(WasmError::UnknownProgram);
        }
        let context = CallContext::new(None, input, MAX_OUTPUT_LEN, None, 0);
        let (output, _) = self.execute(wasm_store, program, context, EXPORT_RUN)?;
        Ok(output)
    }

    /// Starts uploading a `len`-byte `run` input (for inputs too big for
    /// one message) and returns its ID. Like module uploads, one at a time.
    pub fn begin_input(&mut self, len: u32) -> Result<u8, WasmError> {
        let len = len as usize;
        if len > self.config.max_input_len {
            return Err(WasmError::InputTooLarge);
        }
        let id = self.next_upload_id;
        self.next_upload_id = self.next_upload_id.wrapping_add(1);
        self.input = Some(Upload {
            id,
            expected_len: len,
            bytes: Vec::new(),
        });
        Ok(id)
    }

    /// Appends `chunk` to the input being uploaded, at `offset`.
    pub fn input_chunk(
        &mut self,
        input_id: u8,
        offset: u32,
        chunk: &[u8],
    ) -> Result<(), WasmError> {
        let input = self
            .input
            .as_mut()
            .filter(|input| input.id == input_id)
            .ok_or(WasmError::UnknownUpload)?;
        let fits = input.bytes.len() + chunk.len() <= input.expected_len;
        if offset as usize != input.bytes.len() || !fits {
            return Err(WasmError::BadChunk);
        }
        input.bytes.extend_from_slice(chunk);
        Ok(())
    }

    /// [`Self::run_program`] on the completed uploaded input (consumed).
    pub fn run_uploaded_input(
        &mut self,
        wasm_store: &mut impl WasmStore,
        program: &ProgramId,
        input_id: u8,
    ) -> Result<Vec<u8>, WasmError> {
        let complete = self
            .input
            .as_ref()
            .filter(|input| input.id == input_id)
            .ok_or(WasmError::UnknownUpload)?;
        if complete.bytes.len() != complete.expected_len {
            return Err(WasmError::BadChunk);
        }
        let Upload { bytes, .. } = self.input.take().expect("checked above");
        self.run_program(wasm_store, program, bytes)
    }

    /// Instantiates `program`, runs `export` with `context` plus the
    /// program's saved state, wipes the instance, and saves the new state if
    /// the call succeeded. Returns the output and any `cache_put` blob.
    fn execute(
        &self,
        wasm_store: &mut impl WasmStore,
        program: &ProgramId,
        context: CallContext,
        export: &str,
    ) -> Result<(Vec<u8>, Option<Vec<u8>>), WasmError> {
        let installed = self
            .programs
            .get(program)
            .ok_or(WasmError::UnknownProgram)?;
        let state_name = storage::state_name(&installed.namespace.0);
        let state = wasm_store.read(&state_name).map_err(WasmError::Store)?;
        let context = context.with_state(state, MAX_STATE_LEN);
        let mut instance = self
            .runtime
            .instantiate(&installed.program, context, &self.config.call_limits)
            .map_err(|_| WasmError::ProgramFailed)?;
        let result = instance.call(export);
        let output = instance.data_mut().take_output();
        let cache_put = instance.data_mut().take_cache_put();
        let state_put = instance.data_mut().take_state_put();
        instance.wipe_memory();
        drop(instance);

        let output = match result {
            Ok(0) => output.ok_or(WasmError::ProgramFailed)?,
            Ok(_) => return Err(WasmError::ProgramFailed),
            Err(wasm_runtime::Error::OutOfFuel) => return Err(WasmError::OutOfFuel),
            Err(_) => return Err(WasmError::ProgramFailed),
        };
        if let Some(state) = state_put {
            // Not best effort: a program whose state didn't persist must not
            // report success (e.g. an oracle that advanced).
            wasm_store
                .write(&state_name, &state)
                .map_err(WasmError::Store)?;
        }
        Ok((output, cache_put))
    }

    /// Everything install and restore check, in order: the policy (and
    /// signature), revocation, rollback, then the module itself.
    fn admit_and_compile(
        &self,
        wasm_store: &mut impl WasmStore,
        id: &ProgramId,
        bytes: &[u8],
        signed: Option<&Signed>,
    ) -> Result<Installed, WasmError> {
        if ProgramId::of(bytes) != *id {
            return Err(WasmError::BadSignature);
        }
        let admission = self.policy.admit(id, signed)?;
        if self.is_revoked(id, &admission) {
            return Err(WasmError::Revoked);
        }
        if let Some(manifest) = &admission.manifest {
            let min = wasm_store
                .read(&storage::min_version_name(&manifest.name))
                .map_err(WasmError::Store)?
                .and_then(|raw| Some(u32::from_le_bytes(raw.as_slice().try_into().ok()?)))
                .unwrap_or(0);
            if manifest.version < min {
                return Err(WasmError::Rollback);
            }
        }
        let program = self.compile(bytes)?;
        if program.imports(IMPORT_MODULE, IMPORT_ACCOUNT_ENTROPY)
            && !admission.capabilities.account_entropy
        {
            return Err(WasmError::InstallDenied);
        }
        Ok(Installed {
            namespace: KeyNamespace::of(id, &admission),
            program,
            admission,
        })
    }

    fn is_revoked(&self, id: &ProgramId, admission: &Admission) -> bool {
        self.revocations.revoked_programs.contains(id)
            || admission
                .manifest
                .as_ref()
                .is_some_and(|m| self.revocations.revoked_keys.contains(&m.key_id))
    }

    /// Installed programs that `new` replaces: lower or equal versions of
    /// the same signed algorithm.
    fn older_versions(&self, new: &Installed) -> Vec<ProgramId> {
        let Some(new) = &new.admission.manifest else {
            return Vec::new();
        };
        self.programs
            .iter()
            .filter(|(_, old)| {
                old.admission
                    .manifest
                    .as_ref()
                    .is_some_and(|old| old.name == new.name && old.version <= new.version)
            })
            .map(|(id, _)| *id)
            .collect()
    }

    /// Adds a restored or newly installed program, keeping only the highest
    /// version of each signed algorithm in memory.
    fn insert(&mut self, installed: Installed, id: ProgramId) {
        if let Some(new) = &installed.admission.manifest {
            let newer_exists = self.programs.values().any(|old| {
                old.admission
                    .manifest
                    .as_ref()
                    .is_some_and(|old| old.name == new.name && old.version > new.version)
            });
            if newer_exists {
                return;
            }
        }
        if self.programs.len() < self.config.max_programs {
            self.programs.insert(id, installed);
        }
    }

    /// Deletes a program from memory and storage, with its cached keys.
    fn remove(
        &mut self,
        wasm_store: &mut impl WasmStore,
        program: &ProgramId,
    ) -> Result<(), WasmError> {
        delete_program_files(wasm_store, program)?;
        self.programs.remove(program);
        Ok(())
    }

    fn save_cache(
        &self,
        wasm_store: &mut impl WasmStore,
        program: &ProgramId,
        name: &str,
        blob: &[u8],
    ) -> Result<(), WasmError> {
        let existing = wasm_store
            .list(&storage::program_keys_prefix(program))
            .map_err(WasmError::Store)?;
        let full = existing.len() >= self.config.max_cached_keys_per_program;
        if full && !existing.iter().any(|n| n == name) {
            return Ok(());
        }
        wasm_store.write(name, blob).map_err(WasmError::Store)
    }

    fn compile(&self, bytes: &[u8]) -> Result<CompiledProgram, WasmError> {
        let program = self
            .runtime
            .compile(bytes)
            .map_err(|_| WasmError::InvalidModule)?;
        let signs = program.exports_call(EXPORT_PUBLIC_KEY) && program.exports_call(EXPORT_SIGN);
        let follows_abi = program.exports_memory() && (signs || program.exports_call(EXPORT_RUN));
        if follows_abi {
            Ok(program)
        } else {
            Err(WasmError::InvalidModule)
        }
    }

    fn upload_mut(&mut self, upload_id: u8) -> Result<&mut Upload, WasmError> {
        self.upload
            .as_mut()
            .filter(|upload| upload.id == upload_id)
            .ok_or(WasmError::UnknownUpload)
    }
}

/// Deletes a program's module, manifest and cached keys from storage.
fn delete_program_files(
    wasm_store: &mut impl WasmStore,
    program: &ProgramId,
) -> Result<(), WasmError> {
    for name in wasm_store
        .list(&storage::program_keys_prefix(program))
        .map_err(WasmError::Store)?
    {
        wasm_store.delete(&name).map_err(WasmError::Store)?;
    }
    wasm_store
        .delete(&storage::manifest_name(program))
        .map_err(WasmError::Store)?;
    wasm_store
        .delete(&storage::program_name(program))
        .map_err(WasmError::Store)
}

/// Which export to run with which key.
struct Call<'a> {
    program: &'a ProgramId,
    seed_address: &'a str,
    address_index: u8,
    export: &'a str,
}
