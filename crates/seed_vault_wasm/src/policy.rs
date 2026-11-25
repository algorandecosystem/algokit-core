//! Who may install a WASM program, with which powers, and when it may use a
//! key.
//!
//! The policy is stateless verification; [`crate::WasmVault`] keeps the
//! state (rollback versions, applied revocations) and enforces the
//! [`Capabilities`] a policy grants.

use alloc::vec;
use alloc::vec::Vec;

use algokit_crypto::falcon1024::{account_from_pq_seed, PrivateKey, PublicKey};
use wasm_runtime::ProgramId;

use crate::manifest::{Capabilities, Manifest, PublisherKey, Revocations, SignatureError, Signed};
use crate::vault::WasmError;

/// Why a program was let in, and what it may do.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Admission {
    pub capabilities: Capabilities,
    /// The verified manifest, for signed programs.
    pub manifest: Option<Manifest>,
}

/// Decides what [`crate::WasmVault`] allows.
pub trait WasmPolicy {
    /// Whether `program` may be installed (or restored after a restart), and
    /// with which capabilities. `signed` is its signed manifest, if it came
    /// with one.
    fn admit(&self, program: &ProgramId, signed: Option<&Signed>) -> Result<Admission, WasmError>;

    /// Checks a signed revocation list.
    fn verify_revocations(&self, signed: &Signed) -> Result<Revocations, WasmError>;

    /// Last check before `program` uses key `address_index` of the seed whose
    /// human-sign address is `seed_address`, after the vault has checked its
    /// capabilities (e.g. index 0 needs [`Capabilities::human_sign`]).
    fn allow_use_key(&self, program: &ProgramId, seed_address: &str, address_index: u8) -> bool;
}

/// Trusts modules signed by the publisher keys built into the TA, with the
/// capabilities their manifest grants.
///
/// [`Self::allowing_unsigned`] additionally admits unsigned modules with
/// every capability: development only.
pub struct PublisherPolicy {
    keys: Vec<PublisherKey>,
    allow_unsigned: bool,
}

impl PublisherPolicy {
    /// Signed modules only.
    pub fn new(keys: Vec<PublisherKey>) -> Self {
        PublisherPolicy {
            keys,
            allow_unsigned: false,
        }
    }

    /// Also admits unsigned modules, with every capability (including
    /// reading account entropy, i.e. index 0's 25 words).
    ///
    /// TODO(security): development only. Anything that can reach the TA's
    /// port could then install a module that leaks seeds.
    pub fn allowing_unsigned(mut self) -> Self {
        self.allow_unsigned = true;
        self
    }

    /// The development setup: trusts [`dev_publisher_key`] and unsigned
    /// modules. TODO(security): never ship this.
    pub fn dev() -> Self {
        Self::new(vec![dev_publisher_key()]).allowing_unsigned()
    }
}

impl WasmPolicy for PublisherPolicy {
    fn admit(&self, program: &ProgramId, signed: Option<&Signed>) -> Result<Admission, WasmError> {
        let Some(signed) = signed else {
            return if self.allow_unsigned {
                Ok(Admission {
                    capabilities: Capabilities::ALL,
                    manifest: None,
                })
            } else {
                Err(WasmError::InstallDenied)
            };
        };
        let manifest = Manifest::verify(signed, &self.keys).map_err(signature_error)?;
        // The signature pins the exact module bytes.
        if manifest.program_id != *program {
            return Err(WasmError::BadSignature);
        }
        Ok(Admission {
            capabilities: manifest.capabilities,
            manifest: Some(manifest),
        })
    }

    fn verify_revocations(&self, signed: &Signed) -> Result<Revocations, WasmError> {
        Revocations::verify(signed, &self.keys).map_err(signature_error)
    }

    fn allow_use_key(&self, _program: &ProgramId, _seed_address: &str, _address_index: u8) -> bool {
        // TODO(security): index 0 (human-sign) also needs proof of a fresh
        // user authentication (fingerprint/PIN), the same gate
        // `seed_vault_ta`'s native index-0 signing still needs (see the TODO
        // in its `on_message`). The vault has already checked the module's
        // `human_sign` capability.
        true
    }
}

fn signature_error(_: SignatureError) -> WasmError {
    WasmError::BadSignature
}

/// [`dev_publisher_key`]'s `key_id`. Production keys should use others.
pub const DEV_KEY_ID: u8 = 0xFF;

/// Seed of the development publisher key. **Public, not a secret**: anyone
/// can sign with this key, so a TA trusting it trusts everyone.
///
/// TODO(security): production TAs must trust only keys generated offline
/// (`seed_vault_wasm_sign keygen`), never this one.
pub fn dev_publisher_seed() -> [u8; 32] {
    algokit_crypto::sha512_256(
        b"seed_vault_wasm DEV publisher key -- PUBLIC, NOT A SECRET, NEVER SHIP",
    )
}

/// The development publisher key (see [`dev_publisher_seed`]).
pub fn dev_publisher_key() -> PublisherKey {
    let (public_key, _) = publisher_keypair(&dev_publisher_seed());
    PublisherKey {
        id: DEV_KEY_ID,
        public_key,
    }
}

/// The Falcon-1024 publisher key pair for a 32-byte keygen `seed`.
pub fn publisher_keypair(seed: &[u8; 32]) -> (PublicKey, PrivateKey) {
    let account = account_from_pq_seed(seed).expect("Falcon keygen");
    (account.public_key, account.private_key)
}
