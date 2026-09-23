# Hand-written Trusty build module for `cryptoxide` 0.4.4
# (https://crates.io/crates/cryptoxide), confirmed *not* present in AOSP's
# `external/rust/android-crates-io` (unlike `zeroize`/`async-trait`/etc.,
# which are and get referenced directly from there in seed_vault_ta's
# rules.mk). `src/` here is a verbatim copy of the upstream crate: unlike
# `getrandom` (whose non-`custom` backends are unreachable dead code that
# also can't be vendored/verified without real per-OS headers), every other
# `cryptoxide` module is pure Rust with zero dependencies of its own, so
# leaving them present costs nothing at build time -- they're simply never
# `pub mod`-included (see `src/lib.rs`'s `#[cfg(feature = "...")]` gates)
# because only the four feature cfgs below are set.
#
# Those four are the *minimal* set actually reachable from
# `algokit_crypto`'s usage (`crate::hash::sha512_256` and
# `crate::ed25519::CryptoxideEd25519Keypair`, see
# `crates/algokit_crypto/src/hash.rs` and `.../src/ed25519.rs`):
#   - "digest"      -- the `Digest` trait (`cryptoxide::digest::Digest`).
#   - "sha2"        -- `cryptoxide::sha2::Sha512Trunc256`; pulled in
#                      unconditionally anyway since "ed25519" implies it.
#   - "curve25519"  -- pulled in unconditionally by "ed25519".
#   - "ed25519"     -- `cryptoxide::ed25519::{keypair, signature, verify}`.
# Verified against a real minimal-feature build (`cargo build` against just
# these 4 features, no `default-features`) before vendoring, not guessed.

LOCAL_DIR := $(GET_LOCAL_DIR)
MODULE := $(LOCAL_DIR)
MODULE_CRATE_NAME := cryptoxide
MODULE_RUST_CRATE_TYPES := rlib
MODULE_SRCS := $(LOCAL_DIR)/src/lib.rs
MODULE_ADD_IMPLICIT_DEPS := false
MODULE_RUST_EDITION := 2018

MODULE_RUSTFLAGS += \
	--cfg 'feature="digest"' \
	--cfg 'feature="sha2"' \
	--cfg 'feature="curve25519"' \
	--cfg 'feature="ed25519"' \

MODULE_LIBRARY_DEPS := \
	trusty/user/base/lib/liballoc-rust \
	trusty/user/base/lib/libcompiler_builtins-rust \
	trusty/user/base/lib/libcore-rust

include make/library.mk
