package io.github.algorandecosystem.algokitcore.seedvault

/**
 * Kotlin mirror of the wire format defined in
 * `crates/seed_vault/src/protocol.rs` -- kept byte-for-byte in sync with
 * that file by hand, since this app has no way to share the Rust crate
 * directly. If you change one, change the other.
 *
 * ## Request
 *
 * | Byte 0                      | Remaining bytes                                                  |
 * |------------------------------|-------------------------------------------------------------------|
 * | `0x01` = CREATE_ACCOUNT      | `[algorithm: u8][address_index: u8]`                              |
 * | `0x02` = LIST_ACCOUNTS        | (none), or `[start: u32 LE]` (first position to return; none = 0)  |
 * | `0x03` = REVEAL_MNEMONIC      | `[addr_len: u8][addr_len bytes of ASCII address]`                  |
 * | `0x04` = IMPORT_ACCOUNT       | `[algorithm: u8][address_index: u8][mnemonic_len: u8][mnemonic_len bytes of ASCII]` |
 * | `0x05` = DELETE_ACCOUNT       | `[addr_len: u8][addr_len bytes of ASCII address]`                  |
 * | `0x06` = SIGN_TRANSACTION     | `[address_index: u8][addr_len: u8][addr_len bytes of ASCII address][txn_len: u16 LE][txn_len bytes]` |
 *
 * ## Response
 *
 * | Byte 0            | Remaining bytes            |
 * |--------------------|------------------------------|
 * | `0x00` = OK         | opcode-specific, see below |
 * | `0x01` = ERR        | 1 byte: [SeedVaultError] code |
 *
 * CREATE_ACCOUNT/IMPORT_ACCOUNT/DELETE_ACCOUNT success payload (for delete:
 * the account that was removed):
 * `[algorithm: u8][address_index: u32 LE][addr_len: u8][addr_len bytes of ASCII address]`.
 *
 * LIST_ACCOUNTS success payload: `[total: u32 LE][count: u32 LE]`, followed by
 * `count` entries of
 * `[algorithm: u8][address_index: u32 LE][addr_len: u8][addr bytes][pk_len: u16 LE][pk bytes]`
 * -- the accounts at positions `start until start + count` of `total`. A
 * Falcon-1024 public key is 1793 bytes, so only ~2 fit per message; use
 * [listAllAccounts] to follow the pages.
 *
 * REVEAL_MNEMONIC success payload:
 * `[mnemonic_len: u8][mnemonic_len bytes of ASCII, space-separated words]`.
 *
 * `address_index` is which of a seed's keys an account is -- see
 * [AddressIndex]. CREATE/IMPORT take the index of the account to return
 * (LIST shows every account of every seed); SIGN takes the index the caller
 * means to sign with, and the TA refuses ([SeedVaultError.UnknownAddressIndex])
 * if the address isn't that account, or ([SeedVaultError.SenderMismatch]) if
 * the transaction's sender isn't the address.
 *
 * SIGN_TRANSACTION takes a msgpack-encoded transaction *without* the `"TX"`
 * prefix (algosdk's `txn.encode()` / `encodeUnsignedTransaction`). Success
 * payload: `[algorithm: u8][prefix_len: u16 LE][prefix_len bytes]`, where the
 * network-ready msgpack SignedTxn is `prefix + txn` (see [SignedTxnPrefix]).
 * Only the prefix comes back because a Trusty IPC message is capped at
 * about one page, and a Falcon-1024 public key + signature alone is ~3.3 KB.
 */
object SeedVaultProtocol {

    private const val OP_CREATE_ACCOUNT: Int = 0x01
    private const val OP_LIST_ACCOUNTS: Int = 0x02
    private const val OP_REVEAL_MNEMONIC: Int = 0x03
    private const val OP_IMPORT_ACCOUNT: Int = 0x04
    private const val OP_DELETE_ACCOUNT: Int = 0x05
    private const val OP_SIGN_TRANSACTION: Int = 0x06

    private const val STATUS_OK: Int = 0x00
    private const val STATUS_ERR: Int = 0x01

    // ---- Requests -----------------------------------------------------

    fun encodeCreateAccount(
        algorithm: KeyAlgorithm = KeyAlgorithm.DEFAULT,
        addressIndex: Int = AddressIndex.DEFAULT,
    ): ByteArray = byteArrayOf(OP_CREATE_ACCOUNT.toByte(), algorithm.tag.toByte(), addressIndex.toByte())

