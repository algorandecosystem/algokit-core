import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.test.platform.app.InstrumentationRegistry
import org.junit.After
import org.junit.Assert.assertArrayEquals
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Assert.fail
import org.junit.Assume.assumeTrue
import org.junit.Before
import org.junit.Test
import org.junit.runner.RunWith
import uniffi.seed_vault_ffi.Account
import uniffi.seed_vault_ffi.AppletErrorCode
import uniffi.seed_vault_ffi.KeyAlgorithm
import uniffi.seed_vault_ffi.SeedVaultClient
import uniffi.seed_vault_ffi.SeedVaultException
import uniffi.seed_vault_ffi.SeedVaultStatus
import uniffi.seed_vault_ffi.autosignIndex
import uniffi.seed_vault_ffi.humanSignIndex

/**
 * Exercises a real `seed_vault_ta` through the library, the way an app does.
 *
 * Every test is skipped (not failed) unless [SeedVaultClient.status] is
 * `Available`, so this runs green on a stock CI emulator and does the full
 * round trip on Cuttlefish with `seed_vault_ta` built into Trusty (SELinux
 * must let the test app open the vsock -- for local bring-up,
 * `adb root && adb shell setenforce 0`):
 *
 * ```
 * ./gradlew connectedAndroidTest
 * ```
 *
 * [submitsSignedPaymentToAlgod] additionally needs a node and a funded
 * account, passed as instrumentation arguments:
 *
 * ```
 * ./gradlew connectedAndroidTest \
 *   -Pandroid.testInstrumentationRunnerArguments.algodUrl=https://testnet-api.algonode.cloud \
 *   -Pandroid.testInstrumentationRunnerArguments.fundedMnemonic="word1 ... word25" \
 *   -Pandroid.testInstrumentationRunnerArguments.fundedAlgorithm=ALGO25_ED25519
 * ```
 *
 * Optional: `algodToken`, `devicePath` (default `VSOCK:200:0`; use
 * `/dev/trusty-ipc-dev0` on kernel-driver targets).
 */
@RunWith(AndroidJUnit4::class)
class SeedVaultEndToEndTest {
    private val args = InstrumentationRegistry.getArguments()
    private lateinit var vault: SeedVaultClient

    /** Accounts a test created; deleted in [tearDown] so reruns start clean. */
    private val created = mutableListOf<String>()

    @Before
    fun setUp() {
        vault = args.getString("devicePath")?.let {
            SeedVaultClient.withConfig(it, SeedVaultClient().serviceName(), 10_000u)
        } ?: SeedVaultClient()
        val status = vault.status()
        assumeTrue("seed vault not available: $status", status is SeedVaultStatus.Available)
    }

    @After
    fun tearDown() {
        for (address in created) {
            try {
                vault.deleteAccount(address)
            } catch (_: SeedVaultException) {
                // Already deleted by the test.
            }
        }
    }

    private fun create(algorithm: KeyAlgorithm, index: UByte): Account =
        vault.createAccount(algorithm, index).also { created += it.address }

    @Test
    fun createListRevealDelete() {
        for ((algorithm, index) in listOf(
            KeyAlgorithm.ALGO25_ED25519 to humanSignIndex(),
            KeyAlgorithm.FALCON1024 to humanSignIndex(),
            KeyAlgorithm.FALCON1024 to autosignIndex(),
        )) {
            val account = create(algorithm, index)
            assertEquals(algorithm, account.algorithm)
            assertEquals(index.toUInt(), account.addressIndex)
            assertEquals(58, account.address.length)

            val listed = vault.listAccounts().single { it.account.address == account.address }
            val expectedKeySize = if (algorithm == KeyAlgorithm.FALCON1024) 1793 else 32
            assertEquals(expectedKeySize, listed.publicKey.size)

            assertEquals(25, vault.revealMnemonic(account.address).split(" ").size)

            assertEquals(account.address, vault.deleteAccount(account.address).address)
            assertTrue(vault.listAccounts().none { it.account.address == account.address })
            expectAppletError(AppletErrorCode.ACCOUNT_NOT_FOUND) { vault.revealMnemonic(account.address) }
        }
    }

    @Test
    fun importRestoresTheSameAccount() {
        val original = create(KeyAlgorithm.FALCON1024, autosignIndex())
        val mnemonic = vault.revealMnemonic(original.address)

        expectAppletError(AppletErrorCode.ACCOUNT_ALREADY_EXISTS) {
            vault.importAccount(mnemonic, KeyAlgorithm.FALCON1024, autosignIndex())
        }

        vault.deleteAccount(original.address)
        // Extra whitespace between words is accepted.
        val imported = vault.importAccount(
            mnemonic.replace(" ", "  \n"),
            KeyAlgorithm.FALCON1024,
            autosignIndex(),
        )
        created += imported.address
        assertEquals(original, imported)
    }

    @Test
    fun rejectsInvalidMnemonic() {
        expectAppletError(AppletErrorCode.INVALID_MNEMONIC) {
            vault.importAccount(List(25) { "notaword" }.joinToString(" "), KeyAlgorithm.ALGO25_ED25519, humanSignIndex())
        }
    }

