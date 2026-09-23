# Hand-written Trusty build module for this crate (`algokit_crypto`).
#
# `cargo2rulesmk.py` (AOSP's Cargo->Trusty generator) only handles
# crates.io/git dependencies, not first-party in-workspace crates like this
# one -- so this is hand-written, not generated, same as
# `trusty/app/seed_vault_ta/rules.mk` and `trusty/vendor/*/rules.mk`. CI
# copies this whole crate directory into the synced Trusty tree at
# `trusty/user/base/lib/algokit_crypto` (see trusty_ci.yml), matching the
# path referenced from `trusty/app/seed_vault_ta/rules.mk`'s
# `MODULE_LIBRARY_DEPS`.
#
# Built with *no* cargo features set (not even this crate's own `default`
# list) -- i.e. the same shape as `seed_vault`'s
# `algokit_crypto = { path = "...", default-features = false }` dependency:
#   - No "std" -> `#![cfg_attr(not(feature = "std"), no_std)]` applies.
#   - No "xhd" -> `#[cfg(feature = "xhd")] pub mod xhd;` is excluded, so
#     `bip39`/`snafu`/the `ed25519-bip32` git fork are never referenced by
#     this build at all (see the `xhd` feature doc-comment in `Cargo.toml`).
# `seed_vault` (this crate's only consumer on Trusty) never calls into
# `xhd`, so this matches its actual real-world usage exactly, not just a
# convenient subset.

LOCAL_DIR := $(GET_LOCAL_DIR)
MODULE := $(LOCAL_DIR)
MODULE_CRATE_NAME := algokit_crypto
MODULE_RUST_CRATE_TYPES := rlib
MODULE_SRCS := $(LOCAL_DIR)/src/lib.rs
MODULE_ADD_IMPLICIT_DEPS := false

# NOT 2024, despite this crate's real `Cargo.toml` declaring
# `edition = "2024"`: Trusty's own bundled rustc (baked into the synced AOSP
# tree, entirely independent of this workspace's `rust-toolchain`) rejects
# it outright -- "edition 2024 is unstable and only available with
# -Z unstable-options", i.e. it predates edition 2024's stabilization
# (rustc 1.85). This crate's source has no edition-2024-only syntax
# (no `unsafe extern` blocks, no `#[unsafe(..)]` attributes), so 2021
# compiles identically. Same story on `crates/seed_vault/rules.mk` and
# `trusty/vendor/getrandom/rules.mk`.
MODULE_RUST_EDITION := 2021

MODULE_LIBRARY_DEPS := \
	trusty/user/base/lib/cryptoxide \
	trusty/user/base/lib/base32 \
	trusty/user/base/lib/signature \
	trusty/user/base/lib/getrandom \
	external/rust/android-crates-io/crates/zeroize \
	external/rust/android-crates-io/crates/async-trait \
	trusty/user/base/lib/liballoc-rust \
	trusty/user/base/lib/libcompiler_builtins-rust \
	trusty/user/base/lib/libcore-rust

include make/library.mk
