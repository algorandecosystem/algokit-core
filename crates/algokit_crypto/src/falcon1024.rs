//! Native Algorand post-quantum (PQ) accounts backed by deterministic
//! Falcon-1024, derived from the *same* 25-word mnemonic as a standard
//! `algo25` account.
//!
//! This is a Rust port of go-algorand's `cmd/algokey/pq_scheme.go` +
//! `data/basics/pq_address.go` (and the go-algorand-sdk's
//! `mnemonic.ToPQSeed` / `crypto.Falcon1024AccountFromPQSeed`):
//!
//! ```text
//! entropy  = algo25::seed_from_mnemonic(words)              // 32 bytes
//! pq_seed  = SHA512/256("PQK" || "f1" || entropy)           // 32 bytes
//! (pk, sk) = falcon_det1024_keygen(SHAKE256(pq_seed))       // 1793 / 2305 bytes
//! salt     = lowest s in 0..=255 such that
//!            addr(s) = SHA512/256("PQA" || "f1" || s || pk)
//!            does NOT decode as an Ed25519 point
//! address  = algorand_address(addr(salt))
//! ```
//!
//! The Ed25519-point rejection is what makes a PQ address "PQ compliant": it
//! guarantees no Ed25519 private key can ever exist for that address, so the
//! account can only be authorized by its Falcon key.
//!
//! The same mnemonic therefore yields two *different* addresses depending on
//! whether it's interpreted as an `algo25` (Ed25519) account or a Falcon-1024
//! account -- the 25 words are only the backup encoding of the entropy.

use alloc::string::String;

use algorand_falcon::det1024;
use cryptoxide::curve25519::Ge;
use zeroize::Zeroize;

use crate::address::address_from_public_key;
use crate::hash::sha512_256;

pub use algorand_falcon::Error as FalconError;
pub use det1024::{PRIVATE_KEY_SIZE, PUBLIC_KEY_SIZE, PrivateKey, PublicKey, SIGNATURE_MAX_SIZE};

/// Byte length of the mnemonic entropy a Falcon-1024 account is derived from
/// (identical to an `algo25` seed, so the same 25 words back both).
pub const ENTROPY_LEN: usize = 32;

/// Two-byte PQ scheme identifier for Falcon-1024 (`protocol.PQSchemeFalcon1024`).
pub const SCHEME_FALCON1024: [u8; 2] = *b"f1";

/// Domain separator for mnemonic entropy -> PQ keygen seed (`protocol.PostQuantumKey`).
const DOMAIN_PQ_KEY: &[u8] = b"PQK";

/// Domain separator for PQ address derivation (`protocol.PostQuantumAddress`).
const DOMAIN_PQ_ADDRESS: &[u8] = b"PQA";

/// Errors deriving a Falcon-1024 account.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Falcon1024Error {
    /// The underlying Falcon C implementation failed.
    Falcon(FalconError),
    /// Fewer PQ-compliant salts than requested exist in `0..=255` (for the
    /// canonical salt the probability is ~2^-256; effectively unreachable).
    NoCanonicalSalt,
}

impl core::fmt::Display for Falcon1024Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Falcon(e) => write!(f, "{e}"),
            Self::NoCanonicalSalt => f.write_str("no canonical PQ address salt found"),
        }
    }
}

impl core::error::Error for Falcon1024Error {}

impl From<FalconError> for Falcon1024Error {
    fn from(e: FalconError) -> Self {
        Self::Falcon(e)
    }
}

/// A derived native Falcon-1024 PQ account. The private key is zeroized on
/// drop by `algorand_falcon`.
pub struct Falcon1024Account {
    pub public_key: PublicKey,
    pub private_key: PrivateKey,
    /// The canonical address salt (lowest PQ-compliant salt).
    pub salt: u8,
    /// Raw 32-byte Algorand address.
    pub address: [u8; 32],
}

impl Falcon1024Account {
    /// The 58-character base32 Algorand address string.
    pub fn address_string(&self) -> String {
        address_from_public_key(&self.address)
    }
}

impl core::fmt::Debug for Falcon1024Account {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Falcon1024Account")
            .field("salt", &self.salt)
            .field("address", &self.address_string())
            .finish_non_exhaustive()
    }
}

/// Maps 32 bytes of mnemonic entropy to the scheme-specific Falcon-1024
/// keygen seed: `SHA512/256("PQK" || "f1" || entropy)`.
///
/// Mirrors go-algorand's `derivePQKeySeed` / go-algorand-sdk's
/// `mnemonic.ToPQSeed(.., PQSchemeFalcon1024)`.
pub fn pq_seed_from_entropy(entropy: &[u8; ENTROPY_LEN]) -> [u8; 32] {
    let mut input = [0u8; 3 + 2 + ENTROPY_LEN];
    input[..3].copy_from_slice(DOMAIN_PQ_KEY);
    input[3..5].copy_from_slice(&SCHEME_FALCON1024);
    input[5..].copy_from_slice(entropy);
    let seed = sha512_256(&input);
    input.zeroize();
    seed
}

