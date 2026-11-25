#!/usr/bin/env bash
# Builds the rustdoc of the vendored crates the way the Trusty build does (`-D warnings`, no default
# features, the features their rules.mk enables), so a broken intra-doc link in an upstream crate fails
# here in minutes instead of at the end of the Trusty build. Fetches the pinned crate sources from
# crates.io with the same local patches as trusty/fetch_vendor_crates.sh, so a patch that no longer applies
# fails here too.
#
# Usage: trusty/check_vendor_docs.sh [local-dir ...]
# Default: the crates that came in with the optional WASM runtime. The older ones built fine in Trusty.
# Every crate is tried, and the ones that fail are listed at the end.
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

crates=("$@")
if [[ ${#crates[@]} -eq 0 ]]; then
  crates=(wasmi wasmi_core wasmi_ir wasmi_collections wasmparser multi_stash smallvec spin libm downcast_rs)
fi

tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT
FULL_SOURCES_DIR="${tmp}/full" "${script_dir}/fetch_vendor_crates.sh" "${tmp}/vendor"

failed=()
for crate in "${crates[@]}"; do
  # `feature="x"` pairs from the module's rules.mk, as the Trusty build enables them
  features="$(grep -o 'feature="[^"]*"' "${script_dir}/vendor/${crate}/rules.mk" | sed 's/feature="\(.*\)"/\1/' | sort -u | paste -sd, - || true)"
  echo "::group::rustdoc ${crate} (features: ${features:-none})"
  if (cd "${tmp}/full/${crate}" &&
    RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --no-default-features ${features:+--features "$features"}); then
    echo "ok: ${crate}"
  else
    failed+=("$crate")
  fi
  echo "::endgroup::"
done

if [[ ${#failed[@]} -gt 0 ]]; then
  echo "::error::rustdoc with -D warnings failed for: ${failed[*]}" >&2
  exit 1
fi
