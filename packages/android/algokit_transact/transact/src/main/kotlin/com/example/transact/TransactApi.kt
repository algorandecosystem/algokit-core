package com.example.transact

import android.location.Address
import uniffi.algokit_transact_ffi.Transaction
import uniffi.algokit_transact_ffi.addressFromPublicKey

class TransactApi {
    companion object {
        init {
            try {
                System.loadLibrary("algokit_transact_ffi")
            } catch (e: UnsatisfiedLinkError) {
                throw RuntimeException("Failed to load native library: ${e.message}", e)
            }
        }
    }

    fun createAddressFromPubKey(publicKey: ByteArray): Address {
      return addressFromPublicKey(publicKey)
    }

    fun createAddressFromString(publicKey: ByteArray): Address {
        return addressFromPublicKey(publicKey)
    }

    fun encodeTransaction(transaction: Transaction): ByteArray {
        return encodeTransaction(transaction)
    }

    fun decodeTransaction(bytes: ByteArray): Transaction {
        return decodeTransaction(bytes)
    }
}
