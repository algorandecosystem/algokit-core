"""
String case conversion utilities for Rust client generation.

Based on https://github.com/okunishinishi/python-stringcase
with additional Rust-specific naming conventions.
"""

import re


def camelcase(string: object) -> str:
    """Convert string into camel case.

    Args:
        string: String to convert.

    Returns:
        string: Camel case string.

    """
    string_val = str(string)
    if string_val == "":
        return string_val

    string_val = string_val.replace("_", "-")
    lst = string_val.split("-")
    for i in range(len(lst)):
        if i == 0:
            continue
        lst[i] = lst[i].capitalize()

    return "".join(lst)


def capitalcase(string: object) -> str:
    """Convert string into capital case.
    First letters will be uppercase.

    Args:
        string: String to convert.

    Returns:
        string: Capital case string.

    """
    string = str(string)
    if not string:
        return string
    return uppercase(string[0]) + string[1:]


def constcase(string: object) -> str:
    """Convert string into upper snake case.
    Join punctuation with underscore and convert letters into uppercase.

    Args:
        string: String to convert.

    Returns:
        string: Const cased string.

    """
    return uppercase(snakecase(string))


def lowercase(string: object) -> str:
    """Convert string into lower case.

    Args:
        string: String to convert.

    Returns:
        string: Lowercase case string.

    """
    return str(string).lower()


def pascalcase(string: object) -> str:
    """Convert string into pascal case.

    Args:
        string: String to convert.

    Returns:
        string: Pascal case string.

    """
    return capitalcase(camelcase(string))


def snakecase(string: object) -> str:
    """Convert string into snake case.
    Join punctuation with underscore

    Args:
        string: String to convert.

    Returns:
        string: Snake cased string.

    """
    string = re.sub(r"[\-\.\s]", "_", str(string))
    if not string:
        return string
    return lowercase(string[0]) + re.sub(
        r"[A-Z]",
        lambda matched: "_" + lowercase(matched.group(0)),
        string[1:],
    )


def spinalcase(string: object) -> str:
    """Convert string into spinal case.
    Join punctuation with hyphen.

    Args:
        string: String to convert.

    Returns:
        string: Spinal cased string.

    """
    return re.sub(r"_", "-", snakecase(string))


def titlecase(string: object) -> str:
    """Convert string into sentence case.
    First letter capped while each punctuations is capitalised
    and joined with space.

    Args:
        string: String to convert.

    Returns:
        string: Title cased string.

    """
    return " ".join([capitalcase(word) for word in snakecase(string).split("_")])


def trimcase(string: object) -> str:
    """Convert string into trimmed string.

    Args:
        string: String to convert.

    Returns:
        string: Trimmed case string
    """
    return str(string).strip()


def uppercase(string: object) -> str:
    """Convert string into upper case.

    Args:
        string: String to convert.

    Returns:
        string: Uppercase case string.

    """
    return str(string).upper()


def alphanumcase(string: object) -> str:
    """Cuts all non-alphanumeric symbols,
    i.e. cuts all expect except 0-9, a-z and A-Z.

    Args:
        string: String to convert.

    Returns:
        string: String with cutted non-alphanumeric symbols.

    """
    return "".join(filter(str.isalnum, str(string)))


# Rust-specific naming utilities


def rust_snake_case(name: str) -> str:
    """Convert string to snake_case for Rust naming (fields, functions, etc.)."""
    return snakecase(name)


def rust_pascal_case(name: str) -> str:
    """Convert string to PascalCase for Rust types (structs, enums, traits)."""
    if not name:
        return name

    name = re.sub(r"[^\w\-]", "_", str(name))
    parts = re.split(
        r"[-_]+|(?<=[a-z])(?=[A-Z])|(?<=[0-9])(?=[A-Z])|(?<=[A-Z])(?=[0-9])",
        name,
    )

    capitalized_parts = []
    for part in parts:
        if part:
            if part.isdigit():
                capitalized_parts.append(part)
            elif part[0].isdigit():
                digits = ""
                letters = ""
                for char in part:
                    if char.isdigit():
                        digits += char
                    else:
                        letters += char
                if letters:
                    capitalized_parts.append(digits + letters.capitalize())
                else:
                    capitalized_parts.append(digits)
            else:
                capitalized_parts.append(part.capitalize())

    return "".join(capitalized_parts)


def rust_const_case(name: str) -> str:
    """Convert string to CONST_CASE for Rust constants."""
    return constcase(name)


def normalize_rust_identifier(name: str) -> str:
    """Normalize name for Rust identifiers."""
    normalized = re.sub(r"[^a-zA-Z0-9_]", "_", str(name))
    if normalized and normalized[0].isdigit():
        normalized = f"_{normalized}"
    return normalized


# Reserved Rust keywords that need to be escaped with r#
RUST_KEYWORDS = {
    "as",
    "break",
    "const",
    "continue",
    "crate",
    "else",
    "enum",
    "extern",
    "false",
    "fn",
    "for",
    "if",
    "impl",
    "in",
    "let",
    "loop",
    "match",
    "mod",
    "move",
    "mut",
    "pub",
    "ref",
    "return",
    "self",
    "Self",
    "static",
    "struct",
    "super",
    "trait",
    "true",
    "type",
    "unsafe",
    "use",
    "where",
    "while",
    "async",
    "await",
    "dyn",
    "abstract",
    "become",
    "box",
    "do",
    "final",
    "macro",
    "override",
    "priv",
    "typeof",
    "unsized",
    "virtual",
    "yield",
    "try",
    "union",
    "'static",
}


def escape_rust_keyword(name: str) -> str:
    """Escape Rust keywords with r# prefix if necessary."""
    return f"r#{name}" if name in RUST_KEYWORDS else name
