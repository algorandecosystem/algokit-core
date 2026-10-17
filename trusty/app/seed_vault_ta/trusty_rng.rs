//! `getrandom` 0.4's *custom backend* entry point for Trusty.
//!
//! `getrandom` (used by `seed_vault`/`algokit_crypto` to source entropy for
//! brand-new accounts in `CREATE_ACCOUNT`) has no built-in backend for
//! `*-unknown-trusty` at all -- see the `getrandom` dependency comment in
//! `crates/algokit_crypto/Cargo.toml`. Its vendored copy at
//! `trusty/vendor/getrandom` is built with `--cfg
//! getrandom_backend="custom"` (see that crate's `rules.mk`), which makes it
//! call out, at link time, to an `unsafe extern "Rust" fn
//! __getrandom_v03_custom` that the *binary* crate must define exactly once
//! (see <https://docs.rs/getrandom/0.4.1/getrandom/#custom-backend>). This
//! is that definition.
//!
//! The actual entropy source is Trusty's own `trusty_rng_secure_rand`
//! (declared in `include/lib/rng/trusty_rng.h` in the upstream `trusty/lib`
//! repo -- which is checked out at `trusty/user/base` in a synced Trusty
//! tree, per `trusty/manifest`'s `default.xml`, so the real dependency path
//! is `trusty/user/base/lib/rng`; see `MODULE_LIBRARY_DEPS` in
//! `../rules.mk`): a CSPRNG backed by BoringSSL's `RAND_bytes`, itself
//! reseeded from the platform HWRNG. Callers are steered towards this
//! rather than `trusty_rng_hw_rand` (direct HWRNG access) specifically to
//! avoid the IPC round-trip -- see that header's doc comment.

use getrandom::Error;

// Deliberately the plain (pre-Rust-1.82) `extern "C" { .. }` form, not
// `unsafe extern "C" { .. }`: Trusty's own bundled rustc (baked into the
// synced AOSP tree, independent of this workspace's `rust-toolchain`) is
// old enough to reject `edition = "2024"` outright ("unstable, only
// available with -Z unstable-options"), so every module built as part of
// the Trusty OS/applet image is pinned to edition 2021 (see
// `MODULE_RUST_EDITION` in `../rules.mk`) rather than 2024 like the rest of
// this workspace's crates. Declarations inside an `extern` block are always
// unsafe to call regardless of whether the block itself is written with a
// leading `unsafe` -- that keyword only changes whether *writing* the
// block requires `unsafe`, not the callee's safety contract -- so this is
// semantically identical, just compatible with older compilers too.
extern "C" {
    /// `int trusty_rng_secure_rand(uint8_t* data, size_t len);` --
    /// `include/lib/rng/trusty_rng.h` in `trusty/user/base/lib/rng`. Fully
    /// overwrites `data[..len]` on success (`NO_ERROR`, i.e. `0`); on
    /// failure it leaves `data` untouched and returns one of `uapi/err.h`'s
    /// (small, negative) `ERR_*` codes.
    fn trusty_rng_secure_rand(data: *mut u8, len: usize) -> core::ffi::c_int;
}

/// # Safety
///
/// Must only ever be called by `getrandom`'s `custom` backend
/// (`trusty/vendor/getrandom/src/backends/custom.rs`), which upholds the
/// contract this relies on: `dest` is valid for `len` bytes and, even
/// though it may start out uninitialized, is safe to pass to
/// `trusty_rng_secure_rand` because that function only ever writes to it
/// (via `RAND_bytes`), never reads from it.
// Plain `#[no_mangle]`, not `#[unsafe(no_mangle)]` -- same edition-2021
// compatibility reasoning as the `extern "C"` block above. `unsafe extern
// "Rust" fn` here is an ordinary FFI-export function signature (`unsafe` +
// `extern "Rust"` modifiers on a single item), not the newer extern-*block*
// syntax, so it's valid on every Rust edition and needs no changes.
#[no_mangle]
unsafe extern "Rust" fn __getrandom_v03_custom(dest: *mut u8, len: usize) -> Result<(), Error> {
    // SAFETY: forwarding the exact same (valid-for-`len`-bytes,
    // possibly-uninitialized) pointer/length pair this function received,
    // per the contract on `trusty_rng_secure_rand` above.
    let ret = unsafe { trusty_rng_secure_rand(dest, len) };
    if ret == 0 {
        Ok(())
    } else {
        // Trusty's `NO_ERROR` is `0`; every `ERR_*` code in `uapi/err.h` is a
        // small negative number. `getrandom::Error` only carries a bare
        // custom code (no room for arbitrary OS error semantics on this
        // target), so surface the magnitude of that code rather than
        // collapsing every failure into `Error::UNEXPECTED`.
        Err(Error::new_custom(ret.unsigned_abs() as u16))
    }
}
