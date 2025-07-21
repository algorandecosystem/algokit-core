use sha2::{Digest, Sha512_256};

use crate::{
    constants::{
        ALGORAND_ADDRESS_LENGTH, ALGORAND_CHECKSUM_BYTE_LENGTH, ALGORAND_PUBLIC_KEY_BYTE_LENGTH,
        HASH_BYTES_LENGTH,
    },
    error::ABIError,
    ABIType, ABIValue,
};

impl ABIType {
    /// Encode an address value to ABI format.
    /// Addresses are encoded as 32-byte public keys.
    pub(crate) fn encode_address(&self, value: &ABIValue) -> Result<Vec<u8>, ABIError> {
        match self {
            ABIType::Address => {
                let address_str = match value {
                    ABIValue::Address(a) => a,
                    _ => {
                        return Err(ABIError::EncodingError(
                            "Cannot encode value as address: expected a String".to_string(),
                        ));
                    }
                };

                if address_str.len() != ALGORAND_ADDRESS_LENGTH {
                    return Err(ABIError::ValidationError(
                        "Algorand address must be exactly 58 characters".into(),
                    ));
                }
                let decoded_address =
                    base32::decode(base32::Alphabet::Rfc4648 { padding: false }, address_str)
                        .ok_or_else(|| {
                            ABIError::ValidationError(
                                "Invalid base32 encoding for Algorand address".into(),
                            )
                        })?[..ALGORAND_PUBLIC_KEY_BYTE_LENGTH]
                        .to_vec();

                Ok(decoded_address)
            }
            _ => Err(ABIError::EncodingError("Expected Address".to_string())),
        }
    }

    /// Decode an address value from ABI format.
    /// Expects exactly 32 bytes and returns an Address ABIValue.
    pub(crate) fn decode_address(&self, bytes: &[u8]) -> Result<ABIValue, ABIError> {
        match self {
            ABIType::Address => {
                if bytes.len() != ALGORAND_PUBLIC_KEY_BYTE_LENGTH {
                    return Err(ABIError::DecodingError(format!(
                        "Address byte string must be {} bytes long",
                        ALGORAND_PUBLIC_KEY_BYTE_LENGTH
                    )));
                }
                let bytes: &[u8; ALGORAND_PUBLIC_KEY_BYTE_LENGTH] =
                    bytes.try_into().map_err(|_| {
                        ABIError::DecodingError(format!(
                            "Failed to convert bytes to [u8; {}] for checksum",
                            ALGORAND_PUBLIC_KEY_BYTE_LENGTH
                        ))
                    })?;

                let mut buffer =
                    [0u8; ALGORAND_PUBLIC_KEY_BYTE_LENGTH + ALGORAND_CHECKSUM_BYTE_LENGTH];
                buffer[..ALGORAND_PUBLIC_KEY_BYTE_LENGTH].copy_from_slice(bytes);

                let checksum = get_checksum(bytes);
                buffer[ALGORAND_PUBLIC_KEY_BYTE_LENGTH..].copy_from_slice(&checksum);

                let address_str =
                    base32::encode(base32::Alphabet::Rfc4648 { padding: false }, &buffer);

                Ok(ABIValue::Address(address_str))
            }
            _ => Err(ABIError::DecodingError("Expected Address".to_string())),
        }
    }
}

fn get_checksum(
    pub_key: &[u8; ALGORAND_PUBLIC_KEY_BYTE_LENGTH],
) -> [u8; ALGORAND_CHECKSUM_BYTE_LENGTH] {
    let mut hasher = Sha512_256::new();
    hasher.update(pub_key);

    let mut checksum = [0u8; ALGORAND_CHECKSUM_BYTE_LENGTH];
    checksum
        .copy_from_slice(&hasher.finalize()[(HASH_BYTES_LENGTH - ALGORAND_CHECKSUM_BYTE_LENGTH)..]);

    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_wrong_type() {
        let value = ABIValue::String("not an address".to_string());
        let result = ABIType::Address.encode(&value);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Cannot encode value as address"));
    }

    #[test]
    fn test_encode_wrong_abi_type() {
        let value = ABIValue::Address(
            "MO2H6ZU47Q36GJ6GVHUKGEBEQINN7ZWVACMWZQGIYUOE3RBSRVYHV4ACJI".to_string(),
        );
        let result = ABIType::String.encode(&value);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Expected Address"));
    }

    #[test]
    fn test_decode_wrong_length_too_short() {
        let bytes = vec![0u8; 31];
        let result = ABIType::Address.decode(&bytes);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Address byte string must be 32 bytes long"));
    }

    #[test]
    fn test_decode_wrong_length_too_long() {
        let bytes = vec![0u8; 33];
        let result = ABIType::Address.decode(&bytes);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Address byte string must be 32 bytes long"));
    }
}
