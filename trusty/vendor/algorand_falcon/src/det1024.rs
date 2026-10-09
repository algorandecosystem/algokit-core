//! FALCON-DET1024: deterministic Falcon with parameter n = 1024.

use alloc::vec;
use alloc::vec::Vec;
use core::fmt;
use core::ops::Deref;

use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::{Error, ErrorKind, check, ffi, raw};

/// log2 of the degree of Falcon det1024 polynomials.
pub const LOGN: u32 = 10;

/// Size in bytes of a Falcon public key.
pub const PUBLIC_KEY_SIZE: usize = 1793;
/// Size in bytes of a Falcon private key.
pub const PRIVATE_KEY_SIZE: usize = 2305;
/// The salt version number used to compute signatures. The salt version is
/// incremented when the signing procedure changes (rarely).
pub const CURRENT_SALT_VERSION: u8 = 0;
/// Size in bytes of a Falcon signature in CT format.
pub const CT_SIGNATURE_SIZE: usize = 1538;
/// Max possible size in bytes of a Falcon signature in compressed format.
pub const SIGNATURE_MAX_SIZE: usize = 1423;
/// N=1024 is the degree of Falcon det1024 polynomials.
pub const N: usize = 1 << LOGN;

/// A Falcon public key.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct PublicKey(pub [u8; PUBLIC_KEY_SIZE]);

/// A Falcon private key. Its bytes are zeroized when it is dropped.
pub struct PrivateKey(pub [u8; PRIVATE_KEY_SIZE]);

/// A deterministic Falcon signature in compressed format, which is
/// variable-length.
#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct CompressedSignature(pub Vec<u8>);

/// A deterministic Falcon signature in constant-time format, which is
/// fixed-length.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct CTSignature(pub [u8; CT_SIGNATURE_SIZE]);

impl Default for PublicKey {
    fn default() -> Self {
        PublicKey([0; PUBLIC_KEY_SIZE])
    }
}

impl Default for PrivateKey {
    fn default() -> Self {
        PrivateKey([0; PRIVATE_KEY_SIZE])
    }
}

impl Default for CTSignature {
    fn default() -> Self {
        CTSignature([0; CT_SIGNATURE_SIZE])
    }
}

impl Drop for PrivateKey {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}

impl ZeroizeOnDrop for PrivateKey {}

impl fmt::Debug for PrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PrivateKey(..)")
    }
}

impl AsRef<[u8]> for PublicKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<[u8]> for PrivateKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<[u8]> for CTSignature {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Deref for CompressedSignature {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<[u8]> for CompressedSignature {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl From<Vec<u8>> for CompressedSignature {
    fn from(v: Vec<u8>) -> Self {
        CompressedSignature(v)
    }
}

impl From<&[u8]> for CompressedSignature {
    fn from(v: &[u8]) -> Self {
        CompressedSignature(v.to_vec())
    }
}

/// Generates a public/private key pair from the given seed.
pub fn generate_key(seed: &[u8]) -> Result<(PublicKey, PrivateKey), Error> {
    let mut rng = ffi::Shake256Context {
        opaque_contents: [0; 26],
    };
    let (seed_ptr, seed_len) = raw(seed);
    let mut public_key = PublicKey::default();
    let mut private_key = PrivateKey::default();
    // SAFETY: rng is a valid context; key buffers have the sizes the C API requires.
    let r = unsafe {
        ffi::shake256_init_prng_from_seed(&mut rng, seed_ptr, seed_len);
        ffi::falcon_det1024_keygen(
            &mut rng,
            private_key.0.as_mut_ptr().cast(),
            public_key.0.as_mut_ptr().cast(),
        )
    };
    // The RNG state is derived from the seed and could be used to recover the
    // private key.
    rng.opaque_contents.zeroize();
    check(ErrorKind::Keygen, r)?;
    Ok((public_key, private_key))
}

impl PrivateKey {
    /// Signs the message and returns a compressed-format signature, or an
    /// error if signing fails (e.g., due to a malformed private key).
    pub fn sign_compressed(&self, msg: &[u8]) -> Result<CompressedSignature, Error> {
        let mut sig = vec![0u8; SIGNATURE_MAX_SIZE];
        let mut sig_len = 0usize;
        let (msg_ptr, msg_len) = raw(msg);
        // SAFETY: sig has SIGNATURE_MAX_SIZE bytes; privkey has PRIVATE_KEY_SIZE bytes.
        let r = unsafe {
            ffi::falcon_det1024_sign_compressed(
                sig.as_mut_ptr().cast(),
                &mut sig_len,
                self.0.as_ptr().cast(),
                msg_ptr,
                msg_len,
            )
        };
        check(ErrorKind::Sign, r)?;
        sig.truncate(sig_len);
        Ok(CompressedSignature(sig))
    }
}

impl CompressedSignature {
    /// Converts a compressed-format signature to a CT-format signature.
    pub fn convert_to_ct(&self) -> Result<CTSignature, Error> {
        if self.0.len() < 2 {
            return Err(Error::rejected(ErrorKind::Convert));
        }
        let mut sig_ct = CTSignature::default();
        // SAFETY: sig_ct has CT_SIGNATURE_SIZE bytes; the input slice is valid for its length.
        let r = unsafe {
            ffi::falcon_det1024_convert_compressed_to_ct(
                sig_ct.0.as_mut_ptr().cast(),
                self.0.as_ptr().cast(),
                self.0.len(),
            )
        };
        check(ErrorKind::Convert, r)?;
        Ok(sig_ct)
    }

