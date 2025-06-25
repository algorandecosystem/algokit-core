"""
OpenAPI Specification Parser for Rust Client Generation

This module parses OpenAPI 3.x specifications and extracts information
needed to generate Rust API clients.
"""

import json
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any

from rust_oas_generator.utils.string_case import (
    escape_rust_keyword,
    normalize_rust_identifier,
    rust_pascal_case,
    rust_snake_case,
)


def snake_case(name: str) -> str:
    """Convert string to snake_case for Rust naming."""
    return rust_snake_case(name)


def pascal_case(name: str) -> str:
    """Convert string to PascalCase for Rust types."""
    return rust_pascal_case(name)


def normalize_name(name: str) -> str:
    """Normalize name for Rust identifiers."""
    return normalize_rust_identifier(name)


def rust_type_from_openapi(
    schema: dict[str, Any],
    schemas: dict[str, Any],
    visited: set[str] | None = None,
) -> str:
    """Convert OpenAPI schema type to Rust type."""
    if visited is None:
        visited = set()

    if "$ref" in schema:
        ref_name = schema["$ref"].split("/")[-1]
        if ref_name in visited:
            return pascal_case(ref_name)

        visited.add(ref_name)
        if ref_name in schemas:
            return "ModelBox" if ref_name == "Box" else pascal_case(ref_name)
        return pascal_case(ref_name)

    schema_type = schema.get("type", "string")
    schema_format = schema.get("format")

    type_mapping = {
        "string": {
            None: "String",
            "date": "String",
            "date-time": "String",
            "byte": "String",
            "binary": "Vec<u8>",
        },
        "integer": {
            None: "i32",
            "int32": "i32",
            "int64": "i64",
        },
        "number": {
            None: "f64",
            "float": "f32",
            "double": "f64",
        },
        "boolean": {
            None: "bool",
        },
        "object": {
            None: "serde_json::Value",
        },
    }

    if schema_type == "array":
        items_type = rust_type_from_openapi(schema.get("items", {}), schemas, visited)
        return f"Vec<{items_type}>"

    type_formats = type_mapping.get(schema_type, {})
    return type_formats.get(schema_format, "String") if isinstance(type_formats, dict) else "String"


def detect_msgpack_field(prop_data: dict[str, Any]) -> bool:
    """Detect if a property should use msgpack/base64 encoding."""
    if prop_data.get("format") == "byte":
        return True

    description = prop_data.get("description", "").lower()
    if "base64" in description:
        return True

    return bool(prop_data.get("x-msgpack-encoding"))


def detect_msgpack_support_for_operation(operation_data: dict[str, Any]) -> bool:
    """Detect if an operation supports msgpack content type or binary data."""
    request_body = operation_data.get("requestBody", {})
    content = request_body.get("content", {})

    if "application/msgpack" in content:
        return True

    if "application/x-binary" in content:
        binary_content = content["application/x-binary"]
        schema = binary_content.get("schema", {})
        if schema.get("format") == "binary":
            return True

    responses = operation_data.get("responses", {})
    for response_data in responses.values():
        content = response_data.get("content", {})
        if "application/msgpack" in content:
            return True

    return False


def should_implement_algokit_msgpack(
    schema_data: dict[str, Any],
    *,
    operation_msgpack_support: bool = False,
) -> bool:
    """Determine if a schema should implement AlgorandMsgpack trait."""
    if schema_data.get("x-algokit-signed-txn", False):
        return True

    properties = schema_data.get("properties", {})
    for prop_data in properties.values():
        if prop_data.get("x-algokit-signed-txn", False):
            return True

        if prop_data.get("type") == "array":
            items = prop_data.get("items", {})
            if items.get("x-algokit-signed-txn", False):
                return True

    return bool(operation_msgpack_support)


def rust_type_with_msgpack(
    schema: dict[str, Any],
    schemas: dict[str, Any],
    visited: set[str] | None = None,
) -> str:
    """Convert OpenAPI schema type to Rust type with msgpack considerations."""
    return "Vec<u8>" if detect_msgpack_field(schema) else rust_type_from_openapi(schema, schemas, visited)


@dataclass
class Parameter:
    """Represents an OpenAPI parameter."""

    name: str
    param_type: str
    rust_type: str
    required: bool
    description: str | None = None
    rust_name: str = field(init=False)
    rust_field_name: str = field(init=False)

    def __post_init__(self) -> None:
        self.rust_name = snake_case(self.name)
        self.rust_field_name = escape_rust_keyword(self.rust_name)


