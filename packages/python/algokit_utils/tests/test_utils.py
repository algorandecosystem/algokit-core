from typing import override
import typing
from algokit_utils.algokit_http_client import HttpClient, HttpMethod, HttpResponse
from algokit_utils.algokit_transact_ffi import OnApplicationComplete, SignedTransaction, Transaction, encode_transaction
from algokit_utils import AlgodClient, TransactionSigner
from algokit_utils.algokit_utils_ffi import (
    AbiType,
    AppCallParams,
    AppCreateParams,
    CommonParams,
    Composer,
    PaymentParams,
    TransactionSignerGetter,
    AbiValue
)
from algosdk.mnemonic import to_private_key
from nacl.signing import SigningKey
import base64
import pytest
import requests

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
        return (await self.sign_transactions([transaction], [0]))[0]


class SignerGetter(TransactionSignerGetter):
    @override
    def get_signer(self, address: str) -> TransactionSigner:  # type: ignore
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

        # NOTE: Headers needing to be lowercase was a bit surprising, so we need to make sure we document that
        headers = {k.lower(): v for k, v in res.headers.items()}

        return HttpResponse(
            body=res.content,
            headers=headers
        )


@pytest.mark.asyncio
async def test_composer():
    algod = AlgodClient(HttpClientImpl())

    composer = Composer(
        algod_client=algod,
        signer_getter=SignerGetter(),
    )

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
    response = await composer.send()
    assert(len(response.transaction_ids) == 1)
    assert(len(response.transaction_ids[0]) == 52)
    print(response.transaction_ids)

abi = AbiValue

def test_abi_bool():
    abi_bool: AbiValue = abi.bool(True) # type: ignore
    assert abi_bool.encoded_bytes() == b'\x80'
    assert abi_bool.get_bool() == True

    decoded_bool: AbiValue = abi.from_bytes(b'\x80', AbiType(abi_type="bool")) # type: ignore
    assert decoded_bool.get_bool() == True

def test_abi_bool_array():
    assert abi.array(
        element_type=AbiType(abi_type="bool"),
        values=[
            abi.bool(True) # type: ignore
        ]
    ).encoded_bytes() == b'\x00\x01\x80'

INT_1_PROG = bytes.fromhex('0b810143')

@pytest.mark.asyncio
async def test_app_create_and_call():
    algod = AlgodClient(HttpClientImpl())


    create_composer = Composer(
        algod_client=algod,
        signer_getter=SignerGetter(),
    )

    create_composer.add_app_create(
        params=AppCreateParams(
            common_params=CommonParams(
                    sender=ADDR,
            ),
            on_complete=OnApplicationComplete.NO_OP,
            approval_program=INT_1_PROG,
            clear_state_program=INT_1_PROG,
        )
    )

    await create_composer.build()
    response = await create_composer.send()
    assert(len(response.transaction_ids) == 1)
    assert(len(response.transaction_ids[0]) == 52)

    app_id = response.app_ids[0]
    assert app_id

    call_composer = Composer(
        algod_client=algod,
        signer_getter=SignerGetter(),
    )

    call_composer.add_app_call(
        params=AppCallParams(
            common_params=CommonParams(
                sender=ADDR,
            ),
            app_id=app_id,
            on_complete=OnApplicationComplete.NO_OP,
        )
    )

    await call_composer.build()
    response = await call_composer.send()
    assert(len(response.transaction_ids) == 1)
    assert(len(response.transaction_ids[0]) == 52)


