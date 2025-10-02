"""
Python implementation of ComposerTrait and ComposerFactory foreign traits.

The PythonComposer wraps the concrete Rust Composer (FFI object) to provide
async trait compatibility for the foreign trait testing pattern.
"""

from typing import List


class PythonComposer:
    """Python implementation wrapping the concrete Rust Composer for async trait compatibility"""

    def __init__(self, ffi_composer):
        """
        Args:
            ffi_composer: The concrete Composer FFI object from Rust
        """
        self._composer = ffi_composer

    async def build(self, algod_client) -> None:
        """Build the composed transactions"""
        await self._composer.build()

    async def send(self, algod_client) -> List[str]:
        """Send the composed transactions"""
        result = await self._composer.send()
        return result.transaction_ids

    async def add_payment(self, params, algod_client, signer_getter) -> None:
        """Add payment transaction to composition"""
        self._composer.add_payment(params)

    async def add_asset_create(self, params, algod_client, signer_getter) -> None:
        """Add asset create transaction to composition"""
        self._composer.add_asset_create(params)

    async def add_asset_reconfigure(self, params, algod_client, signer_getter) -> None:
        """Add asset reconfigure transaction to composition"""
        self._composer.add_asset_reconfigure(params)

    async def add_asset_destroy(self, params, algod_client, signer_getter) -> None:
        """Add asset destroy transaction to composition"""
        self._composer.add_asset_destroy(params)

    async def add_asset_freeze(self, params, algod_client, signer_getter) -> None:
        """Add asset freeze transaction to composition"""
        self._composer.add_asset_freeze(params)

    async def add_asset_unfreeze(self, params, algod_client, signer_getter) -> None:
        """Add asset unfreeze transaction to composition"""
        self._composer.add_asset_unfreeze(params)

    async def add_asset_transfer(self, params, algod_client, signer_getter) -> None:
        """Add asset transfer transaction to composition"""
        self._composer.add_asset_transfer(params)

    async def add_asset_opt_in(self, params, algod_client, signer_getter) -> None:
        """Add asset opt-in transaction to composition"""
        self._composer.add_asset_opt_in(params)

    async def add_asset_opt_out(self, params, algod_client, signer_getter) -> None:
        """Add asset opt-out transaction to composition"""
        self._composer.add_asset_opt_out(params)

    async def add_asset_clawback(self, params, algod_client, signer_getter) -> None:
        """Add asset clawback transaction to composition"""
        self._composer.add_asset_clawback(params)

    async def add_app_create(self, params, algod_client, signer_getter) -> None:
        """Add app create transaction to composition"""
        self._composer.add_app_create(params)

    async def add_app_call(self, params, algod_client, signer_getter) -> None:
        """Add app call transaction to composition"""
        self._composer.add_app_call(params)

    async def add_app_update(self, params, algod_client, signer_getter) -> None:
        """Add app update transaction to composition"""
        self._composer.add_app_update(params)

    async def add_app_delete(self, params, algod_client, signer_getter) -> None:
        """Add app delete transaction to composition"""
        self._composer.add_app_delete(params)

    async def add_app_call_method_call(self, params, algod_client, signer_getter) -> None:
        """Add app call method call transaction to composition"""
        self._composer.add_app_call_method_call(params)

    async def add_app_create_method_call(self, params, algod_client, signer_getter) -> None:
        """Add app create method call transaction to composition"""
        self._composer.add_app_create_method_call(params)

    async def add_app_update_method_call(self, params, algod_client, signer_getter) -> None:
        """Add app update method call transaction to composition"""
        self._composer.add_app_update_method_call(params)

    async def add_app_delete_method_call(self, params, algod_client, signer_getter) -> None:
        """Add app delete method call transaction to composition"""
        self._composer.add_app_delete_method_call(params)


class PythonComposerFactory:
    """Python implementation of ComposerFactory that creates fresh composer instances"""

    def __init__(self, algod_client, signer_getter):
        """
        Args:
            algod_client: The concrete AlgodClient FFI object
            signer_getter: The TransactionSignerGetter implementation
        """
        self.algod_client = algod_client
        self.signer_getter = signer_getter

    def create_composer(self):
        """Create a fresh composer instance wrapped in PythonComposer"""
        # Import here to avoid circular dependency
        from algokit_utils.algokit_utils_ffi import Composer

        # Create a new FFI Composer instance (concrete Rust object)
        ffi_composer = Composer(self.algod_client, self.signer_getter)
        # Wrap it in our Python trait implementation
        return PythonComposer(ffi_composer)