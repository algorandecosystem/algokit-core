// JNI shim implementing the non-secure (Android) side of Trusty's TIPC
// transport -- the same layer AOSP's own `libtrusty` (system/core/trusty/
// libtrusty) wraps for HALs like keymaster/gatekeeper, reimplemented here
// from scratch (clean-room, against the public ioctl contract documented at
// https://source.android.com/docs/security/features/trusty/trusty-ref) so
// this standalone app doesn't need to vendor the whole AOSP tree.
//
// Protocol, in three syscalls:
//   1. open(devicePath)                              -> fd for the driver
//   2. ioctl(fd, TIPC_IOC_CONNECT, serviceName)       -> connects `fd` to
//      the named Trusty service's port (e.g. "io.github.algorandecosystem.algokitcore.seed_vault_ta")
//   3. write(fd, req, len) / read(fd, buf, cap)       -> one write is one
//      request message, one read is one complete reply message (TIPC is
//      message-oriented, like SOCK_SEQPACKET -- never a byte stream).
//
// close(fd) tears the connection down and tells the TA the peer disconnected.
//
// This talks directly to the kernel driver, so it only works where that
// device node exists and this process is allowed (DAC + SELinux) to open
// and ioctl() it -- see `TipcClient`'s doc comment on the Kotlin side for
// how to arrange that on a Cuttlefish/AOSP userdebug build.

#include <jni.h>

#include <android/log.h>
#include <errno.h>
#include <fcntl.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <sys/ioctl.h>
#include <sys/socket.h>
#include <unistd.h>

#include <linux/vm_sockets.h>

#define LOG_TAG "SeedVaultTipc"
#define LOGE(...) __android_log_print(ANDROID_LOG_ERROR, LOG_TAG, __VA_ARGS__)

// From <linux/trusty/tipc.h> in the Trusty-enabled kernel tree. Not
// included directly since a standalone Android app's NDK sysroot doesn't
// carry Trusty's out-of-tree kernel headers -- the ioctl number itself is
// stable ABI (documented publicly), so it's spelled out here instead.
#define TIPC_IOC_MAGIC 'r'
#define TIPC_IOC_CONNECT _IOW(TIPC_IOC_MAGIC, 0x80, char*)

// Device paths of the form "VSOCK:<cid>:<port>" select the vsock transport
// instead of a kernel device node -- the same convention AOSP's `libtrusty`
// (`tipc_connect`) uses. This is how Android reaches Trusty when it runs as
// a separate VM rather than behind a kernel driver, e.g. the
// `trusty_security_vm` on x86_64 Cuttlefish ("VSOCK:200:0").
#define VSOCK_PREFIX "VSOCK:"

// Opens a kernel Trusty IPC device node and connects it to `service_name`.
// Returns the fd, or -errno.
static int tipc_dev_connect(const char* device_path, const char* service_name) {
    int fd = open(device_path, O_RDWR);
    if (fd < 0) {
        int err = errno;
        LOGE("open(%s) failed: %s", device_path, strerror(err));
        return -err;
    }

    int rc = ioctl(fd, TIPC_IOC_CONNECT, service_name);
    if (rc < 0) {
        int err = errno;
        LOGE("ioctl(TIPC_IOC_CONNECT, \"%s\") failed: %s", service_name, strerror(err));
        close(fd);
        return -err;
    }
    return fd;
}

