import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.Assert.assertFalse
import org.junit.Test
import org.junit.runner.RunWith
import uniffi.seed_vault_ffi.SeedVaultClient
import uniffi.seed_vault_ffi.SeedVaultStatus

/**
 * Runs on a stock emulator (no Trusty, no `seed_vault_ta`), so it checks the
 * path every regular phone takes: the probe reports why the vault is
 * unavailable instead of crashing or hanging.
 */
@RunWith(AndroidJUnit4::class)
class SeedVaultClientInstrumentedTest {
    @Test
    fun probeReportsUnavailableOnStockEmulator() {
        for (devicePath in listOf("VSOCK:200:0", "/dev/trusty-ipc-dev0")) {
            val client = SeedVaultClient.withConfig(
                devicePath,
                "io.github.algorandecosystem.algokitcore.seed_vault_ta",
                2_000u,
            )
            val status = client.status()
            assertFalse("$devicePath: $status", status is SeedVaultStatus.Available)
        }
    }
}
