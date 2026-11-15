use algokit_crypto::ed25519::is_edwards25519_point;
use serde::{Deserialize, Serialize};
use serde_with::{Bytes, serde_as};

use crate::Address;
use crate::constants::{
    PQ_ADDRESS_DOMAIN_SEPARATOR, PQ_KEY_DOMAIN_SEPARATOR, PQ_SCHEME_FALCON1024,
};
use crate::error::AlgoKitTransactError;
use crate::traits::{MsgPackEmpty, Validate};
use crate::utils::{hash, is_zero};

/// A post-quantum signature authorizing a transaction.
///
/// Carries its scheme, salt and public key, so the authorizing address derives from the
/// signature alone.
#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct PQSignature {
    /// Two-byte scheme identifier, `f1` for Falcon-1024.
    #[serde(rename = "sch")]
    #[serde(skip_serializing_if = "is_empty_scheme")]
    #[serde(default)]
    #[serde_as(as = "Bytes")]
    pub scheme: [u8; 2],

    /// Salt selecting which address the public key derives.
    #[serde(rename = "slt")]
    #[serde(skip_serializing_if = "is_zero")]
    #[serde(default)]
    pub salt: u8,

    /// The scheme's public key.
    #[serde(rename = "pk")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    #[serde_as(as = "Bytes")]
    pub public_key: Vec<u8>,

    /// Signature over the signed bytes. Variable length.
    #[serde(rename = "sig")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    #[serde_as(as = "Bytes")]
    pub signature: Vec<u8>,
}

fn is_empty_scheme(scheme: &[u8; 2]) -> bool {
    scheme == &[0, 0]
}

impl MsgPackEmpty for PQSignature {
    fn is_empty(&self) -> bool {
        is_empty_scheme(&self.scheme)
            && self.salt == 0
            && self.public_key.is_empty()
            && self.signature.is_empty()
    }
}

impl PQSignature {
    /// The address this signature authorizes.
    pub fn address(&self) -> Address {
        pq_address(&self.scheme, self.salt, &self.public_key)
    }
}

/// Derives a post-quantum address from a scheme, salt and public key.
pub fn pq_address(scheme: &[u8; 2], salt: u8, public_key: &[u8]) -> Address {
    let mut buffer =
        Vec::with_capacity(PQ_ADDRESS_DOMAIN_SEPARATOR.len() + scheme.len() + 1 + public_key.len());
    buffer.extend_from_slice(PQ_ADDRESS_DOMAIN_SEPARATOR.as_bytes());
    buffer.extend_from_slice(scheme);
    buffer.push(salt);
    buffer.extend_from_slice(public_key);
    Address(hash(&buffer))
}

/// Finds the lowest salt whose address is not also an Edwards25519 point, so a
/// post-quantum account can never collide with an Ed25519 one.
pub fn canonical_pq_address(
    scheme: &[u8; 2],
    public_key: &[u8],
) -> Result<(u8, Address), AlgoKitTransactError> {
    (0..=u8::MAX)
        .map(|salt| (salt, pq_address(scheme, salt, public_key)))
        .find(|(_, address)| !is_edwards25519_point(address.as_bytes()))
        .ok_or_else(|| AlgoKitTransactError::InputError {
            err_msg: "No salt derives a post-quantum address for this public key".to_string(),
        })
}

/// Derives a scheme's key generation seed from 32 bytes of mnemonic entropy.
pub fn derive_pq_key_seed(scheme: &[u8; 2], entropy: &[u8; 32]) -> [u8; 32] {
    let mut buffer =
        Vec::with_capacity(PQ_KEY_DOMAIN_SEPARATOR.len() + scheme.len() + entropy.len());
    buffer.extend_from_slice(PQ_KEY_DOMAIN_SEPARATOR.as_bytes());
    buffer.extend_from_slice(scheme);
    buffer.extend_from_slice(entropy);
    hash(&buffer)
}

