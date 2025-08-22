from typing import override
from algokit_utils.algokit_transact_ffi import Transaction
from algokit_utils import algod_localnet, TransactionSigner
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
    def sign_transactions(self, transactions: list[Transaction], indices: list[int]):
        print("Signing")

    @override
    def sign_transaction(self, transaction: Transaction):
        print("Signing single transaction")


class SignerGetter(TransactionSignerGetter):
    @override
    def get_signer(self, address: str) -> TransactionSigner:
        print("Getting signer")
        return TestSigner()


@pytest.mark.asyncio
async def test_composer():
    # Test that algod_localnet is an object
    algod = algod_localnet()
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
