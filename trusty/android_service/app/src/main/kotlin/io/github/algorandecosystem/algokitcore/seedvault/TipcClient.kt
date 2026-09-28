package io.github.algorandecosystem.algokitcore.seedvault

/**
 * One request/response round trip to `seed_vault_ta` over Trusty's `tipc`
 * transport, from the non-secure (Android) side.
 *
 * Backed by [tipc_jni.c][/src/main/cpp/tipc_jni.c], which talks to the
 * kernel's Trusty IPC device node directly (`open` + `ioctl(TIPC_IOC_CONNECT)`
 * + `read`/`write`) -- the same mechanism AOSP's own `libtrusty` uses, just
 * reimplemented standalone so this app doesn't need the full AOSP source
 * tree to build. See `trusty/app/seed_vault_ta/main.rs` for the TA this
 * talks to.
 *
 * ## Before this will connect to anything
 *
 * `/dev/trusty-ipc-dev0` is normally only accessible to specific system
 * processes (DAC owner/group + an SELinux domain), not arbitrary apps. On
 * a userdebug/eng Cuttlefish image built from this repo's Trusty checkout,
 * the simplest way to unblock a debug build of this app is, over `adb`:
 *
 * ```sh
 * adb root
 * adb shell setenforce 0                       # permissive SELinux (debug only!)
 * adb shell chmod 666 /dev/trusty-ipc-dev0      # world read/write
 * ```
 *
 * For anything beyond local bring-up, replace that with a proper SELinux
 * domain + `file_contexts`/`.te` rule scoped to this app instead of
 * disabling enforcement device-wide.
 *
 * ## Lifecycle
 *
 * Each instance wraps exactly one connection. `seed_vault_ta` is stateless
 * per-request (see its `main.rs`), so this app opens a fresh connection per
 * button press rather than holding one open across the Activity's
 * lifetime -- simpler, and avoids ever reusing a connection that quietly
 * died while the app was backgrounded.
 */
class TipcClient(
    private val devicePath: String = DEFAULT_DEVICE_PATH,
    private val serviceName: String = DEFAULT_SERVICE_NAME,
) {
    private var fd: Int = -1

    /** Opens `/dev/trusty-ipc-dev0` and connects it to [serviceName]. */
    fun connect() {
        check(fd < 0) { "already connected" }
        val result = nativeConnect(devicePath, serviceName)
        if (result < 0) {
            val errno = -result
            throw TipcException(
                "connect to \"$serviceName\" via $devicePath failed (errno=$errno). " +
                    "Is the device node present and accessible to this app? See TipcClient's doc comment.",
            )
        }
        fd = result
    }

    /** Sends one request message and waits for the one reply message. */
    fun sendAndReceive(request: ByteArray, maxResponseSize: Int = MAX_MESSAGE_SIZE): ByteArray {
        check(fd >= 0) { "not connected -- call connect() first" }
        return nativeSendAndReceive(fd, request, maxResponseSize)
            ?: throw TipcException("IPC send/receive failed (see logcat tag SeedVaultTipc for errno)")
    }

    /** Closes the connection. Safe to call more than once. */
    fun close() {
        if (fd >= 0) {
            nativeClose(fd)
            fd = -1
        }
    }

    private external fun nativeConnect(devicePath: String, serviceName: String): Int

    private external fun nativeSendAndReceive(fd: Int, request: ByteArray, maxResponseSize: Int): ByteArray?

    private external fun nativeClose(fd: Int)

    companion object {
        /** The standard Trusty IPC device node on Cuttlefish/most AOSP Trusty targets. */
        const val DEFAULT_DEVICE_PATH: String = "/dev/trusty-ipc-dev0"

        /** Must match `PORT` in `trusty/app/seed_vault_ta/main.rs`. */
        const val DEFAULT_SERVICE_NAME: String = "io.github.algorandecosystem.algokitcore.seed_vault_ta"

        /** Must be >= `MAX_MSG_SIZE` in `trusty/app/seed_vault_ta/main.rs`. */
        const val MAX_MESSAGE_SIZE: Int = 4096

        init {
            System.loadLibrary("seedvault_tipc")
        }
    }
}

class TipcException(message: String) : Exception(message)
