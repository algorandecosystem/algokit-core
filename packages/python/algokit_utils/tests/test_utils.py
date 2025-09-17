# from typing import override
# import typing
# from algokit_utils.algokit_http_client import HttpClient, HttpMethod, HttpResponse
# from algokit_utils.algokit_transact_ffi import (
#     SignedTransaction,
#     Transaction,
#     encode_transaction,
# )
# from algokit_utils import AlgodClient, TransactionSigner
# from algokit_utils.algokit_utils_ffi import (
#     AssetFreezeParams,
#     AssetOptInParams,
#     AssetTransferParams,
#     CommonParams,
#     Composer,
#     OnlineKeyRegistrationParams,
#     PaymentParams,
#     TransactionSignerGetter,
# )
# from algosdk.mnemonic import to_private_key
# from nacl.signing import SigningKey
# import base64
# import pytest
# import requests

# MN = "gloom mobile embark bitter goat hello reflect unfold scrap slow choose object excite lake visual school traffic science history fit idea mystery unknown abstract infant"
# SEED_B64: str = to_private_key(MN)  # type: ignore
# SEED_BYTES = base64.b64decode(SEED_B64)
# print(SEED_BYTES)
# KEY = SigningKey(SEED_BYTES[:32])
# ADDR = "ESQH3U2JCDDIASZYLLNZRNMYZOOYWWTCBVS45FSC7AXWOCTZCKL7BQL3P4"


# class TestSigner(TransactionSigner):
#     @override
#     async def sign_transactions(  # type: ignore
#         self, transactions: list[Transaction], indices: list[int]
#     ) -> list[SignedTransaction]:
#         stxns = []
#         for transaction in transactions:
#             tx_for_signing = encode_transaction(transaction)
#             sig = KEY.sign(tx_for_signing)
#             stxns.append(
#                 SignedTransaction(transaction=transaction, signature=sig.signature)
#             )

#         return stxns

#     @override
#     async def sign_transaction(self, transaction: Transaction) -> SignedTransaction:  # type: ignore
#         return (await self.sign_transactions([transaction], [0]))[0]


# class SignerGetter(TransactionSignerGetter):
#     @override
#     def get_signer(self, address: str) -> TransactionSigner:  # type: ignore
#         return TestSigner()


# class HttpClientImpl(HttpClient):
#     @override
#     async def request(  # type: ignore
#         self,
#         method: HttpMethod,
#         path: str,
#         query: typing.Optional[dict[str, str]],
#         body: typing.Optional[bytes],
#         headers: typing.Optional[dict[str, str]],
#     ) -> HttpResponse:
#         headers = headers or {}
#         headers["X-Algo-API-Token"] = "a" * 64

#         if method == HttpMethod.GET:
#             res = requests.get(
#                 f"http://localhost:4001/{path}", params=query, headers=headers
#             )
#         elif method == HttpMethod.POST:
#             res = requests.post(
#                 f"http://localhost:4001/{path}",
#                 params=query,
#                 data=body,
#                 headers=headers,
#             )
#         else:
#             raise NotImplementedError(
#                 f"HTTP method {method} not implemented in test client"
#             )

#         if res.status_code != 200:
#             raise Exception(f"HTTP request failed: {res.status_code} {res.text}")

#         # NOTE: Headers needing to be lowercase was a bit surprising, so we need to make sure we document that
#         headers = {k.lower(): v for k, v in res.headers.items()}

#         return HttpResponse(
#             body=res.content,
#             headers=headers,  # type: ignore
#         )


# # TODO: Add comprehensive asset transfer integration tests
# #
# # Asset Transfer Transaction Types to Test:
# # 1. AssetTransferParams - Standard asset transfer between accounts
# # 2. AssetOptInParams - Account opts into receiving an asset (amount=0, receiver=sender)
# # 3. AssetOptOutParams - Account opts out of an asset (amount=0, close_remainder_to specified)
# # 4. AssetClawbackParams - Asset manager claws back assets from an account
# #
# # Suggested Integration Test Scenarios:
# # - Create test asset with proper manager/freeze/clawback addresses
# # - Test complete asset lifecycle:
# #   * Asset creation
# #   * Account opt-in (AssetOptInParams)
# #   * Asset transfer from creator to account (AssetTransferParams)
# #   * Asset clawback by manager (AssetClawbackParams)
# #   * Account opt-out with remainder (AssetOptOutParams)
# # - Test error conditions:
# #   * Transfer to non-opted-in account
# #   * Transfer more than balance
# #   * Clawback by non-manager account
# #   * Invalid asset IDs
# # - Test FFI boundary conversions:
# #   * String address parsing validation
# #   * Optional field handling (close_remainder_to)
# #   * Error propagation from Rust to Python

