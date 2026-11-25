# Hand-vendored `algorand-falcon` 0.1.0 (https://crates.io/crates/algorand-falcon,
# MIT, https://github.com/joe-p/algorand-falcon-rs): Rust bindings to
# Algorand's deterministic Falcon-1024 C implementation -- the exact C code
# go-algorand links via cgo. Used by `algokit_crypto::falcon1024` (feature
# `falcon`) for native post-quantum accounts in `seed_vault_ta`.
#
# Why hand-vendored: Trusty doesn't run Cargo `build.rs` scripts, and this
# crate's `build.rs` is what compiles the C sources via `cc`. This directory
# contains exactly the files upstream's `build.rs` compiles (`SOURCES` list,
# `deterministic512.c` intentionally omitted -- only det1024 is bound) plus
# the Rust sources. The crate is already `#![no_std]` + `alloc`.
#
# Local patches vs upstream 0.1.0:
#   - `src/ffi.rs`: `unsafe extern "C"` -> `extern "C"` (edition 2024 syntax
#     Trusty's older rustc rejects). See the comment at that site.
#
# Two modules are declared: this file builds the Rust rlib; `c/rules.mk`
# builds the C sources as a static library it links against.
#
# NOTE: unverified against a live Trusty tree (same caveat as the other
# hand-written `rules.mk` files in this repo). If `make/library.mk` in your
# synced tree accepts mixed C + Rust `MODULE_SRCS`, the two modules can be
# merged.

LOCAL_DIR := $(GET_LOCAL_DIR)
MODULE := $(LOCAL_DIR)
MODULE_CRATE_NAME := algorand_falcon
MODULE_RUST_CRATE_TYPES := rlib
MODULE_SRCS := $(LOCAL_DIR)/src/lib.rs
MODULE_ADD_IMPLICIT_DEPS := false
MODULE_RUST_EDITION := 2021

MODULE_LIBRARY_DEPS := \
	$(LOCAL_DIR)/c \
	external/rust/android-crates-io/crates/zeroize \
	trusty/user/base/lib/liballoc-rust \
	trusty/user/base/lib/libcompiler_builtins-rust \
	trusty/user/base/lib/libcore-rust

include make/library.mk
