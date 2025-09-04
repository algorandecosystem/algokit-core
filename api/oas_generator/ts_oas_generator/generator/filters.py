"""
TypeScript-specific Jinja2 filters and helpers.
Phase 2 adds OpenAPI -> TS type mapping and naming utilities.
"""

from __future__ import annotations

import re
from typing import Any

_IDENTIFIER_RE = re.compile(r"^[A-Za-z_][A-Za-z0-9_]*$")


def ts_doc_comment(text: str | None) -> str:
    """Format text as a TypeScript doc comment."""
    if not text:
        return ""
    lines = str(text).strip().split("\n")
    body = "\n".join(f" * {line.strip()}" if line.strip() else " *" for line in lines)
    return f"/**\n{body}\n */"


def ts_string_literal(text: str) -> str:
    """Escape to a valid TypeScript string literal using backticks."""
    escaped = str(text).replace("`", "\\`").replace("\\", "\\\\")
    return f"`{escaped}`"


def ts_optional(type_str: str) -> str:
    """Return a TS optional type representation."""
    return f"{type_str} | undefined"


def ts_array(type_str: str) -> str:
    """Return a TS array type representation."""
    return f"Array<{type_str}>"


# ---------- Naming helpers ----------


def _split_words(name: str) -> list[str]:
    name = re.sub(r"([a-z0-9])([A-Z])", r"\1 \2", name)
    name = re.sub(r"[^A-Za-z0-9]+", " ", name)
    parts = [p for p in name.strip().split() if p]
    return parts or [name]


def ts_pascal_case(name: str) -> str:
    parts = _split_words(name)
    return "".join(p.capitalize() for p in parts)


def ts_camel_case(name: str) -> str:
    pas = ts_pascal_case(name)
    return pas[:1].lower() + pas[1:] if pas else pas


def ts_property_name(name: str) -> str:
    """Return a safe TS property name, quoting if necessary."""
    return name if _IDENTIFIER_RE.match(name) else f"'{name}'"


# ---------- OpenAPI -> TS type mapping ----------


def _extract_ref_name(ref_string: str) -> str:
    return ref_string.split("/")[-1]


def _union(types: list[str]) -> str:
    uniq: list[str] = []
    for t in types:
        if t not in uniq:
            uniq.append(t)
    return " | ".join(uniq) if uniq else "never"


def _intersection(types: list[str]) -> str:
    parts = [t for t in types if t and t != "any"]
    return " & ".join(parts) if parts else "any"


def _nullable(type_str: str, schema: dict[str, Any]) -> str:
    # OpenAPI 3.0 nullable flag
    if schema.get("nullable") is True:
        return _union([type_str, "null"])

    # OpenAPI 3.1 union type with null
    t = schema.get("type")
    if isinstance(t, list) and "null" in t:
        non_nulls = [x for x in t if x != "null"]
        # If there's exactly one non-null type, union with null
        if len(non_nulls) == 1:
            return _union([ts_type({"type": non_nulls[0]}, None), "null"])
        # Else, build a union of all non-nulls + null
        return _union([_union([ts_type({"type": n}, None) for n in non_nulls]), "null"])

    return type_str


def _inline_object(schema: dict[str, Any], schemas: dict[str, Any] | None) -> str:
    properties: dict[str, Any] = schema.get("properties", {}) or {}
    required = set(schema.get("required", []) or [])
    parts: list[str] = []

    for prop_name, prop_schema in properties.items():
        ts_name = ts_property_name(prop_name)
        ts_t = ts_type(prop_schema, schemas)
        opt = "" if prop_name in required else "?"
        parts.append(f"{ts_name}{opt}: {ts_t};")

    # additionalProperties -> index signature
    if "additionalProperties" in schema:
        addl = schema["additionalProperties"]
        if addl is True:
            parts.append("[key: string]: any;")
        elif isinstance(addl, dict):
            parts.append(f"[key: string]: {ts_type(addl, schemas)};")

    return "{" + (" ".join(parts)) + "}"


def _map_primitive(schema_type: str, _schema_format: str | None, schema: dict[str, Any]) -> str:
    if schema.get("x-algokit-bigint") is True and schema_type == "integer":
        return "bigint"

    if schema_type == "integer":
        return "number"

    if schema_type in ("number",):
        return "number"

    if schema_type == "string":
        # bytes/base64 remain string for JSON representation
        return "string"

    if schema_type == "boolean":
        return "boolean"

    return "any"


