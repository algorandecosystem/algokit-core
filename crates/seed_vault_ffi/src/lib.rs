//! Client bindings for the `seed_vault_ta` Trusty applet.
//!
//! [`SeedVaultClient`] is the whole public surface: each call opens a fresh
//! Trusty IPC connection, sends one request encoded by
//! [`seed_vault::protocol`] (the same code the applet decodes it with),
//! decodes the reply, and closes the connection again.
//!
//! ## When there is no seed vault
//!
//! Most devices have no `seed_vault_ta`: no Trusty at all, Trusty without
//! this applet built in, or an SELinux policy that won't let the app reach
//! it. Call [`SeedVaultClient::status`] first -- it never throws and says
//! which of those it is ([`SeedVaultStatus`]) -- and only show seed vault
//! features when it's [`SeedVaultStatus::Available`]. Every other call
//! reports the same conditions as the matching [`SeedVaultError`] variant,
//! so a vault that disappears later (e.g. the Trusty VM restarted) is still
//! a typed error rather than a crash.

mod transport;

use seed_vault::protocol::{self, DecodeError, ErrorCode, Request};
use seed_vault::store;
use zeroize::Zeroize;

use transport::Connection;

uniffi::setup_scaffolding!();

/// Trusty's vsock tipc bridge on x86_64 Cuttlefish, where Trusty runs as
/// `trusty_security_vm` (CID 200) rather than behind a kernel driver.
const DEFAULT_DEVICE_PATH: &str = "VSOCK:200:0";
/// The kernel Trusty IPC device node on targets with a Trusty-enabled kernel.
const KERNEL_DEVICE_PATH: &str = "/dev/trusty-ipc-dev0";
/// Must match `PORT` in `trusty/app/seed_vault_ta/main.rs`.
const DEFAULT_SERVICE_NAME: &str = "io.github.algorandecosystem.algokitcore.seed_vault_ta";
/// Must be >= `MAX_MSG_SIZE` in `trusty/app/seed_vault_ta/main.rs`; also the
/// most a request may be.
const MAX_MESSAGE_SIZE: usize = 4096;
const DEFAULT_TIMEOUT_MS: u32 = 10_000;

// ---- Types ------------------------------------------------------------------

/// Mirrors `seed_vault::store::KeyAlgorithm`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum KeyAlgorithm {
    /// Classic Ed25519 account; the 25 words encode the Ed25519 seed.
    Algo25Ed25519,
    /// Native post-quantum Falcon-1024 account derived from the *same*
    /// 25-word mnemonic (go-algorand `algokey pq`), at a different address.
    Falcon1024,
}

impl From<store::KeyAlgorithm> for KeyAlgorithm {
    fn from(value: store::KeyAlgorithm) -> Self {
        match value {
            store::KeyAlgorithm::Algo25Ed25519 => Self::Algo25Ed25519,
            store::KeyAlgorithm::Falcon1024 => Self::Falcon1024,
        }
    }
}

impl From<KeyAlgorithm> for store::KeyAlgorithm {
    fn from(value: KeyAlgorithm) -> Self {
        match value {
            KeyAlgorithm::Algo25Ed25519 => Self::Algo25Ed25519,
            KeyAlgorithm::Falcon1024 => Self::Falcon1024,
        }
    }
}

/// An account's public identity.
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct Account {
    pub algorithm: KeyAlgorithm,
    /// Which of its seed's keys this is: [`human_sign_index`], or an
    /// autosign key (Falcon-1024 only: [`autosign_index`], or any added with
    /// [`SeedVaultClient::derive_account`]).
    pub address_index: u32,
    pub address: String,
}

impl From<protocol::WireAccount> for Account {
    fn from(value: protocol::WireAccount) -> Self {
        Self {
            algorithm: value.algorithm.into(),
            address_index: value.address_index,
            address: value.address,
        }
    }
}

/// An account plus its public key (32 bytes for Ed25519, 1793 for
/// Falcon-1024).
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct ListedAccount {
    pub account: Account,
    pub public_key: Vec<u8>,
}

