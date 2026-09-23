//! Algorand address encoding.
//!
//! A standard (non-multisig, non-application) Algorand address is the
//! 58-character base32 (RFC4648, no padding) encoding of a 32-byte ed25519
//! public key followed by a 4-byte checksum (the last 4 bytes of
//! `SHA-512/256(public_key)`).
//!
//! This lives in `algokit_crypto` (rather than only in `algokit_abi`, which
//! already has an internal copy of this same logic for ABI `address` value
//! encoding) so that any consumer holding nothing but a raw ed25519 public
//! key -- e.g. a Trusty Trusted Application that creates/stores algo25
//! accounts and needs to hand back a human-readable address without ever
//! exposing the private key -- can derive it without depending on the ABI
//! crate.

use alloc::string::String;
use sha2::{Digest, Sha512_256};

/// Number of checksum bytes appended to the public key before base32 encoding.
const CHECKSUM_LEN: usize = 4;

/// Derives the standard 58-character Algorand address from a raw 32-byte
/// ed25519 public key.
pub fn address_from_public_key(public_key: &[u8; 32]) -> String {
    let hash = Sha512_256::digest(public_key);

    let mut buf = [0u8; 32 + CHECKSUM_LEN];
    buf[..32].copy_from_slice(public_key);
    buf[32..].copy_from_slice(&hash[hash.len() - CHECKSUM_LEN..]);

    base32::encode(base32::Alphabet::Rfc4648 { padding: false }, &buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_zero_key_address_is_stable_and_58_chars() {
        // Regression/format check: same input must always produce the same
        // address, and it must be exactly the standard 58 characters long.
        let public_key = [0u8; 32];
        let address = address_from_public_key(&public_key);
        assert_eq!(address.len(), 58);
        assert_eq!(address, address_from_public_key(&public_key));
    }

    #[test]
    fn different_keys_produce_different_addresses() {
        let a = address_from_public_key(&[1u8; 32]);
        let b = address_from_public_key(&[2u8; 32]);
        assert_ne!(a, b);
    }
}
