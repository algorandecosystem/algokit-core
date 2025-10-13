use snafu::Snafu;

#[derive(Debug, Snafu, Clone)]
pub enum TransactionError {
    #[snafu(display("Wait for confirmation failed: {message}"))]
    WaitForConfirmationError { message: String },
}