/// The result of [`SeedVaultClient::sign_transaction`].
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct SignedTransaction {
    pub algorithm: KeyAlgorithm,
    /// The network-ready msgpack `SignedTxn` (`POST /v2/transactions` body).
    pub signed_txn: Vec<u8>,
    /// The bare signature over `"TX" || txn` that `signed_txn` carries (64
    /// bytes for Ed25519, compressed Falcon-1024 otherwise), for protocols
    /// that send a transaction signature on its own (e.g. Liquid Auth).
    pub signature: Vec<u8>,
}

/// The result of [`SeedVaultClient::sign_arbitrary_data`].
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct SignedArbitraryData {
    pub algorithm: KeyAlgorithm,
    /// Raw signature over the data exactly as given: 64 bytes for Ed25519
    /// (`ed25519RawSign`), a compressed deterministic signature for
    /// Falcon-1024 (go-algorand `FalconSigner.SignBytes`).
    pub signature: Vec<u8>,
}

/// A wire protocol version (see `seed_vault::protocol`'s versioning docs).
#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Record)]
pub struct ProtocolVersion {
    pub major: u16,
    pub minor: u16,
}

/// Mirrors `seed_vault::protocol::ErrorCode`: an error the applet itself
/// reported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum AppletErrorCode {
    MalformedRequest,
    StoreFailure,
    ResponseTooLarge,
    UnsupportedAlgorithm,
    AccountNotFound,
    InvalidMnemonic,
    AccountAlreadyExists,
    InvalidTransaction,
    SigningFailed,
    UnknownAddressIndex,
    SenderMismatch,
    /// `sign_arbitrary_data` data started with `TX`, `TG`, `Program`, `ProgData` or `MX`.
    ReservedDomainPrefix,
    /// `derive_account` would give the seed more keys than it can hold.
    TooManyKeys,
}

impl From<ErrorCode> for AppletErrorCode {
    fn from(value: ErrorCode) -> Self {
        match value {
            ErrorCode::MalformedRequest => Self::MalformedRequest,
            ErrorCode::StoreFailure => Self::StoreFailure,
            ErrorCode::ResponseTooLarge => Self::ResponseTooLarge,
            ErrorCode::UnsupportedAlgorithm => Self::UnsupportedAlgorithm,
            ErrorCode::AccountNotFound => Self::AccountNotFound,
            ErrorCode::InvalidMnemonic => Self::InvalidMnemonic,
            ErrorCode::AccountAlreadyExists => Self::AccountAlreadyExists,
            ErrorCode::InvalidTransaction => Self::InvalidTransaction,
            ErrorCode::SigningFailed => Self::SigningFailed,
            ErrorCode::UnknownAddressIndex => Self::UnknownAddressIndex,
            ErrorCode::SenderMismatch => Self::SenderMismatch,
            ErrorCode::ReservedDomainPrefix => Self::ReservedDomainPrefix,
            ErrorCode::TooManyKeys => Self::TooManyKeys,
        }
    }
}

/// Whether this device has a usable seed vault (see the crate docs).
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum SeedVaultStatus {
    /// Reachable and speaking a compatible protocol version.
    Available { version: ProtocolVersion },
    /// No Trusty is reachable at the configured device path: a regular
    /// phone/emulator, or the Trusty VM isn't running.
    NotSupported { reason: String },
    /// Trusty may be there, but this app isn't allowed to reach it (SELinux
    /// policy or device node permissions).
    PermissionDenied { reason: String },
    /// Trusty is there, but it has no `seed_vault_ta` service.
    ServiceNotFound { reason: String },
    /// `seed_vault_ta` is there but speaks a different major protocol
    /// version. `applet` is `None` for an applet older than `GET_VERSION`.
    IncompatibleVersion {
        applet: Option<ProtocolVersion>,
        client: ProtocolVersion,
    },
}

