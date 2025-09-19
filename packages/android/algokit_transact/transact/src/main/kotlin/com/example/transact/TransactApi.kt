package com.example.transact

import android.R.attr.publicKey
import android.location.Address
import uniffi.algokit_transact_ffi.Transaction
import uniffi.algokit_transact_ffi.addressFromPublicKey
import uniffi.algokit_transact_ffi.publicKeyFromAddress

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

    fun createAddressFromPubKey(publicKey: ByteArray): String {
      return addressFromPublicKey(publicKey)
    }

    fun getPubKeyFromAddress(address: String): ByteArray {
      return publicKeyFromAddress(address)
    }

    fun encodeTransaction(transaction: Transaction): ByteArray {
        return encodeTransaction(transaction)
    }

    fun decodeTransaction(bytes: ByteArray): Transaction {
        return decodeTransaction(bytes)
    }
}
