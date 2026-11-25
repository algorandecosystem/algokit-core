#!/usr/bin/env bash
# Rebuilds the prebuilt Falcon-1024 guest used by seed_vault_wasm's tests.
#
# Falcon is C, so this needs a clang with the WebAssembly backend (Apple's
# clang doesn't have one; Homebrew's `llvm` does). `c_stubs/` stands in for
# the few libc headers Falcon includes, since wasm32-unknown-unknown has no
# libc.
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")"

llvm="${LLVM_BIN:-$(brew --prefix llvm 2>/dev/null)/bin}"
export CC_wasm32_unknown_unknown="${llvm}/clang"
export AR_wasm32_unknown_unknown="${llvm}/llvm-ar"
export CFLAGS_wasm32_unknown_unknown="-isystem $(pwd)/c_stubs"
# Falcon signing needs ~78 KB of C stack; 256 KB instead of Rust's default
# 1 MB keeps the module's memory well under the vault's 2 MiB per-call cap.
export RUSTFLAGS="-C link-arg=-zstack-size=262144"

# Two variants: isolated (its own derived keys) and `native` (seed access,
# reproduces the vault's native accounts).
out=target/wasm32-unknown-unknown/release/seed_vault_wasm_guest_falcon1024.wasm
cargo build --release --target wasm32-unknown-unknown
cp "$out" ../../tests/fixtures/guest_falcon1024.wasm
cargo build --release --target wasm32-unknown-unknown --features native
cp "$out" ../../tests/fixtures/guest_falcon1024_native.wasm
ls -l ../../tests/fixtures/guest_falcon1024*.wasm
