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
#include <unistd.h>

#define LOG_TAG "SeedVaultTipc"
#define LOGE(...) __android_log_print(ANDROID_LOG_ERROR, LOG_TAG, __VA_ARGS__)

// From <linux/trusty/tipc.h> in the Trusty-enabled kernel tree. Not
// included directly since a standalone Android app's NDK sysroot doesn't
// carry Trusty's out-of-tree kernel headers -- the ioctl number itself is
// stable ABI (documented publicly), so it's spelled out here instead.
#define TIPC_IOC_MAGIC 'r'
#define TIPC_IOC_CONNECT _IOW(TIPC_IOC_MAGIC, 0x80, char*)

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

    int fd = open(device_path, O_RDWR);
    if (fd < 0) {
        int err = errno;
        LOGE("open(%s) failed: %s", device_path, strerror(err));
        (*env)->ReleaseStringUTFChars(env, j_device_path, device_path);
        (*env)->ReleaseStringUTFChars(env, j_service_name, service_name);
        return -err;
    }

    int rc = ioctl(fd, TIPC_IOC_CONNECT, service_name);
    if (rc < 0) {
        int err = errno;
        LOGE("ioctl(TIPC_IOC_CONNECT, \"%s\") failed: %s", service_name, strerror(err));
        close(fd);
        (*env)->ReleaseStringUTFChars(env, j_device_path, device_path);
        (*env)->ReleaseStringUTFChars(env, j_service_name, service_name);
        return -err;
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