// Connects to `service_name` through Trusty's vsock tipc bridge at
// "<cid>:<port>". Mirrors `tipc_vsock_connect` in AOSP's libtrusty: connect a
// SOCK_STREAM vsock, send the service name as the first packet, then read
// one status byte before any request traffic. After that, plain write()/read()
// carry the request/response messages. Returns the fd, or -errno.
static int tipc_vsock_connect(const char* cid_port, const char* service_name) {
    char* end;
    errno = 0;
    unsigned long cid = strtoul(cid_port, &end, 0);
    if (errno != 0 || end == cid_port || *end != ':') {
        LOGE("invalid VSOCK address \"%s\", need <cid>:<port>", cid_port);
        return -EINVAL;
    }
    const char* port_str = end + 1;
    unsigned long port = strtoul(port_str, &end, 0);
    if (errno != 0 || end == port_str || *end != '\0') {
        LOGE("invalid VSOCK address \"%s\", need <cid>:<port>", cid_port);
        return -EINVAL;
    }

    int fd = socket(AF_VSOCK, SOCK_STREAM, 0);
    if (fd < 0) {
        int err = errno;
        LOGE("socket(AF_VSOCK) failed: %s", strerror(err));
        return -err;
    }

    struct sockaddr_vm sa = {
            .svm_family = AF_VSOCK,
            .svm_port = (unsigned int)port,
            .svm_cid = (unsigned int)cid,
    };
    int rc;
    do {
        rc = connect(fd, (struct sockaddr*)&sa, sizeof(sa));
    } while (rc < 0 && errno == EINTR);
    if (rc < 0) {
        int err = errno;
        LOGE("connect(vsock %lu:%lu) for \"%s\" failed: %s", cid, port, service_name, strerror(err));
        close(fd);
        return -err;
    }

    size_t name_len = strlen(service_name);
    ssize_t n;
    do {
        n = write(fd, service_name, name_len);
    } while (n < 0 && errno == EINTR);
    if (n != (ssize_t)name_len) {
        int err = n < 0 ? errno : EIO;
        LOGE("vsock %lu:%lu: sending service name \"%s\" failed: %s", cid, port, service_name,
             strerror(err));
        close(fd);
        return -err;
    }

    int8_t status;
    do {
        n = read(fd, &status, sizeof(status));
    } while (n < 0 && errno == EINTR);
    if (n != (ssize_t)sizeof(status)) {
        int err = n < 0 ? errno : ECONNREFUSED;
        LOGE("vsock %lu:%lu: no connect status for \"%s\" (is the TA running?): %s", cid, port,
             service_name, strerror(err));
        close(fd);
        return -err;
    }
    return fd;
}

JNIEXPORT jint JNICALL
Java_io_github_algorandecosystem_algokitcore_seedvault_TipcClient_nativeConnect(
        JNIEnv* env, jobject thiz, jstring j_device_path, jstring j_service_name) {
    (void)thiz;

    const char* device_path = (*env)->GetStringUTFChars(env, j_device_path, NULL);
    const char* service_name = (*env)->GetStringUTFChars(env, j_service_name, NULL);
    if (device_path == NULL || service_name == NULL) {
        if (device_path != NULL) (*env)->ReleaseStringUTFChars(env, j_device_path, device_path);
        if (service_name != NULL) (*env)->ReleaseStringUTFChars(env, j_service_name, service_name);
        return -ENOMEM;
    }

    int fd;
    if (strncmp(device_path, VSOCK_PREFIX, strlen(VSOCK_PREFIX)) == 0) {
        fd = tipc_vsock_connect(device_path + strlen(VSOCK_PREFIX), service_name);
    } else {
        fd = tipc_dev_connect(device_path, service_name);
    }

    (*env)->ReleaseStringUTFChars(env, j_device_path, device_path);
    (*env)->ReleaseStringUTFChars(env, j_service_name, service_name);
    return fd;
}

JNIEXPORT jbyteArray JNICALL
Java_io_github_algorandecosystem_algokitcore_seedvault_TipcClient_nativeSendAndReceive(
        JNIEnv* env, jobject thiz, jint fd, jbyteArray j_request, jint max_response_size) {
    (void)thiz;

    jsize req_len = (*env)->GetArrayLength(env, j_request);
    jbyte* req_bytes = (*env)->GetByteArrayElements(env, j_request, NULL);
    if (req_bytes == NULL) {
        return NULL;
    }

    ssize_t written;
    do {
        written = write(fd, req_bytes, (size_t)req_len);
    } while (written < 0 && errno == EINTR);
    (*env)->ReleaseByteArrayElements(env, j_request, req_bytes, JNI_ABORT);

    if (written < 0) {
        LOGE("write() failed: %s", strerror(errno));
        return NULL;
    }
    if (written != req_len) {
        LOGE("short write: wrote %zd of %d bytes", written, (int)req_len);
        return NULL;
    }

    if (max_response_size <= 0) {
        return NULL;
    }
    uint8_t* buf = (uint8_t*)malloc((size_t)max_response_size);
    if (buf == NULL) {
        return NULL;
    }

    ssize_t n;
    do {
        n = read(fd, buf, (size_t)max_response_size);
    } while (n < 0 && errno == EINTR);

    if (n < 0) {
        LOGE("read() failed: %s", strerror(errno));
        free(buf);
        return NULL;
    }

    jbyteArray result = (*env)->NewByteArray(env, (jsize)n);
    if (result != NULL) {
        (*env)->SetByteArrayRegion(env, result, 0, (jsize)n, (jbyte*)buf);
    }
    free(buf);
    return result;
}

JNIEXPORT void JNICALL
Java_io_github_algorandecosystem_algokitcore_seedvault_TipcClient_nativeClose(JNIEnv* env, jobject thiz, jint fd) {
    (void)env;
    (void)thiz;
    if (fd >= 0) {
        close(fd);
    }
}
