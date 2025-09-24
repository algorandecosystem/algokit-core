"""
TypeScript-specific Jinja2 filters and helpers.
Phase 2 adds OpenAPI -> TS type mapping and naming utilities.
"""

from __future__ import annotations

import re
from typing import Any

from ts_oas_generator import constants
from ts_oas_generator.constants import MediaType, OperationKey, SchemaKey, TypeScriptType

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


def ts_kebab_case(name: str) -> str:
    parts = _split_words(name)
    return "-".join(p.lower() for p in parts)


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
    parts = [t for t in types if t and t != TypeScriptType.ANY]
    return " & ".join(parts) if parts else TypeScriptType.ANY


def _nullable(type_str: str, schema: dict[str, Any]) -> str:
    # OpenAPI 3.0 nullable flag
    if schema.get(SchemaKey.NULLABLE) is True:
        return _union([type_str, TypeScriptType.NULL])

    # OpenAPI 3.1 union type with null
    t = schema.get(SchemaKey.TYPE)
    if isinstance(t, list) and TypeScriptType.NULL in t:
        non_nulls = [x for x in t if x != TypeScriptType.NULL]
        # If there's exactly one non-null type, union with null
        if len(non_nulls) == 1:
            return _union([ts_type({SchemaKey.TYPE: non_nulls[0]}, None), TypeScriptType.NULL])
        # Else, build a union of all non-nulls + null
        return _union([_union([ts_type({SchemaKey.TYPE: n}, None) for n in non_nulls]), TypeScriptType.NULL])

    return type_str


def _inline_object(schema: dict[str, Any], schemas: dict[str, Any] | None) -> str:
    properties: dict[str, Any] = schema.get(SchemaKey.PROPERTIES, {}) or {}
    required = set(schema.get(SchemaKey.REQUIRED, []) or [])
    parts: list[str] = []

    for prop_name, prop_schema in properties.items():
        canonical_name = prop_schema.get(constants.X_ALGOKIT_FIELD_RENAME) or prop_name
        # Add property description as doc comment
        description = prop_schema.get("description")
        if description:
            doc_comment = ts_doc_comment(description)
            # Format docstring for inline object (indent each line)
            indented_doc = "\n  ".join(doc_comment.split("\n"))
            parts.append(f"\n  {indented_doc}")

        # Generate camelCase TS property names for better DX
        ts_name = ts_camel_case(canonical_name)
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

    if parts:
        # Format with proper indentation
        formatted_parts = []
        for part in parts:
            if part.startswith("\n"):
                formatted_parts.append(part)
            else:
                formatted_parts.append(f"  {part}")
        return "{\n" + "\n".join(formatted_parts) + "\n}"
    return "{}"


def _map_primitive(schema_type: str, _schema_format: str | None, _schema: dict[str, Any]) -> str:
    if schema_type == "integer":
        # Keep small control integers as number when format suggests int32 or when description indicates small discriminator
        if _schema_format == "int32" or "value type" in str(_schema.get("description", "")).lower():
            return TypeScriptType.NUMBER
        return TypeScriptType.BIGINT

    if schema_type in ("number",):
        return TypeScriptType.NUMBER

    if schema_type == "string":
        # bytes/base64 are represented as Uint8Array in domain model
        if _schema_format == "byte" or _schema.get(constants.X_ALGOKIT_BYTES_BASE64) is True:
            return TypeScriptType.UINT8ARRAY
        return TypeScriptType.STRING

    if schema_type == "boolean":
        return TypeScriptType.BOOLEAN

    return TypeScriptType.ANY


def ts_enum_type(schema: dict[str, Any]) -> str | None:
    if SchemaKey.ENUM not in schema:
        return None

    if schema.get(constants.X_ALGOKIT_BIGINT) is True:
        # For bigint-marked enums, use bigint type directly
        return TypeScriptType.BIGINT

    type_val = schema.get(SchemaKey.TYPE)
    values = schema.get(SchemaKey.ENUM, [])

    if type_val == "string":
        return " | ".join([f"'{v!s}'" for v in values])

    if type_val == "integer":
        # Integers used as enum discriminators are small; map to number
        return " | ".join([str(v) for v in values])

    # Fallback: treat as string literals
    return " | ".join([f"'{v!s}'" for v in values])


def ts_type(schema: dict[str, Any] | None, schemas: dict[str, Any] | None = None) -> str:  # noqa: C901, PLR0911
    """Map OpenAPI schema to a TypeScript type string."""
    if not schema:
        return TypeScriptType.ANY

    # Vendor extension: x-algokit-signed-txn -> reference domain SignedTransaction type directly
    if isinstance(schema, dict) and schema.get(constants.X_ALGOKIT_SIGNED_TXN) is True:
        return "SignedTransaction"

    # Handle references
    if "$ref" in schema:
        ref_name = _extract_ref_name(schema["$ref"])  # e.g. #/components/schemas/Foo
        return ts_pascal_case(ref_name)

    # Handle composed schemas
    if SchemaKey.ALL_OF in schema:
        return _intersection([ts_type(s, schemas) for s in schema.get(SchemaKey.ALL_OF, [])])

    if SchemaKey.ONE_OF in schema:
        return _union([ts_type(s, schemas) for s in schema.get(SchemaKey.ONE_OF, [])])

    if SchemaKey.ANY_OF in schema:
        return _union([ts_type(s, schemas) for s in schema.get(SchemaKey.ANY_OF, [])])

    # Enums
    enum_t = ts_enum_type(schema)
    if enum_t:
        return enum_t

    schema_type = schema.get(SchemaKey.TYPE)

    # Handle array of items
    if schema_type == "array":
        items_schema = schema.get(SchemaKey.ITEMS, {})
        # Apply vendor extension on nested items as well
        if isinstance(items_schema, dict) and items_schema.get(constants.X_ALGOKIT_SIGNED_TXN) is True:
            items_type = "SignedTransaction"
        else:
            items_type = ts_type(items_schema, schemas)
        return f"{items_type}[]"

    # Object type
    if schema_type == TypeScriptType.OBJECT or (
        not schema_type and (SchemaKey.PROPERTIES in schema or SchemaKey.ADDITIONAL_PROPERTIES in schema)
    ):
        type_str = _inline_object(schema, schemas)
        return _nullable(type_str, schema)

    # Primitive types
    type_str = _map_primitive(str(schema_type), schema.get(SchemaKey.FORMAT), schema)
    return _nullable(type_str, schema)


