import pytest

from . import TEST_DATA
from algokit_transact import (
    FeeParams,
    assign_fee,
    encode_transaction,
    encode_signed_transaction,
    KeyRegistrationTransactionFields,
    TransactionType,
    decode_transaction,
    get_encoded_transaction_type,
    Transaction,
    SignedTransaction,
    address_from_string,
    address_from_pub_key,
    get_transaction_id,
    get_transaction_id_raw,
)
from nacl.signing import SigningKey

online_key_registration = TEST_DATA.online_key_registration
offline_key_registration = TEST_DATA.offline_key_registration
non_participating_key_registration = TEST_DATA.non_participating_key_registration

# Polytest Suite: Key Registration

# Polytest Group: Transaction Tests

@pytest.mark.group_transaction_tests
def test_example():
    """A human-readable example of forming a transaction and signing it"""
    alice_keypair = SigningKey.generate()  # Keypair generated from PyNaCl
    alice = address_from_pub_key(alice_keypair.verify_key.__bytes__())

    # Example 1: Online key registration
    online_txn = Transaction(
        transaction_type=TransactionType.KEY_REGISTRATION,
        first_valid=1337,
        last_valid=1347,
        sender=alice,
        genesis_hash=b"A" * 32,  # pretend this is a valid hash
        genesis_id="localnet",
        key_registration=KeyRegistrationTransactionFields(
            vote_key=b"1" * 32,  # 32-byte participation key
            selection_key=b"2" * 32,  # 32-byte VRF key
            state_proof_key=b"3" * 64,  # 64-byte state proof key
            vote_first=100,
            vote_last=1000,
            vote_key_dilution=1000,
        ),
    )

    online_txn_with_fee = assign_fee(online_txn, FeeParams(fee_per_byte=0, min_fee=1000))
    assert online_txn_with_fee.fee == 1000

    # Example 2: Offline key registration (going offline)
    offline_txn = Transaction(
        transaction_type=TransactionType.KEY_REGISTRATION,
        first_valid=1337,
        last_valid=1347,
        sender=alice,
        genesis_hash=b"A" * 32,
        genesis_id="localnet",
        key_registration=KeyRegistrationTransactionFields(),  # All fields None for offline
    )

    offline_txn_with_fee = assign_fee(offline_txn, FeeParams(fee_per_byte=0, min_fee=1000))
    assert offline_txn_with_fee.fee == 1000

    # Example 3: Non-participating account
    non_part_txn = Transaction(
        transaction_type=TransactionType.KEY_REGISTRATION,
        first_valid=1337,
        last_valid=1347,
        sender=alice,
        genesis_hash=b"A" * 32,
        genesis_id="localnet",
        key_registration=KeyRegistrationTransactionFields(
            non_participation=True  # Mark as non-participating for no rewards
        ),
    )

    non_part_txn_with_fee = assign_fee(non_part_txn, FeeParams(fee_per_byte=0, min_fee=1000))
    assert non_part_txn_with_fee.fee == 1000

@pytest.mark.group_transaction_tests
def test_get_transaction_id():
    """A transaction id can be obtained from a transaction"""
    # Test with online key registration
    assert get_transaction_id(online_key_registration.transaction) == online_key_registration.id
    assert get_transaction_id_raw(online_key_registration.transaction) == online_key_registration.id_raw
    
    # Test with offline key registration
    assert get_transaction_id(offline_key_registration.transaction) == offline_key_registration.id
    assert get_transaction_id_raw(offline_key_registration.transaction) == offline_key_registration.id_raw

@pytest.mark.group_transaction_tests
def test_get_encoded_transaction_type():
    """The transaction type of an encoded transaction can be retrieved"""
    online_encoded = encode_transaction(online_key_registration.transaction)
    assert get_encoded_transaction_type(online_encoded) == TransactionType.KEY_REGISTRATION
    
    offline_encoded = encode_transaction(offline_key_registration.transaction)
    assert get_encoded_transaction_type(offline_encoded) == TransactionType.KEY_REGISTRATION

@pytest.mark.group_transaction_tests
def test_decode_without_prefix():
    """A transaction without TX prefix and valid fields is decoded properly"""
    # Remove TX prefix (first 2 bytes) and decode
    online_decoded = decode_transaction(online_key_registration.unsigned_bytes[2:])
    assert online_decoded == online_key_registration.transaction
    
    offline_decoded = decode_transaction(offline_key_registration.unsigned_bytes[2:])
    assert offline_decoded == offline_key_registration.transaction

@pytest.mark.group_transaction_tests
def test_decode_with_prefix():
    """A transaction with TX prefix and valid fields is decoded properly"""
    # Decode with TX prefix
    online_decoded = decode_transaction(online_key_registration.unsigned_bytes)
    assert online_decoded == online_key_registration.transaction
    
    offline_decoded = decode_transaction(offline_key_registration.unsigned_bytes)
    assert offline_decoded == offline_key_registration.transaction

@pytest.mark.group_transaction_tests
def test_encode_with_auth_address():
    """An auth address can be attached to a encoded transaction with a signature"""
    signed_txn = SignedTransaction(
        transaction=online_key_registration.transaction,
        signature=online_key_registration.signing_private_key.sign(online_key_registration.unsigned_bytes).signature,
        auth_address=online_key_registration.rekeyed_sender_auth_address,
    )
    
    encoded = encode_signed_transaction(signed_txn)
    assert encoded == online_key_registration.rekeyed_sender_signed_bytes

@pytest.mark.group_transaction_tests
def test_encode_with_signature():
    """A signature can be attached to a encoded transaction"""
    signed_txn = SignedTransaction(
        transaction=online_key_registration.transaction,
        signature=online_key_registration.signing_private_key.sign(online_key_registration.unsigned_bytes).signature,
    )
    
    encoded = encode_signed_transaction(signed_txn)
    assert encoded == online_key_registration.signed_bytes

@pytest.mark.group_transaction_tests
def test_encode():
    """A transaction with valid fields is encoded properly"""
    # Test online key registration encoding
    online_encoded = encode_transaction(online_key_registration.transaction)
    assert online_encoded == online_key_registration.unsigned_bytes
    
    # Test offline key registration encoding
    offline_encoded = encode_transaction(offline_key_registration.transaction)
    assert offline_encoded == offline_key_registration.unsigned_bytes
    
    # Test non-participating key registration encoding
    non_part_encoded = encode_transaction(non_participating_key_registration.transaction)
    assert non_part_encoded == non_participating_key_registration.unsigned_bytes