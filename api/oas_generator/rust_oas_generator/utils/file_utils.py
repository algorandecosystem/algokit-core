"""
File utilities for the OAS generator.
"""

import shutil
from pathlib import Path


def write_files_to_disk(files: dict[str, str]) -> None:
    """Write generated files to disk."""
    for file_path, content in files.items():
        path = Path(file_path)
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(content, encoding="utf-8")


def clean_output_directory(output_dir: Path) -> None:
    """Clean the output directory by removing all files and subdirectories."""
    if output_dir.exists():
        shutil.rmtree(output_dir)
    output_dir.mkdir(parents=True, exist_ok=True)


def copy_file(src: Path, dest: Path) -> None:
    """Copy a file from source to destination."""
    dest.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(src, dest)


def ensure_directory(directory: Path) -> None:
    """Ensure that a directory exists."""
    directory.mkdir(parents=True, exist_ok=True)


def get_relative_path(file_path: Path, base_path: Path) -> Path:
    """Get relative path from base_path to file_path."""
    try:
        return file_path.relative_to(base_path)
    except ValueError:
        return file_path


def list_rust_files(directory: Path) -> list[Path]:
    """List all .rs files in a directory recursively."""
    rust_files = []
    if directory.exists():
        for file_path in directory.rglob("*.rs"):
            rust_files.append(file_path)
    return sorted(rust_files)
