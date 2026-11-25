//! A program that isn't a signing algorithm: an Algorand light client
//! (`guest/light_client`, a Rust port of algorand/light-client-poc), run
//! with persistent state against light-client-poc's sample data
//! (`fixtures/light_client`) -- driven through `seed_vault_wasm::client` and
//! `seed_vault_wasm::light_client`, the helpers the Android FFI uses.
// These drive the TA side, which needs the default `vault` feature.
#![cfg(feature = "vault")]

mod common;

use std::time::Instant;

use base64::Engine;
use common::*;
use seed_vault_wasm::client::{Client, ClientError};
use seed_vault_wasm::light_client::{
    self as lc, LightBlockHeaderProof, LightClientCallError, LightClientError, StateProofMessage,
    TransactionProof,
};
use seed_vault_wasm::protocol::ErrorCode;
use seed_vault_wasm::service::handle_request;
use seed_vault_wasm::{Capabilities, ProgramId, PublisherPolicy, VaultConfig, WasmPolicy};
use serde_json::Value;

const GUEST: &[u8] = include_bytes!("fixtures/guest_light_client.wasm");
const DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/light_client/");

fn json(name: &str) -> Value {
    serde_json::from_str(&std::fs::read_to_string(format!("{DIR}{name}")).unwrap()).unwrap()
}

fn b64(v: &Value) -> Vec<u8> {
    base64::engine::general_purpose::STANDARD
        .decode(v.as_str().unwrap())
        .unwrap()
}

fn array32(v: &Value) -> [u8; 32] {
    let bytes: Vec<u8> = v
        .as_array()
        .unwrap()
        .iter()
        .map(|b| b.as_u64().unwrap() as u8)
        .collect();
    bytes.try_into().unwrap()
}

/// A client over the in-process TA.
fn client<P: WasmPolicy>(ta: &mut Ta<P>) -> Client<impl FnMut(&[u8]) -> Result<Vec<u8>, ()> + '_> {
    Client::new(move |request: &[u8]| {
        assert!(request.len() <= MAX_MSG_LEN);
        Ok(handle_request(
            &mut ta.vault,
            &mut ta.accounts,
            &mut ta.wasm_store,
            request,
        ))
    })
}

type LcResult<T> = Result<T, LightClientCallError<()>>;

/// INIT from the sample genesis data (as in light-client-poc's main.go).
fn init(c: &mut Client<impl FnMut(&[u8]) -> Result<Vec<u8>, ()>>, p: &ProgramId) -> LcResult<()> {
    lc::init(
        c,
        p,
        9,
        8,
        1000,
        &b64(&json("genesis_voters_commitment.txt")),
        json("genesis_voters_ln_proven_weight.txt")
            .as_u64()
            .unwrap(),
    )
}

fn message() -> StateProofMessage {
    let m = json("state_proof_message.json");
    StateProofMessage {
        block_headers_commitment: b64(&m["BlockHeadersCommitment"]),
        voters_commitment: b64(&m["VotersCommitment"]),
        ln_proven_weight: m["LnProvenWeight"].as_u64().unwrap(),
        first_attested_round: m["FirstAttestedRound"].as_u64().unwrap(),
        last_attested_round: m["LastAttestedRound"].as_u64().unwrap(),
    }
}

fn state_proof() -> Vec<u8> {
    b64(&json("state_proof.txt"))
}

/// VERIFY_TXN for the sample transaction, or a different `txid`.
fn verify(
    c: &mut Client<impl FnMut(&[u8]) -> Result<Vec<u8>, ()>>,
    p: &ProgramId,
    tamper: impl FnOnce(&mut [u8; 32]),
) -> LcResult<()> {
    let mut txid = array32(&json("transaction_id.txt"));
    tamper(&mut txid);
    let t = json("transaction_proof_response.json");
    let h = json("light_block_header_proof_response.json");
    lc::verify_transaction(
        c,
        p,
        &txid,
        json("round.txt").as_u64().unwrap(),
        &array32(&json("genesis_hash.txt")),
        &array32(&json("seed.txt")),
        &TransactionProof {
            hash_type: t["hashtype"].as_str().unwrap().into(),
            stib_hash: b64(&t["stibhash"]).try_into().unwrap(),
            index: t["idx"].as_u64().unwrap(),
            tree_depth: t["treedepth"].as_u64().unwrap(),
            proof: b64(&t["proof"]),
        },
        &LightBlockHeaderProof {
            index: h["index"].as_u64().unwrap(),
            tree_depth: h["treedepth"].as_u64().unwrap(),
            proof: b64(&h["proof"]),
        },
    )
}

fn refused<T: std::fmt::Debug>(r: LcResult<T>) -> LightClientError {
    match r {
        Err(LightClientCallError::LightClient(e)) => e,
        other => panic!("expected a light client error, got {other:?}"),
    }
}

