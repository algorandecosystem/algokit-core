use crate::AppClientError;
use crate::applications::app_deployer::AppDeployError;
use crate::transactions::TransactionSenderError;
use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum AppFactoryError {
    #[snafu(display("Method not found: {message}"))]
    MethodNotFound { message: String },
    #[snafu(display("Compilation error: {message}"))]
    CompilationError { message: String },
    #[snafu(display("Validation error: {message}"))]
    ValidationError { message: String },
    #[snafu(display("App client error: {source}"))]
    AppClientError { source: AppClientError },
    #[snafu(display("Transaction sender error: {source}"))]
    TransactionSenderError { source: TransactionSenderError },
    #[snafu(display("App deployer error: {source}"))]
    AppDeployerError { source: AppDeployError },
}
