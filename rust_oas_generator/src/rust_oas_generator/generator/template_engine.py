"""
Rust Template Engine for OpenAPI Client Generation

This module uses Jinja2 templates to generate Rust API client code
from parsed OpenAPI specifications.
"""

from pathlib import Path
from typing import Any, Dict, List, Optional

from jinja2 import Environment, FileSystemLoader, select_autoescape

from rust_oas_generator.generator.filters import FILTERS
from rust_oas_generator.parser.oas_parser import (
    Operation,
    Parameter,
    ParsedSpec,
    Schema,
    normalize_name,
    pascal_case,
    rust_type_from_openapi,
    snake_case,
)


class RustTemplateEngine:
    """Template engine for generating Rust code."""

    def __init__(self, template_dir: Optional[Path] = None):
        """Initialize the template engine."""
        if template_dir is None:
            template_dir = Path(__file__).parent.parent / "templates"

        self.template_dir = template_dir
        self.env = Environment(
            loader=FileSystemLoader(str(template_dir)),
            autoescape=select_autoescape(["html", "xml"]),
            trim_blocks=True,
            lstrip_blocks=True,
        )

        self._register_filters()
        self._register_globals()

    def _register_filters(self):
        """Register custom Jinja2 filters for Rust code generation."""
        self.env.filters["snake_case"] = snake_case
        self.env.filters["pascal_case"] = pascal_case
        self.env.filters["normalize_name"] = normalize_name
        self.env.filters["rust_type"] = lambda schema, schemas: rust_type_from_openapi(
            schema,
            schemas,
        )
        self.env.filters["rust_doc_comment"] = self._rust_doc_comment
        self.env.filters["rust_string_literal"] = self._rust_string_literal
        self.env.filters["rust_optional"] = self._rust_optional
        self.env.filters["rust_vec"] = self._rust_vec

        # Register custom filters from filters module
        for name, filter_func in FILTERS.items():
            self.env.filters[name] = filter_func

    def _register_globals(self):
        """Register global functions available in templates."""
        self.env.globals["get_unique_tags"] = self._get_unique_tags
        self.env.globals["group_operations_by_tag"] = self._group_operations_by_tag
        self.env.globals["get_error_types"] = self._get_error_types
        self.env.globals["get_success_response_type"] = self._get_success_response_type
        self.env.globals["get_all_response_types"] = self._get_all_response_types
        self.env.globals["get_endpoint_response_types"] = (
            self._get_endpoint_response_types
        )
        self.env.globals["has_path_parameters"] = self._has_path_parameters
        self.env.globals["has_query_parameters"] = self._has_query_parameters
        self.env.globals["get_path_parameters"] = self._get_path_parameters
        self.env.globals["get_query_parameters"] = self._get_query_parameters
        self.env.globals["has_request_body"] = self._has_request_body
        self.env.globals["get_request_body_type"] = self._get_request_body_type
        self.env.globals["get_request_body_name"] = self._get_request_body_name
        self.env.globals["is_request_body_required"] = self._is_request_body_required
        self.env.globals["has_header_parameters"] = self._has_header_parameters
        self.env.globals["get_header_parameters"] = self._get_header_parameters
        self.env.globals["should_import_request_body_type"] = (
            self._should_import_request_body_type
        )
        self.env.globals["get_all_used_types"] = self._get_all_used_types
        self.env.globals["get_operation_used_types"] = self._get_operation_used_types

    def render_template(self, template_name: str, context: Dict[str, Any]) -> str:
        """Render a template with the given context."""
        template = self.env.get_template(template_name)
        return template.render(**context)

    def _rust_doc_comment(self, text: str, indent: int = 0) -> str:
        """Format text as Rust doc comment."""
        if not text:
            return ""

        lines = text.strip().split("\n")
        prefix = " " * indent + "/// "
        return "\n".join(prefix + line.strip() for line in lines)

    def _rust_string_literal(self, text: str) -> str:
        """Format text as Rust string literal."""
        # Escape quotes and backslashes
        escaped = text.replace("\\", "\\\\").replace('"', '\\"')
        return f'"{escaped}"'

    def _rust_optional(self, rust_type: str) -> str:
        """Wrap Rust type in Option if not already optional."""
        if rust_type.startswith("Option<"):
            return rust_type
        return f"Option<{rust_type}>"

    def _rust_vec(self, rust_type: str) -> str:
        """Wrap Rust type in Vec."""
        return f"Vec<{rust_type}>"

    def _get_unique_tags(self, operations: List[Operation]) -> List[str]:
        """Get unique tags from operations."""
        tags = set()
        for operation in operations:
            tags.update(operation.tags)
        return sorted(list(tags))

    def _group_operations_by_tag(
        self,
        operations: List[Operation],
    ) -> Dict[str, List[Operation]]:
        """Group operations by their first tag."""
        groups = {}
        for operation in operations:
            tag = operation.tags[0] if operation.tags else "default"
            if tag not in groups:
                groups[tag] = []
            groups[tag].append(operation)
        return groups

    def _get_error_types(self, operation: Operation) -> List[str]:
        """Get error response types for an operation."""
        error_types = []
        for status_code, response in operation.responses.items():
            if (
                status_code.startswith("4")
                or status_code.startswith("5")
                or status_code == "default"
            ):
                if response.rust_type:
                    error_types.append(f"Status{status_code}({response.rust_type})")
                else:
                    error_types.append(f"Status{status_code}()")

        # Always add default and unknown value variants
        if not any("DefaultResponse" in t for t in error_types):
            error_types.append("DefaultResponse()")
        error_types.append("UnknownValue(serde_json::Value)")

        return error_types

    def _get_success_response_type(self, operation: Operation) -> Optional[str]:
        """Get the success response type for an operation."""
        for status_code, response in operation.responses.items():
            if status_code.startswith("2"):  # 2xx success codes
                return response.rust_type
        return None

    def _get_all_response_types(self, operations: List[Operation]) -> List[str]:
        """Get all unique response types used across operations."""
        response_types = set()
        for operation in operations:
            for status_code, response in operation.responses.items():
                if (
                    status_code.startswith("2") and response.rust_type
                ):  # 2xx success codes
                    # Only include types that end with "Response" (our generated response types)
                    if response.rust_type.endswith("Response"):
                        response_types.add(response.rust_type)
        return sorted(list(response_types))

    def _get_endpoint_response_types(self, operation: Operation) -> List[str]:
        """Get response types for a single endpoint."""
        response_types = set()
        for status_code, response in operation.responses.items():
            if status_code.startswith("2") and response.rust_type:  # 2xx success codes
                # Only include types that end with "Response" (our generated response types)
                if response.rust_type.endswith("Response"):
                    response_types.add(response.rust_type)
        return sorted(list(response_types))

    def _has_path_parameters(self, operation: Operation) -> bool:
        """Check if operation has path parameters."""
        return any(p.param_type == "path" for p in operation.parameters)

    def _has_query_parameters(self, operation: Operation) -> bool:
        """Check if operation has query parameters."""
        return any(p.param_type == "query" for p in operation.parameters)

    def _get_path_parameters(self, operation: Operation) -> List[Parameter]:
        """Get path parameters for an operation."""
        return [p for p in operation.parameters if p.param_type == "path"]

    def _get_query_parameters(self, operation: Operation) -> List[Parameter]:
        """Get query parameters for an operation."""
        return [p for p in operation.parameters if p.param_type == "query"]

    def _has_request_body(self, operation: Operation) -> bool:
        """Check if operation has a request body."""
        return operation.request_body is not None

    def _get_request_body_type(self, operation: Operation) -> Optional[str]:
        """Get the request body type for an operation."""
        if not operation.request_body:
            return None

        # Get the first content type (usually application/json)
        content = operation.request_body.get("content", {})
        if not content:
            return None

        first_content_type = next(iter(content.keys()))
        schema = content[first_content_type].get("schema", {})

        # Handle $ref schemas
        if "$ref" in schema:
            ref_name = schema["$ref"].split("/")[-1]
            return pascal_case(ref_name)

        # Handle inline schemas (fallback to generic type)
        return rust_type_from_openapi(schema, {})

    def _get_request_body_name(self, operation: Operation) -> Optional[str]:
        """Get the request body parameter name for an operation."""
        if not operation.request_body:
            return None
        return "request"  # Standard name for request body parameter

    def _is_request_body_required(self, operation: Operation) -> bool:
        """Check if the request body is required for an operation."""
        if not operation.request_body:
            return False
        return operation.request_body.get("required", False)

    def _has_header_parameters(self, operation: Operation) -> bool:
        """Check if operation has header parameters."""
        return any(p.param_type == "header" for p in operation.parameters)

    def _get_header_parameters(self, operation: Operation) -> List[Parameter]:
        """Get header parameters for an operation."""
        return [p for p in operation.parameters if p.param_type == "header"]

    def _should_import_request_body_type(self, request_body_type: str) -> bool:
        """Check if a request body type is a custom model that needs to be imported."""
        if not request_body_type:
            return False

        # Don't import primitive types
        primitive_types = {
            "String",
            "i32",
            "i64",
            "u32",
            "u64",
            "f32",
            "f64",
            "bool",
            "Vec<u8>",
            "Vec<String>",
            "Vec<i32>",
            "Vec<i64>",
            "serde_json::Value",
            "std::path::PathBuf",
        }

        # Check if it's a primitive type or contains generic brackets
        if request_body_type in primitive_types or "<" in request_body_type:
            return False

        # If it's a PascalCase identifier without generics, it's likely a custom model
        return request_body_type[0].isupper() and request_body_type.isalnum()

    def _get_all_used_types(self, operations: List[Operation]) -> List[str]:
        """Get all unique custom types used across operations for imports."""
        used_types = set()

        # Primitive types that don't need imports
        primitive_types = {
            "String",
            "str",
            "i32",
            "i64",
            "u32",
            "u64",
            "f32",
            "f64",
            "bool",
            "Vec<u8>",
            "Vec<String>",
            "Vec<i32>",
            "Vec<i64>",
            "serde_json::Value",
            "std::path::PathBuf",
            "()",
            "Box",
        }

        for operation in operations:
            # Get success response types
            for status_code, response in operation.responses.items():
                if status_code.startswith("2") and response.rust_type:
                    # Extract base type from Vec<Type> or Option<Type>
                    response_type = response.rust_type
                    if response_type.startswith("Vec<") and response_type.endswith(">"):
                        response_type = response_type[4:-1]  # Remove Vec< and >
                    if response_type.startswith("Option<") and response_type.endswith(
                        ">"
                    ):
                        response_type = response_type[7:-1]  # Remove Option< and >

                    if response_type not in primitive_types:
                        used_types.add(response_type)

            # Get error response types
            for status_code, response in operation.responses.items():
                if (
                    status_code.startswith("4")
                    or status_code.startswith("5")
                    or status_code == "default"
                ) and response.rust_type:
                    # Extract base type from Vec<Type> or Option<Type>
                    response_type = response.rust_type
                    if response_type.startswith("Vec<") and response_type.endswith(">"):
                        response_type = response_type[4:-1]  # Remove Vec< and >
                    if response_type.startswith("Option<") and response_type.endswith(
                        ">"
                    ):
                        response_type = response_type[7:-1]  # Remove Option< and >

                    if response_type not in primitive_types:
                        used_types.add(response_type)

            # Get parameter types
            for param in operation.parameters:
                # Extract base type from Vec<Type> or Option<Type>
                param_type = param.rust_type
                if param_type.startswith("Vec<") and param_type.endswith(">"):
                    param_type = param_type[4:-1]  # Remove Vec< and >
                if param_type.startswith("Option<") and param_type.endswith(">"):
                    param_type = param_type[7:-1]  # Remove Option< and >

                if param_type not in primitive_types:
                    used_types.add(param_type)

        return sorted(list(used_types))

    def _get_operation_used_types(self, operation: Operation) -> List[str]:
        """Get all unique custom types used by a single operation for imports."""
        used_types = set()

        # Primitive types that don't need imports
        primitive_types = {
            "String",
            "str",
            "i32",
            "i64",
            "u32",
            "u64",
            "f32",
            "f64",
            "bool",
            "Vec<u8>",
            "Vec<String>",
            "Vec<i32>",
            "Vec<i64>",
            "serde_json::Value",
            "std::path::PathBuf",
            "()",
            "Box",
        }

        # Get success response types
        for status_code, response in operation.responses.items():
            if status_code.startswith("2") and response.rust_type:
                # Extract base type from Vec<Type> or Option<Type>
                response_type = response.rust_type
                if response_type.startswith("Vec<") and response_type.endswith(">"):
                    response_type = response_type[4:-1]  # Remove Vec< and >
                if response_type.startswith("Option<") and response_type.endswith(">"):
                    response_type = response_type[7:-1]  # Remove Option< and >

                if response_type not in primitive_types:
                    used_types.add(response_type)

        # Get error response types
        for status_code, response in operation.responses.items():
            if (
                status_code.startswith("4")
                or status_code.startswith("5")
                or status_code == "default"
            ) and response.rust_type:
                # Extract base type from Vec<Type> or Option<Type>
                response_type = response.rust_type
                if response_type.startswith("Vec<") and response_type.endswith(">"):
                    response_type = response_type[4:-1]  # Remove Vec< and >
                if response_type.startswith("Option<") and response_type.endswith(">"):
                    response_type = response_type[7:-1]  # Remove Option< and >

                if response_type not in primitive_types:
                    used_types.add(response_type)

        # Get parameter types
        for param in operation.parameters:
            # Extract base type from Vec<Type> or Option<Type>
            param_type = param.rust_type
            if param_type.startswith("Vec<") and param_type.endswith(">"):
                param_type = param_type[4:-1]  # Remove Vec< and >
            if param_type.startswith("Option<") and param_type.endswith(">"):
                param_type = param_type[7:-1]  # Remove Option< and >

            if param_type not in primitive_types:
                used_types.add(param_type)

        return sorted(list(used_types))


