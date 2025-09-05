"""Test utilities for algokit_utils FFI workflow tests."""

from algokit_utils import AlgorandClient, SigningAccount
from algokit_utils.assets.asset_manager import AccountAssetInformation, AssetInformation


def get_asset_balance(algorand: AlgorandClient, address: str, asset_id: int) -> int:
    """Get asset balance using existing AssetManager patterns."""
    try:
        account_info = algorand.asset.get_account_information(address, asset_id)
        return account_info.balance
    except Exception:
        return 0


def is_opted_into_asset(algorand: AlgorandClient, address: str, asset_id: int) -> bool:
    """Check if account is opted into asset using existing patterns."""
    try:
        algorand.asset.get_account_information(address, asset_id)
        return True
    except Exception:
        return False


def get_asset_info(algorand: AlgorandClient, asset_id: int) -> AssetInformation:
    """Get asset information using existing AssetManager."""
    return algorand.asset.get_by_id(asset_id)


def get_account_asset_info(algorand: AlgorandClient, address: str, asset_id: int) -> AccountAssetInformation:
    """Get account-specific asset information using existing patterns."""
    return algorand.asset.get_account_information(address, asset_id)


def setup_account_with_assets(
    algorand: AlgorandClient, 
    account: SigningAccount, 
    asset_id: int, 
    amount: int,
    funder: SigningAccount
) -> None:
    """Set up an account with assets using existing high-level client methods."""
    # Use bulk opt-in from existing AssetManager
    algorand.asset.bulk_opt_in(account.address, [asset_id], signer=account.signer)
    
    # Transfer using existing send patterns
    from algokit_utils.transactions.transaction_composer import AssetTransferParams
    algorand.send.asset_transfer(
        AssetTransferParams(
            sender=funder.address,
            receiver=account.address,
            asset_id=asset_id,
            amount=amount,
            signer=funder.signer,
        )
    )


def get_asset_id_from_result(result) -> int:
    """Extract asset ID from transaction result using existing patterns."""
    if hasattr(result, 'confirmation') and result.confirmation:
        return int(result.confirmation["asset-index"])
    if hasattr(result, 'confirmations') and result.confirmations:
        return int(result.confirmations[0]["asset-index"])
    raise ValueError("Could not extract asset ID from transaction result")


def create_multi_signer(accounts: dict[str, SigningAccount]):
    """Create a MultiAccountSignerGetter from a dict of accounts."""
    from .conftest import MultiAccountSignerGetter, TestTransactionSigner
    from nacl.signing import SigningKey
    
    signers = {}
    for key, account in accounts.items():
        # Convert SigningAccount to TestTransactionSigner
        private_key_bytes = account.private_key
        signing_key = SigningKey(private_key_bytes[:32])
        signers[account.address] = TestTransactionSigner(signing_key)
    
    return MultiAccountSignerGetter(signers)


def create_ffi_composer(algorand: AlgorandClient, accounts: dict[str, SigningAccount]):
    """Create a Composer with multi-account signing support following existing patterns."""
    from algokit_utils.algokit_http_client import HttpClient, HttpMethod, HttpResponse
    from algokit_utils import AlgodClient
    from algokit_utils.algokit_utils_ffi import Composer
    import typing
    import requests
    
    # Create HTTP client implementation following existing pattern from test_utils.py
    class HttpClientImpl(HttpClient):
        async def request(
            self,
            method: HttpMethod,
            path: str,
            query: typing.Optional[dict[str, str]],
            body: typing.Optional[bytes],
            headers: typing.Optional[dict[str, str]],
        ) -> HttpResponse:
            headers = headers or {}
            headers["X-Algo-API-Token"] = "a" * 64

            if method == HttpMethod.GET:
                res = requests.get(
                    f"http://localhost:4001/{path}", params=query, headers=headers
                )
            elif method == HttpMethod.POST:
                res = requests.post(
                    f"http://localhost:4001/{path}",
                    params=query,
                    data=body,
                    headers=headers,
                )
            else:
                raise NotImplementedError(
                    f"HTTP method {method} not implemented in test client"
                )

            if res.status_code != 200:
                raise Exception(f"HTTP request failed: {res.status_code} {res.text}")

            headers = {k.lower(): v for k, v in res.headers.items()}
            return HttpResponse(body=res.content, headers=headers)
    
    algod_client = AlgodClient(HttpClientImpl())
    multi_signer = create_multi_signer(accounts)
    
    return Composer(
        algod_client=algod_client,
        signer_getter=multi_signer,
    )