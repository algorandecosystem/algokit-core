"""Asset freeze workflow tests for algokit_utils FFI."""

import pytest

from algokit_utils import AlgorandClient, SigningAccount
from algokit_utils.algokit_utils_ffi import (
    AssetFreezeParams,
    AssetOptInParams,
    AssetTransferParams,
    CommonParams,
)
from algokit_utils.transactions.transaction_composer import AssetCreateParams
from .test_ffi_utils import (
    create_ffi_composer,
    get_account_asset_info,
    get_asset_balance,
    get_asset_id_from_result,
    setup_account_with_assets,
)


class TestAssetFreezeWorkflows:
    """Comprehensive tests for asset freeze operations."""
    
    @pytest.mark.asyncio
    async def test_freeze_unfreeze_workflow(self, test_environment, test_asset):
        """Test freezing and unfreezing asset holdings."""
        env = test_environment
        asset_id = test_asset['asset_id']
        
        # Setup: Alan has some assets
        setup_account_with_assets(
            env['algorand'], 
            env['alan'], 
            asset_id, 
            50_000, 
            env['creator']
        )
        
        # Step 1: Freeze Alan's assets
        composer = create_ffi_composer(
            env['algorand'],
            {
                'freeze_manager': env['freeze_manager'],
            }
        )
        
        composer.add_asset_freeze(
            params=AssetFreezeParams(
                common_params=CommonParams(sender=env['freeze_manager'].address),
                asset_id=asset_id,
                target_address=env['alan'].address,
                frozen=True,  # Freeze the assets
            )
        )
        
        await composer.build()
        await composer.send()
        
        # Verify Alan's assets are frozen
        account_info = get_account_asset_info(env['algorand'], env['alan'].address, asset_id)
        assert account_info.frozen == True
        
        # Step 2: Try to transfer frozen assets (should fail)
        with pytest.raises(Exception, match="asset is frozen|frozen"):
            composer = create_ffi_composer(
                env['algorand'],
                {
                    'alan': env['alan'],
                    'bianca': env['bianca'],
                }
            )
            
            # Bianca opts in first
            composer.add_asset_opt_in(
                params=AssetOptInParams(
                    common_params=CommonParams(sender=env['bianca'].address),
                    asset_id=asset_id,
                )
            )
            
            # Alan tries to transfer (will fail)
            composer.add_asset_transfer(
                params=AssetTransferParams(
                    common_params=CommonParams(sender=env['alan'].address),
                    asset_id=asset_id,
                    amount=10_000,
                    receiver=env['bianca'].address,
                )
            )
            
            await composer.build()
            await composer.send()
        
        # Step 3: Unfreeze Alan's assets
        composer = create_ffi_composer(
            env['algorand'],
            {
                'freeze_manager': env['freeze_manager'],
            }
        )
        
        composer.add_asset_freeze(
            params=AssetFreezeParams(
                common_params=CommonParams(sender=env['freeze_manager'].address),
                asset_id=asset_id,
                target_address=env['alan'].address,
                frozen=False,  # Unfreeze the assets
            )
        )
        
        await composer.build()
        await composer.send()
        
        # Step 4: Now Alan can transfer
        composer = create_ffi_composer(
            env['algorand'],
            {
                'alan': env['alan'],
            }
        )
        
        composer.add_asset_transfer(
            params=AssetTransferParams(
                common_params=CommonParams(sender=env['alan'].address),
                asset_id=asset_id,
                amount=10_000,
                receiver=env['bianca'].address,
            )
        )
        
        await composer.build()
        await composer.send()
        
        # Verify transfer succeeded
        bianca_balance = get_asset_balance(env['algorand'], env['bianca'].address, asset_id)
        assert bianca_balance == 10_000
    
    @pytest.mark.asyncio
    async def test_default_frozen_asset(self, test_environment):
        """Test assets created with default_frozen=True."""
        env = test_environment
        
        # Create an asset that is frozen by default
        asset_params = AssetCreateParams(
            sender=env['creator'].address,
            total=100_000,
            decimals=0,
            default_frozen=True,  # Frozen by default
            freeze=env['freeze_manager'].address,
            manager=env['creator'].address,
        )
        
        result = env['algorand'].send.asset_create(asset_params)
        asset_id = get_asset_id_from_result(result)
        
        # Alan opts in
        composer = create_ffi_composer(
            env['algorand'],
            {
                'alan': env['alan'],
            }
        )
        
        composer.add_asset_opt_in(
            params=AssetOptInParams(
                common_params=CommonParams(sender=env['alan'].address),
                asset_id=asset_id,
            )
        )
        
        await composer.build()
        await composer.send()
        
        # Verify Alan's holding is frozen by default
        account_info = get_account_asset_info(env['algorand'], env['alan'].address, asset_id)
        assert account_info.frozen == True
        
        # Creator transfers assets to Alan (should work even when frozen)
        composer = create_ffi_composer(
            env['algorand'],
            {
                'creator': env['creator'],
            }
        )
        
        composer.add_asset_transfer(
            params=AssetTransferParams(
                common_params=CommonParams(sender=env['creator'].address),
                asset_id=asset_id,
                amount=10_000,
                receiver=env['alan'].address,
            )
        )
        
        await composer.build()
        await composer.send()
        
        # Alan cannot transfer until unfrozen
        with pytest.raises(Exception, match="asset is frozen|frozen"):
            composer = create_ffi_composer(
                env['algorand'],
                {
                    'alan': env['alan'],
                }
            )
            
            composer.add_asset_transfer(
                params=AssetTransferParams(
                    common_params=CommonParams(sender=env['alan'].address),
                    asset_id=asset_id,
                    amount=5_000,
                    receiver=env['creator'].address,
                )
            )
            
            await composer.build()
            await composer.send()
    
    @pytest.mark.asyncio 
    async def test_freeze_permission_errors(self, test_environment, test_asset):
        """Test that only freeze manager can freeze assets."""
        env = test_environment
        asset_id = test_asset['asset_id']
        
        # Test that non-freeze manager cannot freeze
        with pytest.raises(Exception, match="not authorized|permission"):
            composer = create_ffi_composer(
                env['algorand'],
                {
                    'alan': env['alan'],  # Alan is not the freeze manager
                }
            )
            
            composer.add_asset_freeze(
                params=AssetFreezeParams(
                    common_params=CommonParams(sender=env['alan'].address),
                    asset_id=asset_id,
                    target_address=env['bianca'].address,
                    frozen=True,
                )
            )
            
            await composer.build()
            await composer.send()