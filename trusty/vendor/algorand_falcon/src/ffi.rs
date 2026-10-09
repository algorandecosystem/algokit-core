//! Raw FFI declarations for the FALCON-DET1024 C API (falcon/deterministic.h)
//! and the SHAKE256 seeding helper it depends on (falcon/falcon.h).

use core::ffi::{c_int, c_void};

/// Mirrors `shake256_context` from falcon.h.
#[repr(C)]
pub struct Shake256Context {
    pub opaque_contents: [u64; 26],
}

// TRUSTY VENDOR PATCH: upstream writes `unsafe extern "C" {` (edition-2024
// syntax). Trusty's bundled rustc predates edition 2024, so this vendored
// copy uses the always-valid, semantically identical plain `extern "C"`
// form -- same story as `trusty/vendor/getrandom`.
extern "C" {
    pub fn shake256_init_prng_from_seed(
        sc: *mut Shake256Context,
        seed: *const c_void,
        seed_len: usize,
    );

    pub fn falcon_det1024_keygen(
        rng: *mut Shake256Context,
        privkey: *mut c_void,
        pubkey: *mut c_void,
    ) -> c_int;
    pub fn falcon_det1024_sign_compressed(
        sig: *mut c_void,
        sig_len: *mut usize,
        privkey: *const c_void,
        data: *const c_void,
        data_len: usize,
    ) -> c_int;
    pub fn falcon_det1024_verify_compressed(
        sig: *const c_void,
        sig_len: usize,
        pubkey: *const c_void,
        data: *const c_void,
        data_len: usize,
    ) -> c_int;
    pub fn falcon_det1024_verify_ct(
        sig: *const c_void,
        pubkey: *const c_void,
        data: *const c_void,
        data_len: usize,
    ) -> c_int;
    pub fn falcon_det1024_convert_compressed_to_ct(
        sig_ct: *mut c_void,
        sig_compressed: *const c_void,
        sig_compressed_len: usize,
    ) -> c_int;
    pub fn falcon_det1024_pubkey_coeffs(h: *mut u16, pubkey: *const c_void) -> c_int;
    pub fn falcon_det1024_hash_to_point_coeffs(
        c: *mut u16,
        data: *const c_void,
        data_len: usize,
        salt_version: u8,
    );
    pub fn falcon_det1024_s2_coeffs(s2: *mut i16, sig: *const c_void) -> c_int;
    pub fn falcon_det1024_s1_coeffs(
        s1: *mut i16,
        h: *const u16,
        c: *const u16,
        s2: *const i16,
    ) -> c_int;
}
