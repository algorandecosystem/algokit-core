//! Demonstrates creating and recovering an Algorand "algo25" (25-word mnemonic)
//! account using nothing but the `algokit_crypto` crate.
//!
//! This example is intentionally free of filesystem, network, and OS-thread usage,
//! so the exact same logic can run unmodified inside constrained/embedded targets --
//! including a Trusty TEE Trusted Application targeting `aarch64-unknown-trusty`,
//! which only exposes a partial `std` (no fs/net/thread-spawn, but `alloc` and a
//! secure RNG backend are both available). See `trusty/app/seed_vault_ta` for a sketch
//! of what wiring this into a real Trusty applet looks like.
//!
//! Run with:
//!   cargo run -p algokit_crypto --example algo25_roundtrip

use algokit_crypto::Keypair;
use algokit_crypto::address::address_from_public_key;
use algokit_crypto::algo25::{mnemonic_from_seed, seed_from_mnemonic};
use algokit_crypto::ed25519::{CryptoxideEd25519Keypair, Ed25519Signer};

fn hex_encode(bytes: &[u8]) -> String {
    use std::fmt::Write;
    bytes.iter().fold(String::new(), |mut out, b| {
        let _ = write!(out, "{b:02x}");
        out
    })
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // ---- 1. CREATE a brand new algo25 account ------------------------------
    // The 32-byte seed *is* the account's private key material. `getrandom`
    // pulls from the platform CSPRNG -- on `*-unknown-trusty` this resolves to
    // the target's built-in secure RNG backend, so this call is portable as-is.
    let mut seed = [0u8; 32];
    getrandom::fill(&mut seed).expect("failed to source randomness");

    let keypair = CryptoxideEd25519Keypair::try_generate(Some(seed))
        .expect("failed to derive keypair from seed");
    let public_key = keypair.verifying_key();
    let address = address_from_public_key(&public_key);

    let mnemonic = mnemonic_from_seed(&seed).expect("failed to encode 25-word mnemonic");

    println!("== Created new algo25 account ==");
    println!("  Address:  {address}");
    println!("  Mnemonic: {mnemonic}");

    // ---- 2. RECOVER the same account from *only* the mnemonic --------------
    // Simulates a user re-entering their 25-word backup phrase on a new
    // device/applet that has never seen the original seed.
    let recovered_seed = seed_from_mnemonic(&mnemonic).expect("failed to decode mnemonic");
    assert_eq!(
        seed, recovered_seed,
        "recovered seed must match the original seed"
    );

    let recovered_keypair = CryptoxideEd25519Keypair::try_generate(Some(recovered_seed))
        .expect("failed to derive keypair from recovered seed");
    let recovered_public_key = recovered_keypair.verifying_key();
    let recovered_address = address_from_public_key(&recovered_public_key);

    assert_eq!(
        public_key, recovered_public_key,
        "recovered keypair must match the original keypair"
    );
    assert_eq!(address, recovered_address);

    println!("== Recovered account from mnemonic ==");
    println!("  Address:  {recovered_address}");

    // ---- 3. Prove the recovered key is actually usable for signing ---------
    let message = b"hello from algokit_crypto";
    let signature = recovered_keypair
        .try_sign(message)
        .await
        .expect("failed to sign with recovered key");

    let is_valid = cryptoxide::ed25519::verify(message, &recovered_public_key, &signature);
    assert!(
        is_valid,
        "signature produced by the recovered key must verify"
    );

    println!("== Signature round-trip verified ==");
    println!("  Signature: {}", hex_encode(&signature));
    println!("\nSuccess: created and recovered an algo25 account with algokit_crypto.");
}
