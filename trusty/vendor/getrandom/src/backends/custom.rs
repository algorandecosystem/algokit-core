// Vendored verbatim from `getrandom` 0.4.1 -- see `../lib.rs` for
// provenance notes. This is the module that's actually selected for this
// build (`--cfg getrandom_backend="custom"`, see `../../rules.mk`).

//! An implementation which calls out to an externally defined function.
use crate::Error;
use core::mem::MaybeUninit;

pub use crate::util::{inner_u32, inner_u64};

#[inline]
pub fn fill_inner(dest: &mut [MaybeUninit<u8>]) -> Result<(), Error> {
    // Plain `extern "Rust" { .. }`, not `unsafe extern "Rust" { .. }`: this
    // module is built as edition 2021 (see `../../rules.mk`), for
    // compatibility with Trusty's own (older) bundled rustc -- see the
    // comment on the matching definition in
    // `trusty/app/seed_vault_ta/trusty_rng.rs` for why that's semantically
    // identical to upstream's `unsafe extern` form.
    extern "Rust" {
        fn __getrandom_v03_custom(dest: *mut u8, len: usize) -> Result<(), Error>;
    }
    unsafe { __getrandom_v03_custom(dest.as_mut_ptr().cast(), dest.len()) }
}
