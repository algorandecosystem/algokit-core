import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Assert.fail
import org.junit.Test
import uniffi.seed_vault_ffi.SeedVaultClient
import uniffi.seed_vault_ffi.SeedVaultException
import uniffi.seed_vault_ffi.SeedVaultStatus
import uniffi.seed_vault_ffi.clientProtocolVersion

/** Host (JVM) tests against the host build of `seed_vault_ffi`, where no Trusty exists. */
class SeedVaultClientTest {
    @Test
    fun statusDegradesToNotSupportedWithoutTrusty() {
        val status = SeedVaultClient().status()
        assertTrue("expected NotSupported, got $status", status is SeedVaultStatus.NotSupported)
    }

    @Test
    fun callsThrowTypedNotSupported() {
        try {
            SeedVaultClient().listAccounts()
            fail("expected SeedVaultException.NotSupported")
        } catch (e: SeedVaultException.NotSupported) {
            assertTrue(e.reason.isNotEmpty())
        }
    }

    @Test
    fun exposesProtocolVersion() {
        assertEquals(1, clientProtocolVersion().major.toInt())
    }
}
