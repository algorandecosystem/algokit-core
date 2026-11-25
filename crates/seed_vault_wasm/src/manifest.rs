//! Publisher-signed manifests and revocation lists.
//!
//! A signed module ships with a [`Manifest`] signed (Falcon-1024) by a
//! publisher key built into the TA. The manifest pins the exact module bytes
//! (`program_id`) and says what it may do ([`Capabilities`]), so a signature
//! grants specific powers, not just "may load". A [`Revocations`] list, signed
//! the same way, withdraws modules or whole publisher keys without a firmware
//! update.
//!
//! Wire formats (all integers little-endian):
//!
//! ```text
//! Manifest:    "SVWM" [format: u8 = 1] [key_id: u8] [program_id: 32] [version: u32]
//!              [capabilities: u8] [name_len: u8] [name: name_len bytes]
//! Revocations: "SVWR" [format: u8 = 1] [key_id: u8] [sequence: u32]
//!              [key_count: u8] [revoked key_id: u8]*key_count
//!              [program_count: u8] [revoked program_id: 32]*program_count
//! Signed:      [body_len: u16] [body] [signature_len: u16] [compressed Falcon-1024 signature]
//! ```
//!
//! The signature is over the body exactly as sent; the magic (`SVWM`/`SVWR`)
//! keeps a manifest signature from ever passing as a revocation list or vice
//! versa. A publisher key must be used for nothing else.

use alloc::string::String;
use alloc::vec::Vec;

use algokit_crypto::falcon1024::{PrivateKey, PublicKey};

/// The Falcon-1024 public key type publisher keys use (re-exported so
/// clients needn't depend on `algokit_crypto` to build a [`PublisherKey`]).
pub use algokit_crypto::falcon1024::PublicKey as FalconPublicKey;
use wasm_runtime::ProgramId;

const MANIFEST_MAGIC: &[u8; 4] = b"SVWM";
const REVOCATIONS_MAGIC: &[u8; 4] = b"SVWR";
const FORMAT: u8 = 1;

/// Longest algorithm name.
pub const MAX_NAME_LEN: usize = 64;
/// Most entries of each kind in one revocation list (keeps it, plus its
/// signature, inside one IPC message).
pub const MAX_REVOKED: usize = 64;

/// What a signed module is allowed to do.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Capabilities {
    /// May import `account_entropy`: the native key material for the
    /// requested account, so it can reproduce the vault's native accounts.
    /// Leaking it leaks that account (and for index 0, the 25 words).
    pub account_entropy: bool,
    /// May use `address_index` 0, the human-sign key.
    pub human_sign: bool,
}

impl Capabilities {
    /// Everything: what unsigned modules get in dev mode.
    pub const ALL: Capabilities = Capabilities {
        account_entropy: true,
        human_sign: true,
    };

    const ACCOUNT_ENTROPY: u8 = 1 << 0;
    const HUMAN_SIGN: u8 = 1 << 1;

    fn to_byte(self) -> u8 {
        (if self.account_entropy {
            Self::ACCOUNT_ENTROPY
        } else {
            0
        }) | (if self.human_sign { Self::HUMAN_SIGN } else { 0 })
    }

    fn from_byte(byte: u8) -> Option<Self> {
        if byte & !(Self::ACCOUNT_ENTROPY | Self::HUMAN_SIGN) != 0 {
            return None; // unknown capability: refuse rather than ignore
        }
        Some(Capabilities {
            account_entropy: byte & Self::ACCOUNT_ENTROPY != 0,
            human_sign: byte & Self::HUMAN_SIGN != 0,
        })
    }
}

/// What a publisher vouches for about one module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Manifest {
    /// Which publisher key signed it.
    pub key_id: u8,
    /// SHA-256 of the exact module bytes.
    pub program_id: ProgramId,
    /// Algorithm name, e.g. `falcon1024`: `[a-z0-9._-]`, 1 to
    /// [`MAX_NAME_LEN`] bytes. Versions of one algorithm share a name, and
    /// with it their keys and rollback protection.
    pub name: String,
    /// Higher replaces lower; lower is refused once higher was installed.
    pub version: u32,
    pub capabilities: Capabilities,
}

/// A publisher's revocation list. Only a list with a higher `sequence`
/// than the last one applied takes effect.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Revocations {
    pub key_id: u8,
    pub sequence: u32,
    /// Publisher keys whose signatures are no longer accepted.
    pub revoked_keys: Vec<u8>,
    /// Modules that may no longer be installed or run.
    pub revoked_programs: Vec<ProgramId>,
}

/// A body plus its signature, as sent over the wire.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signed {
    pub body: Vec<u8>,
    pub signature: Vec<u8>,
}

/// Why a manifest or revocation list was rejected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignatureError {
    Malformed,
    UnknownKey,
    BadSignature,
}

/// A publisher public key built into the TA.
#[derive(Clone)]
pub struct PublisherKey {
    pub id: u8,
    pub public_key: PublicKey,
}

