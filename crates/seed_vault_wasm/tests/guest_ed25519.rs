//! End to end with a real algorithm compiled to WASM: Ed25519
//! (`guest/ed25519`, prebuilt as `fixtures/guest_ed25519.wasm`), uploaded in
//! chunks over the wire protocol, then used repeatedly.
// These drive the TA side, which needs the default `vault` feature.
#![cfg(feature = "vault")]

mod common;

use common::*;
use seed_vault::KeyAlgorithm;
use seed_vault_wasm::protocol::request;
use seed_vault_wasm::{derive_program_key, KeyNamespace};

const GUEST: &[u8] = include_bytes!("fixtures/guest_ed25519.wasm");

#[test]
fn upload_once_then_sign_repeatedly() {
    let mut ta = Ta::new();
    let address = ta.seed(KeyAlgorithm::Algo25Ed25519, 7);

    // ~55 KB: well past one 4 KB message.
    assert!(GUEST.len() > MAX_MSG_LEN);
    let program = ta.upload(GUEST).unwrap();
    assert_eq!(program, seed_vault_wasm::ProgramId::of(GUEST));
    assert_eq!(ta.call(&request::list_programs()).unwrap()[0], 1);

    let public_key: [u8; 32] = ta
        .public_key(&program, &address, 1)
        .unwrap()
        .try_into()
        .unwrap();

    // The WASM program derived exactly the Ed25519 key the native code
    // derives from its program key.
    let expected = algokit_crypto::ed25519::CryptoxideEd25519Keypair::try_generate(Some(
        derive_program_key(&[7; 32], &KeyNamespace::program(&program), 1),
    ))
    .unwrap();
    use algokit_crypto::Keypair;
    assert_eq!(public_key, expected.verifying_key());

    for i in 0..3u8 {
        let message = [b"TX".as_slice(), &[i; 100]].concat();
        let signature: [u8; 64] = ta
            .sign(&program, &address, 1, &message)
            .unwrap()
            .try_into()
            .unwrap();
        assert!(algokit_crypto::ed25519::verify(
            &message,
            &public_key,
            &signature
        ));
    }
}

#[test]
fn keys_differ_per_index_and_seed_and_never_match_the_native_account() {
    let mut ta = Ta::new();
    let a = ta.seed(KeyAlgorithm::Algo25Ed25519, 1);
    let b = ta.seed(KeyAlgorithm::Falcon1024, 2);
    let program = ta.upload(GUEST).unwrap();

    let keys = [
        ta.public_key(&program, &a, 0).unwrap(),
        ta.public_key(&program, &a, 1).unwrap(),
        ta.public_key(&program, &b, 0).unwrap(),
    ];
    assert_ne!(keys[0], keys[1]);
    assert_ne!(keys[0], keys[2]);
    // The native Ed25519 account for seed `a` is a different key: the
    // program never gets the seed itself.
    let native =
        algokit_crypto::ed25519::CryptoxideEd25519Keypair::try_generate(Some([1; 32])).unwrap();
    use algokit_crypto::Keypair;
    assert_ne!(keys[0], native.verifying_key().to_vec());
    // Deterministic across calls.
    assert_eq!(keys[0], ta.public_key(&program, &a, 0).unwrap());
}

const GUEST_NATIVE: &[u8] = include_bytes!("fixtures/guest_ed25519_native.wasm");

#[test]
fn native_build_reproduces_the_vaults_algo25_account() {
    let mut ta = Ta::signed_only();
    let address = ta.seed(KeyAlgorithm::Algo25Ed25519, 7);
    let native = seed_vault::list_accounts(&mut ta.accounts)
        .unwrap()
        .remove(0);
    // Signed, as in production: the manifest grants the native key
    // material (`account_entropy`) and index 0 (`human_sign`).
    let manifest = manifest_for(
        GUEST_NATIVE,
        "algo25",
        1,
        seed_vault_wasm::Capabilities::ALL,
    );
    let program = ta.upload_signed(GUEST_NATIVE, &manifest).unwrap();

    // Same public key, so the same Algorand address.
    let public_key: [u8; 32] = ta
        .public_key(&program, &address, 0)
        .unwrap()
        .try_into()
        .unwrap();
    assert_eq!(public_key.to_vec(), native.public_key);
    assert_eq!(
        algokit_crypto::address::address_from_public_key(&public_key),
        address
    );

    // Ed25519 is deterministic: byte-identical to the native signature.
    let message = b"wasm and native agree";
    let wasm = ta.sign(&program, &address, 0, message).unwrap();
    let native = seed_vault::store::sign_arbitrary_data(&mut ta.accounts, &address, 0, message)
        .unwrap()
        .signature;
    assert_eq!(wasm, native);

    // Ed25519 accounts only have index 0, natively and in WASM.
    assert_eq!(
        ta.public_key(&program, &address, 1),
        Err(seed_vault_wasm::protocol::ErrorCode::ProgramFailed)
    );
}
