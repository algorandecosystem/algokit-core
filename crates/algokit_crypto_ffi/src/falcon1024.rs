//! UniFFI bindings for [`algokit_crypto::falcon1024`]: native Algorand
//! post-quantum (deterministic Falcon-1024) accounts and signing.
//!
//! These are the same primitives the Trusty `seed_vault` TA uses, exposed to
//! host bindings (Android/iOS/JVM/Python) in the byte-vector shape UniFFI
//! expects. A Falcon-1024 account is derived from the *same* 25-word mnemonic
//! (or 32-byte entropy) as an `algo25` account, but yields a different,
//! PQ-compliant address.
//!
//! To create a new account: `entropy = random_bytes(32)`, then
//! [`falcon1024_account_from_entropy`]; back it up with
//! `mnemonic_from_seed(entropy)`.
//!
//! Note: unlike the core [`algokit_crypto::falcon1024::PrivateKey`], private
//! key bytes handed across the FFI boundary are plain `Vec<u8>`s and are not
//! zeroized on drop -- callers own their lifetime on the host side.

use algokit_crypto::falcon1024::{
    self as rust_falcon, Falcon1024Account as RustFalcon1024Account, PRIVATE_KEY_SIZE,
    PUBLIC_KEY_SIZE, PrivateKey, PublicKey,
};

/// Byte length of a Falcon-1024 public key.
#[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
pub fn falcon1024_public_key_size() -> u32 {
    PUBLIC_KEY_SIZE as u32
}

/// Byte length of a Falcon-1024 private key.
#[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
pub fn falcon1024_private_key_size() -> u32 {
    PRIVATE_KEY_SIZE as u32
}

/// Upper bound on the byte length of a compressed Falcon-1024 signature.
#[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
pub fn falcon1024_signature_max_size() -> u32 {
    rust_falcon::SIGNATURE_MAX_SIZE as u32
}

/// FFI-compatible mirror of [`algokit_crypto::falcon1024::Falcon1024Account`].
#[derive(Clone)]
#[cfg_attr(feature = "ffi_uniffi", derive(uniffi::Record))]
pub struct Falcon1024Account {
    /// 1793-byte Falcon-1024 public key.
    pub public_key: Vec<u8>,
    /// 2305-byte Falcon-1024 private key.
    pub private_key: Vec<u8>,
    /// The address salt (canonical = lowest PQ-compliant salt). Must be
    /// included in the `pqsig` of signed transactions when non-zero.
    pub salt: u8,
    /// Raw 32-byte Algorand address.
    pub address: Vec<u8>,
    /// 58-character base32 Algorand address.
    pub address_string: String,
}

impl std::fmt::Debug for Falcon1024Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Falcon1024Account")
            .field("salt", &self.salt)
            .field("address_string", &self.address_string)
            .finish_non_exhaustive()
    }
}

impl From<RustFalcon1024Account> for Falcon1024Account {
    fn from(value: RustFalcon1024Account) -> Self {
        Self {
            public_key: value.public_key.0.to_vec(),
            private_key: value.private_key.0.to_vec(),
            salt: value.salt,
            address: value.address.to_vec(),
            address_string: value.address_string(),
        }
    }
}

/// A PQ address (salt + address) for a Falcon-1024 public key.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "ffi_uniffi", derive(uniffi::Record))]
pub struct Falcon1024Address {
    pub salt: u8,
    /// Raw 32-byte Algorand address.
    pub address: Vec<u8>,
    /// 58-character base32 Algorand address.
    pub address_string: String,
}

impl Falcon1024Address {
    fn new(salt: u8, address: [u8; 32]) -> Self {
        Self {
            salt,
            address: address.to_vec(),
            address_string: algokit_crypto::address::address_from_public_key(&address),
        }
    }
}

/// FFI-compatible error type for Falcon-1024 operations.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "ffi_uniffi", derive(uniffi::Error))]
pub enum AlgoKitFalcon1024Error {
    Error { err_msg: String },
}

