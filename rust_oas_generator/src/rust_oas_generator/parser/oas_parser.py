"""
OpenAPI Specification Parser for Rust Client Generation

This module parses OpenAPI 3.x specifications and extracts information
needed to generate Rust API clients.
"""

import json
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any, Dict, List, Optional, Tuple, Union

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
    schema: Dict[str, Any],
    schemas: Dict[str, Any],
    visited: set = None,
) -> str:
    """Convert OpenAPI schema type to Rust type."""
    if visited is None:
        visited = set()

    if "$ref" in schema:
        ref_name = schema["$ref"].split("/")[-1]
        if ref_name in visited:
            # Circular reference, return the type name
            return pascal_case(ref_name)
        visited.add(ref_name)
        if ref_name in schemas:
            # Resolve the reference
            return rust_type_from_openapi(schemas[ref_name], schemas, visited)
        return pascal_case(ref_name)

    schema_type = schema.get("type", "string")
    schema_format = schema.get("format")

    type_mapping = {
        "string": {
            None: "String",
            "date": "String",  # Could be chrono::NaiveDate
            "date-time": "String",  # Could be chrono::DateTime
            "byte": "String",  # Base64 encoded
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
            None: "serde_json::Value",  # Generic object
        },
    }

    if schema_type == "array":
        items_type = rust_type_from_openapi(schema.get("items", {}), schemas, visited)
        return f"Vec<{items_type}>"

    return type_mapping.get(schema_type, {}).get(schema_format, "String")


def detect_msgpack_field(prop_data: Dict[str, Any]) -> bool:
    """Detect if a property should use msgpack/base64 encoding."""
    # Check format
    if prop_data.get("format") == "byte":
        return True

    # Check description for base64 mention
    description = prop_data.get("description", "").lower()
    if "base64" in description:
        return True

    # Check vendor extension
    if prop_data.get("x-msgpack-encoding"):
        return True

    return False


def rust_type_with_msgpack(
    schema: Dict[str, Any],
    schemas: Dict[str, Any],
    visited: set = None,
) -> str:
    """Convert OpenAPI schema type to Rust type with msgpack considerations."""
    if detect_msgpack_field(schema):
        return "Vec<u8>"
    return rust_type_from_openapi(schema, schemas, visited)


@dataclass
class Parameter:
    """Represents an OpenAPI parameter."""

    name: str
    param_type: str  # query, path, header, cookie
    rust_type: str
    required: bool
    description: Optional[str] = None
    rust_name: str = field(init=False)
    rust_field_name: str = field(init=False)

    def __post_init__(self):
        self.rust_name = snake_case(self.name)
        self.rust_field_name = escape_rust_keyword(self.rust_name)


@dataclass
class Response:
    """Represents an OpenAPI response."""

    status_code: str
    description: str
    rust_type: Optional[str] = None
    content_types: List[str] = field(default_factory=list)


@dataclass
class Operation:
    """Represents an OpenAPI operation."""

    operation_id: str
    method: str
    path: str
    summary: Optional[str]
    description: Optional[str]
    parameters: List[Parameter]
    request_body: Optional[Dict[str, Any]]
    responses: Dict[str, Response]
    tags: List[str]
    rust_function_name: str = field(init=False)
    rust_error_enum: str = field(init=False)

    def __post_init__(self):
        self.rust_function_name = snake_case(self.operation_id)
        self.rust_error_enum = f"{pascal_case(self.operation_id)}Error"


@dataclass
class Property:
    """Represents a schema property."""

    name: str
    rust_type: str
    required: bool
    description: Optional[str] = None
    is_base64_encoded: bool = False
    vendor_extensions: List[Tuple[str, Any]] = field(default_factory=list)
    format: Optional[str] = None
    items: Optional["Property"] = None
    rust_name: str = field(init=False)
    rust_field_name: str = field(init=False)
    rust_type_with_msgpack: str = field(init=False)
    is_msgpack_field: bool = field(init=False)
    is_signed_transaction: bool = field(init=False)

    def __post_init__(self):
        self.rust_name = snake_case(self.name)
        self.rust_field_name = escape_rust_keyword(self.rust_name)
        self.rust_type_with_msgpack = (
            "Vec<u8>" if self.is_base64_encoded else self.rust_type
        )
        self.is_msgpack_field = self.is_base64_encoded
        # Check if this property or its items have signed transaction extension
        self.is_signed_transaction = any(
            "x-algokit-signed-txn" in ext_name and ext_value
            for ext_name, ext_value in self.vendor_extensions
        )
        if self.items and hasattr(self.items, "vendor_extensions"):
            self.is_signed_transaction = self.is_signed_transaction or any(
                "x-algokit-signed-txn" in ext_name and ext_value
                for ext_name, ext_value in self.items.vendor_extensions
            )


@dataclass
class Schema:
    """Represents an OpenAPI schema."""

    name: str
    schema_type: str
    description: Optional[str]
    properties: List[Property]
    required_fields: List[str]
    vendor_extensions: Dict[str, Any] = field(default_factory=dict)
    rust_struct_name: str = field(init=False)
    has_msgpack_fields: bool = field(init=False)
    has_required_fields: bool = field(init=False)

    def __post_init__(self):
        self.rust_struct_name = pascal_case(self.name)
        self.has_msgpack_fields = any(
            prop.is_base64_encoded for prop in self.properties
        )
        self.has_required_fields = len(self.required_fields) > 0


@dataclass
class ParsedSpec:
    """Represents a parsed OpenAPI specification."""

    info: Dict[str, Any]
    servers: List[Dict[str, Any]]
    operations: List[Operation]
    schemas: Dict[str, Schema]
    content_types: List[str]


