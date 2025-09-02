use crate::clients::app_manager::AppManagerError;
use crate::transactions::TransactionSenderError;
use algokit_abi::error::ABIError;

#[derive(Debug)]
pub enum AppClientError {
    AppIdNotFound {
        network_names: Vec<String>,
        available: Vec<String>,
    },
    Network(String),
    Lookup(String),
    MethodNotFound(String),
    AbiError(String),
    TransactionError(String),
    AppManagerError(String),
    CompilationError(String),
    ValidationError(String),
}

impl std::fmt::Display for AppClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AppIdNotFound {
                network_names,
                available,
            } => write!(
                f,
                "No app ID found for network {:?}. Available keys in spec: {:?}",
                network_names, available
            ),
            Self::Network(msg) => write!(f, "Network error: {}", msg),
            Self::Lookup(msg) => write!(f, "Lookup error: {}", msg),
            Self::MethodNotFound(msg) => write!(f, "Method not found: {}", msg),
            Self::AbiError(msg) => write!(f, "ABI error: {}", msg),
            Self::TransactionError(msg) => write!(f, "Transaction error: {}", msg),
            Self::AppManagerError(msg) => write!(f, "App manager error: {}", msg),
            Self::CompilationError(msg) => write!(f, "Compilation error: {}", msg),
            Self::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl std::error::Error for AppClientError {}

impl From<ABIError> for AppClientError {
    fn from(e: ABIError) -> Self {
        Self::AbiError(e.to_string())
    }
}

impl From<TransactionSenderError> for AppClientError {
    fn from(e: TransactionSenderError) -> Self {
        Self::TransactionError(e.to_string())
    }
}

impl From<AppManagerError> for AppClientError {
    fn from(e: AppManagerError) -> Self {
        Self::AppManagerError(e.to_string())
    }
}