impl std::fmt::Display for AlgoKitFalcon1024Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlgoKitFalcon1024Error::Error { err_msg } => write!(f, "{}", err_msg),
        }
    }
}

impl AlgoKitFalcon1024Error {
    fn msg(err_msg: impl Into<String>) -> Self {
        Self::Error {
            err_msg: err_msg.into(),
        }
    }
}

impl From<rust_falcon::Falcon1024Error> for AlgoKitFalcon1024Error {
    fn from(value: rust_falcon::Falcon1024Error) -> Self {
        Self::msg(value.to_string())
    }
}

impl From<rust_falcon::Falcon1024MnemonicError> for AlgoKitFalcon1024Error {
    fn from(value: rust_falcon::Falcon1024MnemonicError) -> Self {
        Self::msg(value.to_string())
    }
}

impl From<rust_falcon::FalconError> for AlgoKitFalcon1024Error {
    fn from(value: rust_falcon::FalconError) -> Self {
        Self::msg(value.to_string())
    }
}

fn fixed<const N: usize>(bytes: &[u8], what: &str) -> Result<[u8; N], AlgoKitFalcon1024Error> {
    bytes.try_into().map_err(|_| {
        AlgoKitFalcon1024Error::msg(format!("{what} must be {N} bytes, found {}", bytes.len()))
    })
}

/// Derives the Falcon-1024 account from a 25-word Algorand mnemonic.
///
/// The same words also back an `algo25` (Ed25519) account at a *different*
/// address.
#[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
pub fn falcon1024_account_from_mnemonic(
    mnemonic: String,
) -> Result<Falcon1024Account, AlgoKitFalcon1024Error> {
    rust_falcon::account_from_mnemonic(&mnemonic)
        .map(Into::into)
        .map_err(Into::into)
}

/// Derives the Falcon-1024 account from the 32 bytes of entropy a 25-word
/// mnemonic encodes (i.e. the `algo25` seed).
#[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
pub fn falcon1024_account_from_entropy(
    entropy: Vec<u8>,
) -> Result<Falcon1024Account, AlgoKitFalcon1024Error> {
    let entropy = fixed::<{ rust_falcon::ENTROPY_LEN }>(&entropy, "entropy")?;
    rust_falcon::account_from_entropy(&entropy)
        .map(Into::into)
        .map_err(Into::into)
}

/// Derives the Falcon-1024 account directly from a 32-byte PQ keygen seed
/// (go-algorand-sdk's `crypto.Falcon1024AccountFromPQSeed`).
#[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
pub fn falcon1024_account_from_pq_seed(
    pq_seed: Vec<u8>,
) -> Result<Falcon1024Account, AlgoKitFalcon1024Error> {
    let pq_seed = fixed::<32>(&pq_seed, "PQ seed")?;
    rust_falcon::account_from_pq_seed(&pq_seed)
        .map(Into::into)
        .map_err(Into::into)
}

/// Returns the canonical (lowest PQ-compliant salt) address for a Falcon-1024
/// public key.
#[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
pub fn falcon1024_canonical_address(
    public_key: Vec<u8>,
) -> Result<Falcon1024Address, AlgoKitFalcon1024Error> {
    falcon1024_nth_address(public_key, 0)
}

/// Returns the `n`th-lowest (0-based) PQ-compliant salted address for a
/// Falcon-1024 public key. Every such address is authorized by the same key.
#[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
pub fn falcon1024_nth_address(
    public_key: Vec<u8>,
    n: u32,
) -> Result<Falcon1024Address, AlgoKitFalcon1024Error> {
    let public_key = fixed::<PUBLIC_KEY_SIZE>(&public_key, "public key")?;
    let (salt, address) = rust_falcon::nth_pq_address(&public_key, n as usize)?;
    Ok(Falcon1024Address::new(salt, address))
}

