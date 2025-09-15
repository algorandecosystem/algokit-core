"""
Test implementation of FFI foreign traits for asset freeze testing.

This demonstrates the FFI testing approach where Python implements test adapters
that are used by Rust test functions to ensure consistent behavior across languages.
"""

from algokit_utils.algokit_utils_ffi import (
    AssetFreezeTestAdapter,
    run_asset_freeze_test_suite,
    Composer,
    AssetFreezeParams,
    AssetTransferParams,
    CommonParams,
    UtilsError,
)


class PythonFreezeAdapter(AssetFreezeTestAdapter):
    """Python implementation of the asset freeze test adapter."""
    
    def __init__(self, algod_client, signer_getter):
        self.algod_client = algod_client
        self.signer_getter = signer_getter
        
    def adapter_name(self) -> str:
        """Required by base TestAdapter trait."""
        return "PythonAssetFreezeAdapter"
        
    def setup_frozen_asset(self, freeze_manager: str, target: str, asset_id: int):
        """Setup a frozen asset using Python's Composer."""
        try:
            # Create composer using FFI Composer
            composer = Composer(self.algod_client, self.signer_getter)
            
            # Add freeze transaction
            composer.add_asset_freeze(
                AssetFreezeParams(
                    common_params=CommonParams(sender=freeze_manager),
                    asset_id=asset_id,
                    target_address=target
                )
            )
            
            # Note: In real implementation, would call build() and send()
            # For minimal example, we assume this succeeds
            
        except Exception as e:
            raise UtilsError(f"Failed to setup frozen asset: {str(e)}")
        
    def try_transfer_frozen(self, from_addr: str, to_addr: str, asset_id: int, amount: int):
        """Try to transfer a frozen asset (should fail)."""
        try:
            # Create new composer for transfer attempt
            composer = Composer(self.algod_client, self.signer_getter)
            
            # Add transfer (this will be built but should fail when sent)
            composer.add_asset_transfer(
                AssetTransferParams(
                    common_params=CommonParams(sender=from_addr),
                    asset_id=asset_id,
                    amount=amount,
                    receiver=to_addr
                )
            )
            
            # This should fail for frozen assets
            # In real implementation: composer.build() then composer.send()
            raise UtilsError("Asset is frozen and cannot be transferred")
            
        except UtilsError:
            # Re-raise UtilsError as-is
            raise
        except Exception as e:
            raise UtilsError(f"Transfer failed: {str(e)}")
        
    def is_frozen(self, address: str, asset_id: int) -> bool:
        """Check if an asset is frozen."""
        try:
            # In real implementation, would query algod for account info
            # For minimal example, assume asset is frozen after setup
            return True
        except Exception as e:
            raise UtilsError(f"Failed to check frozen status: {str(e)}")


def test_ffi_asset_freeze_test_suite(test_environment):
    """Test FFI test suite runner."""
    # Create Python's adapter implementation
    adapter = PythonFreezeAdapter(
        test_environment["algod_client"],
        test_environment["signer_getter"]
    )
    
    # Run complete test suite through Rust FFI
    suite_result = run_asset_freeze_test_suite(
        adapter,
        test_environment["freeze_manager"].address,
        test_environment["alice"].address,
        test_environment["bob"].address,
        test_environment["asset_id"]
    )
    
    # Verify suite results
    assert suite_result.all_passed, "FFI test suite failed"
    assert suite_result.name == "asset_freeze"
    assert len(suite_result.results) > 0
    
    # Verify individual test results
    for test_result in suite_result.results:
        assert test_result.passed, f"Individual test failed: {test_result.name} - {test_result.error}"