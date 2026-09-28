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
 * | `0x01` = CREATE_ACCOUNT      | 1 byte: [KeyAlgorithm] tag                                        |
 * | `0x02` = LIST_ACCOUNTS        | (none)                                                             |
 * | `0x03` = REVEAL_MNEMONIC      | `[addr_len: u8][addr_len bytes of ASCII address]`                  |
 * | `0x04` = IMPORT_ACCOUNT       | `[algorithm: u8][mnemonic_len: u8][mnemonic_len bytes of ASCII]`   |
 *
 * ## Response
 *
 * | Byte 0            | Remaining bytes            |
 * |--------------------|------------------------------|
 * | `0x00` = OK         | opcode-specific, see below |
 * | `0x01` = ERR        | 1 byte: [SeedVaultError] code |
 *
 * CREATE_ACCOUNT/IMPORT_ACCOUNT success payload:
 * `[algorithm: u8][address_index: u32 LE][addr_len: u8][addr_len bytes of ASCII address]`.
 *
 * LIST_ACCOUNTS success payload: `[count: u32 LE]`, followed by `count`
 * repetitions of the shape above.
 *
 * REVEAL_MNEMONIC success payload:
 * `[mnemonic_len: u8][mnemonic_len bytes of ASCII, space-separated words]`.
 */
object SeedVaultProtocol {

    private const val OP_CREATE_ACCOUNT: Int = 0x01
    private const val OP_LIST_ACCOUNTS: Int = 0x02
    private const val OP_REVEAL_MNEMONIC: Int = 0x03
    private const val OP_IMPORT_ACCOUNT: Int = 0x04

    private const val STATUS_OK: Int = 0x00
    private const val STATUS_ERR: Int = 0x01

    // ---- Requests -----------------------------------------------------

    fun encodeCreateAccount(algorithm: KeyAlgorithm = KeyAlgorithm.ALGO25_ED25519): ByteArray =
        byteArrayOf(OP_CREATE_ACCOUNT.toByte(), algorithm.tag.toByte())

    fun encodeListAccounts(): ByteArray = byteArrayOf(OP_LIST_ACCOUNTS.toByte())

    fun encodeRevealMnemonic(address: String): ByteArray {
        val addrBytes = address.trim().toByteArray(Charsets.US_ASCII)
        require(addrBytes.size <= 0xFF) { "address too long (${addrBytes.size} bytes)" }
        return byteArrayOf(OP_REVEAL_MNEMONIC.toByte(), addrBytes.size.toByte()) + addrBytes
    }

    fun encodeImportAccount(
        mnemonic: String,
        algorithm: KeyAlgorithm = KeyAlgorithm.ALGO25_ED25519,
    ): ByteArray {
        // Words are separated by single spaces on the wire, same as
        // `reveal_mnemonic`'s output -- normalize whitespace defensively in
        // case the user pasted a phrase with newlines/extra spaces.
        val normalized = mnemonic.trim().split(Regex("\\s+")).joinToString(" ")
        val mnemonicBytes = normalized.toByteArray(Charsets.US_ASCII)
        require(mnemonicBytes.size <= 0xFF) { "mnemonic too long (${mnemonicBytes.size} bytes)" }
        return byteArrayOf(OP_IMPORT_ACCOUNT.toByte(), algorithm.tag.toByte(), mnemonicBytes.size.toByte()) +
            mnemonicBytes
    }

    // ---- Responses ------------------------------------------------------

    /** Decodes a CREATE_ACCOUNT/IMPORT_ACCOUNT success response. */
    fun decodeAccountResponse(bytes: ByteArray): WireAccount {
        val payload = decodeOkPayload(bytes)
        val (account, next) = decodeOneAccount(payload, 0)
        require(next == payload.size) { "trailing bytes after account response" }
        return account
    }

    /** Decodes a LIST_ACCOUNTS success response. */
    fun decodeListAccountsResponse(bytes: ByteArray): List<WireAccount> {
        val payload = decodeOkPayload(bytes)
        require(payload.size >= 4) { "truncated list_accounts response" }
        val count = readU32LE(payload, 0)
        var offset = 4
        val accounts = ArrayList<WireAccount>(count)
        repeat(count) {
            val (account, next) = decodeOneAccount(payload, offset)
            accounts.add(account)
            offset = next
        }
        require(offset == payload.size) { "trailing bytes after list_accounts response" }
        return accounts
    }

    /** Decodes a REVEAL_MNEMONIC success response into its mnemonic string. */
    fun decodeRevealMnemonicResponse(bytes: ByteArray): String {
        val payload = decodeOkPayload(bytes)
        val (mnemonicBytes, next) = decodeLengthPrefixed(payload, 0)
        require(next == payload.size) { "trailing bytes after reveal_mnemonic response" }
        return String(mnemonicBytes, Charsets.US_ASCII)
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

    private fun readU32LE(bytes: ByteArray, offset: Int): Int =
        (bytes[offset].toInt() and 0xFF) or
            ((bytes[offset + 1].toInt() and 0xFF) shl 8) or
            ((bytes[offset + 2].toInt() and 0xFF) shl 16) or
            ((bytes[offset + 3].toInt() and 0xFF) shl 24)
}

/** Mirrors `seed_vault::store::KeyAlgorithm`. */
enum class KeyAlgorithm(val tag: Int) {
    ALGO25_ED25519(0x01);

    companion object {
        fun fromTag(tag: Int): KeyAlgorithm? = entries.find { it.tag == tag }
    }
}

/** Mirrors `seed_vault::protocol::WireAccount`. */
data class WireAccount(
    val algorithm: KeyAlgorithm,
    val addressIndex: Long,
    val address: String,
)

/** Mirrors `seed_vault::protocol::ErrorCode`, plus local decode-only cases. */
sealed class SeedVaultError(val code: Int, message: String) : Exception(message) {
    object MalformedRequest : SeedVaultError(0x01, "malformed request")
    object StoreFailure : SeedVaultError(0x02, "secure storage failure")
    object ResponseTooLarge : SeedVaultError(0x03, "response too large")
    object UnsupportedAlgorithm : SeedVaultError(0x04, "unsupported algorithm")
    object AccountNotFound : SeedVaultError(0x05, "account not found")
    object InvalidMnemonic : SeedVaultError(0x06, "invalid mnemonic")
    object AccountAlreadyExists : SeedVaultError(0x07, "account already exists")
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
            else -> Unknown(code)
        }
    }
}
