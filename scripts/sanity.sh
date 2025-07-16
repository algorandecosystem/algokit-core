#!/bin/bash

set -ex

cargo fmt --check

# Run clippy and treat warnings as errors
cargo clippy -- -D warnings

for dir in crates/*/; do
  # Run cargo check
  cargo check --manifest-path "${dir}Cargo.toml"
  cargo check --manifest-path "${dir}Cargo.toml" --target wasm32-unknown-unknown
done

cargo test
