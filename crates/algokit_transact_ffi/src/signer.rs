use crate::{AlgoKitTransactError, SignedTransaction, Transaction};
use algokit_crypto::Signer;
use algokit_crypto::ed25519::CryptoxideEd25519Keypair;

use algokit_transact::AlgorandMsgpack;
#[cfg(feature = "ffi_uniffi")]
use uniffi::{self};

/// Signs a transaction using Ed25519 with the provided secret key.
#[uniffi::export]
pub fn ed25519_sign_transaction(
    secret_key: Vec<u8>,
    txn: Transaction,
) -> Result<SignedTransaction, AlgoKitTransactError> {
    let keypair =
        CryptoxideEd25519Keypair::try_generate(Some(secret_key.try_into().map_err(|_| {
            AlgoKitTransactError::SigningError {
                error_msg: "Secret key must be 32 bytes for Ed25519".to_string(),
            }
        })?))
        .map_err(|e| AlgoKitTransactError::SigningError {
            error_msg: format!("Failed to generate keypair from secret key: {}", e),
        })?;

    let rust_txn: algokit_transact::Transaction =
        txn.clone()
            .try_into()
            .map_err(|e| AlgoKitTransactError::SigningError {
                error_msg: format!(
                    "Failed to convert FFI Transaction to Rust Transaction: {}",
                    e
                ),
            })?;

    let encoded = rust_txn
        .encode()
        .map_err(|e| AlgoKitTransactError::SigningError {
            error_msg: format!("Failed to encode transaction for signing: {}", e),
        })?;
    let signature = keypair.sign(&encoded);

    Ok(SignedTransaction {
        transaction: txn,
        auth_address: None,
        signature: Some(signature.into()),
        multisignature: None,
        logic_signature: None,
    })
}
