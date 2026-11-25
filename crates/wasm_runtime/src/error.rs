use alloc::string::String;
use core::fmt;

/// Errors from [`crate::WasmRuntime`] and [`crate::Instance`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// The engine configuration was rejected.
    Config(String),
    /// A host function could not be registered.
    HostFunction(String),
    /// The module failed to parse or validate.
    InvalidModule(String),
    /// The requested export is missing or has the wrong signature.
    MissingExport(String),
    /// Execution used up its fuel.
    OutOfFuel,
    /// Execution trapped, or instantiation failed (e.g. an unknown import).
    Trap(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Config(m) => write!(f, "invalid runtime configuration: {m}"),
            Error::HostFunction(m) => write!(f, "failed to register host function: {m}"),
            Error::InvalidModule(m) => write!(f, "invalid WASM module: {m}"),
            Error::MissingExport(name) => write!(f, "missing `() -> i32` export `{name}`"),
            Error::OutOfFuel => f.write_str("WASM execution ran out of fuel"),
            Error::Trap(m) => write!(f, "WASM execution trapped: {m}"),
        }
    }
}

pub type Result<T, E = Error> = core::result::Result<T, E>;
