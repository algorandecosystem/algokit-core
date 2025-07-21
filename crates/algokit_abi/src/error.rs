use thiserror::Error;

#[derive(Debug, Error)]
pub enum ABIError {
    #[error("ABI validation failed: {0}")]
    ValidationError(String),

    #[error("ABI encoding failed: {0}")]
    EncodingError(String),

    #[error("ABI decoding failed: {0}")]
    DecodingError(String),
}
