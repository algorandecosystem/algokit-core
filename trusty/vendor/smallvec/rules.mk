# Hand-written Trusty build module for `smallvec` 1.13.2
# (https://crates.io/crates/smallvec), a dependency of the optional WASM
# runtime (`crates/wasm_runtime`). Sources are fetched, pinned by sha256, by
# `trusty/fetch_vendor_crates.sh`. Features match what Cargo resolves for
# `cargo test -p wasm_runtime` (`cargo tree -e features`), so the Trusty build
# compiles the same code the host tests run.
#
# `union` is the one feature wasmi enables (it needs it for `SmallVec`'s
# compact layout); no `build.rs`.

LOCAL_DIR := $(GET_LOCAL_DIR)
MODULE := $(LOCAL_DIR)
MODULE_CRATE_NAME := smallvec
MODULE_RUST_CRATE_TYPES := rlib
MODULE_SRCS := $(LOCAL_DIR)/src/lib.rs
MODULE_ADD_IMPLICIT_DEPS := false
MODULE_RUST_EDITION := 2018

MODULE_RUSTFLAGS += \
	--cfg 'feature="union"' \

MODULE_LIBRARY_DEPS := \
	trusty/user/base/lib/liballoc-rust \
	trusty/user/base/lib/libcompiler_builtins-rust \
	trusty/user/base/lib/libcore-rust

include make/library.mk
