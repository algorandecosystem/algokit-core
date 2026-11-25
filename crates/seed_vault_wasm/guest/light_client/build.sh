#!/usr/bin/env bash
# Rebuilds the prebuilt light client guest used by seed_vault_wasm's tests.
#
# Falcon (used to verify state proof signatures) is C, so this needs a clang
# with the WebAssembly backend (Apple's clang doesn't have one; Homebrew's
# `llvm` does). It reuses `../falcon1024/c_stubs/` for the few libc headers
# Falcon includes, since wasm32-unknown-unknown has no libc.
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")"

llvm="${LLVM_BIN:-$(brew --prefix llvm 2>/dev/null)/bin}"
export CC_wasm32_unknown_unknown="${llvm}/clang"
export AR_wasm32_unknown_unknown="${llvm}/llvm-ar"
export CFLAGS_wasm32_unknown_unknown="-isystem $(pwd)/../falcon1024/c_stubs"
# 256 KB of stack instead of Rust's default 1 MB keeps the module's memory
# well under the vault's per-call cap.
export RUSTFLAGS="-C link-arg=-zstack-size=262144"

out=target/wasm32-unknown-unknown/release/seed_vault_wasm_guest_light_client.wasm
cargo build --release --lib --target wasm32-unknown-unknown
cp "$out" ../../tests/fixtures/guest_light_client.wasm
ls -l ../../tests/fixtures/guest_light_client.wasm
