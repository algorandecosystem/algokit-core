#!/usr/bin/env python3
"""Command-line interface for the Rust OAS Generator."""

import argparse
import json
import sys
import traceback
from pathlib import Path

from rust_oas_generator.generator.template_engine import RustCodeGenerator
from rust_oas_generator.parser.oas_parser import OASParser
from rust_oas_generator.utils.file_utils import write_files_to_disk


def parse_args(args: list[str] | None = None) -> argparse.Namespace:
    """Parse command line arguments."""
    parser = argparse.ArgumentParser(
        description="Generate Rust client from OpenAPI specification",
    )
    parser.add_argument(
        "spec",
        type=Path,
        help="Path to OpenAPI specification file (JSON or YAML)",
    )
    parser.add_argument(
        "--output",
        "-o",
        type=Path,
        default=Path("./generated"),
        help="Output directory for generated files (default: ./generated)",
    )
    parser.add_argument(
        "--package-name",
        "-p",
        default="api_client",
        help="Name for the generated Rust package (default: api_client)",
    )
    parser.add_argument(
        "--template-dir",
        "-t",
        type=Path,
        help="Custom template directory (optional)",
    )
    parser.add_argument(
        "--verbose",
        "-v",
        action="store_true",
        help="Enable verbose output",
    )
    return parser.parse_args(args)


def main(args: list[str] | None = None) -> int:
    """Generate Rust client from OpenAPI specification."""
    parsed_args = parse_args(args)

    try:
        # Parse OpenAPI specification
        parser = OASParser()
        spec = parser.parse_file(parsed_args.spec)

        if parsed_args.verbose:
            print(f"Parsed {len(spec.operations)} operations")
            print(f"Found {len(spec.schemas)} schemas")

        # Generate Rust client
        generator = RustCodeGenerator()
        files = generator.generate_client(
            spec,
            parsed_args.output,
            parsed_args.package_name,
        )

        # Write files to disk
        write_files_to_disk(files)

        if parsed_args.verbose:
            print(f"Generated {len(files)} files:")
            for file_path in sorted(files.keys()):
                print(f"  {file_path}")

        print(f"Rust client generated successfully in {parsed_args.output}")
        return 0

    except FileNotFoundError:
        print(f"Error: Specification file not found: {parsed_args.spec}")
        return 1
    except json.JSONDecodeError as e:
        print(f"Error: Invalid JSON in specification file: {e}")
        return 1
    except Exception as e:
        print(f"Error: {e}")
        if parsed_args.verbose:
            traceback.print_exc()
        return 1


if __name__ == "__main__":
    sys.exit(main())
