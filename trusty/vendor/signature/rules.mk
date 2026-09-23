# Hand-written Trusty build module for `signature` 2.2.0
# (https://crates.io/crates/signature), confirmed *not* present in AOSP's
# `external/rust/android-crates-io`. `src/` is a verbatim copy of the
# upstream crate (minus one doc-comment line, see `src/lib.rs`), with zero
# cargo features enabled: `algokit_crypto` only uses the always-available
# core traits (`Keypair`, `Signer` -- see
# `crates/algokit_crypto/src/ed25519.rs` and `.../src/lib.rs`'s
# `pub use signature::{Keypair, Signer};`), none of which need this crate's
# optional `alloc`/`std`/`digest`/`rand_core`/`derive` features (those are
# all individually `#[cfg(feature = "...")]`-gated in the source, so leaving
# them off costs nothing and pulls in zero extra dependencies -- this crate
# has none of its own to begin with when built this way).

LOCAL_DIR := $(GET_LOCAL_DIR)
MODULE := $(LOCAL_DIR)
MODULE_CRATE_NAME := signature
MODULE_RUST_CRATE_TYPES := rlib
MODULE_SRCS := $(LOCAL_DIR)/src/lib.rs
MODULE_ADD_IMPLICIT_DEPS := false
MODULE_RUST_EDITION := 2021

MODULE_LIBRARY_DEPS := \
	trusty/user/base/lib/libcompiler_builtins-rust \
	trusty/user/base/lib/libcore-rust

include make/library.mk
