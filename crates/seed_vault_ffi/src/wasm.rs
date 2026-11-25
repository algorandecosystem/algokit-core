//! [`SeedVaultClient`] methods for `seed_vault_ta`'s WASM runtime
//! (`seed_vault_wasm`): installing programs, signing with WASM algorithms,
//! running general programs, and typed helpers for the light client
//! program.
//!
//! The generic methods work with any program: [`SeedVaultClient::wasm_list_programs`]
//! says what's installed (a signed program's name identifies it, e.g.
//! `light-client`) and whether it signs ([`SeedVaultClient::wasm_sign`]) or
//! runs ([`SeedVaultClient::wasm_run`]). New programs need no new bindings,
//! only knowledge of their input/output format; the `light_client_*`
//! methods are such knowledge, packaged for one program.
//!
//! Requests are encoded and responses decoded by `seed_vault_wasm::client`
//! and `seed_vault_wasm::light_client`, the same code the TA-side tests run
//! against the real programs.

use seed_vault_wasm::client::{Client, ClientError};
use seed_vault_wasm::light_client::{self as lc, LightClientCallError};
use seed_vault_wasm::protocol::ErrorCode;
use seed_vault_wasm::{ProgramId, Signed};

use crate::{ProtocolVersion, SeedVaultClient, SeedVaultError};

// ---- Types ------------------------------------------------------------------

/// An installed WASM program.
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct WasmProgram {
    /// SHA-256 of the module: pass it to the other `wasm_*` calls.
    pub program_id: Vec<u8>,
    /// Signed by a publisher key the TA trusts.
    pub signed: bool,
    /// The algorithm/program name from its signed manifest (e.g.
    /// `light-client`); `None` for unsigned (development) programs.
    pub name: Option<String>,
    /// Its version (0 if unsigned).
    pub version: u32,
    /// Usable with [`SeedVaultClient::wasm_public_key`]/[`SeedVaultClient::wasm_sign`].
    pub can_sign: bool,
    /// Usable with [`SeedVaultClient::wasm_run`].
    pub can_run: bool,
    /// May read the native accounts' key material.
    pub account_entropy: bool,
    /// May use `address_index` 0.
    pub human_sign: bool,
}

/// Mirrors `seed_vault_wasm::protocol::ErrorCode`: an error the WASM
/// runtime reported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum WasmErrorCode {
    MalformedRequest,
    ProgramTooLarge,
    TooManyPrograms,
    UnknownUpload,
    BadChunk,
    InvalidModule,
    InstallDenied,
    UnknownProgram,
    AccountNotFound,
    KeyUseDenied,
    ProgramFailed,
    OutOfFuel,
    StoreFailure,
    BadSignature,
    Rollback,
    Revoked,
    InputTooLarge,
}

impl From<ErrorCode> for WasmErrorCode {
    fn from(code: ErrorCode) -> Self {
        match code {
            ErrorCode::MalformedRequest => Self::MalformedRequest,
            ErrorCode::ProgramTooLarge => Self::ProgramTooLarge,
            ErrorCode::TooManyPrograms => Self::TooManyPrograms,
            ErrorCode::UnknownUpload => Self::UnknownUpload,
            ErrorCode::BadChunk => Self::BadChunk,
            ErrorCode::InvalidModule => Self::InvalidModule,
            ErrorCode::InstallDenied => Self::InstallDenied,
            ErrorCode::UnknownProgram => Self::UnknownProgram,
            ErrorCode::AccountNotFound => Self::AccountNotFound,
            ErrorCode::KeyUseDenied => Self::KeyUseDenied,
            ErrorCode::ProgramFailed => Self::ProgramFailed,
            ErrorCode::OutOfFuel => Self::OutOfFuel,
            ErrorCode::StoreFailure => Self::StoreFailure,
            ErrorCode::BadSignature => Self::BadSignature,
            ErrorCode::Rollback => Self::Rollback,
            ErrorCode::Revoked => Self::Revoked,
            ErrorCode::InputTooLarge => Self::InputTooLarge,
        }
    }
}

