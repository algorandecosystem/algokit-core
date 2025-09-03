from typing import override
import typing
from algokit_utils.algokit_http_client import HttpClient, HttpMethod, HttpResponse
from algokit_utils.algokit_transact_ffi import SignedTransaction, Transaction, encode_transaction
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
import requests
import msgpack

MN = "gas net tragic valid celery want good neglect maid nuclear core false chunk place asthma three acoustic moon box million finish bargain onion ability shallow"
SEED_B64: str = to_private_key(MN)  # type: ignore
SEED_BYTES = base64.b64decode(SEED_B64)
KEY = SigningKey(SEED_BYTES[:32])
ADDR = "ON6AOPBATSSEL47ML7EPXATHGH7INOWONHWITMQEDRPXHTMDJYMPQXROMA"


class TestSigner(TransactionSigner):
    @override
    async def sign_transactions(  # type: ignore
        self, transactions: list[Transaction], indices: list[int]
    ) -> list[SignedTransaction]:
        stxns = []
        for transaction in transactions:
            tx_for_signing = encode_transaction(transaction)
            sig = KEY.sign(tx_for_signing)
            stxns.append(SignedTransaction(transaction=transaction, signature=sig.signature))

        return stxns

    @override
    async def sign_transaction(self, transaction: Transaction) -> SignedTransaction:  # type: ignore
        print("Signing single transaction")
        return (await self.sign_transactions([transaction], [0]))[0]


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

        headers = headers or {}
        headers["X-Algo-API-Token"] = "a" * 64

        if method == HttpMethod.GET:
            res = requests.get(f"http://localhost:4001/{path}", params=query, headers=headers)
        elif method == HttpMethod.POST:
            res = requests.post(f"http://localhost:4001/{path}", params=query, data=body, headers=headers)
        else:
            raise NotImplementedError(f"HTTP method {method} not implemented in test client")

        if res.status_code != 200:
            raise Exception(f"HTTP request failed: {res.status_code} {res.text}")

        if res.headers.get("Content-Type") == "application/msgpack":
            print(msgpack.unpackb(res.content, raw=False, strict_map_key=False))
        else:
            print(res.text)

        return HttpResponse(
            body=res.content,
            headers=res.headers # type: ignore
        )


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

    await composer.build()
    await composer.send()