    @Test
    fun signsTransactions() {
        for ((algorithm, index) in listOf(
            KeyAlgorithm.ALGO25_ED25519 to humanSignIndex(),
            KeyAlgorithm.FALCON1024 to autosignIndex(),
        )) {
            val account = create(algorithm, index)
            val txn = AlgodTestClient.payment(
                account.address,
                AlgodTestClient.OFFLINE_PARAMS,
                AlgodTestClient.feeFor(algorithm, 1000),
                "seed_vault e2e".toByteArray(),
            )
            val signed = vault.signTransaction(account.address, index, txn)
            assertEquals(algorithm, signed.algorithm)
            // The library appends the unsigned txn after the signature prefix.
            assertArrayEquals(txn, signed.signedTxn.copyOfRange(signed.signedTxn.size - txn.size, signed.signedTxn.size))
            // The bare signature is the one embedded in the SignedTxn.
            if (algorithm == KeyAlgorithm.ALGO25_ED25519) assertEquals(64, signed.signature.size)
            assertTrue(signed.signedTxn.indexOf(signed.signature) >= 0)
        }
    }

    @Test
    fun signsArbitraryDataWithHumanAndAutosignKeys() {
        val challenge = "liquid-auth challenge ${System.nanoTime()}".toByteArray()

        val ed = create(KeyAlgorithm.ALGO25_ED25519, humanSignIndex())
        val edSig = vault.signArbitraryData(ed.address, humanSignIndex(), challenge)
        assertEquals(KeyAlgorithm.ALGO25_ED25519, edSig.algorithm)
        assertEquals(64, edSig.signature.size)
        // Ed25519 is deterministic.
        assertArrayEquals(edSig.signature, vault.signArbitraryData(ed.address, humanSignIndex(), challenge).signature)
        // Ed25519 seeds have no autosign key.
        expectAppletError(AppletErrorCode.UNKNOWN_ADDRESS_INDEX) {
            vault.signArbitraryData(ed.address, autosignIndex(), challenge)
        }

        val human = create(KeyAlgorithm.FALCON1024, humanSignIndex())
        val autosign = vault.listAccounts()
            .single { it.account.addressIndex == autosignIndex().toUInt() && it.publicKey.contentEquals(publicKeyOf(human)) }
            .account
        val humanSig = vault.signArbitraryData(human.address, humanSignIndex(), challenge)
        val autosignSig = vault.signArbitraryData(autosign.address, autosignIndex(), challenge)
        assertEquals(KeyAlgorithm.FALCON1024, humanSig.algorithm)
        // Both accounts share one Falcon key and signing is deterministic.
        assertArrayEquals(humanSig.signature, autosignSig.signature)
        // The index must name the address's own key.
        expectAppletError(AppletErrorCode.UNKNOWN_ADDRESS_INDEX) {
            vault.signArbitraryData(autosign.address, humanSignIndex(), challenge)
        }
    }

    @Test
    fun signArbitraryDataRefusesAlgorandDomainPrefixes() {
        val account = create(KeyAlgorithm.ALGO25_ED25519, humanSignIndex())
        val txn = AlgodTestClient.payment(account.address, AlgodTestClient.OFFLINE_PARAMS, 1000, ByteArray(0))
        for (data in listOf(
            "TX".toByteArray() + txn,
            "TG".toByteArray(),
            "Program".toByteArray(),
            "ProgData".toByteArray(),
            "MXhello".toByteArray(),
        )) {
            expectAppletError(AppletErrorCode.RESERVED_DOMAIN_PREFIX) {
                vault.signArbitraryData(account.address, humanSignIndex(), data)
            }
        }
        // Lookalikes that aren't a reserved prefix are fine.
        assertEquals(64, vault.signArbitraryData(account.address, humanSignIndex(), "mx tx".toByteArray()).signature.size)
    }

    private fun publicKeyOf(account: Account): ByteArray =
        vault.listAccounts().single { it.account.address == account.address }.publicKey

    private fun ByteArray.indexOf(needle: ByteArray): Int =
        (0..size - needle.size).firstOrNull { i -> needle.indices.all { this[i + it] == needle[it] } } ?: -1

    @Test
    fun refusesToSignForAnotherSender() {
        val signer = create(KeyAlgorithm.ALGO25_ED25519, humanSignIndex())
        val other = create(KeyAlgorithm.ALGO25_ED25519, humanSignIndex())
        val txn = AlgodTestClient.payment(other.address, AlgodTestClient.OFFLINE_PARAMS, 1000, ByteArray(0))
        expectAppletError(AppletErrorCode.SENDER_MISMATCH) {
            vault.signTransaction(signer.address, humanSignIndex(), txn)
        }
    }

    @Test
    fun submitsSignedPaymentToAlgod() {
        val algodUrl = args.getString("algodUrl")
        val fundedMnemonic = args.getString("fundedMnemonic")
        assumeTrue("needs algodUrl + fundedMnemonic instrumentation args", algodUrl != null && fundedMnemonic != null)
        val algorithm = KeyAlgorithm.valueOf(args.getString("fundedAlgorithm") ?: "ALGO25_ED25519")
        val index = if (algorithm == KeyAlgorithm.FALCON1024) autosignIndex() else humanSignIndex()

        val account = vault.importAccount(fundedMnemonic!!, algorithm, index)
        created += account.address

        val algod = AlgodTestClient(algodUrl!!, args.getString("algodToken") ?: "")
        val params = algod.suggestedParams()
        val txn = AlgodTestClient.payment(
            account.address,
            params,
            AlgodTestClient.feeFor(algorithm, params.minFee),
            "seed_vault e2e ${System.currentTimeMillis()}".toByteArray(),
        )
        val signed = vault.signTransaction(account.address, index, txn)
        val txId = algod.submit(signed.signedTxn)
        assertTrue(algod.waitForConfirmation(txId) > 0)
    }

    private fun expectAppletError(code: AppletErrorCode, block: () -> Unit) {
        try {
            block()
            fail("expected applet error $code")
        } catch (e: SeedVaultException.Applet) {
            assertEquals(code, e.code)
        }
    }
}
