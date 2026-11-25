//! Non-secure (Android) side of Trusty's TIPC transport.
//!
//! A clean-room port of the same two mechanisms AOSP's `libtrusty` uses
//! (system/core/trusty/libtrusty), so the library needs no AOSP sources:
//!
//! - **Device node** (e.g. `/dev/trusty-ipc-dev0`): `open` +
//!   `ioctl(TIPC_IOC_CONNECT, service_name)`.
//! - **`VSOCK:<cid>:<port>`** (Trusty as a separate VM, e.g. x86_64
//!   Cuttlefish's `trusty_security_vm`): connect a vsock, send the service
//!   name as the first packet, then read one status byte.
//!
//! Either way the result is an fd where one `write` is one request message
//! and one `read` is one complete reply.
//!
//! Each failure is classified into the [`SeedVaultError`] variant an app
//! needs to decide what to show: no Trusty at all, not allowed to reach it,
//! or Trusty without `seed_vault_ta`.

use crate::{MAX_MESSAGE_SIZE, SeedVaultError};

#[cfg(any(target_os = "android", target_os = "linux"))]
pub use imp::Connection;

#[cfg(not(any(target_os = "android", target_os = "linux")))]
pub use stub::Connection;

#[cfg(any(target_os = "android", target_os = "linux"))]
mod imp {
    use super::*;
    use std::ffi::CString;
    use std::io;

    /// From `<linux/trusty/tipc.h>`: `_IOW('r', 0x80, char*)`. Spelled out
    /// because the NDK sysroot doesn't carry Trusty's kernel headers; the
    /// number itself is stable ABI.
    const TIPC_IOC_CONNECT: libc::c_ulong = (1 << 30)
        | ((std::mem::size_of::<*const libc::c_char>() as libc::c_ulong) << 16)
        | ((b'r' as libc::c_ulong) << 8)
        | 0x80;

    const AF_VSOCK: libc::c_int = 40;
    const VSOCK_PREFIX: &str = "VSOCK:";

    /// `struct sockaddr_vm` from `<linux/vm_sockets.h>`.
    #[repr(C)]
    struct SockaddrVm {
        svm_family: libc::sa_family_t,
        svm_reserved1: libc::c_ushort,
        svm_port: libc::c_uint,
        svm_cid: libc::c_uint,
        svm_zero: [u8; 4],
    }

    /// One connection to a Trusty service; closed on drop.
    pub struct Connection {
        fd: libc::c_int,
    }

    impl Drop for Connection {
        fn drop(&mut self) {
            unsafe { libc::close(self.fd) };
        }
    }

    fn is_permission(err: &io::Error) -> bool {
        matches!(err.raw_os_error(), Some(libc::EACCES) | Some(libc::EPERM))
    }

    fn retry<T: PartialEq + From<i8>>(mut f: impl FnMut() -> T) -> T {
        loop {
            let r = f();
            if r != T::from(-1) || io::Error::last_os_error().raw_os_error() != Some(libc::EINTR) {
                return r;
            }
        }
    }

    impl Connection {
        pub fn open(device_path: &str, service_name: &str) -> Result<Self, SeedVaultError> {
            let service =
                CString::new(service_name).map_err(|_| SeedVaultError::InvalidArgument {
                    reason: "service name contains a NUL byte".into(),
                })?;
            match device_path.strip_prefix(VSOCK_PREFIX) {
                Some(cid_port) => Self::open_vsock(cid_port, &service),
                None => Self::open_device(device_path, &service),
            }
        }

        fn open_device(device_path: &str, service: &CString) -> Result<Self, SeedVaultError> {
            let path = CString::new(device_path).map_err(|_| SeedVaultError::InvalidArgument {
                reason: "device path contains a NUL byte".into(),
            })?;
            let fd = unsafe { libc::open(path.as_ptr(), libc::O_RDWR | libc::O_CLOEXEC) };
            if fd < 0 {
                let err = io::Error::last_os_error();
                let reason = format!("open({device_path}): {err}");
                return Err(if is_permission(&err) {
                    SeedVaultError::PermissionDenied { reason }
                } else {
                    // ENOENT/ENODEV/ENXIO: no Trusty driver on this device.
                    SeedVaultError::NotSupported { reason }
                });
            }
            let connection = Self { fd };

            let rc = retry(|| unsafe { libc::ioctl(fd, TIPC_IOC_CONNECT as _, service.as_ptr()) });
            if rc < 0 {
                let err = io::Error::last_os_error();
                let reason = format!("TIPC connect to {service:?} via {device_path}: {err}");
                return Err(if is_permission(&err) {
                    SeedVaultError::PermissionDenied { reason }
                } else {
                    SeedVaultError::ServiceNotFound { reason }
                });
            }
            Ok(connection)
        }