# # TODO: Add comprehensive asset config integration tests
# #
# # Asset Configuration Transaction Types to Test:
# # 1. AssetCreateParams - Create new assets with various configurations
# # 2. AssetReconfigureParams - Modify existing asset management addresses
# # 3. AssetDestroyParams - Destroy assets (only by creator when supply is back to creator)
# #
# # Suggested Integration Test Scenarios:
# # - Test asset creation variations:
# #   * Basic asset (minimal fields)
# #   * Full asset with all optional fields (name, unit_name, url, metadata_hash)
# #   * Asset with management addresses (manager, reserve, freeze, clawback)
# #   * Asset with different decimal places (0-19)
# #   * Asset with default_frozen=true
# # - Test asset reconfiguration:
# #   * Change manager address
# #   * Set addresses to zero (make immutable)
# #   * Attempt reconfiguration by non-manager (should fail)
# # - Test asset destruction:
# #   * Successful destruction when all supply returned to creator
# #   * Failed destruction when supply still distributed
# # - Test FFI boundary conversions:
# #   * String address parsing for all optional addresses
# #   * metadata_hash validation (exactly 32 bytes)
# #   * Optional field handling (None vs Some values)
# #   * Error propagation for invalid addresses and metadata
# # - Test field validation:
# #   * metadata_hash length validation
# #   * URL length limits (96 bytes)
# #   * Asset name length limits (32 bytes)
# #   * Unit name length limits (8 bytes)


# @pytest.mark.asyncio
# async def test_composer():
#     algod = AlgodClient(HttpClientImpl())

#     composer = Composer(
#         algod_client=algod,
#         signer_getter=SignerGetter(),
#     )

#     composer.add_payment(
#         params=PaymentParams(
#             amount=1,
#             receiver=ADDR,
#             common_params=CommonParams(
#                 sender=ADDR,
#             ),
#         )
#     )

#     # Test asset freeze functionality
#     composer.add_asset_freeze(
#         params=AssetFreezeParams(
#             common_params=CommonParams(
#                 sender=ADDR,
#             ),
#             asset_id=12345,
#             target_address=ADDR,
#         )
#     )

#     # # Test key registration functionality
#     # composer.add_online_key_registration(
#     #     params=OnlineKeyRegistrationParams(
#     #         common_params=CommonParams(
#     #             sender=ADDR,
#     #         ),
#     #         vote_key=b"A" * 32,  # 32 bytes
#     #         selection_key=b"B" * 32,  # 32 bytes
#     #         vote_first=1000,
#     #         vote_last=2000,
#     #         vote_key_dilution=10000,
#     #         state_proof_key=b"C" * 64,  # 64 bytes
#     #     )
#     # )

#     # # Test asset transfer functionality
#     # composer.add_asset_transfer(
#     #     params=AssetTransferParams(
#     #         common_params=CommonParams(
#     #             sender=ADDR,
#     #         ),
#     #         asset_id=12345,
#     #         amount=100,
#     #         receiver=ADDR,
#     #     )
#     # )

#     # # Test asset opt-in functionality
#     # composer.add_asset_opt_in(
#     #     params=AssetOptInParams(
#     #         common_params=CommonParams(
#     #             sender=ADDR,
#     #         ),
#     #         asset_id=67890,
#     #     )
#     # )

#     await composer.build()
#     txids = await composer.send()
#     assert len(txids) == 1
#     assert len(txids[0]) == 52
#     print(txids)


# # Helper functions for implementing the TODOs above
# # These functions support testing the FFI transaction types


# def get_asset_balance(algorand, address: str, asset_id: int) -> int:
#     """Get asset balance using existing AlgorandClient AssetManager patterns."""
#     from algokit_utils import AlgorandClient

#     if isinstance(algorand, AlgorandClient):
#         try:
#             account_info = algorand.asset.get_account_information(address, asset_id)
#             return account_info.balance
#         except Exception:
#             return 0
#     else:
#         # Fallback for AlgodClient
#         try:
#             account_info = algorand.account_info(address)
#             for asset in account_info.get("assets", []):
#                 if asset["asset-id"] == asset_id:
#                     return asset["amount"]
#             return 0
#         except Exception:
#             return 0