/// Derives the full Falcon-1024 account from a PQ keygen seed (the output of
/// [`pq_seed_from_entropy`]). Mirrors `crypto.Falcon1024AccountFromPQSeed`.
pub fn account_from_pq_seed(pq_seed: &[u8; 32]) -> Result<Falcon1024Account, Falcon1024Error> {
    let (public_key, private_key) = det1024::generate_key(pq_seed)?;
    let (salt, address) = canonical_pq_address(&public_key.0)?;
    Ok(Falcon1024Account {
        public_key,
        private_key,
        salt,
        address,
    })
}

/// Derives the Falcon-1024 account backed by the same 32-byte entropy a
/// 25-word Algorand mnemonic encodes.
pub fn account_from_entropy(
    entropy: &[u8; ENTROPY_LEN],
) -> Result<Falcon1024Account, Falcon1024Error> {
    let mut pq_seed = pq_seed_from_entropy(entropy);
    let account = account_from_pq_seed(&pq_seed);
    pq_seed.zeroize();
    account
}

/// Derives the Falcon-1024 account from a 25-word Algorand mnemonic.
pub fn account_from_mnemonic(mnemonic: &str) -> Result<Falcon1024Account, Falcon1024MnemonicError> {
    let mut entropy =
        crate::algo25::seed_from_mnemonic(mnemonic).map_err(Falcon1024MnemonicError::Mnemonic)?;
    let account = account_from_entropy(&entropy).map_err(Falcon1024MnemonicError::Derivation);
    entropy.zeroize();
    account
}

/// Errors from [`account_from_mnemonic`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Falcon1024MnemonicError {
    Mnemonic(crate::algo25::MnemonicError),
    Derivation(Falcon1024Error),
}

impl core::fmt::Display for Falcon1024MnemonicError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Mnemonic(e) => write!(f, "{e}"),
            Self::Derivation(e) => write!(f, "{e}"),
        }
    }
}

impl core::error::Error for Falcon1024MnemonicError {}

/// `SHA512/256("PQA" || scheme || salt || public_key)` -- go-algorand's
/// `basics.PQAddress`.
pub fn pq_address(scheme: [u8; 2], salt: u8, public_key: &[u8]) -> [u8; 32] {
    let mut preimage = alloc::vec::Vec::with_capacity(3 + 2 + 1 + public_key.len());
    preimage.extend_from_slice(DOMAIN_PQ_ADDRESS);
    preimage.extend_from_slice(&scheme);
    preimage.push(salt);
    preimage.extend_from_slice(public_key);
    sha512_256(&preimage)
}

/// Whether `address` is eligible for native PQ authorization, i.e. it does
/// **not** decode as an Ed25519 point (go-algorand's
/// `Address.IsPQCompliant` / `!crypto.IsEdwards25519Point`).
///
/// `cryptoxide`'s decoder follows the same permissive RFC 8032 rules as
/// `filippo.io/edwards25519`'s `Point.SetBytes` (non-canonical `y` encodings
/// accepted, only "no square root for x" rejected), so the predicate matches
/// go-algorand bit-for-bit -- see the known-answer tests below.
pub fn is_pq_compliant(address: &[u8; 32]) -> bool {
    Ge::from_bytes(address).is_none()
}

/// Returns the lowest PQ-compliant salt and its address for a Falcon-1024
/// public key (go-algorand's `basics.CanonicalPQAddressSalt`).
pub fn canonical_pq_address(public_key: &[u8]) -> Result<(u8, [u8; 32]), Falcon1024Error> {
    nth_pq_address(public_key, 0)
}

