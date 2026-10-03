package io.github.algorandecosystem.algokitcore.seedvault

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Button
import androidx.compose.material3.ButtonColors
import androidx.compose.material3.ButtonDefaults
import androidx.compose.material3.DropdownMenuItem
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.ExposedDropdownMenuAnchorType
import androidx.compose.material3.ExposedDropdownMenuBox
import androidx.compose.material3.ExposedDropdownMenuDefaults
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.ScrollableTabRow
import androidx.compose.material3.Surface
import androidx.compose.material3.Switch
import androidx.compose.material3.Tab
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
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
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.text.style.TextOverflow
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

    @OptIn(ExperimentalMaterial3Api::class)
    @Composable
    private fun SeedVaultScreen() {
        var address by remember { mutableStateOf("") }
        // Every account stored in the TA, for the account dropdown. Loaded on launch and
        // refreshed after Create / Import / Delete / List.
        var accounts by remember { mutableStateOf(emptyList<WireAccount>()) }
        var accountMenuExpanded by remember { mutableStateOf(false) }
        var mnemonic by remember { mutableStateOf("") }
        var txnBase64 by remember { mutableStateOf("") }
        // Which of a seed's keys Create/Import return and Sign signs with.
        var addressIndex by remember { mutableStateOf(AddressIndex.DEFAULT) }
        var algodUrl by remember { mutableStateOf(AlgodClient.NODE_TESTNET_BASE_URL) }
        var algodToken by remember { mutableStateOf("") }
        var lastSignedTxn by remember { mutableStateOf<ByteArray?>(null) }
        var output by remember { mutableStateOf(getString(R.string.output_placeholder)) }
        val scope = rememberCoroutineScope()
        val outputScroll = rememberScrollState()
        val tabTitles = listOf("Accounts", "Import", "Sign & Submit")
        var selectedTab by remember { mutableStateOf(0) }

        // Keep the newest output line in view.
        LaunchedEffect(output) { outputScroll.animateScrollTo(outputScroll.maxValue) }

        fun appendOutput(text: String) {
            val timestamp = timeFormat.format(Date())
            val separator = if (output.isEmpty()) "" else "\n"
            output += "$separator[$timestamp] $text\n"
        }

        /**
         * Clears the previous output, then runs [body] off the UI thread and prints its result
         * or failure. [after] runs once the result is printed.
         */
        fun runBackground(label: String, after: (suspend () -> Unit)? = null, body: () -> String) {
            output = ""
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
                after?.invoke()
            }
        }

        /** Like [runBackground], with a fresh TIPC connection that is always closed afterwards. */
        fun runRequest(label: String, after: (suspend () -> Unit)? = null, body: (TipcClient) -> String) =
            runBackground(label, after) {
                val client = TipcClient()
                try {
                    client.connect()
                    body(client)
                } finally {
                    client.close()
                }
            }

        /** Replaces the dropdown's account list, keeping the selection if it still exists. */
        fun updateAccounts(updated: List<WireAccount>) {
            accounts = updated
            if (updated.none { it.address == address }) {
                address = updated.firstOrNull()?.address ?: ""
            }
        }

        /**
         * Re-reads every stored account from the TA over a fresh connection and refreshes the
         * dropdown, selecting [select] if given. Leaves the output panel alone on success.
         */
        suspend fun refreshAccounts(select: String? = null) {
            val loaded = withContext(Dispatchers.IO) {
                val client = TipcClient()
                try {
                    client.connect()
                    SeedVaultProtocol.listAllAccounts(client::sendAndReceive).map { it.account }
                } catch (e: Exception) {
                    null
                } finally {
                    client.close()
                }
            }
            if (loaded != null) {
                if (select != null) address = select
                updateAccounts(loaded)
            } else {
                appendOutput("Could not load accounts from seed_vault_ta -- tap List Accounts to retry.")
            }
        }

        // Populate the dropdown on launch.
        LaunchedEffect(Unit) { refreshAccounts() }

        fun formatAccount(label: String, account: WireAccount): String =
            "$label:\n  address=${account.address}\n  algorithm=${account.algorithm}\n" +
                "  index=${account.addressIndex} (${AddressIndex.label(account.addressIndex)})"

        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(horizontal = 12.dp, vertical = 8.dp),
            verticalArrangement = Arrangement.spacedBy(4.dp),
        ) {
            Text(
                text = "${TipcClient.DEFAULT_SERVICE_NAME} over ${TipcClient.DEFAULT_DEVICE_PATH}",
                fontSize = 11.sp,
                fontStyle = FontStyle.Italic,
                maxLines = 1,
                overflow = TextOverflow.MiddleEllipsis,
            )

            // Always visible: which key type to use, and which account to act on.
            Row(verticalAlignment = Alignment.CenterVertically) {
                Switch(
                    checked = addressIndex == AddressIndex.AUTOSIGN,
                    onCheckedChange = {
                        addressIndex = if (it) AddressIndex.AUTOSIGN else AddressIndex.HUMAN_SIGN
                    },
                )
                Text(
                    text = stringResource(R.string.label_autosign),
                    fontSize = 12.sp,
                    modifier = Modifier.padding(start = 8.dp),
                )
            }

            ExposedDropdownMenuBox(
                expanded = accountMenuExpanded,
                onExpandedChange = { accountMenuExpanded = it && accounts.isNotEmpty() },
            ) {
                OutlinedTextField(
                    value = address.ifEmpty { "No accounts yet -- tap Create Account" },
                    onValueChange = {},
                    readOnly = true,
                    label = { Text(stringResource(R.string.hint_address)) },
                    textStyle = MaterialTheme.typography.bodySmall.copy(fontFamily = FontFamily.Monospace),
                    trailingIcon = { ExposedDropdownMenuDefaults.TrailingIcon(expanded = accountMenuExpanded) },
                    modifier = Modifier
                        .fillMaxWidth()
                        .menuAnchor(ExposedDropdownMenuAnchorType.PrimaryNotEditable),
                )
                ExposedDropdownMenu(
                    expanded = accountMenuExpanded,
                    onDismissRequest = { accountMenuExpanded = false },
                ) {
                    accounts.forEachIndexed { i, account ->
                        DropdownMenuItem(
                            text = {
                                // One line per account: shorten the middle of the address
                                // rather than wrapping when it doesn't fit.
                                Text(
                                    text = "[$i] ${account.address}",
                                    fontFamily = FontFamily.Monospace,
                                    fontSize = 12.sp,
                                    maxLines = 1,
                                    softWrap = false,
                                    overflow = TextOverflow.MiddleEllipsis,
                                )
                            },
                            onClick = {
                                address = account.address
                                accountMenuExpanded = false
                            },
                        )
                    }
                }
            }

            ScrollableTabRow(selectedTabIndex = selectedTab, edgePadding = 0.dp) {
                tabTitles.forEachIndexed { index, title ->
                    Tab(
                        selected = selectedTab == index,
                        onClick = { selectedTab = index },
                        text = { Text(title, fontSize = 13.sp, maxLines = 1) },
                    )
                }
            }

            // Tab content scrolls on its own; the output panel below stays pinned.
            Column(
                modifier = Modifier
                    .fillMaxWidth()
                    .weight(1f)
                    .verticalScroll(rememberScrollState()),
                verticalArrangement = Arrangement.spacedBy(6.dp),
            ) {
                when (selectedTab) {
                    0 -> {
                        Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                            CompactButton(
                                text = stringResource(R.string.btn_create_account),
                                modifier = Modifier.weight(1f),
                                onClick = {
                                    var created: String? = null
                                    // Refresh the dropdown from the TA and select the freshly created
                                    // account for a follow-up "Reveal Mnemonic" tap.
                                    runRequest("CREATE_ACCOUNT", after = { created?.let { refreshAccounts(select = it) } }) { client ->
                                        val response = client.sendAndReceive(SeedVaultProtocol.encodeCreateAccount(addressIndex = addressIndex))
                                        val account = SeedVaultProtocol.decodeAccountResponse(response)
                                        created = account.address
                                        formatAccount("Created account", account)
                                    }
                                },
                            )
                            CompactButton(
                                text = stringResource(R.string.btn_list_accounts),
                                modifier = Modifier.weight(1f),
                                onClick = {
                                    runRequest("LIST_ACCOUNTS") { client ->
                                        val listedAccounts = SeedVaultProtocol.listAllAccounts(client::sendAndReceive)
                                        updateAccounts(listedAccounts.map { it.account })
                                        if (listedAccounts.isEmpty()) {
                                            "No accounts stored yet -- tap Create Account first."
                                        } else {
                                            buildString {
                                                append("${listedAccounts.size} account(s):\n")
                                                listedAccounts.forEachIndexed { i, listed ->
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
                            )
                        }
                        Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                            CompactButton(
                                text = stringResource(R.string.btn_reveal_mnemonic),
                                modifier = Modifier.weight(1f),
                                onClick = {
                                    val trimmed = address.trim()
                                    if (trimmed.isEmpty()) {
                                        appendOutput("Select an account above first (or tap Create Account).")
                                    } else {
                                        runRequest("REVEAL_MNEMONIC") { client ->
                                            val response = client.sendAndReceive(SeedVaultProtocol.encodeRevealMnemonic(trimmed))
                                            val revealed = SeedVaultProtocol.decodeRevealMnemonicResponse(response)
                                            "Mnemonic for $trimmed:\n\n$revealed"
                                        }
                                    }
                                },
                            )
                            CompactButton(
                                text = stringResource(R.string.btn_delete_account),
                                modifier = Modifier.weight(1f),
                                colors = ButtonDefaults.buttonColors(
                                    containerColor = MaterialTheme.colorScheme.error,
                                    contentColor = MaterialTheme.colorScheme.onError,
                                ),
                                onClick = {
                                    val trimmed = address.trim()
                                    if (trimmed.isEmpty()) {
                                        appendOutput("Select an account above first (tap List Accounts to see them).")
                                    } else {
                                        runRequest("DELETE_ACCOUNT", after = { refreshAccounts() }) { client ->
                                            val response = client.sendAndReceive(SeedVaultProtocol.encodeDeleteAccount(trimmed))
                                            val account = SeedVaultProtocol.decodeAccountResponse(response)
                                            formatAccount("Deleted account", account)
                                        }
                                    }
                                },
                            )
                        }
                    }

                    1 -> {
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
                        CompactButton(
                            text = stringResource(R.string.btn_import_account),
                            modifier = Modifier.fillMaxWidth(),
                            onClick = {
                                val trimmed = mnemonic.trim()
                                if (trimmed.isEmpty()) {
                                    appendOutput("Enter a 25-word mnemonic above first.")
                                } else {
                                    var imported: String? = null
                                    runRequest("IMPORT_ACCOUNT", after = { imported?.let { refreshAccounts(select = it) } }) { client ->
                                        val response = client.sendAndReceive(SeedVaultProtocol.encodeImportAccount(trimmed, addressIndex = addressIndex))
                                        val account = SeedVaultProtocol.decodeAccountResponse(response)
                                        imported = account.address
                                        formatAccount("Imported account", account)
                                    }
                                }
                            },
                        )
                    }

                    else -> {
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
                        CompactButton(
                            text = stringResource(R.string.btn_build_test_payment),
                            modifier = Modifier.fillMaxWidth(),
                            onClick = {
                                val sender = address.trim()
                                when {
                                    sender.isEmpty() ->
                                        appendOutput("Select the sender account above first (tap List Accounts to see them).")
                                    algodUrl.isBlank() -> appendOutput("Enter an algod URL above first.")
                                    else -> runRequest("BUILD_TEST_PAYMENT") { client ->
                                        // The fee depends on the sender's signature scheme.
                                        val listed = SeedVaultProtocol.listAllAccounts(client::sendAndReceive)
                                        val algorithm = listed.find { it.account.address == sender }?.account?.algorithm
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
                        )
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
                        Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                            CompactButton(
                                text = stringResource(R.string.btn_sign_transaction),
                                modifier = Modifier.weight(1f),
                                onClick = {
                                    val trimmedAddress = address.trim()
                                    val txn = try {
                                        Base64.getDecoder().decode(txnBase64.filterNot { it.isWhitespace() })
                                    } catch (e: IllegalArgumentException) {
                                        null
                                    }
                                    when {
                                        trimmedAddress.isEmpty() ->
                                            appendOutput("Select the signing account above first (tap List Accounts to see them).")
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
                            )
                            CompactButton(
                                text = stringResource(R.string.btn_submit_transaction),
                                modifier = Modifier.weight(1f),
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
                            )
                        }
                    }
                }
            }

            Text(
                text = stringResource(R.string.label_output),
                fontWeight = FontWeight.Bold,
            )

            // Pinned output panel; scrolls itself and follows the newest line.
            Text(
                text = output,
                fontFamily = FontFamily.Monospace,
                fontSize = 12.sp,
                modifier = Modifier
                    .fillMaxWidth()
                    .weight(1f)
                    .verticalScroll(outputScroll)
                    .padding(8.dp),
            )
        }
    }

    @Composable
    private fun CompactButton(
        text: String,
        onClick: () -> Unit,
        modifier: Modifier = Modifier,
        colors: ButtonColors = ButtonDefaults.buttonColors(),
    ) {
        Button(
            onClick = onClick,
            colors = colors,
            modifier = modifier,
            contentPadding = PaddingValues(horizontal = 8.dp, vertical = 4.dp),
        ) {
            Text(text, fontSize = 12.sp, maxLines = 2, textAlign = TextAlign.Center)
        }
    }
}
