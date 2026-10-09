//! Ties the wire protocol (`protocol`) to account creation/listing/reveal
//! (`store`).
//!
//! [`handle_request`] has zero dependency on any particular IPC transport:
//! the real Trusty applet's `tipc::Service::on_message` and this crate's
//! own host-side tests both drive it identically, so `cargo test -p
//! seed_vault` proves the request-handling logic independent of a real
//! Trusty build.

use alloc::vec::Vec;

use crate::protocol::{self, ErrorCode, ListedAccount, Request, WireAccount};
use crate::store::{self, AccountStore, StoreError};

/// Handles one raw request received over IPC and returns the raw response
/// bytes to send back to the caller.
pub fn handle_request(store: &mut impl AccountStore, request: &[u8]) -> Vec<u8> {
    match protocol::decode_request(request) {
        Ok(Request::CreateAccount {
            algorithm,
            address_index,
        }) => match store::create_account(store, algorithm, address_index.into()) {
            Ok(summary) => protocol::encode_create_account_response(
                summary.algorithm,
                summary.address_index,
                &summary.address,
            ),
            Err(err) => protocol::encode_error_response(error_code_for(err)),
        },
        Ok(Request::ListAccounts { start }) => match store::list_accounts(store) {
            Ok(accounts) => {
                let listed: Vec<ListedAccount> = accounts
                    .into_iter()
                    .map(|a| ListedAccount {
                        account: WireAccount {
                            algorithm: a.algorithm,
                            address_index: a.address_index,
                            address: a.address,
                        },
                        public_key: a.public_key,
                    })
                    .collect();
                protocol::encode_list_accounts_response(&listed, start)
            }
            Err(err) => protocol::encode_error_response(error_code_for(err)),
        },
        Ok(Request::RevealMnemonic { address }) => match store::reveal_mnemonic(store, &address) {
            Ok(mnemonic) => protocol::encode_reveal_mnemonic_response(&mnemonic),
            Err(err) => protocol::encode_error_response(error_code_for(err)),
        },
        Ok(Request::ImportAccount {
            algorithm,
            address_index,
            mnemonic,
        }) => match store::import_account(store, algorithm, address_index.into(), &mnemonic) {
            Ok(summary) => protocol::encode_create_account_response(
                summary.algorithm,
                summary.address_index,
                &summary.address,
            ),
            Err(err) => protocol::encode_error_response(error_code_for(err)),
        },
        Ok(Request::DeleteAccount { address }) => match store::delete_account(store, &address) {
            Ok(summary) => protocol::encode_create_account_response(
                summary.algorithm,
                summary.address_index,
                &summary.address,
            ),
            Err(err) => protocol::encode_error_response(error_code_for(err)),
        },
        Ok(Request::SignTransaction {
            address_index,
            address,
            txn,
        }) => match store::sign_transaction(store, &address, address_index.into(), &txn) {
            Ok(signed) => {
                protocol::encode_sign_transaction_response(signed.algorithm, &signed.prefix)
            }
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
        StoreError::InvalidTransaction => ErrorCode::InvalidTransaction,
        StoreError::SigningFailed => ErrorCode::SigningFailed,
        StoreError::UnknownKeyIndex => ErrorCode::UnknownAddressIndex,
        StoreError::SenderMismatch => ErrorCode::SenderMismatch,
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

    /// Every account the TA holds, following `LIST_ACCOUNTS` pages.
    fn list_all(store: &mut MemoryStore) -> Vec<ListedAccount> {
        let mut all = Vec::new();
        loop {
            let page = protocol::decode_list_accounts_response(&handle_request(
                store,
                &protocol::encode_request(Request::ListAccounts {
                    start: all.len() as u32,
                }),
            ))
            .expect("expected an OK response");
            if page.accounts.is_empty() {
                assert_eq!(all.len(), page.total as usize);
                return all;
            }
            all.extend(page.accounts);
        }
    }

    fn list_wire_accounts(store: &mut MemoryStore) -> Vec<WireAccount> {
        list_all(store).into_iter().map(|l| l.account).collect()
    }

    fn create_request() -> Vec<u8> {
        protocol::encode_request(Request::CreateAccount {
            algorithm: KeyAlgorithm::Algo25Ed25519,
            address_index: 0,
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

        assert_eq!(
            list_wire_accounts(&mut store),
            alloc::vec![account1, account2]
        );
    }

    #[test]
    fn list_accounts_on_empty_store_returns_empty_list() {
        let mut store = MemoryStore::default();
        let resp = handle_request(
            &mut store,
            &protocol::encode_request(Request::ListAccounts { start: 0 }),
        );
        let page = protocol::decode_list_accounts_response(&resp).unwrap();
        assert_eq!(page.total, 0);
        assert!(page.accounts.is_empty());
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
        let resp = handle_request(&mut store, &[0x01, 0xFF, 0x00]);
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
                address_index: 0,
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
                address_index: 0,
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
                address_index: 0,
                mnemonic,
            }),
        );
        assert_eq!(
            protocol::decode_error_code(&import_resp),
            Ok(ErrorCode::AccountAlreadyExists)
        );
    }

    #[test]
    fn falcon1024_create_reveal_import_over_the_wire_protocol() {
        let mut store = MemoryStore::default();
        let resp = handle_request(
            &mut store,
            &protocol::encode_request(Request::CreateAccount {
                algorithm: KeyAlgorithm::Falcon1024,
                address_index: 0,
            }),
        );
        let created = protocol::decode_create_account_response(&resp).unwrap();
        assert_eq!(created.algorithm, KeyAlgorithm::Falcon1024);

        let reveal_resp = handle_request(
            &mut store,
            &protocol::encode_request(Request::RevealMnemonic {
                address: created.address.clone(),
            }),
        );
        let mnemonic = protocol::decode_reveal_mnemonic_response(&reveal_resp).unwrap();

        let mut other_store = MemoryStore::default();
        let import_resp = handle_request(
            &mut other_store,
            &protocol::encode_request(Request::ImportAccount {
                algorithm: KeyAlgorithm::Falcon1024,
                address_index: 0,
                mnemonic,
            }),
        );
        let imported = protocol::decode_create_account_response(&import_resp).unwrap();
        assert_eq!(imported, created);
    }

    #[test]
    fn delete_account_end_to_end_over_the_wire_protocol() {
        let mut store = MemoryStore::default();
        let created = protocol::decode_create_account_response(&handle_request(
            &mut store,
            &protocol::encode_request(Request::CreateAccount {
                algorithm: KeyAlgorithm::Falcon1024,
                address_index: 0,
            }),
        ))
        .unwrap();
        let kept = protocol::decode_create_account_response(&handle_request(
            &mut store,
            &create_request(),
        ))
        .unwrap();

        let delete_req = protocol::encode_request(Request::DeleteAccount {
            address: created.address.clone(),
        });
        let deleted =
            protocol::decode_create_account_response(&handle_request(&mut store, &delete_req))
                .unwrap();
        assert_eq!(deleted, created);

        assert_eq!(list_wire_accounts(&mut store), alloc::vec![kept]);

        // Deleting again is AccountNotFound.
        assert_eq!(
            protocol::decode_error_code(&handle_request(&mut store, &delete_req)),
            Ok(ErrorCode::AccountNotFound)
        );
    }

    use crate::signed_txn::test_txn_from;

    fn import(store: &mut MemoryStore, algorithm: KeyAlgorithm, entropy: &[u8; 32]) -> WireAccount {
        let mnemonic = algokit_crypto::algo25::mnemonic_from_seed(entropy).unwrap();
        protocol::decode_create_account_response(&handle_request(
            store,
            &protocol::encode_request(Request::ImportAccount {
                algorithm,
                address_index: 0,
                mnemonic,
            }),
        ))
        .unwrap()
    }

    fn sign(store: &mut MemoryStore, address: &str, txn: &[u8]) -> Vec<u8> {
        handle_request(
            store,
            &protocol::encode_request(Request::SignTransaction {
                address_index: 0,
                address: address.into(),
                txn: txn.to_vec(),
            }),
        )
    }

    /// Reads a msgpack bin8/bin16 at the start of `bytes`: `(body, rest)`.
    fn read_bin(bytes: &[u8]) -> (&[u8], &[u8]) {
        let (len, start) = match bytes[0] {
            0xc4 => (bytes[1] as usize, 2),
            0xc5 => (u16::from_be_bytes([bytes[1], bytes[2]]) as usize, 3),
            other => panic!("not a bin: {other:#x}"),
        };
        bytes[start..].split_at(len)
    }

    fn strip<'a>(bytes: &'a [u8], expected: &[u8]) -> &'a [u8] {
        assert!(bytes.starts_with(expected), "expected {expected:x?}");
        &bytes[expected.len()..]
    }

    #[test]
    fn falcon1024_sign_transaction_over_the_wire_protocol() {
        let entropy: [u8; 32] = core::array::from_fn(|i| (i + 1) as u8);
        let mut store = MemoryStore::default();
        let account = import(&mut store, KeyAlgorithm::Falcon1024, &entropy);

        let txn = test_txn_from(&account.address);
        let resp = sign(&mut store, &account.address, &txn);
        let (algorithm, prefix) = protocol::decode_sign_transaction_response(&resp).unwrap();
        assert_eq!(algorithm, KeyAlgorithm::Falcon1024);
        assert!(prefix.len() <= crate::signed_txn::MAX_PREFIX_LEN);
        // Deterministic Falcon: same txn, same bytes.
        assert_eq!(sign(&mut store, &account.address, &txn), resp);

        // Walk the canonical `{pqsig: {pk, sch, sig[, slt]}, txn}` layout.
        let rest = strip(&prefix, b"\x82\xa5pqsig");
        let has_salt = rest[0] == 0x84;
        let rest = strip(&rest[1..], b"\xa2pk");
        let (pk, rest) = read_bin(rest);
        let rest = strip(rest, b"\xa3sch\xc4\x02f1\xa3sig");
        let (sig, rest) = read_bin(rest);
        let (salt, rest) = if has_salt {
            let rest = strip(rest, b"\xa3slt");
            match rest[0] {
                0xcc => (rest[1], &rest[2..]),
                v => (v, &rest[1..]),
            }
        } else {
            (0, rest)
        };
        assert_eq!(rest, b"\xa3txn");

        let expected = algokit_crypto::falcon1024::account_from_entropy(&entropy).unwrap();
        assert_eq!(pk, &expected.public_key.0[..]);
        assert_eq!(salt, expected.salt);
        assert_eq!(
            algokit_crypto::address::address_from_public_key(
                &algokit_crypto::falcon1024::pq_address(
                    algokit_crypto::falcon1024::SCHEME_FALCON1024,
                    salt,
                    pk,
                )
            ),
            account.address
        );
        expected
            .public_key
            .verify(sig, &crate::signed_txn::bytes_to_sign(&txn))
            .expect("Falcon signature must verify over \"TX\" || txn");
    }

    #[test]
    fn ed25519_sign_transaction_over_the_wire_protocol() {
        let entropy = [9u8; 32];
        let mut store = MemoryStore::default();
        let account = import(&mut store, KeyAlgorithm::Algo25Ed25519, &entropy);

        let (algorithm, prefix) = protocol::decode_sign_transaction_response(&sign(
            &mut store,
            &account.address,
            &test_txn_from(&account.address),
        ))
        .unwrap();
        assert_eq!(algorithm, KeyAlgorithm::Algo25Ed25519);
        let rest = strip(&prefix, b"\x82\xa3sig\xc4\x40");
        let (sig, rest) = rest.split_at(64);
        assert_eq!(rest, b"\xa3txn");

        let keypair =
            algokit_crypto::ed25519::CryptoxideEd25519Keypair::try_generate(Some(entropy)).unwrap();
        assert!(algokit_crypto::ed25519::verify(
            &crate::signed_txn::bytes_to_sign(&test_txn_from(&account.address)),
            &algokit_crypto::Keypair::verifying_key(&keypair),
            sig.try_into().unwrap(),
        ));
    }

    #[test]
    fn sign_transaction_rejects_bad_input() {
        let mut store = MemoryStore::default();
        let account = import(&mut store, KeyAlgorithm::Falcon1024, &[3u8; 32]);

        // Already "TX"-prefixed bytes (or anything that isn't a msgpack map).
        let txn = test_txn_from(&account.address);
        let prefixed = [b"TX".as_slice(), &txn].concat();
        assert_eq!(
            protocol::decode_error_code(&sign(&mut store, &account.address, &prefixed)),
            Ok(ErrorCode::InvalidTransaction)
        );
        // Someone else's transaction.
        let other = algokit_crypto::address::address_from_public_key(&[3u8; 32]);
        assert_eq!(
            protocol::decode_error_code(&sign(
                &mut store,
                &account.address,
                &test_txn_from(&other)
            )),
            Ok(ErrorCode::SenderMismatch)
        );
        // A well-formed request for an address this vault doesn't hold.
        assert_eq!(
            protocol::decode_error_code(&sign(&mut store, &other, &test_txn_from(&other))),
            Ok(ErrorCode::AccountNotFound)
        );
    }

    #[test]
    fn list_accounts_returns_public_keys_across_pages() {
        let mut store = MemoryStore::default();
        let ed_entropy = [9u8; 32];
        let ed = import(&mut store, KeyAlgorithm::Algo25Ed25519, &ed_entropy);
        let falcon_entropies: Vec<[u8; 32]> = (1..=4u8).map(|i| [i; 32]).collect();
        let falcons: Vec<_> = falcon_entropies
            .iter()
            .map(|e| import(&mut store, KeyAlgorithm::Falcon1024, e))
            .collect();

        // 1 Ed25519 account + 2 per Falcon seed; 8 Falcon keys (~1.9 KB per
        // entry) can't fit in one message.
        let first = protocol::decode_list_accounts_response(&handle_request(
            &mut store,
            &protocol::encode_request(Request::ListAccounts { start: 0 }),
        ))
        .unwrap();
        assert_eq!(first.total, 9);
        assert!(first.accounts.len() < 9);

        let listed = list_all(&mut store);
        assert_eq!(
            listed
                .iter()
                .map(|l| l.account.address_index)
                .collect::<Vec<_>>(),
            alloc::vec![0, 0, 1, 0, 1, 0, 1, 0, 1]
        );
        // Only check each Falcon seed's human-sign (index 0) entry below;
        // index 1 is covered by the store tests.
        let listed: Vec<_> = core::iter::once(listed[0].clone())
            .chain(listed[1..].iter().step_by(2).cloned())
            .collect();

        // Ed25519: the 32-byte key *is* the address.
        assert_eq!(listed[0].account, ed);
        assert_eq!(
            algokit_crypto::address::address_from_public_key(
                listed[0].public_key.as_slice().try_into().unwrap()
            ),
            ed.address
        );

        // Falcon-1024: the full 1793-byte key, matching a fresh derivation
        // and hashing (with the canonical salt) to the address.
        for ((entry, expected), entropy) in listed[1..].iter().zip(&falcons).zip(&falcon_entropies)
        {
            assert_eq!(&entry.account, expected);
            let derived = algokit_crypto::falcon1024::account_from_entropy(entropy).unwrap();
            assert_eq!(entry.public_key, derived.public_key.0.to_vec());
            let (_, address) =
                algokit_crypto::falcon1024::canonical_pq_address(&entry.public_key).unwrap();
            assert_eq!(
                algokit_crypto::address::address_from_public_key(&address),
                expected.address
            );
        }
    }

    #[test]
    fn autosign_index_over_the_wire_protocol() {
        let mut store = MemoryStore::default();
        let create = |store: &mut MemoryStore, algorithm, address_index| {
            handle_request(
                store,
                &protocol::encode_request(Request::CreateAccount {
                    algorithm,
                    address_index,
                }),
            )
        };

        // Falcon-1024: index 1 is the seed's autosign account.
        let autosign = protocol::decode_create_account_response(&create(
            &mut store,
            KeyAlgorithm::Falcon1024,
            1,
        ))
        .unwrap();
        assert_eq!(autosign.address_index, 1);
        let sign_with = |store: &mut MemoryStore, address_index| {
            handle_request(
                store,
                &protocol::encode_request(Request::SignTransaction {
                    address_index,
                    address: autosign.address.clone(),
                    txn: test_txn_from(&autosign.address),
                }),
            )
        };
        assert!(protocol::decode_sign_transaction_response(&sign_with(&mut store, 1)).is_ok());
        assert_eq!(
            protocol::decode_error_code(&sign_with(&mut store, 0)),
            Ok(ErrorCode::UnknownAddressIndex)
        );

        // Ed25519 has no autosign key.
        assert_eq!(
            protocol::decode_error_code(&create(&mut store, KeyAlgorithm::Algo25Ed25519, 1)),
            Ok(ErrorCode::UnknownAddressIndex)
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
