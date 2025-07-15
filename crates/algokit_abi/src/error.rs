use thiserror::Error;

/// Represents errors that can occur during Algorand transaction operations.
///
/// This enum encompasses various failure scenarios that may arise when creating,
/// manipulating, serializing, or deserializing Algorand transactions.
#[derive(Debug, Error)]
pub enum ABIError {
    #[error("Error ocurred during encoding: {0}")]
    EncodingError(String),

    #[error("Error ocurred during decoding: {0}")]
    DecodingError(String),
}
