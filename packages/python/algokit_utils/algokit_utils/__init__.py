"""
AlgoKit Utils Library Python Bindings
"""


# Import all symbols from the Rust extension module and re-export them
from codecs import ignore_errors
from typing import override
from .algokit_utils_ffi import *
from . import algokit_transact_ffi as transact

# Definition of ABI types

class ABIBool(AbiType):
    def __init__(self):
        pass

    @override
    def to_string(self) -> str: # type: ignore
        return "bool"

class ABIArray(AbiType):
    def __init__(self, element_type: AbiType, length: int):
        self.element_type = element_type
        self.length = length

    @override
    def to_string(self) -> str: # type: ignore
        return f"{self.element_type.to_string()}[{self.length}]" # type: ignore
