use crate::AppClientError;
use crate::applications::app_deployer::AppDeployError;
use crate::transactions::TransactionSenderError;
use algokit_abi::error::ABIError;

#[derive(Debug)]
pub enum AppFactoryError {
    MethodNotFound(String),
    CompilationError(String),
    ValidationError(String),
    AppClientError(String),
    TransactionError(String),
    AppDeployerError(String),
}

impl std::fmt::Display for AppFactoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MethodNotFound(s) => write!(f, "Method not found: {}", s),
            Self::CompilationError(s) => write!(f, "Compilation error: {}", s),
            Self::ValidationError(s) => write!(f, "Validation error: {}", s),
            Self::AppClientError(s) => write!(f, "App client error: {}", s),
            Self::TransactionError(s) => write!(f, "Transaction error: {}", s),
            Self::AppDeployerError(s) => write!(f, "App deployer error: {}", s),
        }
    }
}

impl std::error::Error for AppFactoryError {}

impl From<AppClientError> for AppFactoryError {
    fn from(e: AppClientError) -> Self {
        Self::AppClientError(e.to_string())
    }
}
impl From<TransactionSenderError> for AppFactoryError {
    fn from(e: TransactionSenderError) -> Self {
        Self::TransactionError(e.to_string())
    }
}
impl From<AppDeployError> for AppFactoryError {
    fn from(e: AppDeployError) -> Self {
        Self::AppDeployerError(e.to_string())
    }
}
impl From<ABIError> for AppFactoryError {
    fn from(e: ABIError) -> Self {
        Self::ValidationError(e.to_string())
    }
}
