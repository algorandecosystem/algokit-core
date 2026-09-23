//! Tiny internal SHA-512/256 helper shared by `address` and `algo25`.
//!
//! Both call sites only ever need a one-shot digest of a single buffer, so
//! rather than depending on the separate `sha2` crate (which pulls in the
//! whole RustCrypto `digest`/`block-buffer`/`generic-array`/`typenum`/
//! `crypto-common`/`cpufeatures` subtree -- none of which are pre-vendored
//! in AOSP, unlike `cryptoxide` itself, see
//! `trusty/app/seed_vault_ta/rules.mk`), this reuses the `Sha512Trunc256`
//! implementation already shipped by `cryptoxide` (an unavoidable direct
//! dependency of this crate for ed25519 anyway). `Sha512Trunc256` is
//! `cryptoxide`'s name for exactly the same NIST SHA-512/256 algorithm the
//! `sha2` crate calls `Sha512_256` -- SHA-512 with the truncated 256-bit IV,
//! not SHA-256 run twice or SHA-512 truncated post-hoc.

use cryptoxide::digest::Digest;
use cryptoxide::sha2::Sha512Trunc256;

/// One-shot SHA-512/256 digest of `data`.
pub(crate) fn sha512_256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha512Trunc256::new();
    hasher.input(data);
    let mut out = [0u8; 32];
    hasher.result(&mut out);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_known_sha512_256_test_vector() {
        // NIST test vector: SHA-512/256("abc")
        let digest = sha512_256(b"abc");
        assert_eq!(
            digest,
            [
                0x53, 0x04, 0x8e, 0x26, 0x81, 0x94, 0x1e, 0xf9, 0x9b, 0x2e, 0x29, 0xb7, 0x6b, 0x4c,
                0x7d, 0xab, 0xe4, 0xc2, 0xd0, 0xc6, 0x34, 0xfc, 0x6d, 0x46, 0xe0, 0xe2, 0xf1, 0x31,
                0x07, 0xe7, 0xaf, 0x23
            ]
        );
    }
}
