# Hand-written Trusty build module for `wasmi` 0.46.0
# (https://crates.io/crates/wasmi), a dependency of the optional WASM
# runtime (`crates/wasm_runtime`). Sources are fetched, pinned by sha256, by
# `trusty/fetch_vendor_crates.sh`. Features match what Cargo resolves for
# `cargo test -p wasm_runtime` (`cargo tree -e features`), so the Trusty build
# compiles the same code the host tests run.
#
# The WebAssembly interpreter behind `crates/wasm_runtime`. 0.46 is the
# last release whose MSRV (1.83) Trusty's prebuilt rustc meets (0.47+
# need 1.86, 2.x also edition 2024). Default features (`std`, `wat`) off.
# `arrayvec` comes from AOSP.

LOCAL_DIR := $(GET_LOCAL_DIR)
MODULE := $(LOCAL_DIR)
MODULE_CRATE_NAME := wasmi
MODULE_RUST_CRATE_TYPES := rlib
MODULE_SRCS := $(LOCAL_DIR)/src/lib.rs
MODULE_ADD_IMPLICIT_DEPS := false
MODULE_RUST_EDITION := 2021

MODULE_RUSTFLAGS += \
	--cfg 'feature="prefer-btree-collections"' \

MODULE_LIBRARY_DEPS := \
	external/rust/android-crates-io/crates/arrayvec \
	trusty/user/base/lib/multi_stash \
	trusty/user/base/lib/smallvec \
	trusty/user/base/lib/spin \
	trusty/user/base/lib/wasmi_collections \
	trusty/user/base/lib/wasmi_core \
	trusty/user/base/lib/wasmi_ir \
	trusty/user/base/lib/wasmparser \
	trusty/user/base/lib/liballoc-rust \
	trusty/user/base/lib/libcompiler_builtins-rust \
	trusty/user/base/lib/libcore-rust

include make/library.mk