impl Signed {
    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(4 + self.body.len() + self.signature.len());
        out.extend_from_slice(&(self.body.len() as u16).to_le_bytes());
        out.extend_from_slice(&self.body);
        out.extend_from_slice(&(self.signature.len() as u16).to_le_bytes());
        out.extend_from_slice(&self.signature);
        out
    }

    pub fn decode(bytes: &[u8]) -> Option<Self> {
        let mut r = Reader(bytes);
        let body_len = r.u16()? as usize;
        let body = r.take(body_len)?.to_vec();
        let signature_len = r.u16()? as usize;
        let signature = r.take(signature_len)?.to_vec();
        r.end()?;
        Some(Signed { body, signature })
    }

    /// Signs `body` with `private_key` (for the publisher's tooling).
    pub fn sign(body: Vec<u8>, private_key: &PrivateKey) -> Self {
        let signature = private_key
            .sign_compressed(&body)
            .expect("Falcon signing")
            .0;
        Signed { body, signature }
    }

    /// Checks the signature against the key named by `key_id`.
    fn verify(&self, key_id: u8, keys: &[PublisherKey]) -> Result<(), SignatureError> {
        let key = keys
            .iter()
            .find(|key| key.id == key_id)
            .ok_or(SignatureError::UnknownKey)?;
        key.public_key
            .verify(&self.signature, &self.body)
            .map_err(|_| SignatureError::BadSignature)
    }
}

impl Manifest {
    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(MANIFEST_MAGIC);
        out.push(FORMAT);
        out.push(self.key_id);
        out.extend_from_slice(&self.program_id.0);
        out.extend_from_slice(&self.version.to_le_bytes());
        out.push(self.capabilities.to_byte());
        out.push(self.name.len() as u8);
        out.extend_from_slice(self.name.as_bytes());
        out
    }

    pub fn decode(bytes: &[u8]) -> Option<Self> {
        let mut r = Reader(bytes);
        r.magic(MANIFEST_MAGIC)?;
        let key_id = r.u8()?;
        let program_id = ProgramId(r.take(32)?.try_into().ok()?);
        let version = r.u32()?;
        let capabilities = Capabilities::from_byte(r.u8()?)?;
        let name_len = r.u8()? as usize;
        let name = r.take(name_len)?;
        r.end()?;
        if !valid_name(name) {
            return None;
        }
        Some(Manifest {
            key_id,
            program_id,
            // `valid_name` only allows ASCII.
            name: String::from(core::str::from_utf8(name).ok()?),
            version,
            capabilities,
        })
    }

    /// Decodes `signed` and checks its signature against `keys`.
    pub fn verify(signed: &Signed, keys: &[PublisherKey]) -> Result<Self, SignatureError> {
        let manifest = Self::decode(&signed.body).ok_or(SignatureError::Malformed)?;
        signed.verify(manifest.key_id, keys)?;
        Ok(manifest)
    }
}

impl Revocations {
    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(REVOCATIONS_MAGIC);
        out.push(FORMAT);
        out.push(self.key_id);
        out.extend_from_slice(&self.sequence.to_le_bytes());
        out.push(self.revoked_keys.len() as u8);
        out.extend_from_slice(&self.revoked_keys);
        out.push(self.revoked_programs.len() as u8);
        for program in &self.revoked_programs {
            out.extend_from_slice(&program.0);
        }
        out
    }

    pub fn decode(bytes: &[u8]) -> Option<Self> {
        let mut r = Reader(bytes);
        r.magic(REVOCATIONS_MAGIC)?;
        let key_id = r.u8()?;
        let sequence = r.u32()?;
        let key_count = r.u8()? as usize;
        let revoked_keys = r.take(key_count)?.to_vec();
        let program_count = r.u8()? as usize;
        if key_count > MAX_REVOKED || program_count > MAX_REVOKED {
            return None;
        }
        let revoked_programs = (0..program_count)
            .map(|_| Some(ProgramId(r.take(32)?.try_into().ok()?)))
            .collect::<Option<_>>()?;
        r.end()?;
        Some(Revocations {
            key_id,
            sequence,
            revoked_keys,
            revoked_programs,
        })
    }

    /// Decodes `signed` and checks its signature against `keys`. A list
    /// signed by a key it revokes is refused.
    pub fn verify(signed: &Signed, keys: &[PublisherKey]) -> Result<Self, SignatureError> {
        let list = Self::decode(&signed.body).ok_or(SignatureError::Malformed)?;
        if list.revoked_keys.contains(&list.key_id) {
            return Err(SignatureError::UnknownKey);
        }
        signed.verify(list.key_id, keys)?;
        Ok(list)
    }
}

fn valid_name(name: &[u8]) -> bool {
    (1..=MAX_NAME_LEN).contains(&name.len())
        && name
            .iter()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b"._-".contains(b))
}

struct Reader<'a>(&'a [u8]);

impl<'a> Reader<'a> {
    fn take(&mut self, n: usize) -> Option<&'a [u8]> {
        if self.0.len() < n {
            return None;
        }
        let (head, tail) = self.0.split_at(n);
        self.0 = tail;
        Some(head)
    }
    fn u8(&mut self) -> Option<u8> {
        Some(self.take(1)?[0])
    }
    fn u16(&mut self) -> Option<u16> {
        Some(u16::from_le_bytes(self.take(2)?.try_into().ok()?))
    }
    fn u32(&mut self) -> Option<u32> {
        Some(u32::from_le_bytes(self.take(4)?.try_into().ok()?))
    }
    fn magic(&mut self, magic: &[u8; 4]) -> Option<()> {
        (self.take(4)? == magic && self.u8()? == FORMAT).then_some(())
    }
    fn end(&self) -> Option<()> {
        self.0.is_empty().then_some(())
    }
}