    fun encodeListAccounts(start: Int = 0): ByteArray =
        byteArrayOf(
            OP_LIST_ACCOUNTS.toByte(),
            start.toByte(),
            (start shr 8).toByte(),
            (start shr 16).toByte(),
            (start shr 24).toByte(),
        )

    /** Every account the TA holds, requesting LIST_ACCOUNTS pages via [sendAndReceive] until done. */
    fun listAllAccounts(sendAndReceive: (ByteArray) -> ByteArray): List<ListedAccount> {
        val all = ArrayList<ListedAccount>()
        while (true) {
            val page = decodeListAccountsResponse(sendAndReceive(encodeListAccounts(all.size)))
            all.addAll(page.accounts)
            if (page.accounts.isEmpty() || all.size >= page.total) return all
        }
    }

    fun encodeRevealMnemonic(address: String): ByteArray = encodeAddressRequest(OP_REVEAL_MNEMONIC, address)

    /** Permanently deletes the account for [address]. Irreversible without its mnemonic. */
    fun encodeDeleteAccount(address: String): ByteArray = encodeAddressRequest(OP_DELETE_ACCOUNT, address)

    /**
     * Signs [txn] (msgpack, no `"TX"` prefix) with the account for [address],
     * which must be the seed's [addressIndex] key. The TA adds the `"TX"`
     * prefix itself and refuses anything that isn't a msgpack map.
     */
    fun encodeSignTransaction(
        address: String,
        txn: ByteArray,
        addressIndex: Int = AddressIndex.DEFAULT,
    ): ByteArray {
        val addrBytes = address.trim().toByteArray(Charsets.US_ASCII)
        require(addrBytes.size <= 0xFF) { "address too long (${addrBytes.size} bytes)" }
        val header = byteArrayOf(OP_SIGN_TRANSACTION.toByte(), addressIndex.toByte(), addrBytes.size.toByte()) +
            addrBytes
        // The whole request has to fit in one TIPC message.
        require(header.size + 2 + txn.size <= TipcClient.MAX_MESSAGE_SIZE) {
            "transaction too large for one seed_vault_ta message (${txn.size} bytes)"
        }
        return header + byteArrayOf((txn.size and 0xFF).toByte(), (txn.size shr 8).toByte()) + txn
    }

    private fun encodeAddressRequest(opcode: Int, address: String): ByteArray {
        val addrBytes = address.trim().toByteArray(Charsets.US_ASCII)
        require(addrBytes.size <= 0xFF) { "address too long (${addrBytes.size} bytes)" }
        return byteArrayOf(opcode.toByte(), addrBytes.size.toByte()) + addrBytes
    }

    fun encodeImportAccount(
        mnemonic: String,
        algorithm: KeyAlgorithm = KeyAlgorithm.DEFAULT,
        addressIndex: Int = AddressIndex.DEFAULT,
    ): ByteArray {
        // Words are separated by single spaces on the wire, same as
        // `reveal_mnemonic`'s output -- normalize whitespace defensively in
        // case the user pasted a phrase with newlines/extra spaces.
        val normalized = mnemonic.trim().split(Regex("\\s+")).joinToString(" ")
        val mnemonicBytes = normalized.toByteArray(Charsets.US_ASCII)
        require(mnemonicBytes.size <= 0xFF) { "mnemonic too long (${mnemonicBytes.size} bytes)" }
        return byteArrayOf(
            OP_IMPORT_ACCOUNT.toByte(),
            algorithm.tag.toByte(),
            addressIndex.toByte(),
            mnemonicBytes.size.toByte(),
        ) + mnemonicBytes
    }

    // ---- Responses ------------------------------------------------------

    /** Decodes a CREATE_ACCOUNT/IMPORT_ACCOUNT/DELETE_ACCOUNT success response. */
    fun decodeAccountResponse(bytes: ByteArray): WireAccount {
        val payload = decodeOkPayload(bytes)
        val (account, next) = decodeOneAccount(payload, 0)
        require(next == payload.size) { "trailing bytes after account response" }
        return account
    }

