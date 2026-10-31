//! End-to-end account lifecycle tests: create from fresh entropy -> back up
//! as a 25-word mnemonic -> recover from *only* the mnemonic -> sign ->
//! verify, for every account type the Trusty `seed_vault` TA supports.
//!
//! The per-module unit tests cover each primitive in isolation; these prove
//! they compose. Everything here is free of fs/net/thread usage and runs
//! against the `no_std` + `alloc` build Trusty uses:
//!
//!   cargo test -p algokit_crypto --no-default-features --test account_roundtrip

use algokit_crypto::Keypair;
use algokit_crypto::address::address_from_public_key;
use algokit_crypto::algo25::{mnemonic_from_seed, seed_from_mnemonic};
use algokit_crypto::ed25519::{CryptoxideEd25519Keypair, Ed25519Signer};
use algokit_crypto::falcon1024;

fn fresh_entropy() -> [u8; 32] {
    let mut entropy = [0u8; 32];
    getrandom::fill(&mut entropy).expect("failed to source randomness");
    entropy
}

/// Same bytes a transaction signer signs: `"TX" || msgpack(txn)`.
const MESSAGE: &[u8] = b"TX\x81\xa3snd\xc4\x20hello from algokit_crypto";

#[tokio::test(flavor = "current_thread")]
async fn algo25_ed25519_create_recover_sign() {
    // Create.
    let seed = fresh_entropy();
    let keypair = CryptoxideEd25519Keypair::try_generate(Some(seed)).unwrap();
    let public_key = keypair.verifying_key();
    let address = address_from_public_key(&public_key);
    let mnemonic = mnemonic_from_seed(&seed).unwrap();
    assert_eq!(mnemonic.split_whitespace().count(), 25);

    // Recover from only the mnemonic.
    let recovered_seed = seed_from_mnemonic(&mnemonic).unwrap();
    assert_eq!(recovered_seed, seed);
    let recovered = CryptoxideEd25519Keypair::try_generate(Some(recovered_seed)).unwrap();
    assert_eq!(recovered.verifying_key(), public_key);
    assert_eq!(address_from_public_key(&recovered.verifying_key()), address);

    // The recovered key signs for the original account.
    let signature = recovered.try_sign(MESSAGE).await.unwrap();
    assert!(cryptoxide::ed25519::verify(
        MESSAGE,
        &public_key,
        &signature
    ));
    assert!(!cryptoxide::ed25519::verify(
        b"tampered",
        &public_key,
        &signature
    ));
}

#[test]
fn falcon1024_create_recover_sign() {
    // Create.
    let entropy = fresh_entropy();
    let account = falcon1024::account_from_entropy(&entropy).unwrap();
    assert!(falcon1024::is_pq_compliant(&account.address));
    assert_eq!(
        (account.salt, account.address),
        falcon1024::canonical_pq_address(&account.public_key.0).unwrap()
    );
    let mnemonic = mnemonic_from_seed(&entropy).unwrap();

    // Recover from only the mnemonic.
    let recovered = falcon1024::account_from_mnemonic(&mnemonic).unwrap();
    assert_eq!(recovered.public_key.0, account.public_key.0);
    assert_eq!(recovered.salt, account.salt);
    assert_eq!(recovered.address_string(), account.address_string());

    // The same 25 words back a *different* algo25 (Ed25519) address.
    let ed25519 = CryptoxideEd25519Keypair::try_generate(Some(entropy)).unwrap();
    assert_ne!(
        address_from_public_key(&ed25519.verifying_key()),
        account.address_string()
    );

    // The recovered key signs for the original account, deterministically.
    let signature = recovered.private_key.sign_compressed(MESSAGE).unwrap();
    assert!(signature.len() <= falcon1024::SIGNATURE_MAX_SIZE);
    assert_eq!(
        signature.0,
        account.private_key.sign_compressed(MESSAGE).unwrap().0
    );
    account.public_key.verify(&signature, MESSAGE).unwrap();
    assert!(account.public_key.verify(&signature, b"tampered").is_err());
}