impl Validate for PQSignature {
    fn validate(&self) -> Result<(), Vec<String>> {
        if self.is_empty() {
            return Err(vec!["PQ signature cannot be empty".to_string()]);
        }

        let mut errors = Vec::new();

        if self.scheme != PQ_SCHEME_FALCON1024 {
            errors.push(format!(
                "Unsupported PQ scheme {:?}",
                String::from_utf8_lossy(&self.scheme)
            ));
        }

        if self.public_key.is_empty() {
            errors.push("PQ signature public key cannot be empty".to_string());
        }

        if is_edwards25519_point(self.address().as_bytes()) {
            errors
                .push("PQ signature derives an address that is also an Ed25519 point".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::TransactionMother;
    use crate::traits::{AlgorandMsgpack, TransactionId};
    use crate::transactions::SignedTransaction;
    use algokit_test_artifacts::pq::{DELEGATED_PAYMENT, MNEMONIC, PAYMENT};
    use base64::{Engine, prelude::BASE64_STANDARD};
    use pretty_assertions::assert_eq;

    fn fixture(json: &str) -> serde_json::Value {
        serde_json::from_str(json).unwrap()
    }

    fn b64(value: &serde_json::Value) -> Vec<u8> {
        BASE64_STANDARD.decode(value.as_str().unwrap()).unwrap()
    }

    fn map_keys(bytes: &[u8]) -> Vec<String> {
        match rmp_serde::from_slice::<rmpv::Value>(bytes).unwrap() {
            rmpv::Value::Map(entries) => entries
                .iter()
                .filter_map(|(k, _)| k.as_str().map(str::to_string))
                .collect(),
            other => panic!("expected a msgpack map, got {other:?}"),
        }
    }

    fn payment_signature() -> PQSignature {
        SignedTransaction::decode(&b64(&fixture(PAYMENT)["stxnBlob"]))
            .unwrap()
            .pq_signature
            .unwrap()
    }

    #[test]
    fn canonical_address_matches_the_payment_vector() {
        let payment = fixture(PAYMENT);
        let pqsig = &payment["stxn"]["pqsig"];

        let (salt, address) =
            canonical_pq_address(&PQ_SCHEME_FALCON1024, &b64(&pqsig["pk"])).unwrap();

        assert_eq!(u64::from(salt), pqsig["slt"].as_u64().unwrap());
        assert_eq!(
            address.to_string(),
            payment["stxn"]["txn"]["snd"].as_str().unwrap()
        );
    }

    #[test]
    fn lower_salts_are_rejected_as_curve_points() {
        let public_key = b64(&fixture(PAYMENT)["stxn"]["pqsig"]["pk"]);
        let (salt, _) = canonical_pq_address(&PQ_SCHEME_FALCON1024, &public_key).unwrap();

        assert!(salt > 0, "the vector should exercise the salt search");
        for lower in 0..salt {
            let address = pq_address(&PQ_SCHEME_FALCON1024, lower, &public_key);
            assert!(is_edwards25519_point(address.as_bytes()));
        }
    }

    #[test]
    fn canonical_address_matches_the_mnemonic_vector() {
        let mnemonic = fixture(MNEMONIC);

        let (_, address) =
            canonical_pq_address(&PQ_SCHEME_FALCON1024, &b64(&mnemonic["publicKey"])).unwrap();

        assert_eq!(address.to_string(), mnemonic["address"].as_str().unwrap());
    }

    #[test]
    fn key_seed_matches_the_mnemonic_vector() {
        let mnemonic = fixture(MNEMONIC);
        let entropy =
            algokit_crypto::algo25::seed_from_mnemonic(mnemonic["mnemonic"].as_str().unwrap())
                .unwrap();

        let seed = derive_pq_key_seed(&PQ_SCHEME_FALCON1024, &entropy);

        assert_eq!(seed.to_vec(), b64(&mnemonic["seed"]));
    }

    #[test]
    fn signed_payment_round_trips_byte_for_byte() {
        let payment = fixture(PAYMENT);
        let blob = b64(&payment["stxnBlob"]);

        let signed = SignedTransaction::decode(&blob).unwrap();

        assert_eq!(signed.encode_raw().unwrap(), blob);
        assert_eq!(signed.id().unwrap(), payment["id"].as_str().unwrap());
        assert_eq!(
            signed.pq_signature.unwrap().address(),
            signed.transaction.header().sender
        );
    }

    #[test]
    fn delegated_payment_round_trips_byte_for_byte() {
        let blob = b64(&fixture(DELEGATED_PAYMENT)["stxnBlob"]);

        let signed = SignedTransaction::decode(&blob).unwrap();
        let pqsig = signed
            .logic_signature
            .as_ref()
            .unwrap()
            .pq_signature
            .as_ref()
            .unwrap();

        assert_eq!(signed.encode_raw().unwrap(), blob);
        assert_eq!(pqsig.address(), signed.transaction.header().sender);
    }

    #[test]
    fn a_zero_salt_is_omitted_and_decodes_back() {
        let pqsig = PQSignature {
            scheme: PQ_SCHEME_FALCON1024,
            salt: 0,
            public_key: vec![1],
            signature: vec![2],
        };
        let encoded = rmp_serde::to_vec_named(&pqsig).unwrap();

        assert!(!map_keys(&encoded).contains(&"slt".to_string()));
        assert_eq!(
            rmp_serde::from_slice::<PQSignature>(&encoded).unwrap(),
            pqsig
        );
    }

    #[test]
    fn a_blank_signature_is_omitted_from_a_signed_transaction() {
        let signed = SignedTransaction {
            transaction: TransactionMother::simple_payment().build().unwrap(),
            signature: None,
            auth_address: None,
            multisignature: None,
            logic_signature: None,
            pq_signature: Some(PQSignature::default()),
        };

        assert_eq!(
            map_keys(&signed.encode_raw().unwrap()),
            vec!["txn".to_string()]
        );
    }

    #[test]
    fn validates_the_payment_vector() {
        assert!(payment_signature().validate().is_ok());
    }

    #[test]
    fn rejects_a_blank_signature() {
        assert!(PQSignature::default().validate().is_err());
    }

    #[test]
    fn rejects_an_unsupported_scheme() {
        let pqsig = PQSignature {
            scheme: *b"f2",
            ..payment_signature()
        };

        assert!(pqsig.validate().is_err());
    }

    #[test]
    fn rejects_a_salt_that_derives_a_curve_point() {
        let pqsig = PQSignature {
            salt: 0,
            ..payment_signature()
        };

        assert!(pqsig.validate().is_err());
    }
}
