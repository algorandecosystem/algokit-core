# Trusty Trusted Application: seed_vault_ta
#
# Exposes a `CREATE_ACCOUNT`/`LIST_ACCOUNTS`/`REVEAL_MNEMONIC` IPC service
# (port "algorandfoundation.seed_vault_ta") for creating, enumerating, and
# (deliberately, explicitly) backing up Algorand accounts. Private key
# material never leaves this TA except via the explicit REVEAL_MNEMONIC
# opcode -- see the doc comment at the top of main.rs and
# `crates/seed_vault::store::AccountStore`/`reveal_mnemonic` for the
# full design.
#
# Trusty does not build with plain `cargo build` -- it generates rules.mk
# fragments for third-party crates via `cargo2rulesmk.py` run against the
# synced Trusty tree (see the "Vendor crates for Trusty" CI step in
# .github/workflows/trusty_ci.yml). That step produces modules at
# `trusty/user/base/lib/algokit_crypto/rules.mk` and
# `trusty/user/base/lib/seed_vault/rules.mk`, added as
# MODULE_LIBRARY_DEPS entries below once generated and reviewed. Until then,
# this app will fail to build in a real Trusty tree with an unresolved-crate
# error at the `use seed_vault::...` line in main.rs -- that is
# expected and is purely a build-plumbing TODO, not a logic problem: all the
# logic those two crates provide is already proven independently via
# `cargo test -p seed_vault` and `cargo run -p algokit_crypto --example
# algo25_roundtrip` (both enforced in CI).

LOCAL_DIR := $(GET_LOCAL_DIR)
MODULE := $(LOCAL_DIR)

MANIFEST := $(LOCAL_DIR)/manifest.json

MODULE_SRCS := \
	$(LOCAL_DIR)/main.rs \
	$(LOCAL_DIR)/trusty_store.rs \

MODULE_LIBRARY_DEPS := \
	trusty/user/base/lib/trusty-std \
	trusty/user/base/lib/tipc/rust \
	trusty/user/base/lib/storage/rust \

# Uncomment once `cargo2rulesmk.py` has generated Trusty build modules for
# these two crates (see the "Vendor crates for Trusty" workflow step):
#
# MODULE_LIBRARY_DEPS += trusty/user/base/lib/algokit_crypto
# MODULE_LIBRARY_DEPS += trusty/user/base/lib/seed_vault

include make/trusted_app.mk
