package io.github.algorandecosystem.algokitcore.seedvault

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Button
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.font.FontStyle
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.text.input.KeyboardCapitalization
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import kotlinx.coroutines.Dispatchers
import java.text.SimpleDateFormat
import java.util.Date
import java.util.Locale

/**
 * A deliberately minimal exerciser for `seed_vault_ta`: four buttons that
 * each open a fresh TIPC connection, send one request, decode + display the
 * response, and close the connection again. Good enough to prove the TA
 * running on a real (or emulated, e.g. Cuttlefish) Trusty device is
 * reachable and behaving correctly from the Android side -- not meant to be
 * the production wallet-facing service (that would front this same
 * request/response flow with an AIDL `bindService()` API instead of a UI).
 *
 * See `trusty/app/seed_vault_ta/main.rs` for the TA this talks to and
 * `SeedVaultProtocol` for the wire format mirrored here from
 * `crates/seed_vault/src/protocol.rs`.
 */
class MainActivity : ComponentActivity() {

    private val timeFormat = SimpleDateFormat("HH:mm:ss", Locale.US)

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            MaterialTheme {
                Surface(modifier = Modifier.fillMaxSize()) {
                    SeedVaultScreen()
                }
            }
        }
    }

    @Composable
    private fun SeedVaultScreen() {
        var address by remember { mutableStateOf("") }
        var mnemonic by remember { mutableStateOf("") }
        var output by remember { mutableStateOf(getString(R.string.output_placeholder)) }
        val scope = rememberCoroutineScope()

        fun appendOutput(text: String) {
            val timestamp = timeFormat.format(Date())
            output += "\n[$timestamp] $text\n"
        }

        /** Opens a fresh connection off the UI thread, runs [body], prints its result or failure, always closes. */
        fun runRequest(label: String, body: (TipcClient) -> String) {
            appendOutput("-> $label")
            scope.launch {
                val result = withContext(Dispatchers.IO) {
                    val client = TipcClient()
                    try {
                        client.connect()
                        body(client)
                    } catch (e: Exception) {
                        "FAILED: ${e.message}"
                    } finally {
                        client.close()
                    }
                }
                appendOutput(result)
            }
        }

        fun formatAccount(label: String, account: WireAccount): String =
            "$label:\n  address=${account.address}\n  algorithm=${account.algorithm}\n  index=${account.addressIndex}"

        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(16.dp),
            verticalArrangement = Arrangement.spacedBy(8.dp),
        ) {
            Text(
                text = "io.github.algorandecosystem.algokitcore.seed_vault_ta over /dev/trusty-ipc-dev0",
                fontSize = 12.sp,
                fontStyle = FontStyle.Italic,
            )

            Button(
                onClick = {
                    runRequest("CREATE_ACCOUNT") { client ->
                        val response = client.sendAndReceive(SeedVaultProtocol.encodeCreateAccount())
                        val account = SeedVaultProtocol.decodeAccountResponse(response)
                        // A freshly created address is the most useful thing to
                        // have sitting in the address box for a follow-up "Reveal
                        // Mnemonic" tap.
                        address = account.address
                        formatAccount("Created account", account)
                    }
                },
                modifier = Modifier.fillMaxWidth(),
            ) {
                Text(stringResource(R.string.btn_create_account))
            }

            Button(
                onClick = {
                    runRequest("LIST_ACCOUNTS") { client ->
                        val response = client.sendAndReceive(SeedVaultProtocol.encodeListAccounts())
                        val accounts = SeedVaultProtocol.decodeListAccountsResponse(response)
                        if (accounts.isEmpty()) {
                            "No accounts stored yet -- tap Create Account first."
                        } else {
                            buildString {
                                append("${accounts.size} account(s):\n")
                                accounts.forEachIndexed { i, a ->
                                    append("  [$i] ${a.address}  (${a.algorithm}, index=${a.addressIndex})\n")
                                }
                            }
                        }
                    }
                },
                modifier = Modifier.fillMaxWidth(),
            ) {
                Text(stringResource(R.string.btn_list_accounts))
            }

            OutlinedTextField(
                value = address,
                onValueChange = { address = it },
                label = { Text(stringResource(R.string.hint_address)) },
                singleLine = true,
                keyboardOptions = KeyboardOptions(
                    capitalization = KeyboardCapitalization.None,
                    imeAction = ImeAction.Done,
                ),
                modifier = Modifier.fillMaxWidth(),
            )

            Button(
                onClick = {
                    val trimmed = address.trim()
                    if (trimmed.isEmpty()) {
                        appendOutput("Enter an Algorand address above first (or tap Create Account).")
                    } else {
                        runRequest("REVEAL_MNEMONIC") { client ->
                            val response = client.sendAndReceive(SeedVaultProtocol.encodeRevealMnemonic(trimmed))
                            val revealed = SeedVaultProtocol.decodeRevealMnemonicResponse(response)
                            "Mnemonic for $trimmed:\n\n$revealed"
                        }
                    }
                },
                modifier = Modifier.fillMaxWidth(),
            ) {
                Text(stringResource(R.string.btn_reveal_mnemonic))
            }

            OutlinedTextField(
                value = mnemonic,
                onValueChange = { mnemonic = it },
                label = { Text(stringResource(R.string.hint_mnemonic)) },
                keyboardOptions = KeyboardOptions(
                    capitalization = KeyboardCapitalization.None,
                    imeAction = ImeAction.Done,
                ),
                modifier = Modifier
                    .fillMaxWidth()
                    .height(96.dp),
            )

            Button(
                onClick = {
                    val trimmed = mnemonic.trim()
                    if (trimmed.isEmpty()) {
                        appendOutput("Enter a 25-word mnemonic above first.")
                    } else {
                        runRequest("IMPORT_ACCOUNT") { client ->
                            val response = client.sendAndReceive(SeedVaultProtocol.encodeImportAccount(trimmed))
                            val account = SeedVaultProtocol.decodeAccountResponse(response)
                            formatAccount("Imported account", account)
                        }
                    }
                },
                modifier = Modifier.fillMaxWidth(),
            ) {
                Text(stringResource(R.string.btn_import_account))
            }

            Text(
                text = stringResource(R.string.label_output),
                fontWeight = FontWeight.Bold,
            )

            Text(
                text = output,
                fontFamily = FontFamily.Monospace,
                fontSize = 13.sp,
                modifier = Modifier
                    .fillMaxWidth()
                    .weight(1f)
                    .verticalScroll(rememberScrollState())
                    .padding(12.dp),
            )
        }
    }
}