/// Why a [`SeedVaultClient`] call failed.
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Error)]
pub enum SeedVaultError {
    /// See [`SeedVaultStatus::NotSupported`].
    NotSupported { reason: String },
    /// See [`SeedVaultStatus::PermissionDenied`].
    PermissionDenied { reason: String },
    /// See [`SeedVaultStatus::ServiceNotFound`].
    ServiceNotFound { reason: String },
    /// Connected, but the request/response exchange failed (I/O error,
    /// timeout, applet closed the connection).
    Transport { reason: String },
    /// The applet answered with an error.
    Applet { code: AppletErrorCode },
    /// The applet's reply didn't parse.
    MalformedResponse { reason: String },
    /// The request can't be sent as given (e.g. too large for one message).
    InvalidArgument { reason: String },
}

impl std::fmt::Display for SeedVaultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotSupported { reason } => write!(f, "seed vault not supported: {reason}"),
            Self::PermissionDenied { reason } => {
                write!(f, "seed vault permission denied: {reason}")
            }
            Self::ServiceNotFound { reason } => write!(f, "seed vault service not found: {reason}"),
            Self::Transport { reason } => write!(f, "seed vault transport error: {reason}"),
            Self::Applet { code } => write!(f, "seed vault applet error: {code:?}"),
            Self::MalformedResponse { reason } => {
                write!(f, "malformed seed vault response: {reason}")
            }
            Self::InvalidArgument { reason } => write!(f, "invalid argument: {reason}"),
        }
    }
}

impl std::error::Error for SeedVaultError {}

impl From<DecodeError> for SeedVaultError {
    fn from(value: DecodeError) -> Self {
        match value {
            DecodeError::ErrorResponse(code) => Self::Applet { code: code.into() },
            other => Self::MalformedResponse {
                reason: format!("{other:?}"),
            },
        }
    }
}

// ---- Free functions ---------------------------------------------------------

/// The protocol version this library speaks.
#[uniffi::export]
pub fn client_protocol_version() -> ProtocolVersion {
    ProtocolVersion {
        major: protocol::PROTOCOL_VERSION_MAJOR,
        minor: protocol::PROTOCOL_VERSION_MINOR,
    }
}

/// `address_index` of the standard account other wallets restore from the
/// 25 words; every seed has one.
#[uniffi::export]
pub fn human_sign_index() -> u8 {
    store::HUMAN_SIGN_INDEX as u8
}

/// `address_index` of a Falcon-1024 seed's default autosign account (a
/// separate Falcon key derived from the same seed). Every new Falcon seed
/// has it; [`SeedVaultClient::derive_account`] adds more (`2..=255`).
#[uniffi::export]
pub fn autosign_index() -> u8 {
    store::AUTOSIGN_INDEX as u8
}

/// Device path for Trusty's vsock bridge on x86_64 Cuttlefish.
#[uniffi::export]
pub fn default_device_path() -> String {
    DEFAULT_DEVICE_PATH.into()
}

/// Device path for targets with a Trusty-enabled kernel.
#[uniffi::export]
pub fn kernel_device_path() -> String {
    KERNEL_DEVICE_PATH.into()
}

/// The `seed_vault_ta` TIPC port name.
#[uniffi::export]
pub fn default_service_name() -> String {
    DEFAULT_SERVICE_NAME.into()
}

// ---- Client -----------------------------------------------------------------

/// Talks to `seed_vault_ta`. Every method blocks on IPC, so call it off the
/// main thread. Cheap to construct; holds no connection between calls.
#[derive(Debug, uniffi::Object)]
pub struct SeedVaultClient {
    device_path: String,
    service_name: String,
    timeout_ms: u32,
}