class OASParser:
    """Parser for OpenAPI 3.x specifications."""

    def __init__(self):
        self.spec_data: Optional[Dict[str, Any]] = None
        self.schemas: Dict[str, Any] = {}

    def parse_file(self, file_path: Union[str, Path]) -> ParsedSpec:
        """Parse OpenAPI specification from file."""
        with open(file_path, encoding="utf-8") as f:
            self.spec_data = json.load(f)
        return self._parse_spec()

    def parse_dict(self, spec_dict: Dict[str, Any]) -> ParsedSpec:
        """Parse OpenAPI specification from dictionary."""
        self.spec_data = spec_dict
        return self._parse_spec()

    def _parse_spec(self) -> ParsedSpec:
        """Parse the loaded specification."""
        if not self.spec_data:
            raise ValueError("No specification data loaded")

        # Extract schemas first for type resolution
        self.schemas = self.spec_data.get("components", {}).get("schemas", {})

        # Parse main components
        info = self.spec_data.get("info", {})
        servers = self.spec_data.get("servers", [])
        operations = self._parse_operations()
        schemas = self._parse_schemas()
        content_types = self._extract_content_types()

        return ParsedSpec(
            info=info,
            servers=servers,
            operations=operations,
            schemas=schemas,
            content_types=content_types,
        )

    def _parse_operations(self) -> List[Operation]:
        """Parse all operations from paths."""
        operations = []
        paths = self.spec_data.get("paths", {})

        for path, path_item in paths.items():
            for method, operation_data in path_item.items():
                if method.lower() in [
                    "get",
                    "post",
                    "put",
                    "delete",
                    "patch",
                    "head",
                    "options",
                ]:
                    operation = self._parse_operation(
                        path,
                        method.upper(),
                        operation_data,
                    )
                    if operation:
                        operations.append(operation)

        return operations

    def _parse_operation(
        self,
        path: str,
        method: str,
        operation_data: Dict[str, Any],
    ) -> Optional[Operation]:
        """Parse a single operation."""
        operation_id = operation_data.get("operationId")
        if not operation_id:
            return None

        # Parse parameters
        parameters = []
        for param_data in operation_data.get("parameters", []):
            param = self._parse_parameter(param_data)
            if param:
                parameters.append(param)

        # Parse responses
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
        )

    def _parse_parameter(self, param_data: Dict[str, Any]) -> Optional[Parameter]:
        """Parse a parameter."""
        if "$ref" in param_data:
            # Resolve reference
            ref_path = param_data["$ref"].split("/")
            resolved = self.spec_data
            for part in ref_path[1:]:  # Skip '#'
                resolved = resolved.get(part, {})
            param_data = resolved

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

    def _parse_response(
        self,
        status_code: str,
        response_data: Dict[str, Any],
        operation_id: str,
    ) -> Response:
        """Parse a response."""
        content = response_data.get("content", {})
        content_types = list(content.keys())

        # Determine Rust type from content
        rust_type = None
        if content_types:
            # Use first content type for type determination
            first_content = content[content_types[0]]
            schema = first_content.get("schema", {})

            # Check if this is an inline schema that should become a response model
            if self._should_create_response_model(schema, status_code):
                # Create a response model name
                response_model_name = f"{operation_id}{status_code}Response"

                # Create a schema for this response and add to schemas collection
                self.schemas[response_model_name] = self._create_response_schema(
                    response_model_name,
                    schema,
                    response_data.get("description", ""),
                )

                # Use the model name as the rust type
                rust_type = pascal_case(response_model_name)
            else:
                rust_type = rust_type_from_openapi(schema, self.schemas, set())

        return Response(
            status_code=status_code,
            description=response_data.get("description", ""),
            rust_type=rust_type,
            content_types=content_types,
        )

    def _should_create_response_model(
        self,
        schema: Dict[str, Any],
        status_code: str,
    ) -> bool:
        """Determine if we should create a response model for this schema."""
        # Create response models for:
        # 1. Success responses (200-299) with object schemas
        # 2. Inline schemas with properties (not just $ref)
        if not status_code.startswith("2"):
            return False

        if "$ref" in schema:
            return False  # Already a reference to existing schema

        if schema.get("type") == "object" and "properties" in schema:
            return True

        # Also create for schemas with required fields or complex structure
        if "required" in schema or "allOf" in schema or "oneOf" in schema:
            return True

        return False

    def _create_response_schema(
        self,
        name: str,
        schema: Dict[str, Any],
        description: str,
    ) -> Dict[str, Any]:
        """Create a response schema from an inline schema."""
        # Return the schema as-is but with a description
        response_schema = schema.copy()
        if description and "description" not in response_schema:
            response_schema["description"] = description
        return response_schema

    def _parse_schemas(self) -> Dict[str, Schema]:
        """Parse all schemas."""
        schemas = {}

        for schema_name, schema_data in self.schemas.items():
            schema = self._parse_schema(schema_name, schema_data)
            if schema:
                schemas[schema_name] = schema

        return schemas

    def _parse_schema(self, name: str, schema_data: Dict[str, Any]) -> Optional[Schema]:
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

    def _extract_content_types(self) -> List[str]:
        """Extract all content types used in the API."""
        content_types = set()

        # Check operations for content types
        for path_item in self.spec_data.get("paths", {}).values():
            for operation in path_item.values():
                if isinstance(operation, dict):
                    # Request body content types
                    request_body = operation.get("requestBody", {})
                    content = request_body.get("content", {})
                    content_types.update(content.keys())

                    # Response content types
                    for response in operation.get("responses", {}).values():
                        response_content = response.get("content", {})
                        content_types.update(response_content.keys())

        return sorted(list(content_types))
