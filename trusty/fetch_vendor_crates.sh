#!/usr/bin/env bash
# Populates trusty/vendor/<crate>/ with upstream crate sources from crates.io.
#
# Trusty builds Rust with make + rustc (no Cargo), so every crate not already
# in AOSP's external/rust/android-crates-io must sit in the source tree next
# to a hand-written rules.mk. Only those rules.mk files are checked in; the
# upstream sources are fetched here instead, pinned by version + sha256 (the
# `cksum` crates.io publishes in its index), with the few local patches
# applied below.
#
# Usage: [FULL_SOURCES_DIR=<dir>] trusty/fetch_vendor_crates.sh [vendor_dir]   (default: trusty/vendor)
# With FULL_SOURCES_DIR, each whole patched crate is also copied to <dir>/<local-dir>.
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
vendor_dir="${1:-${script_dir}/vendor}"

# name version sha256 local-dir
CRATES=(
  "base32          0.5.1 022dfe9eb35f19ebbcb51e0b40a5ab759f46ad60cadf7297e0bd085afb50e076 base32"
  "cryptoxide      0.4.4 382ce8820a5bb815055d3553a610e8cb542b2d767bbacea99038afda96cd760d cryptoxide"
  "signature       2.2.0 77549399552de45a898a580c1b41d445bf730df867cc44e6c0233bbc4b8329de signature"
  "algorand-falcon 0.1.0 aef694c31f7f8e5805adaf125599ab4cec16765c360dc8e3703e1887528aaeea algorand_falcon"
  "getrandom       0.4.1 139ef39800118c7683f2fd3c98c1b23c09ae076556b435f8e9064ae108aaeeec getrandom"
  # WASM runtime (crates/wasm_runtime, optional: see seed_vault_ta/rules.mk).
  # wasmi is pinned to 0.46, the last release whose MSRV (1.83) Trusty's
  # prebuilt rustc meets.
  "wasmi             0.46.0 c602dd253549139314b217cb18557ac290135ed25a3d806c81d73db6b0effd49 wasmi"
  "wasmi_core        0.46.0 909e4dcf38f60274b2d098802371e232d92c2c612b0b47a0016ec5234a3f425d wasmi_core"
  "wasmi_ir          0.46.0 cab7c61bb8aa9de5b09fb63f95649af9d206f3a912b54508756a827f29eae678 wasmi_ir"
  "wasmi_collections 0.46.0 001f3a73d68df33e164c935fd64a4bf63bcd6cd6a6e88ae0e4328fd5aff1b7a7 wasmi_collections"
  "wasmparser        0.228.0 4abf1132c1fdf747d56bbc1bb52152400c70f336870f968b85e89ea422198ae3 wasmparser"
  "multi-stash       0.2.0 685a9ac4b61f4e728e1d2c6a7844609c16527aeb5e6c865915c08e619c16410f multi_stash"
  "smallvec          1.13.2 3c5e1a9a646d36c3599cd173a41282daf47c44583ad367b8e6837255952e5c67 smallvec"
  "spin              0.9.9 3763264f6b73151db08c50ff20d7d8a0b8796e021cdea7ceedad07b80155fa0e spin"
  "libm              0.2.16 b6d2cec3eae94f9f509c767b45932f1ada8350c4bdb85af2fcab4a3c14807981 libm"
  "downcast-rs       2.0.2 117240f60069e65410b3ae1bb213295bd828f707b5bec6596a1afc8793ce0cbc downcast_rs"
)

# Files copied from each extracted crate (everything else -- Cargo.toml,
# tests, benches, build.rs, ... -- is unused by the Trusty build).
files_for() {
  case "$1" in
    algorand-falcon)
      # Exactly what upstream's build.rs compiles (det512 intentionally
      # omitted), plus headers, the Rust bindings and licenses.
      echo LICENSE src csrc/check_sizes.c falcon/README.txt \
        falcon/{codec,common,deterministic1024,falcon,fft,fpr,keygen,rng,shake,sign,vrfy}.c \
        falcon/{config,deterministic,falcon,fpr,inner}.h
      ;;
    getrandom)
      # Only what `--cfg getrandom_backend="custom"` reaches (see its
      # rules.mk). The per-OS backends, `sys_rng` and `std` impls are left
      # out; the references to them in these files are all behind cfgs this
      # build never sets, so they're inert.
      echo src/{lib,backends,error,util}.rs src/backends/custom.rs
      ;;
    *) echo src ;;
  esac
}

sha256() {
  if command -v sha256sum >/dev/null; then sha256sum "$1" | cut -d' ' -f1
  else shasum -a 256 "$1" | cut -d' ' -f1; fi
}

# Replace exactly one occurrence of $2 with $3 in file $1, failing loudly if
# upstream ever changes so the patch no longer applies.
patch_once() {
  local file="$1" from="$2" to="$3"
  FROM="$from" TO="$to" python3 - "$file" <<'PYEOF'
import os, pathlib, sys
p = pathlib.Path(sys.argv[1]); s = p.read_text()
frm, to = os.environ["FROM"], os.environ["TO"]
if s.count(frm) != 1:
    sys.exit(f"patch failed: expected exactly one {frm!r} in {p}")
p.write_text(s.replace(frm, to))
PYEOF
}