#[uniffi::export]
impl SeedVaultClient {
    /// A client for this device's Trusty and the default service name: the
    /// kernel device node ([`kernel_device_path`]) when the device has one
    /// (e.g. NXP i.MX95), otherwise the Cuttlefish vsock bridge
    /// ([`default_device_path`]).
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self::with_config(
            detect_device_path(std::path::Path::new(KERNEL_DEVICE_PATH)).into(),
            DEFAULT_SERVICE_NAME.into(),
            DEFAULT_TIMEOUT_MS,
        )
    }

    /// A client for `device_path` (a device node such as
    /// [`kernel_device_path`], or `VSOCK:<cid>:<port>`) and `service_name`,
    /// waiting at most `timeout_ms` for each reply.
    #[uniffi::constructor]
    pub fn with_config(device_path: String, service_name: String, timeout_ms: u32) -> Self {
        Self {
            device_path,
            service_name,
            timeout_ms,
        }
    }

    pub fn device_path(&self) -> String {
        self.device_path.clone()
    }

    pub fn service_name(&self) -> String {
        self.service_name.clone()
    }

    /// Probes for a usable seed vault. Never fails: every failure is one of
    /// the [`SeedVaultStatus`] variants.
    pub fn status(&self) -> SeedVaultStatus {
        let client = client_protocol_version();
        let reason_of = |err: SeedVaultError| match err {
            SeedVaultError::NotSupported { reason } => SeedVaultStatus::NotSupported { reason },
            SeedVaultError::PermissionDenied { reason } => {
                SeedVaultStatus::PermissionDenied { reason }
            }
            other => SeedVaultStatus::ServiceNotFound {
                reason: other.to_string(),
            },
        };
        match self.protocol_version() {
            Ok(version) if version.major == client.major => SeedVaultStatus::Available { version },
            Ok(version) => SeedVaultStatus::IncompatibleVersion {
                applet: Some(version),
                client,
            },
            // Pre-`GET_VERSION` applets reject the opcode.
            Err(SeedVaultError::Applet {
                code: AppletErrorCode::MalformedRequest,
            }) => SeedVaultStatus::IncompatibleVersion {
                applet: None,
                client,
            },
            Err(err) => reason_of(err),
        }
    }

    /// The applet's protocol version.
    pub fn protocol_version(&self) -> Result<ProtocolVersion, SeedVaultError> {
        let response = self.call(&protocol::encode_request(Request::GetVersion))?;
        let (major, minor) = protocol::decode_get_version_response(&response)?;
        Ok(ProtocolVersion { major, minor })
    }

    /// Creates a new account and returns its `address_index` key.
    pub fn create_account(
        &self,
        algorithm: KeyAlgorithm,
        address_index: u8,
    ) -> Result<Account, SeedVaultError> {
        let response = self.call(&protocol::encode_request(Request::CreateAccount {
            algorithm: algorithm.into(),
            address_index,
        }))?;
        Ok(protocol::decode_create_account_response(&response)?.into())
    }

    /// Every account the applet holds (follows `LIST_ACCOUNTS` paging over
    /// one connection).
    pub fn list_accounts(&self) -> Result<Vec<ListedAccount>, SeedVaultError> {
        let mut connection = self.connect()?;
        let mut all = Vec::new();
        loop {
            let response = connection.call(
                &protocol::encode_request(Request::ListAccounts {
                    start: all.len() as u32,
                }),
                self.timeout_ms,
            )?;
            let page = protocol::decode_list_accounts_response(&response)?;
            let done = page.accounts.is_empty();
            all.extend(page.accounts.into_iter().map(|l| ListedAccount {
                account: l.account.into(),
                public_key: l.public_key,
            }));
            if done || all.len() >= page.total as usize {
                return Ok(all);
            }
        }
    }

    /// The 25-word mnemonic backing `address`. Handle with care: this is
    /// the account's secret.
    pub fn reveal_mnemonic(&self, address: String) -> Result<String, SeedVaultError> {
        let address = check_short_field("address", address.trim())?;
        let mut response = self.call(&protocol::encode_request(Request::RevealMnemonic {
            address,
        }))?;
        let result = protocol::decode_reveal_mnemonic_response(&response);
        response.zeroize();
        Ok(result?)
    }

    /// Imports `mnemonic` (any whitespace between words) and returns its
    /// `address_index` key.
    pub fn import_account(
        &self,
        mnemonic: String,
        algorithm: KeyAlgorithm,
        address_index: u8,
    ) -> Result<Account, SeedVaultError> {
        let mut normalized = mnemonic.split_whitespace().collect::<Vec<_>>().join(" ");
        let checked = check_short_field("mnemonic", &normalized);
        normalized.zeroize();
        let mut request = protocol::encode_request(Request::ImportAccount {
            algorithm: algorithm.into(),
            address_index,
            mnemonic: checked?,
        });
        let response = self.call(&request);
        request.zeroize();
        Ok(protocol::decode_create_account_response(&response?)?.into())
    }

    /// Adds autosign key `address_index` (`1..=255`) to the Falcon-1024 seed
    /// behind `address` (any of its addresses) and returns that account --
    /// e.g. a fresh session key, recoverable from the seed's same 25 words.
    /// Returns the existing account if the seed already has that key. Fails
    /// with [`AppletErrorCode::TooManyKeys`] once the seed holds the
    /// applet's maximum. Needs protocol 1.2: an older applet answers
    /// [`AppletErrorCode::MalformedRequest`].
    pub fn derive_account(
        &self,
        address: String,
        address_index: u8,
    ) -> Result<Account, SeedVaultError> {
        let address = check_short_field("address", address.trim())?;
        let response = self.call(&protocol::encode_request(Request::DeriveAccount {
            address_index,
            address,
        }))?;
        Ok(protocol::decode_create_account_response(&response)?.into())
    }

    /// Permanently deletes the seed whose [`human_sign_index`] address is
    /// `address`, together with all of its autosign accounts. Irreversible
    /// without its mnemonic. An autosign address fails with
    /// [`AppletErrorCode::UnknownAddressIndex`].
    pub fn delete_account(&self, address: String) -> Result<Account, SeedVaultError> {
        let address = check_short_field("address", address.trim())?;
        let response = self.call(&protocol::encode_request(Request::DeleteAccount {
            address,
        }))?;
        Ok(protocol::decode_create_account_response(&response)?.into())
    }

    /// Signs `txn` (msgpack, no `"TX"` prefix) with `address`, which must be
    /// its seed's `address_index` key and the transaction's sender.
    pub fn sign_transaction(
        &self,
        address: String,
        address_index: u8,
        txn: Vec<u8>,
    ) -> Result<SignedTransaction, SeedVaultError> {
        let address = check_short_field("address", address.trim())?;
        let request = protocol::encode_request(Request::SignTransaction {
            address_index,
            address,
            txn: txn.clone(),
        });
        check_request_size("transaction", &request, txn.len())?;
        let response = self.call(&request)?;
        let (algorithm, prefix) = protocol::decode_sign_transaction_response(&response)?;
        let signature = seed_vault::signed_txn::signature_from_prefix(&prefix)
            .ok_or_else(|| SeedVaultError::MalformedResponse {
                reason: "SignedTxn prefix has no signature".into(),
            })?
            .to_vec();
        let mut signed_txn = prefix;
        signed_txn.extend_from_slice(&txn);
        Ok(SignedTransaction {
            algorithm: algorithm.into(),
            signed_txn,
            signature,
        })
    }

    /// Signs `data` exactly as given (no prefix added) with `address`, which
    /// must be its seed's `address_index` key. Works for both
    /// [`human_sign_index`] and [`autosign_index`] accounts.
    ///
    /// Use it for off-chain signatures such as FIDO2/Liquid Auth challenges.
    /// Data starting with `TX`, `TG`, `Program`, `ProgData` or `MX` is refused
    /// ([`AppletErrorCode::ReservedDomainPrefix`]); sign transactions with
    /// [`Self::sign_transaction`]. Needs protocol 1.1: a 1.0 applet answers
    /// [`AppletErrorCode::MalformedRequest`].
    pub fn sign_arbitrary_data(
        &self,
        address: String,
        address_index: u8,
        data: Vec<u8>,
    ) -> Result<SignedArbitraryData, SeedVaultError> {
        let address = check_short_field("address", address.trim())?;
        let data_len = data.len();
        let request = protocol::encode_request(Request::SignArbitraryData {
            address_index,
            address,
            data,
        });
        check_request_size("data", &request, data_len)?;
        let response = self.call(&request)?;
        let (algorithm, signature) = protocol::decode_sign_arbitrary_data_response(&response)?;
        Ok(SignedArbitraryData {
            algorithm: algorithm.into(),
            signature,
        })
    }
}