/// Mirrors `seed_vault_wasm::light_client::LightClientError`: why the light
/// client program refused (e.g. a state proof or transaction didn't verify).
#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum LightClientErrorCode {
    MalformedCommand,
    NotInitialized,
    AlreadyInitialized,
    InvalidArgument,
    CorruptState,
    MalformedStateProof,
    UnknownHash,
    TreeTooDeep,
    PositionOutOfBounds,
    NonEmptyProofForEmptySet,
    MissingSibling,
    MalformedDigest,
    RootMismatch,
    BadSignature,
    ZeroKeyLifetime,
    SaltVersionMismatch,
    TooManyReveals,
    ZeroSignedWeight,
    InsufficientSignedWeight,
    NoRevealInPosition,
    CoinNotInRange,
    ProofLengthMismatch,
    InvalidTreeDepth,
    IndexDepthMismatch,
    RoundTooEarly,
    NoStateProofForRound,
    /// A status this library doesn't know (a newer light client program).
    Unknown {
        status: u8,
    },
}

impl From<lc::LightClientError> for LightClientErrorCode {
    fn from(error: lc::LightClientError) -> Self {
        use lc::LightClientError as E;
        match error {
            E::MalformedCommand => Self::MalformedCommand,
            E::NotInitialized => Self::NotInitialized,
            E::AlreadyInitialized => Self::AlreadyInitialized,
            E::InvalidArgument => Self::InvalidArgument,
            E::CorruptState => Self::CorruptState,
            E::MalformedStateProof => Self::MalformedStateProof,
            E::UnknownHash => Self::UnknownHash,
            E::TreeTooDeep => Self::TreeTooDeep,
            E::PositionOutOfBounds => Self::PositionOutOfBounds,
            E::NonEmptyProofForEmptySet => Self::NonEmptyProofForEmptySet,
            E::MissingSibling => Self::MissingSibling,
            E::MalformedDigest => Self::MalformedDigest,
            E::RootMismatch => Self::RootMismatch,
            E::BadSignature => Self::BadSignature,
            E::ZeroKeyLifetime => Self::ZeroKeyLifetime,
            E::SaltVersionMismatch => Self::SaltVersionMismatch,
            E::TooManyReveals => Self::TooManyReveals,
            E::ZeroSignedWeight => Self::ZeroSignedWeight,
            E::InsufficientSignedWeight => Self::InsufficientSignedWeight,
            E::NoRevealInPosition => Self::NoRevealInPosition,
            E::CoinNotInRange => Self::CoinNotInRange,
            E::ProofLengthMismatch => Self::ProofLengthMismatch,
            E::InvalidTreeDepth => Self::InvalidTreeDepth,
            E::IndexDepthMismatch => Self::IndexDepthMismatch,
            E::RoundTooEarly => Self::RoundTooEarly,
            E::NoStateProofForRound => Self::NoStateProofForRound,
            E::Unknown(status) => Self::Unknown { status },
        }
    }
}

/// A state proof message, as algod returns it with a state proof
/// (`GET /v2/stateproofs/{round}` -> `Message`).
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct StateProofMessage {
    pub block_headers_commitment: Vec<u8>,
    pub voters_commitment: Vec<u8>,
    pub ln_proven_weight: u64,
    pub first_attested_round: u64,
    pub last_attested_round: u64,
}

/// algod's transaction proof (`GET /v2/blocks/{round}/transactions/{txid}/proof`
/// with `hashtype=sha256`).
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct TransactionProof {
    /// Must be `sha256`.
    pub hash_type: String,
    /// 32 bytes.
    pub stib_hash: Vec<u8>,
    pub index: u64,
    pub tree_depth: u64,
    pub proof: Vec<u8>,
}

/// algod's light block header proof (`GET /v2/blocks/{round}/lightheader/proof`).
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct LightBlockHeaderProof {
    pub index: u64,
    pub tree_depth: u64,
    pub proof: Vec<u8>,
}

/// The light client oracle's state.
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct LightClientStatus {
    pub first_attested_round: u64,
    pub interval_size: u64,
    pub capacity: u64,
    pub earliest_interval: u64,
    /// State proof intervals verified so far.
    pub next_interval: u64,
    pub ln_proven_weight: u64,
    pub voters_commitment: Vec<u8>,
}

// ---- Error mapping --------------------------------------------------------------

impl From<ClientError<SeedVaultError>> for SeedVaultError {
    fn from(error: ClientError<SeedVaultError>) -> Self {
        match error {
            ClientError::Transport(e) => e,
            ClientError::Applet(code) => SeedVaultError::Wasm { code: code.into() },
            ClientError::MalformedResponse => SeedVaultError::MalformedResponse {
                reason: "malformed WASM response".into(),
            },
            ClientError::InvalidArgument(reason) => SeedVaultError::InvalidArgument {
                reason: reason.into(),
            },
        }
    }
}