@dataclass
class Response:
    """Represents an OpenAPI response."""

    status_code: str
    description: str
    rust_type: str | None = None
    content_types: list[str] = field(default_factory=list)
    supports_msgpack: bool = False


@dataclass
class Operation:
    """Represents an OpenAPI operation."""

    operation_id: str
    method: str
    path: str
    summary: str | None
    description: str | None
    parameters: list[Parameter]
    request_body: dict[str, Any] | None
    responses: dict[str, Response]
    tags: list[str]
    rust_function_name: str = field(init=False)
    rust_error_enum: str = field(init=False)
    supports_msgpack: bool = False
    request_body_supports_msgpack: bool = False

    def __post_init__(self) -> None:
        self.rust_function_name = snake_case(self.operation_id)
        self.rust_error_enum = f"{pascal_case(self.operation_id)}Error"


@dataclass
class Property:
    """Represents a schema property."""

    name: str
    rust_type: str
    required: bool
    description: str | None = None
    is_base64_encoded: bool = False
    vendor_extensions: list[tuple[str, Any]] = field(default_factory=list)
    format: str | None = None
    items: "Property | None" = None
    rust_name: str = field(init=False)
    rust_field_name: str = field(init=False)
    rust_type_with_msgpack: str = field(init=False)
    is_msgpack_field: bool = field(init=False)
    is_signed_transaction: bool = field(init=False)

    def __post_init__(self) -> None:
        self.rust_name = snake_case(self.name)
        self.rust_field_name = escape_rust_keyword(self.rust_name)
        self.rust_type_with_msgpack = "Vec<u8>" if self.is_base64_encoded else self.rust_type
        self.is_msgpack_field = self.is_base64_encoded

        self.is_signed_transaction = any(
            "x-algokit-signed-txn" in ext_name and ext_value for ext_name, ext_value in self.vendor_extensions
        )

        if self.items and hasattr(self.items, "vendor_extensions"):
            self.is_signed_transaction = self.is_signed_transaction or any(
                "x-algokit-signed-txn" in ext_name and ext_value for ext_name, ext_value in self.items.vendor_extensions
            )


@dataclass
class Schema:
    """Represents an OpenAPI schema."""

    name: str
    schema_type: str
    description: str | None
    properties: list[Property]
    required_fields: list[str]
    vendor_extensions: dict[str, Any] = field(default_factory=dict)
    rust_struct_name: str = field(init=False)
    has_msgpack_fields: bool = field(init=False)
    has_required_fields: bool = field(init=False)
    implements_algokit_msgpack: bool = field(init=False)
    has_signed_transaction_fields: bool = field(init=False)

    def __post_init__(self) -> None:
        self.rust_struct_name = "ModelBox" if self.name == "Box" else pascal_case(self.name)
        self.has_msgpack_fields = any(prop.is_base64_encoded for prop in self.properties)
        self.has_required_fields = len(self.required_fields) > 0
        self.has_signed_transaction_fields = any(prop.is_signed_transaction for prop in self.properties)


@dataclass
class ParsedSpec:
    """Represents a parsed OpenAPI specification."""

    info: dict[str, Any]
    servers: list[dict[str, Any]]
    operations: list[Operation]
    schemas: dict[str, Schema]
    content_types: list[str]
    has_msgpack_operations: bool = False