# def is_opted_into_asset(algorand, address: str, asset_id: int) -> bool:
#     """Check if account is opted into asset using existing patterns."""
#     from algokit_utils import AlgorandClient

#     if isinstance(algorand, AlgorandClient):
#         try:
#             algorand.asset.get_account_information(address, asset_id)
#             return True
#         except Exception:
#             return False
#     else:
#         # Fallback for AlgodClient
#         try:
#             account_info = algorand.account_info(address)
#             return any(
#                 asset["asset-id"] == asset_id
#                 for asset in account_info.get("assets", [])
#             )
#         except Exception:
#             return False


# def get_asset_info(algorand, asset_id: int):
#     """Get asset information using existing AssetManager or AlgodClient."""
#     from algokit_utils import AlgorandClient

#     if isinstance(algorand, AlgorandClient):
#         return algorand.asset.get_by_id(asset_id)
#     else:
#         return algorand.asset_info(asset_id)


# def get_account_asset_info(algorand, address: str, asset_id: int):
#     """Get account-specific asset information using existing patterns."""
#     from algokit_utils import AlgorandClient

#     if isinstance(algorand, AlgorandClient):
#         return algorand.asset.get_account_information(address, asset_id)
#     else:
#         account_info = algorand.account_info(address)
#         for asset in account_info.get("assets", []):
#             if asset["asset-id"] == asset_id:
#                 return asset
#         raise ValueError(f"Account {address} not opted into asset {asset_id}")


# def setup_account_with_assets(algorand, account, asset_id: int, amount: int, funder):
#     """Set up an account with assets using existing high-level client methods.

#     This helper supports testing FFI asset transfer functionality by using
#     the existing AlgorandClient patterns to prepare test data.
#     """
#     from algokit_utils import AlgorandClient

#     if isinstance(algorand, AlgorandClient):
#         # Use bulk opt-in from existing AssetManager
#         algorand.asset.bulk_opt_in(account.address, [asset_id], signer=account.signer)

#         # Transfer using existing send patterns
#         from algokit_utils.transactions.transaction_composer import AssetTransferParams

#         algorand.send.asset_transfer(
#             AssetTransferParams(
#                 sender=funder.address,
#                 receiver=account.address,
#                 asset_id=asset_id,
#                 amount=amount,
#                 signer=funder.signer,
#             )
#         )


# def get_asset_id_from_result(result) -> int:
#     """Extract asset ID from transaction result using existing patterns."""
#     if hasattr(result, "confirmation") and result.confirmation:
#         return int(result.confirmation["asset-index"])
#     if hasattr(result, "confirmations") and result.confirmations:
#         return int(result.confirmations[0]["asset-index"])
#     raise ValueError("Could not extract asset ID from transaction result")


# def create_multi_signer(accounts: dict):
#     """Create a MultiAccountSignerGetter for testing FFI Composer with multiple signers.

#     This supports testing FFI workflows that require different signers for different
#     addresses (e.g., asset creator, freeze manager, clawback manager).
#     """
#     from nacl.signing import SigningKey

#     signers = {}
#     for key, account in accounts.items():
#         # Convert SigningAccount to TestSigner-compatible format
#         if hasattr(account, "private_key"):
#             private_key_bytes = account.private_key
#             signing_key = SigningKey(private_key_bytes[:32])
#             # Create a signer that works with the existing TestSigner pattern
#             signers[account.address] = TestSigner()
#         else:
#             signers[account.address] = TestSigner()

#     # Return a simple multi-signer that delegates to TestSigner
#     class MultiSigner(TransactionSignerGetter):
#         def __init__(self, signers_dict):
#             self.signers = signers_dict

#         def get_signer(self, address: str) -> TransactionSigner:
#             if address not in self.signers:
#                 # Default to TestSigner for any address
#                 return TestSigner()
#             return self.signers[address]

#     return MultiSigner(signers)


# def create_composer(algorand_client, accounts: dict):
#     """Create a Composer with multi-account signing support for FFI testing.

#     This helper creates the FFI Composer with proper multi-account signing
#     support needed for comprehensive workflow testing.
#     """
#     from algokit_utils import AlgodClient

#     # Use existing AlgodClient if provided, or create one
#     if isinstance(algorand_client, AlgodClient):
#         algod = algorand_client
#     else:
#         algod = AlgodClient(HttpClientImpl())

#     multi_signer = create_multi_signer(accounts) if accounts else SignerGetter()

#     return Composer(
#         algod_client=algod,
#         signer_getter=multi_signer,
#     )
