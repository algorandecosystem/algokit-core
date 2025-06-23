"""
OpenAPI Parser Module for Rust Client Generation

This module provides parsing capabilities for OpenAPI specifications
to extract information needed for Rust client generation.
"""

from .oas_parser import (
    OASParser,
    Operation,
    Parameter,
    ParsedSpec,
    Property,
    Response,
    Schema,
    normalize_name,
    pascal_case,
    rust_type_from_openapi,
    snake_case,
)

__all__ = [
    "OASParser",
    "Operation",
    "Parameter",
    "ParsedSpec",
    "Property",
    "Response",
    "Schema",
    "normalize_name",
    "pascal_case",
    "rust_type_from_openapi",
    "snake_case",
]