/// Requests carry a `u16` payload length and must fit one TIPC message.
fn check_request_size(
    name: &str,
    request: &[u8],
    payload_len: usize,
) -> Result<(), SeedVaultError> {
    if request.len() > MAX_MESSAGE_SIZE || payload_len > u16::MAX as usize {
        return Err(SeedVaultError::InvalidArgument {
            reason: format!("{name} too large for one seed_vault_ta message ({payload_len} bytes)"),
        });
    }
    Ok(())
}

impl Default for SeedVaultClient {
    fn default() -> Self {
        Self::new()
    }
}

impl SeedVaultClient {
    fn connect(&self) -> Result<Connection, SeedVaultError> {
        Connection::open(&self.device_path, &self.service_name)
    }

    /// One request/response round trip over a fresh connection.
    fn call(&self, request: &[u8]) -> Result<Vec<u8>, SeedVaultError> {
        self.connect()?.call(request, self.timeout_ms)
    }
}

/// The kernel device node if `kernel_node` exists, else the vsock bridge.
/// SELinux usually denies apps even a `stat` of the node, which still proves
/// it's there -- so that counts as present, and the later `open` reports
/// [`SeedVaultError::PermissionDenied`] instead of a misleading
/// `NotSupported` from a vsock that doesn't exist on this device.
fn detect_device_path(kernel_node: &std::path::Path) -> &'static str {
    match std::fs::metadata(kernel_node) {
        Ok(_) => KERNEL_DEVICE_PATH,
        Err(err) if err.kind() == std::io::ErrorKind::PermissionDenied => KERNEL_DEVICE_PATH,
        Err(_) => DEFAULT_DEVICE_PATH,
    }
}

