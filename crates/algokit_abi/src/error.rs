use thiserror::Error;

#[derive(Debug, Error)]
pub enum ABIError {
    // TODO: error message
    #[error("Validation: {0}")]
    ValidationError(String),

    #[error("Error ocurred during encoding: {0}")]
    EncodingError(String),

    #[error("Error ocurred during decoding: {0}")]
    DecodingError(String),
}
