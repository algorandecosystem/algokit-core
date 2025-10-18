import base64
from algokit_transact.algokit_transact_ffi import decode_transaction, encode_transaction, encode_transaction_raw
from . import TEST_DATA
from algosdk import encoding

def core_to_sdk_transaction(txn):
    encoded_txn = encode_transaction_raw(txn)
    return encoding.msgpack_decode(base64.b64encode(encoded_txn))


core_txn = TEST_DATA.app_create.transaction
sdk_txn = core_to_sdk_transaction(core_txn)


def test_bench_core_encode(benchmark):
    """Benchmark encoding a simple payment transaction"""
    def encode():
        encode_transaction(core_txn)

    benchmark(encode)

def test_bench_sdk_encode(benchmark):
    """Benchmark encoding a simple payment transaction using the SDK function"""
    def encode():
        encoding.msgpack_encode(sdk_txn)

    benchmark(encode)

def test_bench_core_decode(benchmark):
    """Benchmark decoding a simple payment transaction"""
    encoded_txn = encode_transaction(core_txn)

    def decode():
        decode_transaction(encoded_txn)

    benchmark(decode)

def test_bench_sdk_decode(benchmark):
    """Benchmark decoding a simple payment transaction using the SDK function"""
    sdk_encoded = encoding.msgpack_encode(sdk_txn)

    def decode():
        encoding.msgpack_decode(sdk_encoded)

    benchmark(decode)