class OASParser:
    """Parser for OpenAPI 3.x specifications."""

    def __init__(self) -> None:
        self.spec_data: dict[str, Any] | None = None
        self.schemas: dict[str, Any] = {}
        self.msgpack_operations: list[str] = []

    def parse_file(self, file_path: str | Path) -> ParsedSpec:
        """Parse OpenAPI specification from file."""
        path = Path(file_path)
        with path.open(encoding="utf-8") as f:
            self.spec_data = json.load(f)
        return self._parse_spec()

    def parse_dict(self, spec_dict: dict[str, Any]) -> ParsedSpec:
        """Parse OpenAPI specification from dictionary."""
        self.spec_data = spec_dict
        return self._parse_spec()

    def _parse_spec(self) -> ParsedSpec:
        """Parse the loaded specification."""
        if not self.spec_data:
            msg = "No specification data loaded"
            raise ValueError(msg)

        self.schemas = self.spec_data.get("components", {}).get("schemas", {})

        info = self.spec_data.get("info", {})
        servers = self.spec_data.get("servers", [])
        operations = self._parse_operations()
        schemas = self._parse_schemas()
        content_types = self._extract_content_types()

        has_msgpack_operations = len(self.msgpack_operations) > 0
        self._update_schemas_for_msgpack(schemas, has_msgpack_operations=has_msgpack_operations)

        return ParsedSpec(
            info=info,
            servers=servers,
            operations=operations,
            schemas=schemas,
            content_types=content_types,
            has_msgpack_operations=has_msgpack_operations,
        )

    def _update_schemas_for_msgpack(
        self,
        schemas: dict[str, Schema],
        *,
        has_msgpack_operations: bool,
    ) -> None:
        """Update schemas to implement AlgorandMsgpack trait when appropriate."""
        msgpack_request_types: set[str] = set()
        msgpack_response_types: set[str] = set()

        if has_msgpack_operations and self.spec_data:
            for operation in self.spec_data.get("paths", {}).values():
                for method_data in operation.values():
                    if isinstance(method_data, dict) and method_data.get("operationId") in self.msgpack_operations:
                        self._collect_msgpack_types(
                            method_data,
                            msgpack_request_types,
                            msgpack_response_types,
                        )

        all_msgpack_types = msgpack_request_types | msgpack_response_types
        all_required_msgpack_types = set()

        for root_type in all_msgpack_types:
            dependencies = self._find_nested_dependencies(root_type)
            all_required_msgpack_types.update(dependencies)

        for schema_name, schema in schemas.items():
            raw_schema = self.schemas.get(schema_name, {})

            should_implement = should_implement_algokit_msgpack(
                raw_schema,
                operation_msgpack_support=has_msgpack_operations
                and self._should_implement_msgpack_for_schema(
                    schema_name, msgpack_request_types, msgpack_response_types, all_required_msgpack_types
                ),
            )

            schema.implements_algokit_msgpack = should_implement

    def _collect_msgpack_types(
        self,
        method_data: dict[str, Any],
        request_types: set[str],
        response_types: set[str],
    ) -> None:
        """Collect msgpack request and response types from method data."""
        request_body = method_data.get("requestBody", {})
        if "application/msgpack" in request_body.get("content", {}):
            content = request_body["content"]["application/msgpack"]
            schema = content.get("schema", {})
            if "$ref" in schema:
                ref_name = schema["$ref"].split("/")[-1]
                request_types.add(ref_name)

        responses = method_data.get("responses", {})
        for status_code, response_data in responses.items():
            if status_code.startswith("2"):
                content = response_data.get("content", {})
                if "application/msgpack" in content:
                    msgpack_content = content["application/msgpack"]
                    schema = msgpack_content.get("schema", {})
                    if "$ref" in schema:
                        ref_name = schema["$ref"].split("/")[-1]
                        response_types.add(ref_name)

    def _should_implement_msgpack_for_schema(
        self, schema_name: str, request_types: set[str], response_types: set[str], required_types: set[str]
    ) -> bool:
        """Check if schema should implement msgpack based on its usage."""
        is_response_model = schema_name.endswith("Response")
        is_request_model = schema_name in request_types
        is_msgpack_response_model = schema_name in response_types
        is_nested_dependency = schema_name in required_types

        return is_response_model or is_request_model or is_msgpack_response_model or is_nested_dependency

    def _find_nested_dependencies(
        self,
        type_name: str,
        visited: set[str] | None = None,
    ) -> set[str]:
        """Find all nested dependencies recursively."""
        if visited is None:
            visited = set()

        if type_name in visited:
            return set()

        visited.add(type_name)
        dependencies = {type_name}

        raw_schema = self.schemas.get(type_name, {})
        properties = raw_schema.get("properties", {})

        for prop_data in properties.values():
            deps = self._extract_schema_references(prop_data, visited.copy())
            dependencies.update(deps)

        for key in ["oneOf", "anyOf", "allOf"]:
            if key in raw_schema:
                for item in raw_schema[key]:
                    deps = self._extract_schema_references(item, visited.copy())
                    dependencies.update(deps)

        return dependencies

    def _extract_schema_references(
        self,
        schema_item: dict[str, Any],
        visited: set[str] | None = None,
    ) -> set[str]:
        """Extract all schema references from a schema item."""
        if visited is None:
            visited = set()

        references = set()

        if "$ref" in schema_item:
            ref_name = schema_item["$ref"].split("/")[-1]
            if ref_name not in visited:
                references.add(ref_name)
                nested_deps = self._find_nested_dependencies_helper(
                    ref_name,
                    visited.copy(),
                )
                references.update(nested_deps)

        if schema_item.get("type") == "array" and "items" in schema_item:
            item_refs = self._extract_schema_references(
                schema_item["items"],
                visited.copy(),
            )
            references.update(item_refs)

        if "properties" in schema_item:
            for prop_data in schema_item["properties"].values():
                prop_refs = self._extract_schema_references(prop_data, visited.copy())
                references.update(prop_refs)

        for key in ["oneOf", "anyOf", "allOf"]:
            if key in schema_item:
                for item in schema_item[key]:
                    item_refs = self._extract_schema_references(item, visited.copy())
                    references.update(item_refs)

        return references

    def _find_nested_dependencies_helper(
        self,
        type_name: str,
        visited: set[str],
    ) -> set[str]:
        """Helper method to find nested dependencies."""
        if type_name in visited:
            return set()

        visited.add(type_name)
        dependencies = set()

        raw_schema = self.schemas.get(type_name, {})
        properties = raw_schema.get("properties", {})

        for prop_data in properties.values():
            deps = self._extract_schema_references(prop_data, visited.copy())
            dependencies.update(deps)

        return dependencies

    def _parse_operations(self) -> list[Operation]:
        """Parse all operations from paths."""
        operations: list[Operation] = []
        if not self.spec_data:
            return operations
        paths = self.spec_data.get("paths", {})

        http_methods = {"get", "post", "put", "delete", "patch", "head", "options"}

        for path, path_item in paths.items():
            for method, operation_data in path_item.items():
                if method.lower() in http_methods:
                    operation = self._parse_operation(
                        path,
                        method.upper(),
                        operation_data,
                    )
                    if operation:
                        operations.append(operation)

                        if operation.supports_msgpack:
                            self.msgpack_operations.append(operation.operation_id)

        return operations

    def _parse_operation(
        self,
        path: str,
        method: str,
        operation_data: dict[str, Any],
    ) -> Operation | None:
        """Parse a single operation."""
        operation_id = operation_data.get("operationId")
        if not operation_id:
            return None

        supports_msgpack = detect_msgpack_support_for_operation(operation_data)
        request_body_supports_msgpack = self._check_request_body_msgpack_support(
            operation_data,
        )

        parameters = []
        for param_data in operation_data.get("parameters", []):
            param = self._parse_parameter(param_data)
            if param:
                parameters.append(param)

        responses = {}
        for status_code, response_data in operation_data.get("responses", {}).items():
            response = self._parse_response(status_code, response_data, operation_id)
            responses[status_code] = response

        return Operation(
            operation_id=operation_id,
            method=method,
            path=path,
            summary=operation_data.get("summary"),
            description=operation_data.get("description"),
            parameters=parameters,
            request_body=operation_data.get("requestBody"),
            responses=responses,
            tags=operation_data.get("tags", []),
            supports_msgpack=supports_msgpack,
            request_body_supports_msgpack=request_body_supports_msgpack,
        )

    def _check_request_body_msgpack_support(
        self,
        operation_data: dict[str, Any],
    ) -> bool:
        """Check if request body supports msgpack or binary transmission."""
        request_body = operation_data.get("requestBody", {})
        content = request_body.get("content", {})

        if "application/msgpack" in content:
            return True

        if "application/x-binary" in content:
            binary_content = content["application/x-binary"]
            schema = binary_content.get("schema", {})
            format_value: str | None = schema.get("format")
            return format_value == "binary"

        return False

    def _parse_parameter(self, param_data: dict[str, Any]) -> Parameter | None:
        """Parse a parameter."""
        if "$ref" in param_data:
            param_data = self._resolve_reference(param_data["$ref"])

        name = param_data.get("name")
        if not name:
            return None

        schema = param_data.get("schema", {})
        rust_type = rust_type_from_openapi(schema, self.schemas, set())

        return Parameter(
            name=name,
            param_type=param_data.get("in", "query"),
            rust_type=rust_type,
            required=param_data.get("required", False),
            description=param_data.get("description"),
        )

    def _resolve_reference(self, ref: str) -> dict[str, Any]:
        """Resolve a JSON reference."""
        if not self.spec_data:
            return {}

        ref_path = ref.split("/")
        resolved: dict[str, Any] | None = self.spec_data
        for part in ref_path[1:]:  # Skip '#'
            if resolved is None:
                return {}
            resolved = resolved.get(part)
        return resolved or {}

    def _parse_response(
        self,
        status_code: str,
        response_data: dict[str, Any],
        operation_id: str,
    ) -> Response:
        """Parse a response."""
        content = response_data.get("content", {})
        content_types = list(content.keys())
        supports_msgpack = "application/msgpack" in content_types

        rust_type = self._determine_response_rust_type(
            content_types,
            content,
            status_code,
            operation_id,
            response_data,
        )

        return Response(
            status_code=status_code,
            description=response_data.get("description", ""),
            rust_type=rust_type,
            content_types=content_types,
            supports_msgpack=supports_msgpack,
        )

    def _determine_response_rust_type(
        self,
        content_types: list[str],
        content: dict[str, Any],
        status_code: str,
        operation_id: str,
        response_data: dict[str, Any],
    ) -> str | None:
        """Determine the Rust type for a response."""
        if not content_types:
            return None

        first_content = content[content_types[0]]
        schema = first_content.get("schema", {})

        if self._should_create_response_model(schema, status_code):
            response_model_name = f"{operation_id}{status_code}Response"

            self.schemas[response_model_name] = self._create_response_schema(
                response_model_name,
                schema,
                response_data.get("description", ""),
            )

            return pascal_case(response_model_name)

        return rust_type_from_openapi(schema, self.schemas, set())

    def _should_create_response_model(
        self,
        schema: dict[str, Any],
        status_code: str,
    ) -> bool:
        """Determine if we should create a response model for this schema."""
        if not status_code.startswith("2") or "$ref" in schema:
            return False

        if schema.get("type") == "object" and "properties" in schema:
            return True

        return bool("required" in schema or "allOf" in schema or "oneOf" in schema)

    def _create_response_schema(
        self,
        _name: str,
        schema: dict[str, Any],
        description: str,
    ) -> dict[str, Any]:
        """Create a response schema from an inline schema."""
        response_schema = schema.copy()
        if description and "description" not in response_schema:
            response_schema["description"] = description
        return response_schema

    def _parse_schemas(self) -> dict[str, Schema]:
        """Parse all schemas."""
        schemas = {}

        for schema_name, schema_data in self.schemas.items():
            schema = self._parse_schema(schema_name, schema_data)
            if schema:
                schemas[schema_name] = schema

        return schemas

    def _parse_schema(self, name: str, schema_data: dict[str, Any]) -> Schema | None:
        """Parse a single schema."""
        schema_type = schema_data.get("type", "object")
        properties_data = schema_data.get("properties", {})
        required_fields = schema_data.get("required", [])

        # Extract vendor extensions
        vendor_extensions = {}
        for key, value in schema_data.items():
            if key.startswith("x-"):
                vendor_extensions[key] = value

        properties = []
        for prop_name, prop_data in properties_data.items():
            rust_type = rust_type_from_openapi(prop_data, self.schemas, set())
            is_base64_encoded = detect_msgpack_field(prop_data)

            # Extract vendor extensions for this property
            prop_vendor_extensions = []
            for key, value in prop_data.items():
                if key.startswith("x-"):
                    prop_vendor_extensions.append((key, value))

            # Handle array items with vendor extensions
            items_property = None
            if prop_data.get("type") == "array" and "items" in prop_data:
                items_data = prop_data["items"]
                items_vendor_extensions = []
                for key, value in items_data.items():
                    if key.startswith("x-"):
                        items_vendor_extensions.append((key, value))

                items_property = Property(
                    name=f"{prop_name}_item",
                    rust_type=rust_type_from_openapi(items_data, self.schemas, set()),
                    required=False,
                    description=items_data.get("description"),
                    is_base64_encoded=detect_msgpack_field(items_data),
                    vendor_extensions=items_vendor_extensions,
                    format=items_data.get("format"),
                )

            prop = Property(
                name=prop_name,
                rust_type=rust_type,
                required=prop_name in required_fields,
                description=prop_data.get("description"),
                is_base64_encoded=is_base64_encoded,
                vendor_extensions=prop_vendor_extensions,
                format=prop_data.get("format"),
                items=items_property,
            )
            properties.append(prop)

        return Schema(
            name=name,
            schema_type=schema_type,
            description=schema_data.get("description"),
            properties=properties,
            required_fields=required_fields,
            vendor_extensions=vendor_extensions,
        )

    def _extract_content_types(self) -> list[str]:
        """Extract all content types used in the API."""
        content_types = set()

        if not self.spec_data:
            return []

        for path_item in self.spec_data.get("paths", {}).values():
            for operation in path_item.values():
                if isinstance(operation, dict):
                    request_body = operation.get("requestBody", {})
                    content = request_body.get("content", {})
                    content_types.update(content.keys())

                    for response in operation.get("responses", {}).values():
                        response_content = response.get("content", {})
                        content_types.update(response_content.keys())

        return sorted(content_types)