    /** Decodes one page of a LIST_ACCOUNTS success response. */
    fun decodeListAccountsResponse(bytes: ByteArray): ListPage {
        val payload = decodeOkPayload(bytes)
        require(payload.size >= 8) { "truncated list_accounts response" }
        val total = readU32LE(payload, 0)
        val count = readU32LE(payload, 4)
        var offset = 8
        val accounts = ArrayList<ListedAccount>()
        repeat(count) {
            val (account, afterAccount) = decodeOneAccount(payload, offset)
            require(payload.size >= afterAccount + 2) { "truncated public key length" }
            val pkLen = readU16LE(payload, afterAccount)
            val pkStart = afterAccount + 2
            require(payload.size >= pkStart + pkLen) { "truncated public key" }
            accounts.add(ListedAccount(account, payload.copyOfRange(pkStart, pkStart + pkLen)))
            offset = pkStart + pkLen
        }
        require(offset == payload.size) { "trailing bytes after list_accounts response" }
        return ListPage(total, accounts)
    }

    /** Decodes a REVEAL_MNEMONIC success response into its mnemonic string. */
    fun decodeRevealMnemonicResponse(bytes: ByteArray): String {
        val payload = decodeOkPayload(bytes)
        val (mnemonicBytes, next) = decodeLengthPrefixed(payload, 0)
        require(next == payload.size) { "trailing bytes after reveal_mnemonic response" }
        return String(mnemonicBytes, Charsets.US_ASCII)
    }

    /** Decodes a SIGN_TRANSACTION success response. */
    fun decodeSignTransactionResponse(bytes: ByteArray): SignedTxnPrefix {
        val payload = decodeOkPayload(bytes)
        require(payload.size >= 3) { "truncated sign_transaction response" }
        val algorithm = KeyAlgorithm.fromTag(payload[0].toInt() and 0xFF)
            ?: throw SeedVaultError.MalformedResponse
        val len = readU16LE(payload, 1)
        require(payload.size == 3 + len) { "sign_transaction prefix length mismatch" }
        return SignedTxnPrefix(algorithm, payload.copyOfRange(3, 3 + len))
    }

    // ---- Shared decode helpers -----------------------------------------

    /** Strips the status byte, throwing [SeedVaultError] for an ERR response. */
    private fun decodeOkPayload(bytes: ByteArray): ByteArray {
        require(bytes.isNotEmpty()) { "empty response" }
        return when (bytes[0].toInt() and 0xFF) {
            STATUS_OK -> bytes.copyOfRange(1, bytes.size)
            STATUS_ERR -> {
                require(bytes.size >= 2) { "truncated error response" }
                throw SeedVaultError.fromCode(bytes[1].toInt() and 0xFF)
            }
            else -> throw SeedVaultError.MalformedResponse
        }
    }

    private fun decodeOneAccount(bytes: ByteArray, offset: Int): Pair<WireAccount, Int> {
        require(bytes.size > offset) { "truncated account entry" }
        val tag = bytes[offset].toInt() and 0xFF
        val algorithm = KeyAlgorithm.fromTag(tag) ?: throw SeedVaultError.MalformedResponse
        require(bytes.size >= offset + 5) { "truncated account entry (index)" }
        val addressIndex = readU32LE(bytes, offset + 1).toLong() and 0xFFFFFFFFL
        val (addressBytes, next) = decodeLengthPrefixed(bytes, offset + 5)
        val address = String(addressBytes, Charsets.US_ASCII)
        return WireAccount(algorithm, addressIndex, address) to next
    }

    /** Returns `(payload_slice, offset_after_it)` for a `[len: u8][len bytes]` field. */
    private fun decodeLengthPrefixed(bytes: ByteArray, offset: Int): Pair<ByteArray, Int> {
        require(bytes.size > offset) { "truncated length-prefixed field" }
        val len = bytes[offset].toInt() and 0xFF
        val start = offset + 1
        require(bytes.size >= start + len) { "truncated length-prefixed field body" }
        return bytes.copyOfRange(start, start + len) to (start + len)
    }

    private fun readU16LE(bytes: ByteArray, offset: Int): Int =
        (bytes[offset].toInt() and 0xFF) or ((bytes[offset + 1].toInt() and 0xFF) shl 8)

    private fun readU32LE(bytes: ByteArray, offset: Int): Int =
        (bytes[offset].toInt() and 0xFF) or
            ((bytes[offset + 1].toInt() and 0xFF) shl 8) or
            ((bytes[offset + 2].toInt() and 0xFF) shl 16) or
            ((bytes[offset + 3].toInt() and 0xFF) shl 24)
}

/** Mirrors `seed_vault::store::{HUMAN_SIGN_INDEX, AUTOSIGN_INDEX}`. */
object AddressIndex {
    /** The standard account other wallets/SDKs restore from the 25 words; every seed has one. */
    const val HUMAN_SIGN: Int = 0

