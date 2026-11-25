# Hand-written Trusty build module for `seed_vault_wasm` (first-party crate).
#
# Optional WASM signing-algorithm support for seed_vault_ta (see
# `src/lib.rs`). Only built when the TA's `SEED_VAULT_WASM` is `true`. CI
# copies this crate into the synced tree at
# `trusty/user/base/lib/seed_vault_wasm`. `guest/` and `tests/` are host-only.

LOCAL_DIR := $(GET_LOCAL_DIR)
MODULE := $(LOCAL_DIR)
MODULE_CRATE_NAME := seed_vault_wasm
MODULE_RUST_CRATE_TYPES := rlib
MODULE_SRCS := $(LOCAL_DIR)/src/lib.rs
MODULE_ADD_IMPLICIT_DEPS := false
MODULE_RUST_EDITION := 2021

# The TA side (the default Cargo feature `vault`).
MODULE_RUSTFLAGS += --cfg 'feature="vault"'

MODULE_LIBRARY_DEPS := \
	trusty/user/base/lib/algokit_crypto \
	trusty/user/base/lib/seed_vault \
	trusty/user/base/lib/wasm_runtime \
	external/rust/android-crates-io/crates/zeroize \
	trusty/user/base/lib/liballoc-rust \
	trusty/user/base/lib/libcompiler_builtins-rust \
	trusty/user/base/lib/libcore-rust

include make/library.mk
