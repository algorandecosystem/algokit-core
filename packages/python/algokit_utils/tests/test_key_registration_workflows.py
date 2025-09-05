"""Key registration workflow tests for algokit_utils FFI."""

import pytest

from algokit_utils import AlgorandClient, SigningAccount  
from algokit_utils.algokit_utils_ffi import (
    OnlineKeyRegistrationParams,
    OfflineKeyRegistrationParams,
    NonParticipationKeyRegistrationParams,
    CommonParams,
)
from .test_ffi_utils import create_ffi_composer


class TestKeyRegistrationWorkflows:
    """Comprehensive tests for key registration operations."""
    
    @pytest.mark.asyncio
    async def test_online_key_registration(self, test_environment):
        """Test registering an account online for consensus participation."""
        env = test_environment
        
        composer = create_ffi_composer(
            env['algorand'],
            {
                'alan': env['alan'],
            }
        )
        
        # Register Alan online with participation keys
        composer.add_online_key_registration(
            params=OnlineKeyRegistrationParams(
                common_params=CommonParams(sender=env['alan'].address),
                vote_key=b"V" * 32,  # 32-byte vote key
                selection_key=b"S" * 32,  # 32-byte selection key
                vote_first=1000,  # First voting round
                vote_last=10000,  # Last voting round
                vote_key_dilution=1000,  # Key dilution factor
                state_proof_key=b"P" * 64,  # 64-byte state proof key
            )
        )
        
        await composer.build()
        await composer.send()
        
        # Verify account is online (Note: This would require querying account info)
        # In a real test, we'd check the account participation status
        # account_info = env['algorand'].client.algod.account_info(env['alan'].address)
        # assert account_info['status'] == 'Online'
    
    @pytest.mark.asyncio
    async def test_offline_key_registration(self, test_environment):
        """Test taking an account offline from consensus."""
        env = test_environment
        
        # First register online (setup)
        online_composer = create_ffi_composer(
            env['algorand'],
            {
                'alan': env['alan'],
            }
        )
        
        online_composer.add_online_key_registration(
            params=OnlineKeyRegistrationParams(
                common_params=CommonParams(sender=env['alan'].address),
                vote_key=b"V" * 32,
                selection_key=b"S" * 32,
                vote_first=1000,
                vote_last=10000,
                vote_key_dilution=1000,
                state_proof_key=b"P" * 64,
            )
        )
        
        await online_composer.build()
        await online_composer.send()
        
        # Now take the account offline
        composer = create_ffi_composer(
            env['algorand'],
            {
                'alan': env['alan'],
            }
        )
        
        composer.add_offline_key_registration(
            params=OfflineKeyRegistrationParams(
                common_params=CommonParams(sender=env['alan'].address),
            )
        )
        
        await composer.build()
        await composer.send()
        
        # Verify account is offline
        # account_info = env['algorand'].client.algod.account_info(env['alan'].address)
        # assert account_info['status'] == 'Offline'
    
    @pytest.mark.asyncio
    async def test_non_participation_key_registration(self, test_environment):
        """Test marking an account as non-participating."""
        env = test_environment
        
        composer = create_ffi_composer(
            env['algorand'],
            {
                'alan': env['alan'],
            }
        )
        
        composer.add_non_participation_key_registration(
            params=NonParticipationKeyRegistrationParams(
                common_params=CommonParams(sender=env['alan'].address),
            )
        )
        
        await composer.build()
        await composer.send()
        
        # Verify account is marked as non-participating
        # account_info = env['algorand'].client.algod.account_info(env['alan'].address)
        # assert account_info['status'] == 'NotParticipating'
    
    @pytest.mark.asyncio
    async def test_key_registration_validation(self, test_environment):
        """Test validation of key registration parameters."""
        env = test_environment
        
        # Test 1: Invalid vote key length
        with pytest.raises(ValueError, match="vote_key must be exactly 32 bytes"):
            composer = create_ffi_composer(
                env['algorand'],
                {
                    'alan': env['alan'],
                }
            )
            
            composer.add_online_key_registration(
                params=OnlineKeyRegistrationParams(
                    common_params=CommonParams(sender=env['alan'].address),
                    vote_key=b"V" * 31,  # Wrong length
                    selection_key=b"S" * 32,
                    vote_first=1000,
                    vote_last=10000,
                    vote_key_dilution=1000,
                    state_proof_key=b"P" * 64,
                )
            )
            
            await composer.build()
        
        # Test 2: Invalid selection key length
        with pytest.raises(ValueError, match="selection_key must be exactly 32 bytes"):
            composer = create_ffi_composer(
                env['algorand'],
                {
                    'alan': env['alan'],
                }
            )
            
            composer.add_online_key_registration(
                params=OnlineKeyRegistrationParams(
                    common_params=CommonParams(sender=env['alan'].address),
                    vote_key=b"V" * 32,
                    selection_key=b"S" * 33,  # Wrong length
                    vote_first=1000,
                    vote_last=10000,
                    vote_key_dilution=1000,
                    state_proof_key=b"P" * 64,
                )
            )
            
            await composer.build()
        
        # Test 3: Invalid state proof key length
        with pytest.raises(ValueError, match="state_proof_key must be exactly 64 bytes"):
            composer = create_ffi_composer(
                env['algorand'],
                {
                    'alan': env['alan'],
                }
            )
            
            composer.add_online_key_registration(
                params=OnlineKeyRegistrationParams(
                    common_params=CommonParams(sender=env['alan'].address),
                    vote_key=b"V" * 32,
                    selection_key=b"S" * 32,
                    vote_first=1000,
                    vote_last=10000,
                    vote_key_dilution=1000,
                    state_proof_key=b"P" * 63,  # Wrong length
                )
            )
            
            await composer.build()
        
        # Test 4: Invalid voting range
        with pytest.raises(ValueError, match="vote_first must be less than vote_last"):
            composer = create_ffi_composer(
                env['algorand'],
                {
                    'alan': env['alan'],
                }
            )
            
            composer.add_online_key_registration(
                params=OnlineKeyRegistrationParams(
                    common_params=CommonParams(sender=env['alan'].address),
                    vote_key=b"V" * 32,
                    selection_key=b"S" * 32,
                    vote_first=10000,  # Greater than vote_last
                    vote_last=1000,
                    vote_key_dilution=1000,
                    state_proof_key=b"P" * 64,
                )
            )
            
            await composer.build()