# ---------- Response helpers ----------


def has_msgpack_2xx(responses: dict[str, Any]) -> bool:
    for status, resp in (responses or {}).items():
        if not str(status).startswith(constants.SUCCESS_STATUS_PREFIX):
            continue
        content = (resp or {}).get(OperationKey.CONTENT, {})
        if any(ct in (content or {}) for ct in (MediaType.MSGPACK, MediaType.BINARY)):
            return True
    return False


def response_content_types(responses: dict[str, Any]) -> list[str]:
    cts: set[str] = set()
    for status, resp in (responses or {}).items():
        if not str(status).startswith(constants.SUCCESS_STATUS_PREFIX):
            continue
        content = (resp or {}).get(OperationKey.CONTENT, {})
        for ct in content:
            cts.add(ct)
    return sorted(cts)


def collect_schema_refs(schema: dict[str, Any], current_schema_name: str | None = None) -> list[str]:  # noqa: C901
    """Recursively collect all $ref model names from a schema, excluding self-references."""
    refs: set[str] = set()

    def _traverse(obj: dict[str, Any]) -> None:  # noqa: C901
        if not obj or not isinstance(obj, dict):
            return

        # Do not include external domain types (e.g., SignedTransaction) in local refs

        # Direct $ref
        if "$ref" in obj:
            ref_name = ts_pascal_case(_extract_ref_name(obj["$ref"]))
            # Exclude self-references
            if current_schema_name and ref_name != ts_pascal_case(current_schema_name):
                refs.add(ref_name)
            return  # Don't traverse further if this is a ref

        # Properties
        if SchemaKey.PROPERTIES in obj and isinstance(obj[SchemaKey.PROPERTIES], dict):
            for prop_schema in obj[SchemaKey.PROPERTIES].values():
                _traverse(prop_schema)

        # Array items
        if SchemaKey.ITEMS in obj:
            _traverse(obj[SchemaKey.ITEMS])

        # Composed schemas
        for key in [SchemaKey.ALL_OF, SchemaKey.ONE_OF, SchemaKey.ANY_OF]:
            if key in obj and isinstance(obj[key], list):
                for sub_schema in obj[key]:
                    _traverse(sub_schema)

        # Additional properties
        if SchemaKey.ADDITIONAL_PROPERTIES in obj and isinstance(obj[SchemaKey.ADDITIONAL_PROPERTIES], dict):
            _traverse(obj[SchemaKey.ADDITIONAL_PROPERTIES])

    _traverse(schema)
    return sorted(refs)


def schema_uses_signed_txn(schema: dict[str, Any]) -> bool:  # noqa: C901
    """Detect if a schema (recursively) uses the x-algokit-signed-txn vendor extension."""
    found = False

    def _traverse(obj: Any) -> None:  # noqa: C901
        nonlocal found
        if found:
            return
        if not isinstance(obj, dict):
            return
        if obj.get(constants.X_ALGOKIT_SIGNED_TXN) is True:
            found = True
            return
        if "$ref" in obj:
            return
        # properties
        props = obj.get(constants.SchemaKey.PROPERTIES)
        if isinstance(props, dict):
            for v in props.values():
                _traverse(v)
        # items
        if constants.SchemaKey.ITEMS in obj:
            _traverse(obj[constants.SchemaKey.ITEMS])
        # composition
        for key in [constants.SchemaKey.ALL_OF, constants.SchemaKey.ONE_OF, constants.SchemaKey.ANY_OF]:
            if key in obj and isinstance(obj[key], list):
                for sub in obj[key]:
                    _traverse(sub)
        # additionalProperties
        addl = obj.get(constants.SchemaKey.ADDITIONAL_PROPERTIES)
        if isinstance(addl, dict):
            _traverse(addl)

    _traverse(schema)
    return found


FILTERS: dict[str, Any] = {
    "ts_doc_comment": ts_doc_comment,
    "ts_string_literal": ts_string_literal,
    "ts_optional": ts_optional,
    "ts_array": ts_array,
    "ts_type": ts_type,
    "ts_pascal_case": ts_pascal_case,
    "ts_camel_case": ts_camel_case,
    "ts_kebab_case": ts_kebab_case,
    "ts_property_name": ts_property_name,
    "has_msgpack_2xx": has_msgpack_2xx,
    "response_content_types": response_content_types,
    "collect_schema_refs": collect_schema_refs,
    "schema_uses_signed_txn": schema_uses_signed_txn,
}
