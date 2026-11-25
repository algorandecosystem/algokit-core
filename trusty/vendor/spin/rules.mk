# Hand-written Trusty build module for `spin` 0.9.9
# (https://crates.io/crates/spin), a dependency of the optional WASM
# runtime (`crates/wasm_runtime`). Sources are fetched, pinned by sha256, by
# `trusty/fetch_vendor_crates.sh`. Features match what Cargo resolves for
# `cargo test -p wasm_runtime` (`cargo tree -e features`), so the Trusty build
# compiles the same code the host tests run.
#
# AOSP does ship `spin`, but its Trusty `rules.mk` doesn't enable `rwlock`,
# which wasmi uses. Nothing else in seed_vault_ta links AOSP's `spin`
# (checked trusty-std, tipc, storage), so this copy doesn't clash.
# Features: exactly the ones wasmi enables.

LOCAL_DIR := $(GET_LOCAL_DIR)
MODULE := $(LOCAL_DIR)
MODULE_CRATE_NAME := spin
MODULE_RUST_CRATE_TYPES := rlib
MODULE_SRCS := $(LOCAL_DIR)/src/lib.rs
MODULE_ADD_IMPLICIT_DEPS := false
MODULE_RUST_EDITION := 2015

MODULE_RUSTFLAGS += \
	--cfg 'feature="mutex"' \
	--cfg 'feature="spin_mutex"' \
	--cfg 'feature="rwlock"' \

MODULE_LIBRARY_DEPS := \
	trusty/user/base/lib/liballoc-rust \
	trusty/user/base/lib/libcompiler_builtins-rust \
	trusty/user/base/lib/libcore-rust

include make/library.mk
