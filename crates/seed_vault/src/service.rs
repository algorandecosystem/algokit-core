//! Ties the wire protocol (`protocol`) to account creation/listing/reveal
//! (`store`).
//!
//! [`handle_request`] has zero dependency on any particular IPC transport:
//! the real Trusty applet's `tipc::Service::on_message` and this crate's
//! own host-side tests both drive it identically, so `cargo test -p
//! seed_vault` proves the request-handling logic independent of a real
//! Trusty build.

use alloc::vec::Vec;

use crate::protocol::{self, ErrorCode, Request, WireAccount};
use crate::store::{self, AccountStore, StoreError};

/// Handles one raw request received over IPC and returns the raw response
/// bytes to send back to the caller.
pub fn handle_request(store: &mut impl AccountStore, request: &[u8]) -> Vec<u8> {
    match protocol::decode_request(request) {
        Ok(Request::CreateAccount { algorithm }) => match store::create_account(store, algorithm) {
            Ok(summary) => protocol::encode_create_account_response(
                summary.algorithm,
                summary.address_index,
                &summary.address,
            ),
            Err(err) => protocol::encode_error_response(error_code_for(err)),
        },
        Ok(Request::ListAccounts) => match store::list_accounts(store) {
            Ok(accounts) => {
                let wire_accounts: Vec<WireAccount> = accounts
                    .into_iter()
                    .map(|a| WireAccount {
                        algorithm: a.algorithm,
                        address_index: a.address_index,
                        address: a.address,
                    })
                    .collect();
                protocol::encode_list_accounts_response(&wire_accounts)
            }
            Err(err) => protocol::encode_error_response(error_code_for(err)),
        },
        Ok(Request::RevealMnemonic { address }) => match store::reveal_mnemonic(store, &address) {
            Ok(mnemonic) => protocol::encode_reveal_mnemonic_response(&mnemonic),
            Err(err) => protocol::encode_error_response(error_code_for(err)),
        },
        Ok(Request::ImportAccount {
            algorithm,
            mnemonic,
        }) => match store::import_account(store, algorithm, &mnemonic) {
            Ok(summary) => protocol::encode_create_account_response(
                summary.algorithm,
                summary.address_index,
                &summary.address,
            ),
            Err(err) => protocol::encode_error_response(error_code_for(err)),
        },
        Err(code) => protocol::encode_error_response(code),
    }
}

