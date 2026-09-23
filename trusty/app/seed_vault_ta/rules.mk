# Trusty Trusted Application: seed_vault_ta
#
# Exposes a `CREATE_ACCOUNT`/`LIST_ACCOUNTS`/`REVEAL_MNEMONIC` IPC service
# (port "algorandecosystem.seed_vault_ta") for creating, enumerating, and
# (deliberately, explicitly) backing up Algorand accounts. Private key
# material never leaves this TA except via the explicit REVEAL_MNEMONIC
# opcode -- see the doc comment at the top of main.rs and
# `crates/seed_vault::store::AccountStore`/`reveal_mnemonic` for the
# full design.
#
# Trusty does not build with plain `cargo build` -- every crate needs its own
# hand-written or generated `rules.mk`. This app will fail to build in a real
# Trusty tree with an unresolved-crate error at the `use seed_vault::...`
# line in main.rs until that's done -- that is expected and is purely a
# build-plumbing TODO, not a logic problem: all the logic `seed_vault`/
# `algokit_crypto` provide is already proven independently via
# `cargo test -p seed_vault` and `cargo run -p algokit_crypto --example
# algo25_roundtrip` (both enforced in CI).
#
# Current, *verified*, state -- every dependency this TA needs is now wired
# below, either straight from AOSP or hand-vendored in this repo (confirmed
# by fetching the real https://android.googlesource.com/platform/external/
# rust/android-crates-io tree, not guessed):
#
# - `seed_vault`/`algokit_crypto` no longer need `bip39`, `snafu`, or the
#   `ed25519-bip32` git fork at all for this TA: `algokit_crypto`'s `xhd`
#   (BIP32/HD-wallet) module -- the only thing that pulled those in, plus
#   their own transitive tree (`bitcoin_hashes`, `unicode-normalization`,
#   `snafu-derive`, ...) -- is now gated behind a Cargo feature that's off
#   by default features (`algokit_crypto/Cargo.toml`'s `xhd` feature), since
#   `seed_vault` never calls into it.
#  - `algokit_crypto`'s SHA-512/256 usage (`algo25`/`address`) was moved off
#   the separate `sha2` crate onto `cryptoxide`'s own `Sha512Trunc256` (see
#   `crates/algokit_crypto/src/hash.rs`) -- `cryptoxide` is a mandatory
#   dependency here regardless (for ed25519), so this drops `sha2` and its
#   whole `digest`/`block-buffer`/`generic-array`/`typenum`/`crypto-common`/
#   `cpufeatures` subtree for free.
# - Already vendored upstream (verified present in android-crates-io as of
#   this writing), wired in below directly -- no generation step needed:
#   `zeroize`, `zeroize_derive`, `async-trait`, `proc-macro2`, `quote`,
#   `syn`, `unicode-ident`, `cfg-if`, `libc`.
# - Hand-vendored in this repo (confirmed *not* present upstream; all three
#   are plain, zero-dependency crates.io crates with no `build.rs`, so this
#   was mechanical): `cryptoxide` (`trusty/vendor/cryptoxide`), `signature`
#   (`trusty/vendor/signature`), `base32` (`trusty/vendor/base32`). See each
#   one's `rules.mk` for exactly which cargo features are enabled and why.
# - `algokit_crypto`/`seed_vault` themselves are first-party in-workspace
#   crates, not crates.io deps -- `cargo2rulesmk.py` doesn't apply to them
#   at all. Each has its own hand-written `rules.mk` living right next to
#   its `Cargo.toml` (`crates/algokit_crypto/rules.mk`,
#   `crates/seed_vault/rules.mk`), copied into the synced tree by CI at
#   `trusty/user/base/lib/algokit_crypto` / `.../seed_vault` -- see
#   trusty_ci.yml.
#
# RESOLVED, separately, and more security-sensitive than plain vendoring:
# `getrandom` 0.4 (used by `seed_vault::store::create_algo25_account` and
# `algokit_crypto::ed25519::CryptoxideEd25519Keypair::try_generate` to source
# entropy for brand-new accounts) has no Trusty backend upstream at all --
# its `src/backends.rs` target-selection `cfg_if!` has no `trusty` arm.
# Fixed by hand-vendoring a trimmed copy of `getrandom` at
# `trusty/vendor/getrandom` in *this* repo (CI copies it into the synced
# tree as `trusty/user/base/lib/getrandom`, see trusty_ci.yml -- that's the
# path referenced below), built with `--cfg getrandom_backend="custom"`,
# plus a real (not stubbed) implementation of its custom-backend hook
# (`__getrandom_v03_custom`, in `main.rs`'s `trusty_rng` module) that calls
# Trusty's own `trusty_rng_secure_rand` (`trusty/user/base/lib/rng` -- a
# BoringSSL CSPRNG reseeded from the platform HWRNG). See
# `trusty/vendor/getrandom/rules.mk` and `trusty_rng.rs`'s doc comments for
# the full picture.

LOCAL_DIR := $(GET_LOCAL_DIR)
MODULE := $(LOCAL_DIR)

MANIFEST := $(LOCAL_DIR)/manifest.json

MODULE_CRATE_NAME := seed_vault_ta

MODULE_SRCS := $(LOCAL_DIR)/main.rs

# NOT 2024 (unlike every other crate in this workspace's `Cargo.toml`s):
# Trusty's own bundled rustc (baked into the synced AOSP tree, entirely
# independent of this workspace's `rust-toolchain`) is old enough to reject
# it outright -- "edition 2024 is unstable and only available with
# -Z unstable-options", i.e. it predates edition 2024's stabilization
# (rustc 1.85). `trusty_rng.rs` originally used a couple of edition-2024
# syntax forms (`unsafe extern "C" { .. }` blocks, `#[unsafe(no_mangle)]`);
# both were hand-reverted to their always-valid plain equivalents
# (`extern "C" { .. }`, `#[no_mangle]` -- semantically identical, see the
# comments at each site in `trusty_rng.rs`), so 2021 compiles identically
# with no behavior change and works on much older compilers. See
# `trusty/vendor/getrandom/rules.mk` for the same story on that crate.
MODULE_RUST_EDITION := 2021

# `main.rs`/`trusty_store.rs` only directly `use seed_vault::...` (never
# `algokit_crypto`/`cryptoxide`/etc. directly) -- `seed_vault`'s own
# `rules.mk` declares those as *its* `MODULE_LIBRARY_DEPS`, and the build
# pulls in the rest of the DAG transitively from there, the same way
# nothing that depends on `tipc/rust` has to separately list `zerocopy`
# just because `tipc`'s own `rules.mk` depends on it.
MODULE_LIBRARY_DEPS := \
	trusty/user/base/lib/trusty-std \
	trusty/user/base/lib/tipc/rust \
	trusty/user/base/lib/storage/rust \
	trusty/user/base/lib/rng \
	trusty/user/base/lib/getrandom \
	trusty/user/base/lib/seed_vault \
	external/rust/android-crates-io/crates/zeroize

include make/trusted_app.mk
