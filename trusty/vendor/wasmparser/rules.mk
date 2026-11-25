# Hand-written Trusty build module for `wasmparser` 0.228.0
# (https://crates.io/crates/wasmparser), a dependency of the optional WASM
# runtime (`crates/wasm_runtime`). Sources are fetched, pinned by sha256, by
# `trusty/fetch_vendor_crates.sh`. Features match what Cargo resolves for
# `cargo test -p wasm_runtime` (`cargo tree -e features`), so the Trusty build
# compiles the same code the host tests run.
#
# Upstream's `build.rs` only sets `--cfg core_error` when rustc >= 1.81 (so
# `core::error::Error` is used without `std`); Trusty's rustc is 1.83, so
# it's set unconditionally below. `bitflags` comes from AOSP.

LOCAL_DIR := $(GET_LOCAL_DIR)
MODULE := $(LOCAL_DIR)
MODULE_CRATE_NAME := wasmparser
MODULE_RUST_CRATE_TYPES := rlib
MODULE_SRCS := $(LOCAL_DIR)/src/lib.rs
MODULE_ADD_IMPLICIT_DEPS := false
MODULE_RUST_EDITION := 2021

MODULE_RUSTFLAGS += \
	--cfg 'feature="features"' \
	--cfg 'feature="prefer-btree-collections"' \
	--cfg 'feature="validate"' \
	--cfg core_error \

MODULE_LIBRARY_DEPS := \
	external/rust/android-crates-io/crates/bitflags \
	trusty/user/base/lib/liballoc-rust \
	trusty/user/base/lib/libcompiler_builtins-rust \
	trusty/user/base/lib/libcore-rust

include make/library.mk