        fn open_vsock(cid_port: &str, service: &CString) -> Result<Self, SeedVaultError> {
            let (cid, port) = cid_port
                .split_once(':')
                .and_then(|(c, p)| Some((c.parse::<u32>().ok()?, p.parse::<u32>().ok()?)))
                .ok_or_else(|| SeedVaultError::InvalidArgument {
                    reason: format!("invalid VSOCK address {cid_port:?}, need <cid>:<port>"),
                })?;
            let where_ = format!("vsock {cid}:{port}");

            let fd = unsafe { libc::socket(AF_VSOCK, libc::SOCK_STREAM | libc::SOCK_CLOEXEC, 0) };
            if fd < 0 {
                let err = io::Error::last_os_error();
                let reason = format!("socket(AF_VSOCK): {err}");
                return Err(if is_permission(&err) {
                    SeedVaultError::PermissionDenied { reason }
                } else {
                    SeedVaultError::NotSupported { reason }
                });
            }
            let connection = Self { fd };

            let addr = SockaddrVm {
                svm_family: AF_VSOCK as libc::sa_family_t,
                svm_reserved1: 0,
                svm_port: port,
                svm_cid: cid,
                svm_zero: [0; 4],
            };
            let rc = retry(|| unsafe {
                libc::connect(
                    fd,
                    &addr as *const SockaddrVm as *const libc::sockaddr,
                    std::mem::size_of::<SockaddrVm>() as libc::socklen_t,
                )
            });
            if rc < 0 {
                let err = io::Error::last_os_error();
                let reason = format!("connect({where_}): {err}");
                return Err(if is_permission(&err) {
                    SeedVaultError::PermissionDenied { reason }
                } else {
                    // Nothing listening at that CID/port: no Trusty VM.
                    SeedVaultError::NotSupported { reason }
                });
            }

            // Reached the Trusty VM; from here on, failures mean the service
            // itself isn't there.
            let name = service.as_bytes();
            let n = retry(|| unsafe { libc::write(fd, name.as_ptr().cast(), name.len()) });
            if n != name.len() as isize {
                return Err(SeedVaultError::ServiceNotFound {
                    reason: format!(
                        "{where_}: sending service name {service:?}: {}",
                        io::Error::last_os_error()
                    ),
                });
            }
            // libtrusty reads (and ignores the value of) this status byte;
            // AOSP reverted an attempt to drop it, so mirror it exactly.
            let mut status = 0u8;
            let n = retry(|| unsafe { libc::read(fd, (&mut status as *mut u8).cast(), 1) });
            if n != 1 {
                return Err(SeedVaultError::ServiceNotFound {
                    reason: format!(
                        "{where_}: no connect status for {service:?} (is seed_vault_ta built into Trusty?)"
                    ),
                });
            }
            Ok(connection)
        }

        /// Sends `request` as one message and waits up to `timeout_ms` for
        /// the one reply message.
        pub fn call(&mut self, request: &[u8], timeout_ms: u32) -> Result<Vec<u8>, SeedVaultError> {
            let transport = |what: &str| SeedVaultError::Transport {
                reason: format!("{what}: {}", io::Error::last_os_error()),
            };

            let written =
                retry(|| unsafe { libc::write(self.fd, request.as_ptr().cast(), request.len()) });
            if written < 0 {
                return Err(transport("write"));
            }
            if written as usize != request.len() {
                return Err(SeedVaultError::Transport {
                    reason: format!("short write: {written} of {} bytes", request.len()),
                });
            }

            let mut pfd = libc::pollfd {
                fd: self.fd,
                events: libc::POLLIN,
                revents: 0,
            };
            let timeout = timeout_ms.min(i32::MAX as u32) as libc::c_int;
            let ready = retry(|| unsafe { libc::poll(&mut pfd, 1, timeout) });
            if ready < 0 {
                return Err(transport("poll"));
            }
            if ready == 0 {
                return Err(SeedVaultError::Transport {
                    reason: format!("no reply from seed_vault_ta within {timeout_ms} ms"),
                });
            }

            let mut buf = vec![0u8; MAX_MESSAGE_SIZE];
            let n = retry(|| unsafe { libc::read(self.fd, buf.as_mut_ptr().cast(), buf.len()) });
            if n < 0 {
                return Err(transport("read"));
            }
            if n == 0 {
                return Err(SeedVaultError::Transport {
                    reason: "seed_vault_ta closed the connection".into(),
                });
            }
            buf.truncate(n as usize);
            Ok(buf)
        }
    }
}

/// Hosts with no Trusty transport at all (e.g. macOS, where the host-side
/// unit tests run).
#[cfg(not(any(target_os = "android", target_os = "linux")))]
mod stub {
    use super::*;

    pub struct Connection;

    impl Connection {
        pub fn open(device_path: &str, _service_name: &str) -> Result<Self, SeedVaultError> {
            Err(SeedVaultError::NotSupported {
                reason: format!("no Trusty IPC on this platform ({device_path})"),
            })
        }

        pub fn call(
            &mut self,
            _request: &[u8],
            _timeout_ms: u32,
        ) -> Result<Vec<u8>, SeedVaultError> {
            let _ = MAX_MESSAGE_SIZE;
            unreachable!("Connection::open never succeeds on this platform")
        }
    }
}
