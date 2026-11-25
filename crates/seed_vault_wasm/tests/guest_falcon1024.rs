//! End to end with Falcon-1024 compiled to WASM (`guest/falcon1024`,
//! prebuilt as `fixtures/guest_falcon1024.wasm`): the C implementation the
//! TA links natively, run in the sandbox instead. Falcon here is
//! deterministic, so the WASM output must equal native `algokit_crypto`
//! byte for byte.
// These drive the TA side, which needs the default `vault` feature.
#![cfg(feature = "vault")]

mod common;

use std::time::{Duration, Instant};

use algokit_crypto::falcon1024::{account_from_entropy, canonical_pq_address};
use common::*;
use seed_vault::KeyAlgorithm;
use seed_vault_wasm::{derive_program_key, KeyNamespace, PublisherPolicy, VaultConfig};

const GUEST: &[u8] = include_bytes!("fixtures/guest_falcon1024.wasm");

fn timed<T>(f: impl FnOnce() -> T) -> (T, Duration) {
    let started = Instant::now();
    let result = f();
    (result, started.elapsed())
}

#[test]
fn falcon1024_in_wasm_matches_native_and_caches_its_key_across_restarts() {
    let mut ta = Ta::new();
    let address = ta.seed(KeyAlgorithm::Falcon1024, 11);
    let program = ta.upload(GUEST).unwrap();
    let native = account_from_entropy(&derive_program_key(
        &[11; 32],
        &KeyNamespace::program(&program),
        1,
    ))
    .unwrap();

    // First use: keygen, then cached.
    let (public_key, keygen) = timed(|| ta.public_key(&program, &address, 1).unwrap());
    assert_eq!(public_key, native.public_key.0.to_vec());
    let (_, wasm_address) = canonical_pq_address(&public_key).unwrap();
    assert_eq!(wasm_address, native.address);
    assert_eq!(ta.stored("svw1_key_").len(), 1);

    // After a restart: no re-upload, and the cached key pair is used.
    let mut ta = ta.restart(VaultConfig::default(), PublisherPolicy::dev());
    for i in 0..2u8 {
        let message = [b"TX".as_slice(), &[i; 200]].concat();
        let (signature, sign) = timed(|| ta.sign(&program, &address, 1, &message).unwrap());
        native.public_key.verify(&signature, &message).unwrap();
        assert_eq!(
            signature,
            native.private_key.sign_compressed(&message).unwrap().0
        );
        std::eprintln!("falcon1024 in wasmi: first use (keygen) {keygen:?}, cached sign {sign:?}");
    }
}

#[test]
fn falcon1024_wasm_accounts_are_separate_from_the_native_falcon_account() {
    let mut ta = Ta::new();
    let address = ta.seed(KeyAlgorithm::Falcon1024, 12);
    let program = ta.upload(GUEST).unwrap();
    let wasm_key = ta.public_key(&program, &address, 0).unwrap();
    // The vault's own Falcon account for this seed (same 25 words).
    let native = account_from_entropy(&[12; 32]).unwrap();
    assert_ne!(wasm_key, native.public_key.0.to_vec());
}

const GUEST_NATIVE: &[u8] = include_bytes!("fixtures/guest_falcon1024_native.wasm");

#[test]
fn native_build_reproduces_the_vaults_falcon1024_accounts() {
    let mut ta = Ta::signed_only();
    let address = ta.seed(KeyAlgorithm::Falcon1024, 11);
    // A new Falcon seed holds human-sign (0) and autosign (1); add 2 too.
    seed_vault::derive_account(&mut ta.accounts, &address, 2).unwrap();
    let native = seed_vault::list_accounts(&mut ta.accounts).unwrap();
    assert_eq!(native.len(), 3);
    // Signed, as in production: the manifest grants the native key
    // material (`account_entropy`) and index 0 (`human_sign`).
    let manifest = manifest_for(
        GUEST_NATIVE,
        "falcon1024",
        1,
        seed_vault_wasm::Capabilities::ALL,
    );
    let program = ta.upload_signed(GUEST_NATIVE, &manifest).unwrap();

    for account in &native {
        let index = account.address_index as u8;
        // Same public key, so the same PQ address.
        let public_key = ta.public_key(&program, &address, index).unwrap();
        assert_eq!(public_key, account.public_key, "index {index}");
        let (_, raw_address) = canonical_pq_address(&public_key).unwrap();
        assert_eq!(
            algokit_crypto::address::address_from_public_key(&raw_address),
            account.address,
            "index {index}"
        );

        // Deterministic Falcon: byte-identical to the native signature.
        let message = [b"wasm and native agree ".as_slice(), &[index]].concat();
        let wasm = ta.sign(&program, &address, index, &message).unwrap();
        let native = seed_vault::store::sign_arbitrary_data(
            &mut ta.accounts,
            &account.address,
            account.address_index,
            &message,
        )
        .unwrap()
        .signature;
        assert_eq!(wasm, native, "index {index}");
    }
}
