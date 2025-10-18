"""
Python implementation of ComposerTrait and ComposerFactory foreign traits.

The PythonComposer wraps the concrete Rust Composer (FFI object) to provide
async trait compatibility for the foreign trait testing pattern.

STATEFUL DESIGN: PythonComposer stores algod_client and signer_getter internally,
eliminating the need to pass them on every method call.
"""

from algokit_utils import Composer, ComposerFactory
class PythonComposerFactory(ComposerFactory):
    """Python implementation of ComposerFactory that creates fresh composer instances"""

    def __init__(self, algod_client, signer_getter):
        """
        Args:
            algod_client: The concrete AlgodClient FFI object
            signer_getter: The TransactionSignerGetter implementation
        """
        self.algod_client = algod_client
        self.signer_getter = signer_getter

    def create_composer(self) -> Composer:
        """Create a fresh composer instance with stored dependencies"""
        return Composer(self.algod_client, self.signer_getter)
