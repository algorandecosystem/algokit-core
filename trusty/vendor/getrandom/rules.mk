# Hand-written Trusty build module for `getrandom` 0.4.1
# (https://crates.io/crates/getrandom), which is not vendored in AOSP's
# `external/rust/android-crates-io` as of this writing (unlike `zeroize`,
# `async-trait`, `cfg-if`, etc., which are and get referenced directly from
# there in seed_vault_ta/rules.mk). `src/` here is a deliberately trimmed
# subset of the real crate -- see `src/lib.rs` for exactly what's included
# and why.
#
# This crate is built with `--cfg getrandom_backend="custom"`, which selects
# `src/backends/custom.rs`: instead of one of getrandom's own (unavailable
# for `*-unknown-trusty`) platform backends, it calls out to an
# `unsafe extern "Rust" fn __getrandom_v03_custom` that the *dependent*
# binary crate must define exactly once (see
# https://docs.rs/getrandom/0.4.1/getrandom/#custom-backend). That's done in
# `trusty/app/seed_vault_ta/main.rs` (see its `trusty_rng` module), backed by
# Trusty's own `trusty_rng_secure_rand` (`trusty/user/base/lib/rng`) --
# BoringSSL's CSPRNG, reseeded from the platform HWRNG. This module only
# needs to link against `trusty/user/base/lib/rng` transitively through
# whatever binary pulls it in; it does not need it directly, since it never
# calls into libc/BoringSSL itself.

LOCAL_DIR := $(GET_LOCAL_DIR)
MODULE := $(LOCAL_DIR)
MODULE_CRATE_NAME := getrandom
MODULE_RUST_CRATE_TYPES := rlib
MODULE_SRCS := $(LOCAL_DIR)/src/lib.rs
MODULE_ADD_IMPLICIT_DEPS := false
MODULE_RUST_EDITION := 2024

MODULE_RUSTFLAGS += \
	--cfg 'getrandom_backend="custom"' \

MODULE_LIBRARY_DEPS := \
	external/rust/android-crates-io/crates/cfg-if \
	trusty/user/base/lib/libcompiler_builtins-rust \
	trusty/user/base/lib/libcore-rust

include make/library.mk
