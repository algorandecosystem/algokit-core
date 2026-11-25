# Hand-written Trusty build module for `downcast-rs` 2.0.2
# (https://crates.io/crates/downcast-rs), a dependency of the optional WASM
# runtime (`crates/wasm_runtime`). Sources are fetched, pinned by sha256, by
# `trusty/fetch_vendor_crates.sh`. Features match what Cargo resolves for
# `cargo test -p wasm_runtime` (`cargo tree -e features`), so the Trusty build
# compiles the same code the host tests run.
#
# AOSP does ship `downcast-rs`, but builds it with `std`; wasmi only needs
# `sync`. Not linked anywhere else in seed_vault_ta, so no clash.

LOCAL_DIR := $(GET_LOCAL_DIR)
MODULE := $(LOCAL_DIR)
MODULE_CRATE_NAME := downcast_rs
MODULE_RUST_CRATE_TYPES := rlib
MODULE_SRCS := $(LOCAL_DIR)/src/lib.rs
MODULE_ADD_IMPLICIT_DEPS := false
MODULE_RUST_EDITION := 2021

MODULE_RUSTFLAGS += \
	--cfg 'feature="sync"' \

MODULE_LIBRARY_DEPS := \
	trusty/user/base/lib/liballoc-rust \
	trusty/user/base/lib/libcompiler_builtins-rust \
	trusty/user/base/lib/libcore-rust

include make/library.mk
