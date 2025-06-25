#!/usr/bin/env python3
"""
CLI entry point for rust_oas_generator

This module provides the command-line interface for the Rust OpenAPI client generator.
"""

import argparse
import sys
from pathlib import Path
from typing import Optional

from rust_oas_generator.generator import RustCodeGenerator
from rust_oas_generator.parser import OASParser
from rust_oas_generator.utils.file_utils import write_files_to_disk


def parse_args(args: Optional[list] = None) -> argparse.Namespace:
    """Parse command line arguments."""
    parser = argparse.ArgumentParser(
        description="Generate Rust API Client from OpenAPI Specification",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  rust_oas_generator path/to/spec.json
  rust_oas_generator ../api/specs/algod.oas3.json
  rust_oas_generator --output-dir ./my_client spec.json
        """,
    )

    parser.add_argument(
        "spec_path",
        type=Path,
        help="Path to the OpenAPI specification file (JSON or YAML)",
    )

    parser.add_argument(
        "--output-dir",
        type=Path,
        default=Path.cwd() / "generated_rust_client",
        help="Output directory for generated client (default: ./generated_rust_client)",
    )

    parser.add_argument(
        "--package-name",
        default="algod_client",
        help="Name of the generated Rust package (default: algod_client)",
    )

    return parser.parse_args(args)


def main(args: Optional[list] = None) -> int:
    """Generate Rust client from OpenAPI specification."""
    parsed_args = parse_args(args)

    print("🦀 Generating Rust API Client...")

    # Validate spec path
    if not parsed_args.spec_path.exists():
        print(f"❌ OpenAPI specification not found at {parsed_args.spec_path}")
        return 1

    # Parse the OpenAPI specification
    print("📋 Parsing OpenAPI specification...")
    parser = OASParser()

    try:
        spec = parser.parse_file(parsed_args.spec_path)
        print(f"   ✅ Parsed {len(spec.operations)} operations")
        print(f"   ✅ Parsed {len(spec.schemas)} schemas")
        print(f"   ✅ Found {len(spec.content_types)} content types")
    except Exception as e:
        print(f"❌ Failed to parse specification: {e}")
        return 1

    # Initialize code generator
    print("🔧 Initializing Rust code generator...")
    generator = RustCodeGenerator()

    # Generate client code
    print("⚙️  Generating Rust client code...")

    try:
        files = generator.generate_client(
            spec, parsed_args.output_dir, parsed_args.package_name,
        )
        print(f"   ✅ Generated {len(files)} files")
    except Exception as e:
        print(f"❌ Failed to generate client: {e}")
        return 1

    # Write files to disk
    print("📝 Writing generated files...")
    try:
        write_files_to_disk(files)
        print(f"   ✅ Written {len(files)} files to disk")
    except Exception as e:
        print(f"❌ Failed to write files: {e}")
        return 1

    # Generate project structure summary
    print("📦 Generating project files...")

    print("✅ Successfully generated Rust API client!")
    print(f"📁 Output directory: {parsed_args.output_dir}")
    print(f"📊 Total files generated: {len(files)}")

    # Show generated structure
    print("📂 Generated structure:")
    for file_path in sorted(files.keys()):
        rel_path = Path(file_path).relative_to(parsed_args.output_dir)
        print(f"   {rel_path}")

    return 0


if __name__ == "__main__":
    sys.exit(main())