    /// Returns the salt version used in a compressed-format signature. By
    /// definition, the default salt version is 0, if the signature is too
    /// short to specify one. (Such a signature is malformed, and would not pass
    /// verification, but is still considered to have a salt version.)
    pub fn salt_version(&self) -> u8 {
        self.0.get(1).copied().unwrap_or(0)
    }
}

impl CTSignature {
    /// Returns the salt version used in a CT-format signature.
    pub fn salt_version(&self) -> u8 {
        self.0[1]
    }

    /// Unpacks a signature in CT format to the vector of polynomial
    /// coefficients of the associated ring element s_2. See Section 3.10 of
    /// the Falcon specification for details. Returns an error if the signature
    /// cannot be properly unpacked.
    pub fn s2_coefficients(&self) -> Result<[i16; N], Error> {
        let mut s2 = [0i16; N];
        // SAFETY: s2 has N entries; sig has CT_SIGNATURE_SIZE bytes.
        let r = unsafe { ffi::falcon_det1024_s2_coeffs(s2.as_mut_ptr(), self.0.as_ptr().cast()) };
        check(ErrorKind::S2Coefficients, r)?;
        Ok(s2)
    }
}

impl PublicKey {
    /// Reports whether `signature` is a valid compressed-format signature of
    /// `msg` under this public key.
    ///
    /// Accepts any byte slice, so a `&CompressedSignature` can be passed
    /// directly, as can raw bytes without first copying them into one.
    pub fn verify(&self, signature: &[u8], msg: &[u8]) -> Result<(), Error> {
        if signature.is_empty() {
            return Err(Error::rejected(ErrorKind::Verify));
        }
        let (msg_ptr, msg_len) = raw(msg);
        // SAFETY: the signature slice is valid for its length; pubkey has PUBLIC_KEY_SIZE bytes.
        let r = unsafe {
            ffi::falcon_det1024_verify_compressed(
                signature.as_ptr().cast(),
                signature.len(),
                self.0.as_ptr().cast(),
                msg_ptr,
                msg_len,
            )
        };
        check(ErrorKind::Verify, r)
    }

    /// Reports whether `signature` is a valid CT-format signature of `msg`
    /// under this public key.
    pub fn verify_ct_signature(&self, signature: &CTSignature, msg: &[u8]) -> Result<(), Error> {
        let (msg_ptr, msg_len) = raw(msg);
        // SAFETY: sig has CT_SIGNATURE_SIZE bytes; pubkey has PUBLIC_KEY_SIZE bytes.
        let r = unsafe {
            ffi::falcon_det1024_verify_ct(
                signature.0.as_ptr().cast(),
                self.0.as_ptr().cast(),
                msg_ptr,
                msg_len,
            )
        };
        check(ErrorKind::Verify, r)
    }

    /// Unpacks a public key representing a ring element h to its vector of
    /// polynomial coefficients, i.e.,
    ///
    /// h(x) = h\[0\] + h\[1\] * x + h\[2\] * x^2 + ... + h\[1023\] * x^1023.
    ///
    /// Returns an error if the public key is invalid.
    pub fn coefficients(&self) -> Result<[u16; N], Error> {
        let mut h = [0u16; N];
        // SAFETY: h has N entries; pubkey has PUBLIC_KEY_SIZE bytes.
        let r =
            unsafe { ffi::falcon_det1024_pubkey_coeffs(h.as_mut_ptr(), self.0.as_ptr().cast()) };
        check(ErrorKind::PubkeyCoefficients, r)?;
        Ok(h)
    }
}

/// Computes the vector of polynomial coefficients of s_1 = c - s_2 * h, given
/// the unpacked values h, c, and s_2. See Section 3.10 of the Falcon
/// specification for details. Returns an error if the aggregate (s_1,s_2)
/// vector is not short enough to constitute a valid signature (for the public
/// key corresponding to h, the hash digest corresponding to c, and the
/// signature corresponding to s_2).
pub fn s1_coefficients(h: &[u16; N], c: &[u16; N], s2: &[i16; N]) -> Result<[i16; N], Error> {
    let mut s1 = [0i16; N];
    // SAFETY: all arrays have N entries.
    let r = unsafe {
        ffi::falcon_det1024_s1_coeffs(s1.as_mut_ptr(), h.as_ptr(), c.as_ptr(), s2.as_ptr())
    };
    check(ErrorKind::S1Coefficients, r)?;
    Ok(s1)
}

/// Hashes `msg` using the fixed 40-byte salt specified by `salt_version`, to a
/// ring element c, represented by its vector of polynomial coefficients. See
/// Section 3.7 of the Falcon specification for the details of the hashing, and
/// Section 2.3.2-3 of the Deterministic Falcon specification for the
/// definition of the fixed salt.
pub fn hash_to_point_coefficients(msg: &[u8], salt_version: u8) -> [u16; N] {
    let mut c = [0u16; N];
    let (msg_ptr, msg_len) = raw(msg);
    // SAFETY: c has N entries; msg is valid for msg_len bytes.
    unsafe {
        ffi::falcon_det1024_hash_to_point_coeffs(c.as_mut_ptr(), msg_ptr, msg_len, salt_version)
    };
    c
}
