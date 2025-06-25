"""
Enhanced Jinja2 filters for Rust code generation with msgpack support
"""

from typing import Any, Dict, List


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
        # Handle list items by adding proper indentation
        if stripped_line.startswith(("* ", "- ", "+ ")):
            result.append(f"{indent_str}///   {stripped_line}")
        elif stripped_line.startswith(("Or ", "And ", "But ")):
            # These might be continuation of list items
            result.append(f"{indent_str}///   {stripped_line}")
        else:
            result.append(f"{indent_str}/// {stripped_line}")
    return "\n".join(result)


def detect_signed_transaction_field(vendor_extensions: Dict[str, Any]) -> bool:
    """Detect if this schema represents a SignedTransaction."""
    return vendor_extensions.get("x-algokit-signed-txn", False)


def needs_msgpack_trait(schema: Dict[str, Any]) -> bool:
    """Determine if schema needs AlgorandMsgpack trait implementation."""
    # Check if it has msgpack-related vendor extensions
    for key in schema.get("vendor_extensions", {}):
        if "msgpack" in key.lower():
            return True
    return False


def get_dependencies_for_schema(schema: Dict[str, Any]) -> List[str]:
    """Get list of dependencies needed for this schema."""
    deps = ["use serde::{Deserialize, Serialize};"]

    if schema.get("has_msgpack_fields", False):
        deps.append("use serde_with::serde_as;")

    if schema.get("vendor_extensions", {}).get("x-algokit-signed-txn"):
        deps.append(
            "use algokit_transact::SignedTransaction as AlgokitSignedTransaction;",
        )
        deps.append("use algokit_transact::AlgorandMsgpack;")

    return deps


def ensure_semver(version_str: str) -> str:
    """Ensure version string is valid semantic versioning format."""
    if not version_str:
        return "0.1.0"

    parts = version_str.split(".")
    if len(parts) == 1:
        return f"{parts[0]}.0.0"
    elif len(parts) == 2:
        return f"{parts[0]}.{parts[1]}.0"
    else:
        return version_str


# Register filters that will be available in Jinja templates
FILTERS = {
    "rust_doc_comment": rust_doc_comment,
    "detect_signed_transaction_field": detect_signed_transaction_field,
    "needs_msgpack_trait": needs_msgpack_trait,
    "get_dependencies_for_schema": get_dependencies_for_schema,
    "ensure_semver": ensure_semver,
}
