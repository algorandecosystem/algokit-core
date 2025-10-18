# FFI Performance Overhead

By default, uniffi will send most data structures (Records, Enums, and Errors) over the FFI via [serialization](https://mozilla.github.io/uniffi-rs/next/internals/lifting_and_lowering.html). This results in higher safety (e.g not dealing with raw pointers), but also higher overhead when crossing the FFI boundary.

## Benchmarks

Below are benchmarks for encoding and decoding an application create transaction with the current native `algosdk` python package and the core FFI python package. It should be noted that an application create is one of the more complex transaction types. Simpler transaction types, such as a payment, see a lower relative overhead (~3x).

Encoding:

```
------------- benchmark: 2 tests ------------
Name (time in us)              Mean          
---------------------------------------------
test_bench_sdk_encode        9.7800 (1.0)    
test_bench_core_encode     157.2340 (16.08)  
---------------------------------------------
```

Decoding:

```
------------- benchmark: 2 tests ------------
Name (time in us)              Mean          
---------------------------------------------
test_bench_sdk_decode        8.1749 (1.0)    
test_bench_core_decode     128.3318 (15.70)  
---------------------------------------------
```

## Is This Acceptable?

At first, the overhead seems quite high with a ~16x overhead for the FFI. In absolute terms, however, the overhead is only around 150 microseconds.

It should also be noted that this is Python, which is a very slow language. Other languages, such as Go, have not yet been benchmarked, but are expected to have lower uniffi serialization overhead because they are faster languages.

This is deemed to be acceptable because we know the network is only capable of processing around 1,500 transactions of this size per second. This means even in the worst-case scenario with one client issuing the full capacity of the network, the FFI overhead would only account for around 180 milliseconds. This also assumes that someone wanting to saturate the network would be using a single-threaded python client, which is unlikely.

## What About Blocks?

The above section mentions that the overhead is acceptable because of the network's max TPS. Clients, however, may not just be encoding transactions but also decoding them from blocks. For example, one may wonder what the impact to a subscriber library would be, which involves processing all the transactions in many historical blocks.

Since algokit core also offers an FFI algod and indexer client, the native language (in this case python) will never have to manually decode a block. Instead, the decoding of each transaction in the block is done on the Rust side and then sent over the FFI. Assuming that most of the overhead is the cost of going from bytes to the python data class, we can expect a ~0.5 second overhead per block when deserializing a FULL block of transactions. This math holds up for both complex transactions or simple transactions (which are smaller and deserialize faster) because block size is constant. While this is a fairly high overhead, it is still acceptable given that the network is nowhere near full load and the network produces a block every ~3 seconds.

The above 0.5 second overhead per block is under max loads conditions. In reality, we average around 30 transactions per block, which would result in a ~3 millisecond overhead per block. This is acceptable because it is likely a fraction of the total round-trip API call time.

## Future Improvements

Currently we use Uniffi records to send transactions over the FFI, which incur a high serialization overhead. In the future, if we feel the need to optimize this further, we could use a Uniffi object, which does not require serialization (instead it uses a pointer into Rust-owned memory). This would not require a breaking change to the API.

It is also plausible that in the future we have a subscriber package dedicated to this use case that is implemented in Rust, which would outperform a native Python implementation.