/// Signs `msg` with a Falcon-1024 private key, returning the compressed
/// deterministic signature (as used in a transaction's `pqsig.sig`).
///
/// For transactions, `msg` must be `"TX" || msgpack(txn)` -- i.e. the same
/// bytes an Ed25519 signer signs.
#[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
pub fn falcon1024_raw_sign(
    private_key: Vec<u8>,
    msg: Vec<u8>,
) -> Result<Vec<u8>, AlgoKitFalcon1024Error> {
    let private_key = PrivateKey(fixed::<PRIVATE_KEY_SIZE>(&private_key, "private key")?);
    let signature = private_key.sign_compressed(&msg)?;
    Ok(signature.0)
}

/// Verifies a compressed Falcon-1024 `signature` over `msg`.
///
/// Returns `false` for an invalid signature; errors only on a malformed
/// public key length.
#[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
pub fn falcon1024_verify(
    public_key: Vec<u8>,
    signature: Vec<u8>,
    msg: Vec<u8>,
) -> Result<bool, AlgoKitFalcon1024Error> {
    let public_key = PublicKey(fixed::<PUBLIC_KEY_SIZE>(&public_key, "public key")?);
    Ok(public_key.verify(&signature, &msg).is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// go-algorand `cmd/algokey/pq_test.go` known answer.
    const KAT_ADDRESS: &str = "ZEJ4BLG3XWAUUZQGCEDJLYIC6D2NCWHRSX5DJMDPE54PXXR7G3PCQTARXU";

    fn kat_entropy() -> Vec<u8> {
        (1..=32u8).collect()
    }

    #[test]
    fn entropy_and_mnemonic_match_known_answer() {
        let from_entropy = falcon1024_account_from_entropy(kat_entropy()).unwrap();
        assert_eq!(from_entropy.address_string, KAT_ADDRESS);
        assert_eq!(from_entropy.public_key.len(), PUBLIC_KEY_SIZE);
        assert_eq!(from_entropy.private_key.len(), PRIVATE_KEY_SIZE);

        let mnemonic = crate::algo25::mnemonic_from_seed(kat_entropy()).unwrap();
        let from_mnemonic = falcon1024_account_from_mnemonic(mnemonic).unwrap();
        assert_eq!(from_mnemonic.address_string, KAT_ADDRESS);
        assert_eq!(from_mnemonic.public_key, from_entropy.public_key);
    }

    #[test]
    fn canonical_address_matches_account() {
        let account = falcon1024_account_from_entropy(kat_entropy()).unwrap();
        let addr = falcon1024_canonical_address(account.public_key.clone()).unwrap();
        assert_eq!(addr.salt, account.salt);
        assert_eq!(addr.address, account.address);
        assert_eq!(addr.address_string, KAT_ADDRESS);
        assert_ne!(
            falcon1024_nth_address(account.public_key, 1)
                .unwrap()
                .address_string,
            KAT_ADDRESS
        );
    }

    #[test]
    fn sign_and_verify_round_trip() {
        let account = falcon1024_account_from_entropy(vec![7u8; 32]).unwrap();
        let msg = b"TX\x01\x02".to_vec();
        let sig = falcon1024_raw_sign(account.private_key.clone(), msg.clone()).unwrap();
        assert!(sig.len() <= falcon1024_signature_max_size() as usize);
        // Deterministic Falcon: same message, same signature.
        assert_eq!(
            sig,
            falcon1024_raw_sign(account.private_key, msg.clone()).unwrap()
        );
        assert!(falcon1024_verify(account.public_key.clone(), sig.clone(), msg).unwrap());
        assert!(!falcon1024_verify(account.public_key, sig, b"TX\x01\x03".to_vec()).unwrap());
    }

    #[test]
    fn rejects_bad_lengths() {
        assert!(falcon1024_account_from_entropy(vec![0u8; 31]).is_err());
        assert!(falcon1024_raw_sign(vec![0u8; 10], vec![]).is_err());
        assert!(falcon1024_verify(vec![0u8; 10], vec![1], vec![]).is_err());
        assert!(falcon1024_account_from_mnemonic("not a mnemonic".into()).is_err());
    }
}
