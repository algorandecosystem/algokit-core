//! The port, natively, against light-client-poc's sample data
//! (`../../tests/fixtures/light_client`): the same steps as its `main.go`.

use base64::Engine;
use seed_vault_wasm_guest_light_client::oracle::{Message, Oracle};
use seed_vault_wasm_guest_light_client::stateproof::StateProof;
use seed_vault_wasm_guest_light_client::sumhash::Matrix;
use seed_vault_wasm_guest_light_client::txverify::{
    verify_transaction, LightBlockHeaderProof, TransactionProof,
};
use seed_vault_wasm_guest_light_client::Error;
use serde_json::Value;

const DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/light_client/"
);

fn json(name: &str) -> Value {
    serde_json::from_str(&std::fs::read_to_string(format!("{DIR}{name}")).unwrap()).unwrap()
}

fn b64(v: &Value) -> Vec<u8> {
    base64::engine::general_purpose::STANDARD
        .decode(v.as_str().unwrap())
        .unwrap()
}

fn bytes32(v: &Value) -> [u8; 32] {
    let bytes: Vec<u8> = v
        .as_array()
        .unwrap()
        .iter()
        .map(|b| b.as_u64().unwrap() as u8)
        .collect();
    bytes.try_into().unwrap()
}

struct Sample {
    oracle: Oracle,
    message: Message,
    proof_bytes: Vec<u8>,
    txid: [u8; 32],
    round: u64,
    genesis_hash: [u8; 32],
    seed: [u8; 32],
    txn_proof: TransactionProof,
    header_proof: LightBlockHeaderProof,
}

fn sample() -> Sample {
    let oracle = Oracle::new(
        9,
        8,
        b64(&json("genesis_voters_commitment.txt")),
        json("genesis_voters_ln_proven_weight.txt")
            .as_u64()
            .unwrap(),
        1000,
    )
    .unwrap();
    let m = json("state_proof_message.json");
    let message = Message {
        block_headers_commitment: b64(&m["BlockHeadersCommitment"]),
        voters_commitment: b64(&m["VotersCommitment"]),
        ln_proven_weight: m["LnProvenWeight"].as_u64().unwrap(),
        first_attested_round: m["FirstAttestedRound"].as_u64().unwrap(),
        last_attested_round: m["LastAttestedRound"].as_u64().unwrap(),
    };
    let t = json("transaction_proof_response.json");
    let h = json("light_block_header_proof_response.json");
    Sample {
        oracle,
        message,
        proof_bytes: b64(&json("state_proof.txt")),
        txid: bytes32(&json("transaction_id.txt")),
        round: json("round.txt").as_u64().unwrap(),
        genesis_hash: bytes32(&json("genesis_hash.txt")),
        seed: bytes32(&json("seed.txt")),
        txn_proof: TransactionProof {
            hash_type: t["hashtype"].as_str().unwrap().as_bytes().to_vec(),
            stib_hash: b64(&t["stibhash"]).try_into().unwrap(),
            index: t["idx"].as_u64().unwrap(),
            proof: b64(&t["proof"]),
            tree_depth: t["treedepth"].as_u64().unwrap(),
        },
        header_proof: LightBlockHeaderProof {
            index: h["index"].as_u64().unwrap(),
            proof: b64(&h["proof"]),
            tree_depth: h["treedepth"].as_u64().unwrap(),
        },
    }
}

impl Sample {
    fn verify(&self, oracle: &Oracle, txid: &[u8; 32]) -> Result<(), Error> {
        let commitment = oracle.commitment_for_round(self.round)?;
        verify_transaction(
            txid,
            &self.txn_proof,
            &self.header_proof,
            self.round,
            &self.genesis_hash,
            &self.seed,
            &commitment,
        )
    }
}

#[test]
fn main_go_end_to_end() {
    let s = sample();
    let matrix = Matrix::new();
    let proof = StateProof::decode(&s.proof_bytes).expect("decode state proof");
    let mut oracle = s.oracle.clone();
    oracle
        .advance(&matrix, &proof, &s.message)
        .expect("state proof verifies");
    assert_eq!(oracle.next_interval, 1);
    assert_eq!(oracle.voters_commitment, s.message.voters_commitment);
    s.verify(&oracle, &s.txid).expect("transaction verifies");

    // The oracle state round-trips.
    assert_eq!(Oracle::decode(&oracle.encode()).unwrap(), oracle);
}

#[test]
fn transactions_need_a_verified_interval_and_a_matching_proof() {
    let s = sample();
    let matrix = Matrix::new();
    let mut oracle = s.oracle.clone();
    // Before any state proof: nothing is trusted yet.
    assert_eq!(s.verify(&oracle, &s.txid), Err(Error::NoStateProofForRound));
    oracle
        .advance(
            &matrix,
            &StateProof::decode(&s.proof_bytes).unwrap(),
            &s.message,
        )
        .unwrap();

    let mut other = s.txid;
    other[0] ^= 1;
    assert_eq!(s.verify(&oracle, &other), Err(Error::RootMismatch));
    assert_eq!(oracle.commitment_for_round(8), Err(Error::RoundTooEarly));
    assert_eq!(
        oracle.commitment_for_round(17),
        Err(Error::NoStateProofForRound)
    );
    // Every round of interval 0 (9..=16), including 16, which the Go PoC
    // wrongly rejects (see `Oracle::commitment_for_round`).
    for round in 9..=16 {
        assert_eq!(
            oracle.commitment_for_round(round),
            Ok(oracle.commitments[0]),
            "{round}"
        );
    }
}

#[test]
fn tampered_state_proofs_and_messages_are_rejected() {
    let s = sample();
    let matrix = Matrix::new();
    let proof = StateProof::decode(&s.proof_bytes).unwrap();

    // A message the proof doesn't attest to (e.g. a forged commitment).
    let mut forged = s.message.clone();
    forged.block_headers_commitment[0] ^= 1;
    let mut oracle = s.oracle.clone();
    assert!(oracle.advance(&matrix, &proof, &forged).is_err());
    assert_eq!(oracle, s.oracle, "a failed advance changes nothing");

    // Trusting different voters.
    let mut wrong_voters = s.oracle.clone();
    wrong_voters.voters_commitment[0] ^= 1;
    assert!(wrong_voters.advance(&matrix, &proof, &s.message).is_err());

    // Too little claimed weight for the reveals.
    let mut weak = proof.clone();
    weak.signed_weight /= 4;
    assert!(s
        .oracle
        .clone()
        .advance(&matrix, &weak, &s.message)
        .is_err());

    // Flipping bytes anywhere in the encoded proof never verifies.
    for i in (0..s.proof_bytes.len()).step_by(997) {
        let mut bytes = s.proof_bytes.clone();
        bytes[i] ^= 0x10;
        if let Ok(p) = StateProof::decode(&bytes) {
            assert!(
                s.oracle.clone().advance(&matrix, &p, &s.message).is_err(),
                "byte {i}"
            );
        }
    }
}
