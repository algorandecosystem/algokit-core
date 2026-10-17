# Hand-written Trusty build module for this crate (`seed_vault`).
#
# Same story as `crates/algokit_crypto/rules.mk`: first-party in-workspace
# crate, so `cargo2rulesmk.py` doesn't apply and this is hand-written. CI
# copies this whole crate directory into the synced Trusty tree at
# `trusty/user/base/lib/seed_vault` (see trusty_ci.yml), matching the path
# referenced from `trusty/app/seed_vault_ta/rules.mk`'s
# `MODULE_LIBRARY_DEPS`.
#
# This crate is `#![no_std]` unconditionally (see `src/lib.rs`) and
# deliberately free of any Trusty-specific dependency (`tipc`, `storage`)
# -- see the doc-comment at the top of `Cargo.toml` for why. The real
# Trusty applet (`trusty/app/seed_vault_ta/main.rs`) only has to implement
# `AccountStore` against Trusty's secure-storage service and wire
# `service::handle_request` up to a `tipc::Service`.

LOCAL_DIR := $(GET_LOCAL_DIR)
MODULE := $(LOCAL_DIR)
MODULE_CRATE_NAME := seed_vault
MODULE_RUST_CRATE_TYPES := rlib
MODULE_SRCS := $(LOCAL_DIR)/src/lib.rs
MODULE_ADD_IMPLICIT_DEPS := false

# NOT 2024, despite this crate's real `Cargo.toml` declaring
# `edition = "2024"`: Trusty's own bundled rustc (baked into the synced AOSP
# tree, entirely independent of this workspace's `rust-toolchain`) rejects
# it outright -- "edition 2024 is unstable and only available with
# -Z unstable-options", i.e. it predates edition 2024's stabilization
# (rustc 1.85). This crate's source has no edition-2024-only syntax
# (no `unsafe extern` blocks, no `#[unsafe(..)]` attributes -- it's pure,
# non-FFI `#![no_std]` Rust), so 2021 compiles identically. Same story on
# `crates/algokit_crypto/rules.mk` and `trusty/vendor/getrandom/rules.mk`.
MODULE_RUST_EDITION := 2021

MODULE_LIBRARY_DEPS := \
	trusty/user/base/lib/algokit_crypto \
	trusty/user/base/lib/getrandom \
	external/rust/android-crates-io/crates/zeroize \
	trusty/user/base/lib/liballoc-rust \
	trusty/user/base/lib/libcompiler_builtins-rust \
	trusty/user/base/lib/libcore-rust

include make/library.mk
