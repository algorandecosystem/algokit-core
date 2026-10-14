use ffi_macros::{ffi_func, ffi_record};
use serde::{Deserialize, Serialize};

use crate::{AlgoKitTransactError, vec_to_array};

/// A native post-quantum signature authorizing a transaction (`pqsig`).
///
/// For Falcon-1024: `public_key` is the 1793-byte key, `scheme` is `"f1"`
/// (see [`pq_scheme_falcon1024`]), `signature` is the compressed signature
/// over `"TX" || encoded transaction` (e.g. from `algokit_crypto`'s
/// `falcon1024_raw_sign`), and `salt` is the account's address salt.
#[ffi_record]
pub struct PqSignature {
    /// The signer's PQ public key.
    pub public_key: Vec<u8>,
    /// Two-byte scheme identifier, e.g. `"f1"` for Falcon-1024.
    pub scheme: Vec<u8>,
    /// The signature over `"TX" || encoded transaction`.
    pub signature: Vec<u8>,
    /// Which PQ-compliant address of `public_key` this authorizes.
    pub salt: u8,
}

impl From<algokit_transact::PqSignature> for PqSignature {
    fn from(value: algokit_transact::PqSignature) -> Self {
        Self {
            public_key: value.public_key,
            scheme: value.scheme.to_vec(),
            signature: value.signature,
            salt: value.salt,
        }
    }
}

impl TryFrom<PqSignature> for algokit_transact::PqSignature {
    type Error = AlgoKitTransactError;

    fn try_from(value: PqSignature) -> Result<Self, Self::Error> {
        Ok(Self {
            public_key: value.public_key,
            scheme: vec_to_array(&value.scheme, "PQ scheme")?,
            signature: value.signature,
            salt: value.salt,
        })
    }
}

/// The two-byte `pqsig` scheme identifier for Falcon-1024 (`"f1"`).
#[ffi_func]
pub fn pq_scheme_falcon1024() -> Vec<u8> {
    algokit_transact::PQ_SCHEME_FALCON1024.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{SignedTransaction, decode_signed_transaction, encode_signed_transaction};
    use algokit_transact::test_utils::TransactionMother;

    #[test]
    fn signed_transaction_with_pq_signature_round_trips_through_ffi() {
        let pq = PqSignature {
            public_key: vec![1u8; 1793],
            scheme: pq_scheme_falcon1024(),
            signature: vec![2u8; 1200],
            salt: 3,
        };
        let stxn = SignedTransaction {
            transaction: TransactionMother::simple_payment().build().unwrap().into(),
            signature: None,
            auth_address: None,
            multisignature: None,
            logic_signature: None,
            pq_signature: Some(pq.clone()),
        };
        let encoded = encode_signed_transaction(stxn).unwrap();
        assert!(encoded.starts_with(b"\x82\xa5pqsig\x84\xa2pk"));
        let decoded = decode_signed_transaction(&encoded).unwrap();
        assert_eq!(decoded.pq_signature, Some(pq));
    }

    #[test]
    fn rejects_malformed_scheme() {
        let pq = PqSignature {
            public_key: vec![],
            scheme: b"f1x".to_vec(),
            signature: vec![],
            salt: 0,
        };
        assert!(algokit_transact::PqSignature::try_from(pq).is_err());
    }
}
