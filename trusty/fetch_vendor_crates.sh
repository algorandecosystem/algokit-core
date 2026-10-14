#!/usr/bin/env bash
# Populates trusty/vendor/<crate>/ with upstream crate sources from crates.io.
#
# Trusty builds Rust with make + rustc (no Cargo), so every crate not already
# in AOSP's external/rust/android-crates-io must sit in the source tree next
# to a hand-written rules.mk. Only those rules.mk files (plus `getrandom`,
# which is heavily trimmed/patched) are checked in; the upstream sources are
# fetched here instead, pinned by version + sha256 (the `cksum` crates.io
# publishes in its index), with the few local patches applied below.
#
# Usage: trusty/fetch_vendor_crates.sh [vendor_dir]   (default: trusty/vendor)
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
vendor_dir="${1:-${script_dir}/vendor}"

# name version sha256 local-dir
CRATES=(
  "base32          0.5.1 022dfe9eb35f19ebbcb51e0b40a5ab759f46ad60cadf7297e0bd085afb50e076 base32"
  "cryptoxide      0.4.4 382ce8820a5bb815055d3553a610e8cb542b2d767bbacea99038afda96cd760d cryptoxide"
  "signature       2.2.0 77549399552de45a898a580c1b41d445bf730df867cc44e6c0233bbc4b8329de signature"
  "algorand-falcon 0.1.0 aef694c31f7f8e5805adaf125599ab4cec16765c360dc8e3703e1887528aaeea algorand_falcon"
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

  mkdir -p "${vendor_dir}/${dest}"
  for f in $(files_for "$name"); do
    rm -rf "${vendor_dir}/${dest}/${f}"
    mkdir -p "$(dirname "${vendor_dir}/${dest}/${f}")"
    cp -R "${src}/${f}" "${vendor_dir}/${dest}/${f}"
  done
done