impl From<LightClientCallError<SeedVaultError>> for SeedVaultError {
    fn from(error: LightClientCallError<SeedVaultError>) -> Self {
        match error {
            LightClientCallError::Client(e) => e.into(),
            LightClientCallError::LightClient(e) => SeedVaultError::LightClient { code: e.into() },
        }
    }
}

fn program_id(bytes: &[u8]) -> Result<ProgramId, SeedVaultError> {
    Ok(ProgramId(bytes.try_into().map_err(|_| {
        SeedVaultError::InvalidArgument {
            reason: "program_id must be 32 bytes".into(),
        }
    })?))
}

fn bytes32(name: &str, bytes: &[u8]) -> Result<[u8; 32], SeedVaultError> {
    bytes
        .try_into()
        .map_err(|_| SeedVaultError::InvalidArgument {
            reason: format!("{name} must be 32 bytes"),
        })
}

fn signed(name: &str, bytes: &[u8]) -> Result<Signed, SeedVaultError> {
    Signed::decode(bytes).ok_or_else(|| SeedVaultError::InvalidArgument {
        reason: format!("{name} is not a signed manifest/revocation file"),
    })
}

impl SeedVaultClient {
    /// Runs `f` with a WASM client over one connection (chunked uploads
    /// take many messages).
    fn with_wasm<T>(
        &self,
        f: impl FnOnce(
            &mut Client<&mut dyn FnMut(&[u8]) -> Result<Vec<u8>, SeedVaultError>>,
        ) -> Result<T, SeedVaultError>,
    ) -> Result<T, SeedVaultError> {
        #[cfg(feature = "emulator")]
        if let Some(emulator) = &self.emulator {
            let mut exchange =
                |request: &[u8]| -> Result<Vec<u8>, SeedVaultError> { emulator.exchange(request) };
            let mut client = Client::new(&mut exchange as &mut dyn FnMut(&[u8]) -> _);
            return f(&mut client);
        }
        let mut connection = self.connect()?;
        let timeout_ms = self.timeout_ms;
        let mut exchange = |request: &[u8]| -> Result<Vec<u8>, SeedVaultError> {
            connection.call(request, timeout_ms)
        };
        let mut client = Client::new(&mut exchange as &mut dyn FnMut(&[u8]) -> _);
        f(&mut client)
    }
}

// ---- Methods ----------------------------------------------------------------------

#[uniffi::export]
impl SeedVaultClient {
    /// The TA's WASM protocol version (`seed_vault_wasm::protocol`), separate
    /// from [`SeedVaultClient::protocol_version`]. A TA built without WASM
    /// answers [`SeedVaultError::Applet`] `MalformedRequest`.
    pub fn wasm_protocol_version(&self) -> Result<ProtocolVersion, SeedVaultError> {
        self.with_wasm(|c| {
            let (major, minor) = c.version()?;
            Ok(ProtocolVersion { major, minor })
        })
    }

    /// Installs a WASM module (any size up to the TA's limit; it's uploaded
    /// in pieces) and returns its `program_id`. `signed_manifest` is the
    /// publisher-signed manifest file (`seed_vault_wasm_sign sign` output);
    /// production TAs refuse modules without one. Installing an installed
    /// program again is harmless.
    pub fn wasm_install(
        &self,
        module: Vec<u8>,
        signed_manifest: Option<Vec<u8>>,
    ) -> Result<Vec<u8>, SeedVaultError> {
        let manifest = signed_manifest
            .map(|bytes| signed("signed_manifest", &bytes))
            .transpose()?;
        self.with_wasm(|c| Ok(c.install(&module, manifest.as_ref())?.0.to_vec()))
    }

    /// Removes an installed program, with its cached keys and state.
    pub fn wasm_unload(&self, program_id: Vec<u8>) -> Result<(), SeedVaultError> {
        let program = self::program_id(&program_id)?;
        self.with_wasm(|c| Ok(c.unload(&program)?))
    }

    /// The installed programs, and what each can be used for.
    pub fn wasm_list_programs(&self) -> Result<Vec<WasmProgram>, SeedVaultError> {
        self.with_wasm(|c| {
            Ok(c.list_programs()?
                .into_iter()
                .map(|p| WasmProgram {
                    program_id: p.id.0.to_vec(),
                    signed: p.signed,
                    name: p.signed.then_some(p.name),
                    version: p.version,
                    can_sign: p.can_sign,
                    can_run: p.can_run,
                    account_entropy: p.account_entropy,
                    human_sign: p.human_sign,
                })
                .collect())
        })
    }

