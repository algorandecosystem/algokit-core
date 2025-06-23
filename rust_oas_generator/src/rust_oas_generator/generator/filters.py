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
        result.append(f"{indent_str}/// {line.strip()}")
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


# Register filters that will be available in Jinja templates
FILTERS = {
    "rust_doc_comment": rust_doc_comment,
    "detect_signed_transaction_field": detect_signed_transaction_field,
    "needs_msgpack_trait": needs_msgpack_trait,
    "get_dependencies_for_schema": get_dependencies_for_schema,
}
