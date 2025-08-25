from typing import override
import typing
from algokit_utils.algokit_http_client import HttpClient, HttpMethod, HttpResponse
from algokit_utils.algokit_transact_ffi import SignedTransaction, Transaction
from algokit_utils import AlgodClient, TransactionSigner
from algokit_utils.algokit_utils_ffi import (
    CommonParams,
    Composer,
    PaymentParams,
    TransactionSignerGetter,
)
from algosdk.mnemonic import to_private_key
from nacl.signing import SigningKey
import base64
import pytest

MN = "limb estate enhance elegant merry worry spell trophy elegant lab truly step enter destroy split leave beach chalk slight they ignore square tower abandon rough"
SEED_B64: str = to_private_key(MN)  # type: ignore
SEED_BYTES = base64.b64decode(SEED_B64)
KEY = SigningKey(SEED_BYTES[:32])
ADDR = "BEMJKX676TZOOWJYMJPOMGIW4UT6S7UDTUUDCUQNKFC42TY6HVKOIWFOYA"


class TestSigner(TransactionSigner):
    @override
    def sign_transactions(  # type: ignore
        self, transactions: list[Transaction], indices: list[int]
    ) -> list[SignedTransaction]:
        print("Signing transactions")
        return []

    @override
    def sign_transaction(self, transaction: Transaction) -> SignedTransaction:  # type: ignore
        print("Signing single transaction")
        return self.sign_transactions([transaction], [0])[0]


class SignerGetter(TransactionSignerGetter):
    @override
    def get_signer(self, address: str) -> TransactionSigner:  # type: ignore
        print("Getting signer")
        return TestSigner()


class HttpClientImpl(HttpClient):
    @override
    async def request(  # type: ignore
        self,
        method: HttpMethod,
        path: str,
        query: typing.Optional[dict[str, str]],
        body: typing.Optional[bytes],
        headers: typing.Optional[dict[str, str]],
    ) -> HttpResponse:
        print(f"HTTP {method} {path} {query} {headers}")
        return HttpResponse(body=b"", headers={})


@pytest.mark.asyncio
async def test_composer():
    algod = AlgodClient(HttpClientImpl())
    # Test that algod_localnet is an object
    composer = Composer(
        algod_client=algod,
        signer_getter=SignerGetter(),
    )
    print(algod)
    print(composer)

    composer.add_payment(
        params=PaymentParams(
            amount=1,
            receiver=ADDR,
            common_params=CommonParams(
                sender=ADDR,
            ),
        )
    )

    await composer.build()  # <-- Error here
