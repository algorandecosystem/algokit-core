#!/usr/bin/env bash
# Rebuilds the prebuilt example guest used by seed_vault_wasm's tests.
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")"
# Two variants: isolated (its own derived keys) and `native` (seed access,
# reproduces the vault's native accounts).
out=target/wasm32-unknown-unknown/release/seed_vault_wasm_guest_ed25519.wasm
cargo build --release --target wasm32-unknown-unknown
cp "$out" ../../tests/fixtures/guest_ed25519.wasm
cargo build --release --target wasm32-unknown-unknown --features native
cp "$out" ../../tests/fixtures/guest_ed25519_native.wasm
ls -l ../../tests/fixtures/guest_ed25519*.wasm
