# Hand-written Trusty build module for `wasm_runtime` (first-party crate).
#
# `no_std` WASM sandbox (wasmi 0.46). Optional for seed_vault_ta: only
# built when the TA's `SEED_VAULT_WASM` is `true` (see its rules.mk). CI
# copies this crate into the synced tree at `trusty/user/base/lib/wasm_runtime`.
#
# Cargo.toml already says edition 2021, so `cargo test` checks exactly what
# Trusty compiles.

LOCAL_DIR := $(GET_LOCAL_DIR)
MODULE := $(LOCAL_DIR)
MODULE_CRATE_NAME := wasm_runtime
MODULE_RUST_CRATE_TYPES := rlib
MODULE_SRCS := $(LOCAL_DIR)/src/lib.rs
MODULE_ADD_IMPLICIT_DEPS := false
MODULE_RUST_EDITION := 2021

# The sandbox itself (the default Cargo feature `runtime`).
MODULE_RUSTFLAGS += --cfg 'feature="runtime"'

MODULE_LIBRARY_DEPS := \
	trusty/user/base/lib/cryptoxide \
	trusty/user/base/lib/wasmi \
	external/rust/android-crates-io/crates/zeroize \
	trusty/user/base/lib/liballoc-rust \
	trusty/user/base/lib/libcompiler_builtins-rust \
	trusty/user/base/lib/libcore-rust

include make/library.mk
