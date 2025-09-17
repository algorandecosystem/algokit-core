# """Asset transfer workflow tests for algokit_utils FFI."""

# import pytest

# from algokit_utils import AlgorandClient, SigningAccount
# from algokit_utils.algokit_utils_ffi import (
#     AssetOptInParams,
#     AssetTransferParams,
#     AssetOptOutParams,
#     AssetClawbackParams,
#     CommonParams,
# )
# from .test_ffi_utils import (
#     create_ffi_composer,
#     get_asset_balance,
#     is_opted_into_asset,
#     setup_account_with_assets,
# )


# class TestAssetTransferWorkflows:
#     """Comprehensive tests for asset transfer operations."""

#     @pytest.mark.asyncio
#     async def test_complete_asset_lifecycle(self, test_environment, test_asset):
#         """Test the complete lifecycle of an asset from creation to destruction."""
#         env = test_environment
#         asset_id = test_asset['asset_id']

#         composer = create_ffi_composer(
#             env['algorand'],
#             {
#                 'creator': env['creator'],
#                 'alan': env['alan'],
#                 'bianca': env['bianca'],
#             }
#         )

#         # Step 1: Alan opts into the asset
#         composer.add_asset_opt_in(
#             params=AssetOptInParams(
#                 common_params=CommonParams(sender=env['alan'].address),
#                 asset_id=asset_id,
#             )
#         )

#         # Step 2: Creator transfers assets to Alan
#         composer.add_asset_transfer(
#             params=AssetTransferParams(
#                 common_params=CommonParams(sender=env['creator'].address),
#                 asset_id=asset_id,
#                 amount=100_000,  # 0.1 TEST (with 6 decimals)
#                 receiver=env['alan'].address,
#             )
#         )

#         # Step 3: Bianca opts into the asset
#         composer.add_asset_opt_in(
#             params=AssetOptInParams(
#                 common_params=CommonParams(sender=env['bianca'].address),
#                 asset_id=asset_id,
#             )
#         )

#         # Step 4: Alan transfers some assets to Bianca
#         composer.add_asset_transfer(
#             params=AssetTransferParams(
#                 common_params=CommonParams(sender=env['alan'].address),
#                 asset_id=asset_id,
#                 amount=25_000,  # 0.025 TEST
#                 receiver=env['bianca'].address,
#             )
#         )

#         await composer.build()
#         txids = await composer.send()

#         # Verify balances
#         alan_balance = get_asset_balance(env['algorand'], env['alan'].address, asset_id)
#         bianca_balance = get_asset_balance(env['algorand'], env['bianca'].address, asset_id)

#         assert alan_balance == 75_000  # 0.075 TEST
#         assert bianca_balance == 25_000    # 0.025 TEST

#     @pytest.mark.asyncio
#     async def test_asset_opt_out_with_remainder(self, test_environment, test_asset):
#         """Test opting out of an asset with remainder handling."""
#         env = test_environment
#         asset_id = test_asset['asset_id']

#         # Setup: Alan has some assets
#         setup_account_with_assets(
#             env['algorand'],
#             env['alan'],
#             asset_id,
#             50_000,
#             env['creator']
#         )

#         composer = create_ffi_composer(
#             env['algorand'],
#             {
#                 'alan': env['alan'],
#             }
#         )

#         # Alan opts out, sending remainder back to creator
#         composer.add_asset_opt_out(
#             params=AssetOptOutParams(
#                 common_params=CommonParams(sender=env['alan'].address),
#                 asset_id=asset_id,
#                 close_remainder_to=env['creator'].address,
#             )
#         )

#         await composer.build()
#         await composer.send()

#         # Verify Alan no longer holds the asset
#         alan_opted_in = is_opted_into_asset(env['algorand'], env['alan'].address, asset_id)
#         assert not alan_opted_in

#         # Verify creator received the remainder
#         creator_balance = get_asset_balance(env['algorand'], env['creator'].address, asset_id)
#         assert creator_balance == test_asset['total']  # All assets back to creator

#     @pytest.mark.asyncio
#     async def test_asset_clawback_scenario(self, test_environment, test_asset):
#         """Test clawback functionality in a compliance scenario."""
#         env = test_environment
#         asset_id = test_asset['asset_id']

#         # Setup: Alan has assets that need to be clawed back
#         setup_account_with_assets(
#             env['algorand'],
#             env['alan'],
#             asset_id,
#             100_000,
#             env['creator']
#         )

#         composer = create_ffi_composer(
#             env['algorand'],
#             {
#                 'clawback_manager': env['clawback_manager'],
#             }
#         )

#         # Clawback manager retrieves assets from Alan
#         composer.add_asset_clawback(
#             params=AssetClawbackParams(
#                 common_params=CommonParams(sender=env['clawback_manager'].address),
#                 asset_id=asset_id,
#                 amount=100_000,
#                 receiver=env['creator'].address,  # Return to creator
#                 clawback_target=env['alan'].address,
#             )
#         )

#         await composer.build()
#         await composer.send()

#         # Verify Alan's balance is now 0
#         alan_balance = get_asset_balance(env['algorand'], env['alan'].address, asset_id)
#         assert alan_balance == 0

#         # Verify creator received the clawed back assets
#         creator_balance = get_asset_balance(env['algorand'], env['creator'].address, asset_id)
#         assert creator_balance == test_asset['total']

#     @pytest.mark.asyncio
#     async def test_asset_transfer_error_conditions(self, test_environment, test_asset):
#         """Test error conditions in asset transfers."""
#         env = test_environment
#         asset_id = test_asset['asset_id']

#         # Test 1: Transfer to non-opted-in account should fail
#         with pytest.raises(Exception, match="not opted in|account not opted in"):
#             composer = create_ffi_composer(
#                 env['algorand'],
#                 {
#                     'creator': env['creator'],
#                 }
#             )

#             composer.add_asset_transfer(
#                 params=AssetTransferParams(
#                     common_params=CommonParams(sender=env['creator'].address),
#                     asset_id=asset_id,
#                     amount=1000,
#                     receiver=env['alan'].address,  # Not opted in
#                 )
#             )

#             await composer.build()
#             await composer.send()

#         # Test 2: Transfer more than balance should fail
#         setup_account_with_assets(
#             env['algorand'],
#             env['alan'],
#             asset_id,
#             1000,
#             env['creator']
#         )

#         with pytest.raises(Exception, match="insufficient|balance"):
#             composer = create_ffi_composer(
#                 env['algorand'],
#                 {
#                     'alan': env['alan'],
#                 }
#             )

#             composer.add_asset_transfer(
#                 params=AssetTransferParams(
#                     common_params=CommonParams(sender=env['alan'].address),
#                     asset_id=asset_id,
#                     amount=2000,  # More than Alan has
#                     receiver=env['bianca'].address,
#                 )
#             )

#             await composer.build()
#             await composer.send()

#         # Test 3: Invalid asset ID should fail
#         with pytest.raises(Exception, match="asset does not exist|invalid asset"):
#             composer = create_ffi_composer(
#                 env['algorand'],
#                 {
#                     'alan': env['alan'],
#                 }
#             )

#             composer.add_asset_opt_in(
#                 params=AssetOptInParams(
#                     common_params=CommonParams(sender=env['alan'].address),
#                     asset_id=999999999,  # Non-existent asset
#                 )
#             )

#             await composer.build()
#             await composer.send()