apply_patches() {
  local dir="$1"
  case "$2" in
    signature)
      # Upstream's README isn't vendored, so drop its doc include.
      patch_once "${dir}/src/lib.rs" \
        '#![doc = include_str!("../README.md")]' \
        "// Upstream has \`#![doc = include_str!(\"../README.md\")]\` here; dropped since
// we don't vendor the README (see \`../rules.mk\`'s provenance comment).
// Purely a doc-comment difference, no functional change."
      ;;
    algorand-falcon)
      # Edition-2024 syntax that Trusty's bundled rustc rejects.
      patch_once "${dir}/src/ffi.rs" 'unsafe extern "C" {' \
        "// TRUSTY VENDOR PATCH: upstream writes \`unsafe extern \"C\" {\` (edition-2024
// syntax). Trusty's bundled rustc predates edition 2024, so this vendored
// copy uses the always-valid, semantically identical plain \`extern \"C\"\`
// form -- same story as \`trusty/vendor/getrandom\`.
extern \"C\" {"
      ;;
    getrandom)
      # Upstream's README isn't vendored, so drop its doc include.
      patch_once "${dir}/src/lib.rs" '#![doc = include_str!("../README.md")]' \
        "// Upstream's \`#![doc = include_str!(\"../README.md\")]\` dropped: README not vendored."
      # Edition-2024 syntax that Trusty's bundled rustc rejects -- even
      # inside cfg'd-out code, since parsing happens before cfg stripping.
      # This one is under `#[cfg(getrandom_msan)]`, never set here.
      patch_once "${dir}/src/lib.rs" 'unsafe extern "C" {' 'extern "C" {'
      # The matching definition lives in trusty/app/seed_vault_ta/trusty_rng.rs.
      patch_once "${dir}/src/backends/custom.rs" 'unsafe extern "Rust" {' \
        "// TRUSTY VENDOR PATCH: upstream's \`unsafe extern \"Rust\"\` is edition-2024
    // syntax that Trusty's bundled rustc rejects; plain \`extern\` is identical.
    extern \"Rust\" {"
      ;;
    wasmi_collections)
      # Its docs link `std::collections::BTreeMap`/`BTreeSet`, which rustdoc can't resolve in this no_std
      # build, and the Trusty build runs rustdoc with `-D warnings`. Point the links at the std docs on the
      # web instead. Doc-comment difference only, no functional change.
      local url=https://doc.rust-lang.org/std/collections
      patch_once "${dir}/src/lib.rs" '//! [`BTreeMap`]: std::collections::BTreeMap' \
        "//! [\`BTreeMap\`]: ${url}/struct.BTreeMap.html"
      patch_once "${dir}/src/lib.rs" '//! [`BTreeSet`]: std::collections::BTreeSet' \
        "//! [\`BTreeSet\`]: ${url}/struct.BTreeSet.html"
      patch_once "${dir}/src/map.rs" '/// [`BTreeMap`]: std::collections::BTreeMap' \
        "/// [\`BTreeMap\`]: ${url}/struct.BTreeMap.html"
      patch_once "${dir}/src/set.rs" '/// [`BTreeSet`]: std::collections::BTreeSet' \
        "/// [\`BTreeSet\`]: ${url}/struct.BTreeSet.html"
      ;;
  esac
}

tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

for entry in "${CRATES[@]}"; do
  read -r name version want_sha dest <<<"$entry"
  archive="${tmp}/${name}-${version}.crate"
  echo "Fetching ${name} ${version} -> ${vendor_dir}/${dest}"
  curl -sSfL --retry 3 -o "$archive" \
    "https://static.crates.io/crates/${name}/${name}-${version}.crate"

  got_sha="$(sha256 "$archive")"
  if [[ "$got_sha" != "$want_sha" ]]; then
    echo "::error::sha256 mismatch for ${name} ${version}: got ${got_sha}, want ${want_sha}" >&2
    exit 1
  fi

  src="${tmp}/${name}-${version}"
  tar -xzf "$archive" -C "$tmp"
  apply_patches "$src" "$name"

  # Optionally keep the whole patched crate (Cargo.toml and all) so cargo can build it, e.g. for
  # trusty/check_vendor_docs.sh. The Trusty build only needs what files_for copies below.
  if [[ -n "${FULL_SOURCES_DIR:-}" ]]; then
    rm -rf "${FULL_SOURCES_DIR:?}/${dest}"
    mkdir -p "${FULL_SOURCES_DIR}/${dest}"
    cp -R "${src}/." "${FULL_SOURCES_DIR}/${dest}/"
  fi

  mkdir -p "${vendor_dir}/${dest}"
  for f in $(files_for "$name"); do
    rm -rf "${vendor_dir}/${dest}/${f}"
    mkdir -p "$(dirname "${vendor_dir}/${dest}/${f}")"
    cp -R "${src}/${f}" "${vendor_dir}/${dest}/${f}"
  done
done