fn error_code_for(err: StoreError) -> ErrorCode {
    match err {
        StoreError::UnsupportedAlgorithm => ErrorCode::UnsupportedAlgorithm,
        StoreError::NotFound => ErrorCode::AccountNotFound,
        StoreError::InvalidMnemonic => ErrorCode::InvalidMnemonic,
        StoreError::AlreadyExists => ErrorCode::AccountAlreadyExists,
        StoreError::Backend
        | StoreError::CorruptRecord
        | StoreError::RandomnessUnavailable
        | StoreError::KeyDerivationFailed => ErrorCode::StoreFailure,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory_store::MemoryStore;
    use crate::store::KeyAlgorithm;
    use alloc::string::String;

    fn create_request() -> Vec<u8> {
        protocol::encode_request(Request::CreateAccount {
            algorithm: KeyAlgorithm::Algo25Ed25519,
        })
    }

    #[test]
    fn create_then_list_end_to_end_over_the_wire_protocol() {
        let mut store = MemoryStore::default();

        let resp1 = handle_request(&mut store, &create_request());
        let account1 =
            protocol::decode_create_account_response(&resp1).expect("expected an OK response");

        let resp2 = handle_request(&mut store, &create_request());
        let account2 =
            protocol::decode_create_account_response(&resp2).expect("expected an OK response");

        assert_ne!(account1.address, account2.address);

        let list_req = protocol::encode_request(Request::ListAccounts);
        let list_resp = handle_request(&mut store, &list_req);
        let accounts =
            protocol::decode_list_accounts_response(&list_resp).expect("expected an OK response");

        assert_eq!(accounts, alloc::vec![account1, account2]);
    }

    #[test]
    fn list_accounts_on_empty_store_returns_empty_list() {
        let mut store = MemoryStore::default();
        let resp = handle_request(&mut store, &protocol::encode_request(Request::ListAccounts));
        let accounts = protocol::decode_list_accounts_response(&resp).unwrap();
        assert!(accounts.is_empty());
    }

    #[test]
    fn malformed_request_yields_error_response() {
        let mut store = MemoryStore::default();
        let resp = handle_request(&mut store, &[0xFF]);
        assert_eq!(
            protocol::decode_error_code(&resp),
            Ok(ErrorCode::MalformedRequest)
        );
    }

    #[test]
    fn unsupported_algorithm_yields_error_response() {
        let mut store = MemoryStore::default();
        let resp = handle_request(&mut store, &[0x01, 0xFF]);
        assert_eq!(
            protocol::decode_error_code(&resp),
            Ok(ErrorCode::UnsupportedAlgorithm)
        );
    }

    #[test]
    fn reveal_mnemonic_end_to_end_over_the_wire_protocol() {
        let mut store = MemoryStore::default();
        let resp = handle_request(&mut store, &create_request());
        let account = protocol::decode_create_account_response(&resp).unwrap();

        let reveal_req = protocol::encode_request(Request::RevealMnemonic {
            address: account.address.clone(),
        });
        let reveal_resp = handle_request(&mut store, &reveal_req);
        let mnemonic = protocol::decode_reveal_mnemonic_response(&reveal_resp)
            .expect("expected an OK response");
        assert_eq!(mnemonic.split_whitespace().count(), 25);

        // Must reproduce the same address returned by CREATE_ACCOUNT.
        let seed = algokit_crypto::algo25::seed_from_mnemonic(&mnemonic).unwrap();
        let keypair =
            algokit_crypto::ed25519::CryptoxideEd25519Keypair::try_generate(Some(seed)).unwrap();
        use algokit_crypto::Keypair;
        let address = algokit_crypto::address::address_from_public_key(&keypair.verifying_key());
        assert_eq!(address, account.address);
    }

    #[test]
    fn reveal_mnemonic_unknown_address_is_account_not_found() {
        let mut store = MemoryStore::default();
        let resp = handle_request(
            &mut store,
            &protocol::encode_request(Request::RevealMnemonic {
                address: "A".repeat(58),
            }),
        );
        assert_eq!(
            protocol::decode_error_code(&resp),
            Ok(ErrorCode::AccountNotFound)
        );
    }

    #[test]
    fn import_account_end_to_end_over_the_wire_protocol() {
        let mut store = MemoryStore::default();
        let resp = handle_request(&mut store, &create_request());
        let created = protocol::decode_create_account_response(&resp).unwrap();

        let reveal_resp = handle_request(
            &mut store,
            &protocol::encode_request(Request::RevealMnemonic {
                address: created.address.clone(),
            }),
        );
        let mnemonic = protocol::decode_reveal_mnemonic_response(&reveal_resp).unwrap();

        // Import into a fresh store and confirm the same address comes back.
        let mut other_store = MemoryStore::default();
        let import_resp = handle_request(
            &mut other_store,
            &protocol::encode_request(Request::ImportAccount {
                algorithm: KeyAlgorithm::Algo25Ed25519,
                mnemonic,
            }),
        );
        let imported = protocol::decode_create_account_response(&import_resp)
            .expect("expected an OK response");
        assert_eq!(imported.address, created.address);
    }

    #[test]
    fn import_account_invalid_mnemonic_yields_error_response() {
        let mut store = MemoryStore::default();
        let resp = handle_request(
            &mut store,
            &protocol::encode_request(Request::ImportAccount {
                algorithm: KeyAlgorithm::Algo25Ed25519,
                mnemonic: "not a real mnemonic".into(),
            }),
        );
        assert_eq!(
            protocol::decode_error_code(&resp),
            Ok(ErrorCode::InvalidMnemonic)
        );
    }

    #[test]
    fn import_account_duplicate_yields_error_response() {
        let mut store = MemoryStore::default();
        let resp = handle_request(&mut store, &create_request());
        let created = protocol::decode_create_account_response(&resp).unwrap();
        let reveal_resp = handle_request(
            &mut store,
            &protocol::encode_request(Request::RevealMnemonic {
                address: created.address,
            }),
        );
        let mnemonic = protocol::decode_reveal_mnemonic_response(&reveal_resp).unwrap();

        let import_resp = handle_request(
            &mut store,
            &protocol::encode_request(Request::ImportAccount {
                algorithm: KeyAlgorithm::Algo25Ed25519,
                mnemonic,
            }),
        );
        assert_eq!(
            protocol::decode_error_code(&import_resp),
            Ok(ErrorCode::AccountAlreadyExists)
        );
    }

    #[test]
    fn responses_never_contain_the_seed() {
        // CREATE_ACCOUNT/LIST_ACCOUNTS addresses must decode as printable
        // base32 text, never raw key bytes. (REVEAL_MNEMONIC is exempt --
        // returning key material is its entire purpose.)
        let mut store = MemoryStore::default();
        let resp = handle_request(&mut store, &create_request());
        let account: WireAccount =
            protocol::decode_create_account_response(&resp).expect("expected an OK response");
        let address: String = account.address;
        assert!(
            address
                .bytes()
                .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit()),
            "address must be plain base32 text, not raw key bytes: {address:?}"
        );
    }
}