/// Returns the `n`th-lowest (0-based) PQ-compliant salt and its address for
/// a Falcon-1024 public key; `n = 0` is [`canonical_pq_address`].
///
/// Every compliant salt gives a distinct address authorized by the *same*
/// key: go-algorand consensus only checks `PQAddress(scheme, salt, pk)`
/// against the authorizer, and algod's submission check only requires the
/// address to be PQ-compliant, so non-canonical salts are valid on-chain
/// (go-algorand-sdk signs with one via `SaltedPQSigner`). Roughly half of
/// all salts are compliant, so small `n` always exist in practice.
pub fn nth_pq_address(public_key: &[u8], n: usize) -> Result<(u8, [u8; 32]), Falcon1024Error> {
    (0..=u8::MAX)
        .map(|salt| (salt, pq_address(SCHEME_FALCON1024, salt, public_key)))
        .filter(|(_, addr)| is_pq_compliant(addr))
        .nth(n)
        .ok_or(Falcon1024Error::NoCanonicalSalt)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// go-algorand `falconPublicKeyForPQAddressTest`: keygen from a 32-byte
    /// seed that's all zero except the first byte.
    fn test_public_key(first_seed_byte: u8) -> PublicKey {
        let mut seed = [0u8; 32];
        seed[0] = first_seed_byte;
        det1024::generate_key(&seed).unwrap().0
    }

    /// go-algorand `data/basics/pq_address_test.go::TestPQAddressKnownAnswers`.
    #[test]
    fn pq_address_known_answers() {
        let cases: &[(u8, u8, &str, bool)] = &[
            (
                3,
                0,
                "KJGJA2DTCQH6LT2I2OH2YO5GIIBFC6JHX5O6UPA5ZZ5ZURFT3LHKMTRCEM",
                true,
            ),
            (
                1,
                1,
                "GYBWVYVQIQF6CO7BUMG4UQ66DQYHASFOCA2P7PBYOIPKGWUZIBX4KA3TP4",
                true,
            ),
            (
                0,
                255,
                "YJFADDEP6Z3WAWY6ZMLN6MF4T4NK3BXKCVLPCYB6C4SQHE76LLQSZ5JG7Q",
                true,
            ),
            (
                2,
                2,
                "II4DO6IIP3EAEQMWJEOLOUU3VBRVCH3WF4MX6UCRUD36DOQJ3YSHA2DV5A",
                true,
            ),
            (
                255,
                255,
                "3JXWI6BYYEO6WO6M7TC4SOZAZUWAD4RQO5GJ2ED6MYIEVVLOVJOETMGG4A",
                true,
            ),
            (
                1,
                0,
                "FLX4VRWXQ65HD5G5BI2EPHJWMERHA2EBBQ7XMTZLATXH4XEOWPQSIYVIF4",
                false,
            ),
        ];
        for &(first_seed_byte, salt, expected, compliant) in cases {
            let pk = test_public_key(first_seed_byte);
            let addr = pq_address(SCHEME_FALCON1024, salt, &pk.0);
            assert_eq!(address_from_public_key(&addr), expected);
            assert_eq!(is_pq_compliant(&addr), compliant, "{expected}");
        }
    }

    /// go-algorand `TestCanonicalPQAddressSalt`.
    #[test]
    fn canonical_salt_known_answer() {
        let pk = test_public_key(1);
        let (salt, addr) = canonical_pq_address(&pk.0).unwrap();
        assert_eq!(salt, 1);
        assert_eq!(
            address_from_public_key(&addr),
            "GYBWVYVQIQF6CO7BUMG4UQ66DQYHASFOCA2P7PBYOIPKGWUZIBX4KA3TP4"
        );
    }

    #[test]
    fn nth_pq_address_walks_compliant_salts_in_order() {
        // go-algorand KATs for this key: salt 0 is an Ed25519 point, salt 1
        // is the canonical (lowest compliant) salt.
        let pk = test_public_key(1);
        assert_eq!(
            nth_pq_address(&pk.0, 0).unwrap(),
            canonical_pq_address(&pk.0).unwrap()
        );
        assert_eq!(nth_pq_address(&pk.0, 0).unwrap().0, 1);

        let compliant: alloc::vec::Vec<_> = (0..=u8::MAX)
            .filter(|&s| is_pq_compliant(&pq_address(SCHEME_FALCON1024, s, &pk.0)))
            .collect();
        for (n, &salt) in compliant.iter().enumerate().take(4) {
            let (got_salt, addr) = nth_pq_address(&pk.0, n).unwrap();
            assert_eq!(got_salt, salt);
            assert_eq!(addr, pq_address(SCHEME_FALCON1024, salt, &pk.0));
        }
        assert!(nth_pq_address(&pk.0, compliant.len()).is_err());
    }

    /// go-algorand `cmd/algokey/pq_test.go::TestPQGenerateUsesMnemonicSizedEntropy`:
    /// end-to-end entropy -> PQK seed -> keygen -> canonical address.
    #[test]
    fn entropy_to_address_known_answer() {
        let entropy: [u8; 32] = core::array::from_fn(|i| (i + 1) as u8);
        let account = account_from_entropy(&entropy).unwrap();
        assert!(is_pq_compliant(&account.address));
        assert_eq!(
            account.address_string(),
            "ZEJ4BLG3XWAUUZQGCEDJLYIC6D2NCWHRSX5DJMDPE54PXXR7G3PCQTARXU"
        );
    }

    #[test]
    fn mnemonic_round_trip_matches_entropy_derivation() {
        let entropy: [u8; 32] = core::array::from_fn(|i| (i + 1) as u8);
        let mnemonic = crate::algo25::mnemonic_from_seed(&entropy).unwrap();
        let from_mnemonic = account_from_mnemonic(&mnemonic).unwrap();
        assert_eq!(
            from_mnemonic.address_string(),
            "ZEJ4BLG3XWAUUZQGCEDJLYIC6D2NCWHRSX5DJMDPE54PXXR7G3PCQTARXU"
        );
    }

    #[test]
    fn derived_key_signs_and_verifies() {
        let entropy = [7u8; 32];
        let account = account_from_entropy(&entropy).unwrap();
        let sig = account.private_key.sign_compressed(b"TX\x01\x02").unwrap();
        account.public_key.verify(&sig, b"TX\x01\x02").unwrap();
    }
}