def ts_enum_type(schema: dict[str, Any]) -> str | None:
    if "enum" not in schema:
        return None

    if schema.get("x-algokit-bigint") is True:
        # For bigint-marked enums, use bigint type directly
        return "bigint"

    type_val = schema.get("type")
    values = schema.get("enum", [])

    if type_val == "string":
        return " | ".join([f"'{v!s}'" for v in values])

    if type_val == "integer":
        return " | ".join([str(v) for v in values])

    # Fallback: treat as string literals
    return " | ".join([f"'{v!s}'" for v in values])


def ts_type(schema: dict[str, Any] | None, schemas: dict[str, Any] | None = None) -> str:  # noqa: PLR0911
    """Map OpenAPI schema to a TypeScript type string."""
    if not schema:
        return "any"

    # Handle references
    if "$ref" in schema:
        ref_name = _extract_ref_name(schema["$ref"])  # e.g. #/components/schemas/Foo
        return ts_pascal_case(ref_name)

    # Handle composed schemas
    if "allOf" in schema:
        return _intersection([ts_type(s, schemas) for s in schema.get("allOf", [])])

    if "oneOf" in schema:
        return _union([ts_type(s, schemas) for s in schema.get("oneOf", [])])

    if "anyOf" in schema:
        return _union([ts_type(s, schemas) for s in schema.get("anyOf", [])])

    # Enums
    enum_t = ts_enum_type(schema)
    if enum_t:
        return enum_t

    schema_type = schema.get("type")

    # Handle array of items
    if schema_type == "array":
        items_schema = schema.get("items", {})
        items_type = ts_type(items_schema, schemas)
        return f"{items_type}[]"

    # Object type
    if schema_type == "object" or (not schema_type and ("properties" in schema or "additionalProperties" in schema)):
        type_str = _inline_object(schema, schemas)
        return _nullable(type_str, schema)

    # Primitive types
    type_str = _map_primitive(str(schema_type), schema.get("format"), schema)
    return _nullable(type_str, schema)


# ---------- Response helpers ----------


def has_msgpack_2xx(responses: dict[str, Any]) -> bool:
    for status, resp in (responses or {}).items():
        if not str(status).startswith("2"):
            continue
        content = (resp or {}).get("content", {})
        if any(ct in (content or {}) for ct in ("application/msgpack", "application/x-binary")):
            return True
    return False


def response_content_types(responses: dict[str, Any]) -> list[str]:
    cts: set[str] = set()
    for status, resp in (responses or {}).items():
        if not str(status).startswith("2"):
            continue
        content = (resp or {}).get("content", {})
        for ct in content:
            cts.add(ct)
    return sorted(cts)


def collect_schema_refs(schema: dict[str, Any], current_schema_name: str | None = None) -> list[str]:  # noqa: C901
    """Recursively collect all $ref model names from a schema, excluding self-references."""
    refs: set[str] = set()

    def _traverse(obj: dict[str, Any]) -> None:  # noqa: C901
        if not obj or not isinstance(obj, dict):
            return

        # Direct $ref
        if "$ref" in obj:
            ref_name = ts_pascal_case(_extract_ref_name(obj["$ref"]))
            # Exclude self-references
            if current_schema_name and ref_name != ts_pascal_case(current_schema_name):
                refs.add(ref_name)
            return  # Don't traverse further if this is a ref

        # Properties
        if "properties" in obj and isinstance(obj["properties"], dict):
            for prop_schema in obj["properties"].values():
                _traverse(prop_schema)

        # Array items
        if "items" in obj:
            _traverse(obj["items"])

        # Composed schemas
        for key in ["allOf", "oneOf", "anyOf"]:
            if key in obj and isinstance(obj[key], list):
                for sub_schema in obj[key]:
                    _traverse(sub_schema)

        # Additional properties
        if "additionalProperties" in obj and isinstance(obj["additionalProperties"], dict):
            _traverse(obj["additionalProperties"])

    _traverse(schema)
    return sorted(refs)


FILTERS: dict[str, Any] = {
    "ts_doc_comment": ts_doc_comment,
    "ts_string_literal": ts_string_literal,
    "ts_optional": ts_optional,
    "ts_array": ts_array,
    "ts_type": ts_type,
    "ts_pascal_case": ts_pascal_case,
    "ts_camel_case": ts_camel_case,
    "ts_property_name": ts_property_name,
    "has_msgpack_2xx": has_msgpack_2xx,
    "response_content_types": response_content_types,
    "collect_schema_refs": collect_schema_refs,
}
