# Hand-written Trusty build module for `libm` 0.2.16
# (https://crates.io/crates/libm), a dependency of the optional WASM
# runtime (`crates/wasm_runtime`). Sources are fetched, pinned by sha256, by
# `trusty/fetch_vendor_crates.sh`. Features match what Cargo resolves for
# `cargo test -p wasm_runtime` (`cargo tree -e features`), so the Trusty build
# compiles the same code the host tests run.
#
# Not in AOSP for Trusty. Upstream's `build.rs` only sets optional cfgs
# (`arch_enabled`, `intrinsics_enabled`, `f16_enabled`, ...) for faster or
# extra paths; with none set, as here (wasmi turns off default features),
# libm uses its portable software implementations.

LOCAL_DIR := $(GET_LOCAL_DIR)
MODULE := $(LOCAL_DIR)
MODULE_CRATE_NAME := libm
MODULE_RUST_CRATE_TYPES := rlib
MODULE_SRCS := $(LOCAL_DIR)/src/lib.rs
MODULE_ADD_IMPLICIT_DEPS := false
MODULE_RUST_EDITION := 2021

MODULE_LIBRARY_DEPS := \
	trusty/user/base/lib/liballoc-rust \
	trusty/user/base/lib/libcompiler_builtins-rust \
	trusty/user/base/lib/libcore-rust

include make/library.mk