    /**
     * Falcon-1024 seeds only: the same key at its next PQ-compliant salt, so
     * a different address with the same public key.
     */
    const val AUTOSIGN: Int = 1

    /** Index used unless the caller asks otherwise. */
    const val DEFAULT: Int = AUTOSIGN

    fun label(index: Long): String = when (index) {
        HUMAN_SIGN.toLong() -> "human signing"
        AUTOSIGN.toLong() -> "autosign"
        else -> "unknown"
    }
}

/** Mirrors `seed_vault::store::KeyAlgorithm`. */
enum class KeyAlgorithm(val tag: Int) {
    /** Classic Ed25519 account; the 25 words encode the Ed25519 seed. */
    ALGO25_ED25519(0x01),

    /**
     * Native post-quantum Falcon-1024 account derived from the *same* 25-word
     * mnemonic (go-algorand `algokey pq` / go-algorand-sdk `ToPQSeed`). The
     * same words yield a different address than [ALGO25_ED25519].
     */
    FALCON_1024(0x02);

    companion object {
        /** Algorithm used for new/imported accounts unless the caller asks otherwise. */
        val DEFAULT: KeyAlgorithm = FALCON_1024

        fun fromTag(tag: Int): KeyAlgorithm? = entries.find { it.tag == tag }
    }
}

/** Mirrors `seed_vault::protocol::WireAccount`. */
data class WireAccount(
    val algorithm: KeyAlgorithm,
    val addressIndex: Long,
    val address: String,
)

/**
 * Mirrors `seed_vault::protocol::ListedAccount`. [publicKey] is 32 bytes for
 * Ed25519 (same bytes as the address) and 1793 bytes for Falcon-1024.
 */
class ListedAccount(val account: WireAccount, val publicKey: ByteArray)

/** Mirrors `seed_vault::protocol::ListPage`: [total] accounts exist; this page holds [accounts]. */
class ListPage(val total: Int, val accounts: List<ListedAccount>)

/**
 * Mirrors `seed_vault::store::SignedTxnPrefix`: msgpack SignedTxn bytes up to
 * and including the `"txn"` key.
 */
class SignedTxnPrefix(val algorithm: KeyAlgorithm, val prefix: ByteArray) {
    /** The network-ready msgpack SignedTxn (`POST /v2/transactions` body) for the [txn] that was signed. */
    fun signedTxn(txn: ByteArray): ByteArray = prefix + txn
}

/** Mirrors `seed_vault::protocol::ErrorCode`, plus local decode-only cases. */
sealed class SeedVaultError(val code: Int, message: String) : Exception(message) {
    object MalformedRequest : SeedVaultError(0x01, "malformed request")
    object StoreFailure : SeedVaultError(0x02, "secure storage failure")
    object ResponseTooLarge : SeedVaultError(0x03, "response too large")
    object UnsupportedAlgorithm : SeedVaultError(0x04, "unsupported algorithm")
    object AccountNotFound : SeedVaultError(0x05, "account not found")
    object InvalidMnemonic : SeedVaultError(0x06, "invalid mnemonic")
    object AccountAlreadyExists : SeedVaultError(0x07, "account already exists")
    object InvalidTransaction : SeedVaultError(0x08, "not a msgpack-encoded transaction")
    object SigningFailed : SeedVaultError(0x09, "signing failed")
    object SenderMismatch : SeedVaultError(0x0B, "transaction sender (snd) isn't the signing address")
    object UnknownAddressIndex : SeedVaultError(
        0x0A,
        "address_index doesn't exist for this account's algorithm, or the address isn't that index's key",
    )
    class Unknown(code: Int) : SeedVaultError(code, "unknown error code 0x${code.toString(16)}")

    /** Not a real TA error code -- this app failed to parse the response bytes. */
    object MalformedResponse : SeedVaultError(-1, "malformed response from seed_vault_ta")

    companion object {
        fun fromCode(code: Int): SeedVaultError = when (code) {
            0x01 -> MalformedRequest
            0x02 -> StoreFailure
            0x03 -> ResponseTooLarge
            0x04 -> UnsupportedAlgorithm
            0x05 -> AccountNotFound
            0x06 -> InvalidMnemonic
            0x07 -> AccountAlreadyExists
            0x08 -> InvalidTransaction
            0x09 -> SigningFailed
            0x0A -> UnknownAddressIndex
            0x0B -> SenderMismatch
            else -> Unknown(code)
        }
    }
}
