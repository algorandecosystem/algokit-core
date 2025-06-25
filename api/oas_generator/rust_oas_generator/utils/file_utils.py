"""
File Utilities for Rust Client Generation

This module provides utilities for writing generated files to disk
and managing file operations.
"""

from pathlib import Path
from typing import Dict


def write_files_to_disk(files: Dict[str, str]) -> None:
    """Write generated files to disk."""
    for file_path, content in files.items():
        path = Path(file_path)

        # Create directory if it doesn't exist
        path.parent.mkdir(parents=True, exist_ok=True)

        # Write file content
        with open(path, "w", encoding="utf-8") as f:
            f.write(content)


def ensure_directory(directory: Path) -> None:
    """Ensure directory exists."""
    directory.mkdir(parents=True, exist_ok=True)


def clean_directory(directory: Path) -> None:
    """Clean directory by removing all files."""
    if directory.exists():
        import shutil
        shutil.rmtree(directory)
    directory.mkdir(parents=True, exist_ok=True)