#[test]
fn light_client_verifies_a_state_proof_and_a_transaction_in_the_ta() {
    let mut ta = Ta::new();
    let mut c = client(&mut ta);
    // No keys, no seed: a general program.
    let p = c.install(GUEST, None).unwrap();

    assert_eq!(lc::status(&mut c, &p), Ok(None));
    init(&mut c, &p).unwrap();
    assert_eq!(
        refused(init(&mut c, &p)),
        LightClientError::AlreadyInitialized
    );

    // Nothing verified yet, so the transaction can't be.
    assert_eq!(
        refused(verify(&mut c, &p, |_| {})),
        LightClientError::NoStateProofForRound
    );

    // The ~16.5 KB state proof goes up in chunks.
    assert!(state_proof().len() > MAX_MSG_LEN);
    let started = Instant::now();
    assert_eq!(lc::advance(&mut c, &p, &message(), &state_proof()), Ok(1));
    eprintln!(
        "light client in wasmi: ADVANCE (state proof verification) {:?}",
        started.elapsed()
    );
    let status = lc::status(&mut c, &p).unwrap().unwrap();
    assert_eq!(
        (
            status.first_attested_round,
            status.interval_size,
            status.next_interval
        ),
        (9, 8, 1)
    );
    assert_eq!(status.voters_commitment, message().voters_commitment);

    assert_eq!(verify(&mut c, &p, |_| {}), Ok(()));
    assert_eq!(
        refused(verify(&mut c, &p, |txid| txid[0] ^= 1)),
        LightClientError::RootMismatch
    );
}

#[test]
fn a_forged_state_proof_message_changes_nothing() {
    let mut ta = Ta::new();
    let mut c = client(&mut ta);
    let p = c.install(GUEST, None).unwrap();
    init(&mut c, &p).unwrap();

    // A relayer lying about the block headers commitment.
    let mut forged = message();
    forged.block_headers_commitment[0] ^= 1;
    assert!(matches!(
        lc::advance(&mut c, &p, &forged, &state_proof()),
        Err(LightClientCallError::LightClient(_))
    ));
    assert_eq!(lc::status(&mut c, &p).unwrap().unwrap().next_interval, 0);
    assert_eq!(
        refused(verify(&mut c, &p, |_| {})),
        LightClientError::NoStateProofForRound
    );
}

#[test]
fn the_oracle_survives_restarts_and_upgrades_and_goes_with_unload() {
    let mut ta = Ta::signed_only();
    // Signed, with no capabilities: a light client needs none.
    let none = Capabilities::default();
    let v1_manifest = manifest_for(GUEST, "light-client", 1, none);
    {
        let mut c = client(&mut ta);
        let v1 = c.install(GUEST, Some(&v1_manifest)).unwrap();
        init(&mut c, &v1).unwrap();
        lc::advance(&mut c, &v1, &message(), &state_proof()).unwrap();
    }

    // Restart: the verified state is still there.
    let mut ta = ta.restart(
        VaultConfig::default(),
        PublisherPolicy::new(vec![seed_vault_wasm::dev_publisher_key()]),
    );
    let mut c = client(&mut ta);
    let v1 = ProgramId::of(GUEST);
    assert_eq!(verify(&mut c, &v1, |_| {}), Ok(()));

    // v2 (same code plus a custom section, so a different module) takes
    // over v1's oracle.
    let mut v2_bytes = GUEST.to_vec();
    v2_bytes.extend_from_slice(&[0, 4, 3, b'v', b'2', b'!']);
    let v2_manifest = manifest_for(&v2_bytes, "light-client", 2, none);
    let v2 = c.install(&v2_bytes, Some(&v2_manifest)).unwrap();
    let listed = c.list_programs().unwrap();
    assert_eq!(listed.len(), 1);
    assert_eq!(
        (listed[0].id, listed[0].name.as_str(), listed[0].version),
        (v2, "light-client", 2)
    );
    // What the Android service can do with it.
    assert!(listed[0].can_run && !listed[0].can_sign);
    assert_eq!(lc::status(&mut c, &v2).unwrap().unwrap().next_interval, 1);
    assert_eq!(verify(&mut c, &v2, |_| {}), Ok(()));

    // Unloading the last version deletes the oracle.
    c.unload(&v2).unwrap();
    drop(c);
    assert!(ta.stored("svw1_state_").is_empty());
}

#[test]
fn run_only_and_sign_only_programs_stay_in_their_lane() {
    let mut ta = Ta::new();
    let address = ta.seed(seed_vault::KeyAlgorithm::Falcon1024, 3);
    let mut c = client(&mut ta);
    let light_client = c.install(GUEST, None).unwrap();
    assert_eq!(
        c.public_key(&light_client, &address, 1),
        Err(ClientError::Applet(ErrorCode::ProgramFailed))
    );
    let signer = c
        .install(include_bytes!("fixtures/guest_ed25519.wasm"), None)
        .unwrap();
    let listed = c.list_programs().unwrap();
    let kinds = |id| {
        let p = listed.iter().find(|p| p.id == id).unwrap();
        (p.can_sign, p.can_run)
    };
    assert_eq!(kinds(light_client), (false, true));
    assert_eq!(kinds(signer), (true, false));
    assert_eq!(
        lc::status(&mut c, &signer),
        Err(LightClientCallError::Client(ClientError::Applet(
            ErrorCode::ProgramFailed
        )))
    );
    assert_eq!(
        c.run(&ProgramId([0; 32]), &[4]),
        Err(ClientError::Applet(ErrorCode::UnknownProgram))
    );
}