class RustCodeGenerator:
    """Main code generator for Rust clients."""

    def __init__(self, template_engine: Optional[RustTemplateEngine] = None):
        """Initialize the code generator."""
        self.template_engine = template_engine or RustTemplateEngine()

    def generate_client(
        self,
        spec: ParsedSpec,
        output_dir: Path,
        package_name: str = "api_client",
    ) -> Dict[str, str]:
        """Generate complete Rust client from OpenAPI spec."""
        output_dir = Path(output_dir)
        files = {}

        # Create context for templates
        context = {
            "spec": spec,
            "package_name": package_name,
            "operations": spec.operations,
            "schemas": spec.schemas,
            "content_types": spec.content_types,
        }

        # Generate base files
        files.update(self._generate_base_files(context, output_dir))

        # Generate model files
        files.update(self._generate_model_files(spec.schemas, context, output_dir))

        # Generate API files
        files.update(self._generate_api_files(spec.operations, context, output_dir))

        # Generate project files
        files.update(self._generate_project_files(context, output_dir))

        return files

    def _generate_base_files(
        self,
        context: Dict[str, Any],
        output_dir: Path,
    ) -> Dict[str, str]:
        """Generate base library files."""
        files = {}
        src_dir = output_dir / "src"

        # lib.rs
        content = self.template_engine.render_template("base/lib.rs.j2", context)
        files[str(src_dir / "lib.rs")] = content

        # Configuration
        content = self.template_engine.render_template(
            "base/configuration.rs.j2",
            context,
        )
        files[str(src_dir / "apis" / "configuration.rs")] = content

        return files

    def _generate_model_files(
        self,
        schemas: Dict[str, Schema],
        context: Dict[str, Any],
        output_dir: Path,
    ) -> Dict[str, str]:
        """Generate model files."""
        files = {}
        models_dir = output_dir / "src" / "models"

        # Individual model files
        for schema_name, schema in schemas.items():
            model_context = {**context, "schema": schema}
            content = self.template_engine.render_template(
                "models/model.rs.j2",
                model_context,
            )
            snake_case_name = snake_case(schema_name)
            # Handle Rust reserved keywords
            if snake_case_name in [
                "box",
                "type",
                "match",
                "fn",
                "let",
                "use",
                "mod",
                "struct",
                "enum",
                "impl",
                "trait",
                "true",
                "false",
                "if",
                "else",
                "while",
                "for",
                "loop",
                "break",
                "continue",
                "return",
            ]:
                filename = f"model_{snake_case_name}.rs"
            else:
                filename = f"{snake_case_name}.rs"
            files[str(models_dir / filename)] = content

        # Models mod.rs
        models_context = {**context, "schemas": schemas}
        content = self.template_engine.render_template(
            "models/mod.rs.j2",
            models_context,
        )
        files[str(models_dir / "mod.rs")] = content

        return files

    def _generate_api_files(
        self,
        operations: List[Operation],
        context: Dict[str, Any],
        output_dir: Path,
    ) -> Dict[str, str]:
        """Generate individual API files per endpoint."""
        files = {}
        apis_dir = output_dir / "src" / "apis"

        # Generate individual endpoint files
        for operation in operations:
            endpoint_context = {**context, "operation": operation}
            content = self.template_engine.render_template(
                "apis/endpoint.rs.j2", endpoint_context
            )
            files[str(apis_dir / f"{operation.rust_function_name}.rs")] = content

        # Generate mod.rs file that includes all endpoints
        api_context = {**context, "operations": operations}
        content = self.template_engine.render_template("apis/mod.rs.j2", api_context)
        files[str(apis_dir / "mod.rs")] = content

        return files

    def _generate_project_files(
        self,
        context: Dict[str, Any],
        output_dir: Path,
    ) -> Dict[str, str]:
        """Generate project configuration files."""
        files = {}

        # Cargo.toml
        content = self.template_engine.render_template("base/Cargo.toml.j2", context)
        files[str(output_dir / "Cargo.toml")] = content

        # README.md
        content = self.template_engine.render_template("base/README.md.j2", context)
        files[str(output_dir / "README.md")] = content

        return files
