import org.junit.Assert.assertArrayEquals
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
import org.junit.Test
import uniffi.algokit_transact_ffi.PaymentTransactionFields
import uniffi.algokit_transact_ffi.PqSignature
import uniffi.algokit_transact_ffi.SignedTransaction
import uniffi.algokit_transact_ffi.Transaction
import uniffi.algokit_transact_ffi.TransactionType
import uniffi.algokit_transact_ffi.decodeSignedTransaction
import uniffi.algokit_transact_ffi.encodeSignedTransaction
import uniffi.algokit_transact_ffi.pqSchemeFalcon1024

/**
 * Native post-quantum (`pqsig`) signed transactions through the generated
 * Kotlin bindings. In an app, `signature` comes from `algokit_crypto`'s
 * `falcon1024RawSign(account.privateKey, encodeTransaction(txn))`.
 */
class PqSignatureTest {
    private val address = "VXH5UP6JLU2CGIYPUFZ4Z5OTLJCLMA5EXD3YHTMVNDE5P7ILZ324FSYSPQ"

    private fun payment() =
        Transaction(
            transactionType = TransactionType.PAYMENT,
            sender = address,
            fee = 1000uL,
            firstValid = 1uL,
            lastValid = 1000uL,
            genesisHash = ByteArray(32) { 7 },
            genesisId = "testnet-v1.0",
            payment = PaymentTransactionFields(receiver = address, amount = 1uL),
        )

    private fun pqSig(salt: UByte) =
        PqSignature(
            publicKey = ByteArray(1793) { 1 },
            scheme = pqSchemeFalcon1024(),
            signature = ByteArray(1230) { 2 },
            salt = salt,
        )

    @Test
    fun falcon1024SchemeIsF1() {
        assertArrayEquals("f1".toByteArray(), pqSchemeFalcon1024())
    }

    @Test
    fun encodesPqsigAsGoAlgorandSignedTxn() {
        val pq = pqSig(salt = 3u)
        val encoded = encodeSignedTransaction(SignedTransaction(transaction = payment(), pqSignature = pq))

        // `{pqsig: {pk, sch, sig, slt}, txn}` with canonical (sorted) keys.
        val header = byteArrayOf(0x82.toByte(), 0xa5.toByte()) + "pqsig".toByteArray() +
            byteArrayOf(0x84.toByte(), 0xa2.toByte()) + "pk".toByteArray()
        assertArrayEquals(header, encoded.copyOfRange(0, header.size))

        val decoded = decodeSignedTransaction(encoded)
        assertNull(decoded.signature)
        val got = decoded.pqSignature!!
        assertArrayEquals(pq.publicKey, got.publicKey)
        assertArrayEquals(pq.scheme, got.scheme)
        assertArrayEquals(pq.signature, got.signature)
        assertEquals(pq.salt, got.salt)
        assertEquals(payment().sender, decoded.transaction.sender)
    }

    @Test
    fun zeroSaltIsOmitted() {
        val encoded = encodeSignedTransaction(SignedTransaction(transaction = payment(), pqSignature = pqSig(0u)))
        // 3-entry pqsig map (no `slt`).
        assertEquals(0x83.toByte(), encoded[7])
        assertTrue(!String(encoded, Charsets.ISO_8859_1).contains("slt"))
        assertEquals(0.toUByte(), decodeSignedTransaction(encoded).pqSignature!!.salt)
    }
}
