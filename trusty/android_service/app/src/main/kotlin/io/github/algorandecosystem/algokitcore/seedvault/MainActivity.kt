package io.github.algorandecosystem.algokitcore.seedvault

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Button
import androidx.compose.material3.ButtonDefaults
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Surface
import androidx.compose.material3.Switch
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
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
import java.util.Base64
import java.util.Date
import java.util.Locale

/**
 * A deliberately minimal exerciser for `seed_vault_ta`: buttons that
 * each open a fresh TIPC connection, send one request, decode + display the
 * response, and close the connection again. Signed transactions can then be
 * submitted to an algod node ([AlgodClient]) to prove the TA's Ed25519 /
 * Falcon-1024 signatures are accepted on-chain. Good enough to prove the TA
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
        var txnBase64 by remember { mutableStateOf("") }
        // Which of a seed's keys Create/Import return and Sign signs with.
        var addressIndex by remember { mutableStateOf(AddressIndex.DEFAULT) }
        var algodUrl by remember { mutableStateOf("") }
        var algodToken by remember { mutableStateOf("") }
        var lastSignedTxn by remember { mutableStateOf<ByteArray?>(null) }
        var output by remember { mutableStateOf(getString(R.string.output_placeholder)) }
        val scope = rememberCoroutineScope()

        fun appendOutput(text: String) {
            val timestamp = timeFormat.format(Date())
            output += "\n[$timestamp] $text\n"
        }

        /** Runs [body] off the UI thread and prints its result or failure. */
        fun runBackground(label: String, body: () -> String) {
            appendOutput("-> $label")
            scope.launch {
                val result = withContext(Dispatchers.IO) {
                    try {
                        body()
                    } catch (e: Exception) {
                        "FAILED: ${e.message}"
                    }
                }
                appendOutput(result)
            }
        }

        /** Like [runBackground], with a fresh TIPC connection that is always closed afterwards. */
        fun runRequest(label: String, body: (TipcClient) -> String) = runBackground(label) {
            val client = TipcClient()
            try {
                client.connect()
                body(client)
            } finally {
                client.close()
            }
        }

        fun formatAccount(label: String, account: WireAccount): String =
            "$label:\n  address=${account.address}\n  algorithm=${account.algorithm}\n" +
                "  index=${account.addressIndex} (${AddressIndex.label(account.addressIndex)})"

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

            Row(verticalAlignment = Alignment.CenterVertically) {
                Switch(
                    checked = addressIndex == AddressIndex.AUTOSIGN,
                    onCheckedChange = {
                        addressIndex = if (it) AddressIndex.AUTOSIGN else AddressIndex.HUMAN_SIGN
                    },
                )
                Text(
                    text = stringResource(R.string.label_autosign),
                    modifier = Modifier.padding(start = 8.dp),
                )
            }

            Button(
                onClick = {
                    runRequest("CREATE_ACCOUNT") { client ->
                        val response = client.sendAndReceive(SeedVaultProtocol.encodeCreateAccount(addressIndex = addressIndex))
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
                        val accounts = SeedVaultProtocol.listAllAccounts(client::sendAndReceive)
                        if (accounts.isEmpty()) {
                            "No accounts stored yet -- tap Create Account first."
                        } else {
                            buildString {
                                append("${accounts.size} account(s):\n")
                                accounts.forEachIndexed { i, listed ->
                                    val a = listed.account
                                    val pk = Base64.getEncoder().encodeToString(listed.publicKey)
                                    val role = AddressIndex.label(a.addressIndex)
                                    append("  [$i] ${a.address}  (${a.algorithm}, index=${a.addressIndex} $role)\n")
                                    // Falcon keys are ~2.4 KB of base64; show the start only.
                                    append("      pk (${listed.publicKey.size} bytes) = ${pk.take(44)}")
                                    append(if (pk.length > 44) "...\n" else "\n")
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

            Button(
                onClick = {
                    val trimmed = address.trim()
                    if (trimmed.isEmpty()) {
                        appendOutput("Enter an Algorand address above first (tap List Accounts to see them).")
                    } else {
                        runRequest("DELETE_ACCOUNT") { client ->
                            val response = client.sendAndReceive(SeedVaultProtocol.encodeDeleteAccount(trimmed))
                            val account = SeedVaultProtocol.decodeAccountResponse(response)
                            address = ""
                            formatAccount("Deleted account", account)
                        }
                    }
                },
                colors = ButtonDefaults.buttonColors(
                    containerColor = MaterialTheme.colorScheme.error,
                    contentColor = MaterialTheme.colorScheme.onError,
                ),
                modifier = Modifier.fillMaxWidth(),
            ) {
                Text(stringResource(R.string.btn_delete_account))
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
                            val response = client.sendAndReceive(SeedVaultProtocol.encodeImportAccount(trimmed, addressIndex = addressIndex))
                            val account = SeedVaultProtocol.decodeAccountResponse(response)
                            formatAccount("Imported account", account)
                        }
                    }
                },
                modifier = Modifier.fillMaxWidth(),
            ) {
                Text(stringResource(R.string.btn_import_account))
            }

            OutlinedTextField(
                value = algodUrl,
                onValueChange = { algodUrl = it },
                label = { Text(stringResource(R.string.hint_algod_url)) },
                singleLine = true,
                keyboardOptions = KeyboardOptions(
                    capitalization = KeyboardCapitalization.None,
                    imeAction = ImeAction.Next,
                ),
                modifier = Modifier.fillMaxWidth(),
            )

            OutlinedTextField(
                value = algodToken,
                onValueChange = { algodToken = it },
                label = { Text(stringResource(R.string.hint_algod_token)) },
                singleLine = true,
                keyboardOptions = KeyboardOptions(
                    capitalization = KeyboardCapitalization.None,
                    imeAction = ImeAction.Done,
                ),
                modifier = Modifier.fillMaxWidth(),
            )

            Button(
                onClick = {
                    val sender = address.trim()
                    when {
                        sender.isEmpty() ->
                            appendOutput("Enter the sender's address above first (tap List Accounts to see them).")
                        algodUrl.isBlank() -> appendOutput("Enter an algod URL above first.")
                        else -> runRequest("BUILD_TEST_PAYMENT") { client ->
                            // The fee depends on the sender's signature scheme.
                            val accounts = SeedVaultProtocol.listAllAccounts(client::sendAndReceive)
                            val algorithm = accounts.find { it.account.address == sender }?.account?.algorithm
                                ?: throw SeedVaultError.AccountNotFound
                            val algod = AlgodClient(algodUrl, algodToken)
                            val params = algod.suggestedParams()
                            val fee = AlgodClient.feeFor(algorithm, params.minFee)
                            val note = "seed_vault_ta test ${System.currentTimeMillis()}".toByteArray()
                            val txn = AlgodClient.selfPayment(sender, params, fee, note)
                            txnBase64 = Base64.getEncoder().encodeToString(txn)
                            "Built 0-ALGO self-payment on ${params.genesisId} for $sender ($algorithm), " +
                                "fee=$fee microAlgos, rounds ${params.firstValid}-${params.firstValid + 1000}. " +
                                "Fund the account with at least the fee, then tap Sign Transaction."
                        }
                    }
                },
                modifier = Modifier.fillMaxWidth(),
            ) {
                Text(stringResource(R.string.btn_build_test_payment))
            }

            OutlinedTextField(
                value = txnBase64,
                onValueChange = { txnBase64 = it },
                label = { Text(stringResource(R.string.hint_txn)) },
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
                    val trimmedAddress = address.trim()
                    val txn = try {
                        Base64.getDecoder().decode(txnBase64.filterNot { it.isWhitespace() })
                    } catch (e: IllegalArgumentException) {
                        null
                    }
                    when {
                        trimmedAddress.isEmpty() ->
                            appendOutput("Enter the signing account's address above first (tap List Accounts to see them).")
                        txn == null || txn.isEmpty() ->
                            appendOutput("Enter a base64 msgpack transaction above (algosdk txn.encode(), no \"TX\" prefix).")
                        else -> runRequest("SIGN_TRANSACTION") { client ->
                            val response = client.sendAndReceive(
                                SeedVaultProtocol.encodeSignTransaction(trimmedAddress, txn, addressIndex),
                            )
                            val signed = SeedVaultProtocol.decodeSignTransactionResponse(response)
                            val signedTxn = signed.signedTxn(txn)
                            lastSignedTxn = signedTxn
                            "Signed by $trimmedAddress (${signed.algorithm}), ${signedTxn.size} bytes " +
                                "(POST /v2/transactions body), base64:\n\n" +
                                Base64.getEncoder().encodeToString(signedTxn)
                        }
                    }
                },
                modifier = Modifier.fillMaxWidth(),
            ) {
                Text(stringResource(R.string.btn_sign_transaction))
            }

            Button(
                onClick = {
                    val signedTxn = lastSignedTxn
                    when {
                        signedTxn == null -> appendOutput("Tap Sign Transaction first.")
                        algodUrl.isBlank() -> appendOutput("Enter an algod URL above first.")
                        else -> runBackground("SUBMIT ${signedTxn.size} bytes to $algodUrl") {
                            val algod = AlgodClient(algodUrl, algodToken)
                            val txId = algod.submit(signedTxn)
                            appendOutput("Submitted, txId=$txId; waiting for confirmation...")
                            val round = algod.waitForConfirmation(txId)
                            "Confirmed txId=$txId in round $round"
                        }
                    }
                },
                modifier = Modifier.fillMaxWidth(),
            ) {
                Text(stringResource(R.string.btn_submit_transaction))
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
