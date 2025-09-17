"""
Test implementation of FFI foreign traits for asset freeze testing.

This demonstrates the FFI testing approach where Python implements test adapters
that are used by Rust test functions to ensure consistent behavior across languages.
"""

import base64
from typing import override
import typing
from algokit_utils.algokit_http_client import (
    HttpClient,
    HttpMethod,
    HttpResponse,
)
from algokit_utils.algokit_transact_ffi import (
    SignedTransaction,
    Transaction,
    encode_transaction,
)
from algokit_utils.algokit_utils_ffi import (
    AlgodClient,
    ComposerTestAdapter,
    Composer,
    TransactionSigner,
    TransactionSignerGetter,
    run_tests,
)
import requests
import pytest
from algosdk.mnemonic import to_private_key
from nacl.signing import SigningKey

MN = "gloom mobile embark bitter goat hello reflect unfold scrap slow choose object excite lake visual school traffic science history fit idea mystery unknown abstract infant"
SEED_B64: str = to_private_key(MN)  # type: ignore
SEED_BYTES = base64.b64decode(SEED_B64)
print(SEED_BYTES)
KEY = SigningKey(SEED_BYTES[:32])
ADDR = "ESQH3U2JCDDIASZYLLNZRNMYZOOYWWTCBVS45FSC7AXWOCTZCKL7BQL3P4"


class TestSigner(TransactionSigner):
    @override
    async def sign_transactions(  # type: ignore
        self, transactions: list[Transaction], indices: list[int]
    ) -> list[SignedTransaction]:
        stxns = []
        for transaction in transactions:
            tx_for_signing = encode_transaction(transaction)
            sig = KEY.sign(tx_for_signing)
            stxns.append(
                SignedTransaction(transaction=transaction, signature=sig.signature)
            )

        return stxns

    @override
    async def sign_transaction(self, transaction: Transaction) -> SignedTransaction:  # type: ignore
        return (await self.sign_transactions([transaction], [0]))[0]


class SignerGetter(TransactionSignerGetter):
    @override
    def get_signer(self, address: str) -> TransactionSigner:  # type: ignore
        return TestSigner()


class HttpClientImpl(HttpClient):
    @override
    async def request(  # type: ignore
        self,
        http_method: HttpMethod,
        path: str,
        query: typing.Optional[dict[str, str]],
        body: typing.Optional[bytes],
        headers: typing.Optional[dict[str, str]],
    ) -> HttpResponse:
        headers = headers or {}
        headers["X-Algo-API-Token"] = "a" * 64

        if http_method == HttpMethod.GET:
            res = requests.get(
                f"http://localhost:4001/{path}", params=query, headers=headers
            )
        elif http_method == HttpMethod.POST:
            res = requests.post(
                f"http://localhost:4001/{path}",
                params=query,
                data=body,
                headers=headers,
            )
        else:
            raise NotImplementedError(
                f"HTTP method {http_method} not implemented in test client"
            )

        if res.status_code != 200:
            raise Exception(f"HTTP request failed: {res.status_code} {res.text}")

        # NOTE: Headers needing to be lowercase was a bit surprising, so we need to make sure we document that
        headers = {k.lower(): v for k, v in res.headers.items()}

        return HttpResponse(
            body=res.content,
            headers=headers,  # type: ignore
        )


class PythonComposerAdapter(ComposerTestAdapter):
    def __init__(self, algod_client, signer_getter):
        self.algod_client = algod_client
        self.signer_getter = signer_getter

    def new(self) -> Composer:
        """Create a new Composer instance."""
        return Composer(self.algod_client, self.signer_getter)


@pytest.mark.asyncio
async def test_run_suite():
    """Test FFI test suite runner."""
    # Create Python's adapter implementation

    algod_client = AlgodClient(HttpClientImpl())
    signer_getter = SignerGetter()

    adapter = PythonComposerAdapter(algod_client, signer_getter)

    res = await run_tests(adapter)

    assert res == "Tests executed"
