"""
AlgoKit Utils Library Python Bindings
"""


# Import all symbols from the Rust extension module and re-export them
from codecs import ignore_errors
from typing import override
from .algokit_utils_ffi import *
from . import algokit_transact_ffi as transact

def abi_init(self, *, abi_type: "str"):
    self.abi_type = normalize_abi_type(abi_type)

AbiType.__init__ = abi_init  # type: ignore
