//! Rust bindings for the deterministic variant of the Falcon signature scheme,
//! mirroring the Go bindings in `falcon/falcon.go`.
//!
//! Each parameter set lives in its own module:
//!
//! - [`det1024`]: FALCON-DET1024 (n = 1024).
//!
//! The crate is `no_std` and only requires `alloc` (for
//! [`det1024::CompressedSignature`]).

#![no_std]

extern crate alloc;

use core::ffi::{c_int, c_void};
use core::fmt;
use core::ptr;

pub mod det1024;
mod ffi;

/// The operation that failed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ErrorKind {
    Keygen,
    Sign,
    Verify,
    Convert,
    PubkeyCoefficients,
    S1Coefficients,
    S2Coefficients,
}

/// Error returned by Falcon operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Error {
    kind: ErrorKind,
    code: Option<i32>,
}

impl Error {
    pub(crate) fn from_code(kind: ErrorKind, code: c_int) -> Self {
        Error {
            kind,
            code: Some(code),
        }
    }

    pub(crate) fn rejected(kind: ErrorKind) -> Self {
        Error { kind, code: None }
    }

    /// The operation that failed.
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    /// The error code returned by the C library, or `None` if the input was
    /// rejected before reaching it (e.g. an empty or too-short signature).
    pub fn code(&self) -> Option<i32> {
        self.code
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self.kind {
            ErrorKind::Keygen => "falcon keygen failed",
            ErrorKind::Sign => "falcon sign failed",
            ErrorKind::Verify => "falcon verify failed",
            ErrorKind::Convert => "falcon convert to CT failed",
            ErrorKind::PubkeyCoefficients => "falcon pubkey coefficients failed",
            ErrorKind::S1Coefficients => "falcon computing S1 coefficients failed",
            ErrorKind::S2Coefficients => "falcon computing S2 coefficients failed",
        };
        match self.code {
            Some(code) => write!(f, "{msg}: error code {code}"),
            None => f.write_str(msg),
        }
    }
}

impl core::error::Error for Error {}

pub(crate) fn check(kind: ErrorKind, r: c_int) -> Result<(), Error> {
    if r == 0 {
        Ok(())
    } else {
        Err(Error::from_code(kind, r))
    }
}

/// Returns a (pointer, length) pair for `data`, using a null pointer for empty
/// input as the Go bindings do.
pub(crate) fn raw(data: &[u8]) -> (*const c_void, usize) {
    if data.is_empty() {
        (ptr::null(), 0)
    } else {
        (data.as_ptr().cast(), data.len())
    }
}