/// Fields with a `u8` length prefix on the wire.
fn check_short_field(name: &str, value: &str) -> Result<String, SeedVaultError> {
    if !value.is_ascii() {
        return Err(SeedVaultError::InvalidArgument {
            reason: format!("{name} must be ASCII"),
        });
    }
    if value.len() > u8::MAX as usize {
        return Err(SeedVaultError::InvalidArgument {
            reason: format!("{name} too long ({} bytes)", value.len()),
        });
    }
    Ok(value.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unreachable_vault_reports_not_supported() {
        // No Trusty on a dev/CI host: the probe must degrade, not fail.
        let client = SeedVaultClient::with_config(
            "/nonexistent/trusty-ipc-dev0".into(),
            DEFAULT_SERVICE_NAME.into(),
            1_000,
        );
        assert!(matches!(
            client.status(),
            SeedVaultStatus::NotSupported { .. }
        ));
        assert!(matches!(
            client.list_accounts(),
            Err(SeedVaultError::NotSupported { .. })
        ));
    }

    #[test]
    fn oversized_arguments_are_rejected_before_connecting() {
        let client = SeedVaultClient::new();
        assert!(matches!(
            client.reveal_mnemonic("A".repeat(256)),
            Err(SeedVaultError::InvalidArgument { .. })
        ));
        assert!(matches!(
            client.sign_transaction("A".repeat(58), 0, vec![0x81; 4096]),
            Err(SeedVaultError::InvalidArgument { .. })
        ));
        assert!(matches!(
            client.sign_arbitrary_data("A".repeat(58), 1, vec![0x00; 4096]),
            Err(SeedVaultError::InvalidArgument { .. })
        ));
    }

    #[test]
    fn default_client_prefers_the_kernel_device_node() {
        let present = std::env::temp_dir();
        assert_eq!(detect_device_path(&present), KERNEL_DEVICE_PATH);
        let missing = present.join("no-such-trusty-ipc-dev0");
        assert_eq!(detect_device_path(&missing), DEFAULT_DEVICE_PATH);
    }

    #[test]
    fn applet_errors_map_to_typed_codes() {
        let err: SeedVaultError = DecodeError::ErrorResponse(ErrorCode::AccountNotFound).into();
        assert_eq!(
            err,
            SeedVaultError::Applet {
                code: AppletErrorCode::AccountNotFound
            }
        );
    }
}