    /// A signing program's public key for key `address_index` of the seed
    /// whose index-0 address is `address`.
    pub fn wasm_public_key(
        &self,
        program_id: Vec<u8>,
        address: String,
        address_index: u8,
    ) -> Result<Vec<u8>, SeedVaultError> {
        let program = self::program_id(&program_id)?;
        self.with_wasm(|c| Ok(c.public_key(&program, address.trim(), address_index)?))
    }

    /// A signing program's signature over `message` (raw bytes, signed as
    /// given), with the same key as [`SeedVaultClient::wasm_public_key`].
    pub fn wasm_sign(
        &self,
        program_id: Vec<u8>,
        address: String,
        address_index: u8,
        message: Vec<u8>,
    ) -> Result<Vec<u8>, SeedVaultError> {
        let program = self::program_id(&program_id)?;
        self.with_wasm(|c| Ok(c.sign(&program, address.trim(), address_index, &message)?))
    }

    /// Runs a program's `run` export on `input` (uploaded in pieces if
    /// large) and returns its output. The format of both is the program's.
    pub fn wasm_run(&self, program_id: Vec<u8>, input: Vec<u8>) -> Result<Vec<u8>, SeedVaultError> {
        let program = self::program_id(&program_id)?;
        self.with_wasm(|c| Ok(c.run(&program, &input)?))
    }

    /// Applies a publisher-signed revocation list (`seed_vault_wasm_sign
    /// revoke` output); returns its sequence number.
    pub fn wasm_revoke(&self, signed_revocations: Vec<u8>) -> Result<u32, SeedVaultError> {
        let list = signed("signed_revocations", &signed_revocations)?;
        self.with_wasm(|c| Ok(c.revoke(&list)?))
    }

    // ---- Light client (`guest/light_client`) ----

    /// Initializes the light client program's oracle from trusted genesis
    /// data -- once: later calls fail with `AlreadyInitialized` (unload the
    /// program to start over). `capacity` is how many state proof intervals
    /// of history it keeps.
    pub fn light_client_init(
        &self,
        program_id: Vec<u8>,
        first_attested_round: u64,
        interval_size: u64,
        capacity: u64,
        genesis_voters_commitment: Vec<u8>,
        genesis_ln_proven_weight: u64,
    ) -> Result<(), SeedVaultError> {
        let program = self::program_id(&program_id)?;
        self.with_wasm(|c| {
            Ok(lc::init(
                c,
                &program,
                first_attested_round,
                interval_size,
                capacity,
                &genesis_voters_commitment,
                genesis_ln_proven_weight,
            )?)
        })
    }

    /// Verifies `state_proof` (msgpack, as algod returns it) attests to
    /// `message`, and if so advances the oracle. Returns the number of
    /// intervals verified. A proof that doesn't verify changes nothing and
    /// fails with [`SeedVaultError::LightClient`].
    pub fn light_client_advance(
        &self,
        program_id: Vec<u8>,
        message: StateProofMessage,
        state_proof: Vec<u8>,
    ) -> Result<u64, SeedVaultError> {
        let program = self::program_id(&program_id)?;
        let message = lc::StateProofMessage {
            block_headers_commitment: message.block_headers_commitment,
            voters_commitment: message.voters_commitment,
            ln_proven_weight: message.ln_proven_weight,
            first_attested_round: message.first_attested_round,
            last_attested_round: message.last_attested_round,
        };
        self.with_wasm(|c| Ok(lc::advance(c, &program, &message, &state_proof)?))
    }

