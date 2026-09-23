# Hand-written Trusty build module for `base32` 0.5.1
# (https://crates.io/crates/base32), confirmed *not* present in AOSP's
# `external/rust/android-crates-io`. `src/lib.rs` is a verbatim, single-file
# copy of the upstream crate: it has zero dependencies of its own, no
# `build.rs`, no cargo features to select, and is `no_std` + `alloc` by
# design -- about as simple a vendoring job as they come.
#
# Used by `algokit_crypto::address` (`crates/algokit_crypto/src/address.rs`)
# for RFC4648 base32-encoding Algorand addresses (checksum + public key).

LOCAL_DIR := $(GET_LOCAL_DIR)
MODULE := $(LOCAL_DIR)
MODULE_CRATE_NAME := base32
MODULE_RUST_CRATE_TYPES := rlib
MODULE_SRCS := $(LOCAL_DIR)/src/lib.rs
MODULE_ADD_IMPLICIT_DEPS := false
MODULE_RUST_EDITION := 2015

MODULE_LIBRARY_DEPS := \
	trusty/user/base/lib/liballoc-rust \
	trusty/user/base/lib/libcompiler_builtins-rust \
	trusty/user/base/lib/libcore-rust

include make/library.mk
