"""
Enhanced Jinja2 filters for Rust code generation with msgpack support
"""

from typing import Any

MIN_SEMVER_PARTS = 2
MAX_SEMVER_PARTS = 3


def rust_doc_comment(text: str, indent: int = 0) -> str:
    """Convert text to Rust doc comment format."""
    if not text:
        return ""

    lines = text.strip().split("\n")
    indent_str = " " * indent

    if len(lines) == 1:
        return f"{indent_str}/// {lines[0]}"

    result = []
    for line in lines:
        stripped_line = line.strip()
        if stripped_line.startswith(("* ", "- ", "+ ", "Or ", "And ", "But ")):
            result.append(f"{indent_str}///   {stripped_line}")
        else:
            result.append(f"{indent_str}/// {stripped_line}")

    return "\n".join(result)


def detect_signed_transaction_field(vendor_extensions: dict[str, Any]) -> bool:
    """Detect if this schema represents a SignedTransaction."""
    return bool(vendor_extensions.get("x-algokit-signed-txn", False))


def needs_msgpack_trait(schema: dict[str, Any]) -> bool:
    """Determine if schema needs AlgorandMsgpack trait implementation."""
    return any("msgpack" in key.lower() for key in schema.get("vendor_extensions", {}))


def get_dependencies_for_schema(schema: dict[str, Any]) -> list[str]:
    """Get list of dependencies needed for this schema."""
    deps = ["use serde::{Deserialize, Serialize};"]

    if schema.get("has_msgpack_fields", False):
        deps.append("use serde_with::serde_as;")

    vendor_exts = schema.get("vendor_extensions", {})
    if vendor_exts.get("x-algokit-signed-txn"):
        deps.extend(
            [
                "use algokit_transact::SignedTransaction as AlgokitSignedTransaction;",
                "use algokit_transact::AlgorandMsgpack;",
            ],
        )

    return deps


def ensure_semver(version_str: str) -> str:
    """Ensure version string is valid semantic versioning format."""
    if not version_str:
        return "0.1.0"

    parts = version_str.split(".")
    if len(parts) == 1:
        return f"{parts[0]}.0.0"
    if len(parts) == MIN_SEMVER_PARTS:
        return f"{parts[0]}.{parts[1]}.0"

    return version_str


def semver_string(version: str) -> str:
    """Format version string for Cargo.toml semver compatibility."""
    if not version:
        return "0.1.0"

    # Remove 'v' prefix if present
    version = version.lstrip("v")

    # Split by dots and ensure we have at least 2 parts
    parts = version.split(".")
    if len(parts) < MIN_SEMVER_PARTS:
        parts.extend(["0"] * (MIN_SEMVER_PARTS - len(parts)))
    elif len(parts) > MAX_SEMVER_PARTS:
        parts = parts[:MAX_SEMVER_PARTS]

    # Ensure all parts are numeric
    numeric_parts = []
    for part in parts:
        try:
            numeric_parts.append(str(int(part)))
        except ValueError:
            numeric_parts.append("0")

    return ".".join(numeric_parts)


# Register filters that will be available in Jinja templates
FILTERS = {
    "rust_doc_comment": rust_doc_comment,
    "detect_signed_transaction_field": detect_signed_transaction_field,
    "needs_msgpack_trait": needs_msgpack_trait,
    "get_dependencies_for_schema": get_dependencies_for_schema,
    "ensure_semver": ensure_semver,
    "semver_string": semver_string,
}
