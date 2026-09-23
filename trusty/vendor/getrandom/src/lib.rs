// Vendored (hand-trimmed, not full-copy) from `getrandom` 0.4.1
// (https://crates.io/crates/getrandom), which is not vendored in AOSP's
// `external/rust/android-crates-io` as of this writing.
//
// Only the files needed to build with `--cfg getrandom_backend="custom"` are
// included here: `lib.rs`, `backends.rs`, `backends/custom.rs`, `error.rs`,
// `util.rs`. All of the upstream crate's real per-OS backends (linux, bsd,
// windows, wasm, ...), its `sys_rng`/`rand_core` adapter, and its `std`-only
// `Error` impls are deliberately left out: none of them are reachable once
// the `custom` backend is selected (see `backends.rs`'s `cfg_if!`), and they
// pull in things (`libc`, target-specific `unsafe extern "C"` syscalls) this
// build has no way to vendor or verify against real Trusty headers anyway.
//
// The one intentional content change vs. upstream: the
// `#![doc = include_str!("../README.md")]` line is dropped, since we don't
// vendor the README. Purely a doc-comment difference, no functional change.
//
// See `../rules.mk` for how `--cfg getrandom_backend="custom"` gets set, and
// `trusty/app/seed_vault_ta/main.rs` for the actual
// `__getrandom_v03_custom` implementation this crate calls out to (backed by
// Trusty's real HWRNG-derived CSPRNG, `trusty_rng_secure_rand`).

// Overwrite links to crate items with intra-crate links
//! [`Error::UNEXPECTED`]: Error::UNEXPECTED
//! [`fill_uninit`]: fill_uninit

#![no_std]
#![doc(
    html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk.png",
    html_favicon_url = "https://www.rust-lang.org/favicon.ico"
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(getrandom_backend = "efi_rng", feature(uefi_std))]
#![cfg_attr(getrandom_backend = "extern_impl", feature(extern_item_impls))]

#[macro_use]
extern crate cfg_if;

use core::mem::MaybeUninit;

mod backends;
mod error;
mod util;

pub use crate::error::{Error, RawOsError};

/// Attribute macros for overwriting the core functionality of this crate.
///
/// This allows `getrandom` to provide a default implementation and a common interface
/// for all crates to use, while giving users a safe way to override that default where required.
///
/// Must be enabled via the `extern_impl` opt-in backend, as this functionality
/// is currently limited to nightly.
#[cfg(getrandom_backend = "extern_impl")]
pub mod implementation {
    pub use crate::backends::extern_impl::{fill_uninit, u32, u64};
}

/// Fill `dest` with random bytes from the system's preferred random number source.
///
/// This function returns an error on any failure, including partial reads. We
/// make no guarantees regarding the contents of `dest` on error. If `dest` is
/// empty, `getrandom` immediately returns success, making no calls to the
/// underlying operating system.
///
/// Blocking is possible, at least during early boot; see module documentation.
///
/// In general, `getrandom` will be fast enough for interactive usage, though
/// significantly slower than a user-space CSPRNG; for the latter consider
/// [`rand::thread_rng`](https://docs.rs/rand/*/rand/fn.thread_rng.html).
#[inline]
pub fn fill(dest: &mut [u8]) -> Result<(), Error> {
    // SAFETY: The `&mut MaybeUninit<_>` reference doesn't escape,
    // and `fill_uninit` guarantees it will never de-initialize
    // any part of `dest`.
    fill_uninit(unsafe { util::slice_as_uninit_mut(dest) })?;
    Ok(())
}

/// Fill potentially uninitialized buffer `dest` with random bytes from
/// the system's preferred random number source and return a mutable
/// reference to those bytes.
///
/// On successful completion this function is guaranteed to return a slice
/// which points to the same memory as `dest` and has the same length.
/// In other words, it's safe to assume that `dest` is initialized after
/// this function has returned `Ok`.
///
/// No part of `dest` will ever be de-initialized at any point, regardless
/// of what is returned.
#[inline]
pub fn fill_uninit(dest: &mut [MaybeUninit<u8>]) -> Result<&mut [u8], Error> {
    if !dest.is_empty() {
        backends::fill_inner(dest)?;
    }

    #[cfg(getrandom_msan)]
    unsafe extern "C" {
        fn __msan_unpoison(a: *mut core::ffi::c_void, size: usize);
    }

    // SAFETY: `dest` has been fully initialized by `imp::fill_inner`
    // since it returned `Ok`.
    Ok(unsafe { util::slice_assume_init_mut(dest) })
}

/// Get random `u32` from the system's preferred random number source.
#[inline]
pub fn u32() -> Result<u32, Error> {
    backends::inner_u32()
}

/// Get random `u64` from the system's preferred random number source.
#[inline]
pub fn u64() -> Result<u64, Error> {
    backends::inner_u64()
}