    /// Checks, inside the TEE, that transaction `txid` was confirmed in
    /// `round`, against the state proofs the oracle verified. Returns
    /// normally if it was; otherwise [`SeedVaultError::LightClient`] says why
    /// not (e.g. `RootMismatch`, or `NoStateProofForRound` if the oracle
    /// hasn't verified that round's interval yet).
    #[allow(clippy::too_many_arguments)]
    pub fn light_client_verify_transaction(
        &self,
        program_id: Vec<u8>,
        txid: Vec<u8>,
        round: u64,
        genesis_hash: Vec<u8>,
        seed: Vec<u8>,
        transaction_proof: TransactionProof,
        light_block_header_proof: LightBlockHeaderProof,
    ) -> Result<(), SeedVaultError> {
        let program = self::program_id(&program_id)?;
        let txid = bytes32("txid", &txid)?;
        let genesis_hash = bytes32("genesis_hash", &genesis_hash)?;
        let seed = bytes32("seed", &seed)?;
        let transaction_proof = lc::TransactionProof {
            hash_type: transaction_proof.hash_type,
            stib_hash: bytes32("stib_hash", &transaction_proof.stib_hash)?,
            index: transaction_proof.index,
            tree_depth: transaction_proof.tree_depth,
            proof: transaction_proof.proof,
        };
        let header_proof = lc::LightBlockHeaderProof {
            index: light_block_header_proof.index,
            tree_depth: light_block_header_proof.tree_depth,
            proof: light_block_header_proof.proof,
        };
        self.with_wasm(|c| {
            Ok(lc::verify_transaction(
                c,
                &program,
                &txid,
                round,
                &genesis_hash,
                &seed,
                &transaction_proof,
                &header_proof,
            )?)
        })
    }

    /// The oracle's state, or `None` before [`SeedVaultClient::light_client_init`].
    pub fn light_client_status(
        &self,
        program_id: Vec<u8>,
    ) -> Result<Option<LightClientStatus>, SeedVaultError> {
        let program = self::program_id(&program_id)?;
        self.with_wasm(|c| {
            Ok(lc::status(c, &program)?.map(|s| LightClientStatus {
                first_attested_round: s.first_attested_round,
                interval_size: s.interval_size,
                capacity: s.capacity,
                earliest_interval: s.earliest_interval,
                next_interval: s.next_interval,
                ln_proven_weight: s.ln_proven_weight,
                voters_commitment: s.voters_commitment,
            }))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unreachable_client() -> SeedVaultClient {
        SeedVaultClient::with_config(
            "/nonexistent/trusty-ipc-dev0".into(),
            crate::default_service_name(),
            1_000,
        )
    }

    #[test]
    fn bad_arguments_are_rejected_before_connecting() {
        let client = unreachable_client();
        fn invalid<T>(r: Result<T, SeedVaultError>) -> bool {
            matches!(r, Err(SeedVaultError::InvalidArgument { .. }))
        }
        assert!(invalid(client.wasm_unload(vec![0; 31])));
        assert!(invalid(client.wasm_run(vec![0; 33], vec![])));
        assert!(invalid(
            client.wasm_install(vec![0; 10], Some(vec![1, 2, 3]))
        ));
        assert!(invalid(client.wasm_revoke(vec![])));
        assert!(invalid(client.light_client_verify_transaction(
            vec![0; 32],
            vec![0; 31],
            1,
            vec![0; 32],
            vec![0; 32],
            TransactionProof {
                hash_type: "sha256".into(),
                stib_hash: vec![0; 32],
                index: 0,
                tree_depth: 0,
                proof: vec![],
            },
            LightBlockHeaderProof {
                index: 0,
                tree_depth: 0,
                proof: vec![],
            },
        )));
    }

    #[test]
    fn unreachable_vault_reports_not_supported() {
        let client = unreachable_client();
        assert!(matches!(
            client.wasm_list_programs(),
            Err(SeedVaultError::NotSupported { .. })
        ));
        assert!(matches!(
            client.light_client_status(vec![0; 32]),
            Err(SeedVaultError::NotSupported { .. })
        ));
    }

    #[test]
    fn errors_map_to_typed_variants() {
        let e: SeedVaultError = ClientError::<SeedVaultError>::Applet(ErrorCode::Revoked).into();
        assert_eq!(
            e,
            SeedVaultError::Wasm {
                code: WasmErrorCode::Revoked
            }
        );
        let e: SeedVaultError =
            LightClientCallError::<SeedVaultError>::LightClient(lc::LightClientError::RootMismatch)
                .into();
        assert_eq!(
            e,
            SeedVaultError::LightClient {
                code: LightClientErrorCode::RootMismatch
            }
        );
        let e: SeedVaultError =
            LightClientCallError::<SeedVaultError>::LightClient(lc::LightClientError::Unknown(99))
                .into();
        assert_eq!(
            e,
            SeedVaultError::LightClient {
                code: LightClientErrorCode::Unknown { status: 99 }
            }
        );
        // Every runtime error code has a binding.
        for byte in 0..=255u8 {
            if let Some(code) = ErrorCode::from_byte(byte) {
                let _ = WasmErrorCode::from(code);
            }
        }
    }
}
