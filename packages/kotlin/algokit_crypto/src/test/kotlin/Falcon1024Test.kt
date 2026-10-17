import org.junit.Assert.assertArrayEquals
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertNotEquals
import org.junit.Assert.assertTrue
import org.junit.Assert.assertThrows
import org.junit.Test
import uniffi.algokit_crypto_ffi.AlgoKitFalcon1024Exception
import uniffi.algokit_crypto_ffi.ed25519PublicKeyFromSeed
import uniffi.algokit_crypto_ffi.falcon1024AccountFromEntropy
import uniffi.algokit_crypto_ffi.falcon1024AccountFromMnemonic
import uniffi.algokit_crypto_ffi.falcon1024CanonicalAddress
import uniffi.algokit_crypto_ffi.falcon1024PrivateKeySize
import uniffi.algokit_crypto_ffi.falcon1024PublicKeySize
import uniffi.algokit_crypto_ffi.falcon1024RawSign
import uniffi.algokit_crypto_ffi.falcon1024SignatureMaxSize
import uniffi.algokit_crypto_ffi.falcon1024Verify
import uniffi.algokit_crypto_ffi.mnemonicFromSeed
import uniffi.algokit_crypto_ffi.randomBytes

class Falcon1024Test {
    /** go-algorand `cmd/algokey/pq_test.go`: entropy = 0x01..0x20. */
    private val katEntropy = ByteArray(32) { (it + 1).toByte() }
    private val katAddress = "ZEJ4BLG3XWAUUZQGCEDJLYIC6D2NCWHRSX5DJMDPE54PXXR7G3PCQTARXU"

    @Test
    fun accountFromEntropyMatchesGoAlgorandKnownAnswer() {
        val account = falcon1024AccountFromEntropy(katEntropy)
        assertEquals(katAddress, account.addressString)
        assertEquals(falcon1024PublicKeySize().toInt(), account.publicKey.size)
        assertEquals(falcon1024PrivateKeySize().toInt(), account.privateKey.size)
        assertEquals(32, account.address.size)
    }

    @Test
    fun createBackupRecoverAndSign() {
        // Create from fresh entropy and back up as 25 words.
        val entropy = randomBytes(32u)
        val account = falcon1024AccountFromEntropy(entropy)
        val mnemonic = mnemonicFromSeed(entropy)
        assertEquals(25, mnemonic.split(" ").size)

        // Recover from only the mnemonic.
        val recovered = falcon1024AccountFromMnemonic(mnemonic)
        assertEquals(account.addressString, recovered.addressString)
        assertArrayEquals(account.publicKey, recovered.publicKey)
        assertEquals(account.salt, recovered.salt)

        val canonical = falcon1024CanonicalAddress(account.publicKey)
        assertEquals(account.salt, canonical.salt)
        assertEquals(account.addressString, canonical.addressString)

        // Same words, different (Ed25519) key: the PQ account is distinct.
        assertFalse(ed25519PublicKeyFromSeed(entropy).contentEquals(account.publicKey))

        // Deterministic sign + verify over "TX" || txn-bytes.
        val msg = "TX".toByteArray() + byteArrayOf(1, 2, 3)
        val sig = falcon1024RawSign(recovered.privateKey, msg)
        assertTrue(sig.size <= falcon1024SignatureMaxSize().toInt())
        assertArrayEquals(sig, falcon1024RawSign(account.privateKey, msg))
        assertTrue(falcon1024Verify(account.publicKey, sig, msg))
        assertFalse(falcon1024Verify(account.publicKey, sig, "tampered".toByteArray()))
    }

    @Test
    fun differentAccountsHaveDifferentAddresses() {
        val a = falcon1024AccountFromEntropy(randomBytes(32u))
        val b = falcon1024AccountFromEntropy(randomBytes(32u))
        assertNotEquals(a.addressString, b.addressString)
    }

    @Test
    fun rejectsMalformedInput() {
        assertThrows(AlgoKitFalcon1024Exception::class.java) {
            falcon1024AccountFromEntropy(ByteArray(31))
        }
        assertThrows(AlgoKitFalcon1024Exception::class.java) {
            falcon1024AccountFromMnemonic("not a valid mnemonic")
        }
        assertThrows(AlgoKitFalcon1024Exception::class.java) {
            falcon1024RawSign(ByteArray(10), ByteArray(0))
        }
    }
}
