import java.io.ByteArrayOutputStream
import java.net.HttpURLConnection
import java.net.URL
import java.util.Base64
import org.json.JSONObject
import uniffi.seed_vault_ffi.KeyAlgorithm

/**
 * Just enough of the algod v2 REST API (and transaction encoding) to check
 * `seed_vault_ta` signatures end to end, without an algosdk dependency.
 *
 * Falcon-1024 (`pqsig`) transactions are only accepted by a network whose
 * consensus enables `EnablePQSchemeFalcon1024`, and their fee must cover
 * go-algorand's extra Falcon contribution: 3x the min fee in total.
 */
class AlgodTestClient(baseUrl: String, private val token: String) {
    private val baseUrl = baseUrl.trim().trimEnd('/')

    fun suggestedParams(): SuggestedParams {
        val json = JSONObject(String(request("GET", "/v2/transactions/params"), Charsets.UTF_8))
        return SuggestedParams(
            minFee = json.getLong("min-fee"),
            firstValid = json.getLong("last-round"),
            genesisId = json.getString("genesis-id"),
            genesisHash = Base64.getDecoder().decode(json.getString("genesis-hash")),
        )
    }

    /** POSTs a msgpack SignedTxn and returns the transaction ID. */
    fun submit(signedTxn: ByteArray): String {
        val response = request("POST", "/v2/transactions", signedTxn)
        return JSONObject(String(response, Charsets.UTF_8)).getString("txId")
    }

    /** Polls the pending pool until [txId] is confirmed; returns its round. */
    fun waitForConfirmation(txId: String, maxPolls: Int = 20): Long {
        repeat(maxPolls) {
            val json = JSONObject(String(request("GET", "/v2/transactions/pending/$txId"), Charsets.UTF_8))
            val round = json.optLong("confirmed-round", 0)
            if (round > 0) return round
            val poolError = json.optString("pool-error", "")
            if (poolError.isNotEmpty()) error("rejected from pool: $poolError")
            Thread.sleep(1000)
        }
        error("not confirmed after $maxPolls polls")
    }

    private fun request(method: String, path: String, body: ByteArray? = null): ByteArray {
        val conn = URL(baseUrl + path).openConnection() as HttpURLConnection
        try {
            conn.requestMethod = method
            conn.connectTimeout = 10_000
            conn.readTimeout = 10_000
            if (token.isNotBlank()) conn.setRequestProperty("X-Algo-API-Token", token.trim())
            if (body != null) {
                conn.doOutput = true
                conn.setRequestProperty("Content-Type", "application/x-binary")
                conn.outputStream.use { it.write(body) }
            }
            val code = conn.responseCode
            val stream = if (code in 200..299) conn.inputStream else conn.errorStream
            val bytes = stream?.use { it.readBytes() } ?: ByteArray(0)
            check(code in 200..299) { "$method $path -> HTTP $code: ${String(bytes, Charsets.UTF_8)}" }
            return bytes
        } finally {
            conn.disconnect()
        }
    }

    data class SuggestedParams(
        val minFee: Long,
        val firstValid: Long,
        val genesisId: String,
        val genesisHash: ByteArray,
    )

    companion object {
        /** Falcon-1024 senders pay the min fee plus a 2x min fee PQ surcharge. */
        fun feeFor(algorithm: KeyAlgorithm, minFee: Long): Long =
            if (algorithm == KeyAlgorithm.FALCON1024) minFee * 3 else minFee

        /** Offline params for tests that only sign (never submit). */
        val OFFLINE_PARAMS = SuggestedParams(
            minFee = 1000,
            firstValid = 1000,
            genesisId = "testnet-v1.0",
            genesisHash = ByteArray(32),
        )

        /**
         * A 0-ALGO payment from [sender] to [receiver] (default: itself), valid
         * for 1000 rounds, as canonical msgpack *without* the `"TX"` prefix
         * (what `SIGN_TRANSACTION` expects). [note] keeps repeated test
         * transactions from colliding on the same txid.
         */
        fun payment(
            sender: String,
            params: SuggestedParams,
            fee: Long,
            note: ByteArray,
            receiver: String = sender,
        ): ByteArray {
            val out = MsgpackWriter()
            // Keys in sorted order; "amt" (0) is omitted like go-algorand does.
            out.mapHeader(9)
            out.str("fee"); out.uint(fee)
            out.str("fv"); out.uint(params.firstValid)
            out.str("gen"); out.str(params.genesisId)
            out.str("gh"); out.bin(params.genesisHash)
            out.str("lv"); out.uint(params.firstValid + 1000)
            out.str("note"); out.bin(note)
            out.str("rcv"); out.bin(decodeAddress(receiver))
            out.str("snd"); out.bin(decodeAddress(sender))
            out.str("type"); out.str("pay")
            return out.toByteArray()
        }

        /** The 32 key bytes of a 58-char Algorand address (checksum not verified). */
        private fun decodeAddress(address: String): ByteArray {
            val alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567"
            require(address.length == 58) { "Algorand address must be 58 characters" }
            val out = ByteArrayOutputStream()
            var buffer = 0
            var bits = 0
            for (c in address) {
                val v = alphabet.indexOf(c)
                require(v >= 0) { "invalid base32 character '$c' in address" }
                buffer = (buffer shl 5) or v
                bits += 5
                if (bits >= 8) {
                    bits -= 8
                    out.write((buffer shr bits) and 0xFF)
                }
            }
            return out.toByteArray().copyOfRange(0, 32)
        }
    }
}

/** Minimal canonical msgpack writer (smallest encodings, as go-algorand's `msgp`). */
private class MsgpackWriter {
    private val out = ByteArrayOutputStream()

    fun toByteArray(): ByteArray = out.toByteArray()

    fun mapHeader(size: Int) {
        require(size < 16)
        out.write(0x80 or size)
    }

    fun str(s: String) {
        val bytes = s.toByteArray(Charsets.UTF_8)
        when {
            bytes.size < 32 -> out.write(0xa0 or bytes.size)
            bytes.size < 256 -> { out.write(0xd9); out.write(bytes.size) }
            else -> error("string too long")
        }
        out.write(bytes)
    }

    fun bin(bytes: ByteArray) {
        require(bytes.size < 256)
        out.write(0xc4)
        out.write(bytes.size)
        out.write(bytes)
    }

    fun uint(v: Long) {
        require(v >= 0)
        when {
            v < 0x80 -> out.write(v.toInt())
            v <= 0xFF -> { out.write(0xcc); out.write(v.toInt()) }
            v <= 0xFFFF -> { out.write(0xcd); writeBE(v, 2) }
            v <= 0xFFFFFFFFL -> { out.write(0xce); writeBE(v, 4) }
            else -> { out.write(0xcf); writeBE(v, 8) }
        }
    }

    private fun writeBE(v: Long, bytes: Int) {
        for (i in bytes - 1 downTo 0) out.write(((v shr (8 * i)) and 0xFF).toInt())
    }
}
