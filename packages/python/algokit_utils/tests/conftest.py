"""Test configuration and fixtures for algokit_utils FFI tests."""

import math
import random
from pathlib import Path
from typing import override
from uuid import uuid4

import pytest
from dotenv import load_dotenv
from nacl.signing import SigningKey

from algokit_utils import AlgorandClient, SigningAccount, TransactionSigner
from algokit_utils.algokit_transact_ffi import (
    SignedTransaction,
    Transaction,
    encode_transaction,
)
from algokit_utils.algokit_utils_ffi import TransactionSignerGetter
from algokit_utils.models.amount import AlgoAmount
from algokit_utils.transactions.transaction_composer import AssetCreateParams


@pytest.fixture(autouse=True, scope="session")
def _environment_fixture() -> None:
    """Load environment configuration following existing pattern."""
    env_path = Path(__file__).parent / ".." / "example.env"
    if env_path.exists():
        load_dotenv(env_path)


@pytest.fixture
def algorand() -> AlgorandClient:
    """AlgorandClient fixture following existing pattern."""
    return AlgorandClient.default_localnet()


class TestTransactionSigner(TransactionSigner):
    """Test transaction signer implementation following existing pattern."""
    
    def __init__(self, signing_key: SigningKey):
        self.signing_key = signing_key
    
    @override
    async def sign_transactions(
        self, transactions: list[Transaction], indices: list[int]
    ) -> list[SignedTransaction]:
        stxns = []
        for transaction in transactions:
            tx_for_signing = encode_transaction(transaction)
            sig = self.signing_key.sign(tx_for_signing)
            stxns.append(
                SignedTransaction(transaction=transaction, signature=sig.signature)
            )
        return stxns

    @override
    async def sign_transaction(self, transaction: Transaction) -> SignedTransaction:
        return (await self.sign_transactions([transaction], [0]))[0]


class MultiAccountSignerGetter(TransactionSignerGetter):
    """Signer getter that can handle multiple accounts for workflow tests."""
    
    def __init__(self, signers: dict[str, TransactionSigner]):
        self.signers = signers
    
    @override
    def get_signer(self, address: str) -> TransactionSigner:
        if address not in self.signers:
            raise ValueError(f"No signer available for address: {address}")
        return self.signers[address]


@pytest.fixture
def creator_account(algorand: AlgorandClient) -> SigningAccount:
    """Creator account with funding following existing patterns."""
    new_account = algorand.account.random()
    dispenser = algorand.account.localnet_dispenser()
    algorand.account.ensure_funded(
        new_account, 
        dispenser, 
        AlgoAmount.from_algo(100), 
        min_funding_increment=AlgoAmount.from_algo(1)
    )
    algorand.set_signer(sender=new_account.address, signer=new_account.signer)
    return new_account


@pytest.fixture
def alan_account(algorand: AlgorandClient) -> SigningAccount:
    """Alan account with funding."""
    new_account = algorand.account.random()
    dispenser = algorand.account.localnet_dispenser()
    algorand.account.ensure_funded(
        new_account, 
        dispenser, 
        AlgoAmount.from_algo(100), 
        min_funding_increment=AlgoAmount.from_algo(1)
    )
    return new_account


@pytest.fixture
def bianca_account(algorand: AlgorandClient) -> SigningAccount:
    """Bianca account with funding."""
    new_account = algorand.account.random()
    dispenser = algorand.account.localnet_dispenser()
    algorand.account.ensure_funded(
        new_account, 
        dispenser, 
        AlgoAmount.from_algo(100), 
        min_funding_increment=AlgoAmount.from_algo(1)
    )
    return new_account


def generate_test_asset_ffi(
    algorand: AlgorandClient, 
    sender: SigningAccount, 
    total: int | None = None,
    with_management: bool = True
) -> int:
    """Generate a test asset using existing pattern from algokit-utils-py conftest.py."""
    if total is None:
        total = math.floor(random.random() * 100) + 20

    decimals = 0
    asset_name = f"FFI_TEST ${math.floor(random.random() * 100) + 1}_${total}"

    params = AssetCreateParams(
        sender=sender.address,
        total=total,
        decimals=decimals,
        default_frozen=False,
        unit_name="FFIT",  # FFI Test
        asset_name=asset_name,
        url="https://ffi.test.example.com",
    )
    
    # Add management addresses if requested
    if with_management:
        params.manager = sender.address
        params.reserve = sender.address
        params.freeze = sender.address
        params.clawback = sender.address

    create_result = algorand.send.asset_create(params)
    return int(create_result.confirmation["asset-index"])


@pytest.fixture
def test_environment(
    algorand: AlgorandClient,
    creator_account: SigningAccount,
    alan_account: SigningAccount,
    bianca_account: SigningAccount
) -> dict:
    """Set up a complete test environment with multiple accounts."""
    # Create additional specialized accounts
    freeze_manager = algorand.account.random()
    clawback_manager = algorand.account.random()
    
    # Fund the specialized accounts
    dispenser = algorand.account.localnet_dispenser()
    
    for account in [freeze_manager, clawback_manager]:
        algorand.account.ensure_funded(
            account, 
            dispenser, 
            AlgoAmount.from_algo(10), 
            min_funding_increment=AlgoAmount.from_algo(1)
        )
    
    return {
        'algorand': algorand,
        'creator': creator_account,
        'alan': alan_account,
        'bianca': bianca_account,
        'freeze_manager': freeze_manager,
        'clawback_manager': clawback_manager,
    }


@pytest.fixture
def test_asset(test_environment: dict) -> dict:
    """Create a test asset with all management features enabled."""
    env = test_environment
    algorand = env['algorand']
    creator = env['creator']
    
    # Create asset using existing pattern with management addresses
    asset_params = AssetCreateParams(
        sender=creator.address,
        total=1_000_000,
        decimals=6,
        unit_name="FFITEST",
        asset_name="FFI Test Asset",
        url="https://ffi.test.example.com",
        default_frozen=False,
        manager=creator.address,
        reserve=creator.address,
        freeze=env['freeze_manager'].address,
        clawback=env['clawback_manager'].address,
    )
    
    result = algorand.send.asset_create(asset_params)
    asset_id = int(result.confirmation["asset-index"])
    
    return {
        'asset_id': asset_id,
        'total': 1_000_000,
        'decimals': 6,
    }


def get_unique_name() -> str:
    """Generate unique names for test resources following existing pattern."""
    name = str(uuid4()).replace("-", "")
    assert name.isalnum()
    return name