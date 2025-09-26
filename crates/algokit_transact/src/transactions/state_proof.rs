//! State proof transaction module for AlgoKit Core.
//!
//! This module provides functionality for creating and managing state proof transactions,
//! which are used to submit Algorand state proofs on-chain.

use crate::Transaction;
use crate::transactions::common::TransactionHeader;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::{Bytes, serde_as};
use std::collections::BTreeMap;

fn vec_is_empty<T>(value: &Vec<T>) -> bool {
    value.is_empty()
}

fn map_is_empty<K, V>(value: &BTreeMap<K, V>) -> bool {
    value.is_empty()
}

/// Represents the hash factory used within a Merkle array proof.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Builder)]
#[builder(name = "HashFactoryBuilder")]
pub struct HashFactory {
    /// Hash type.
    #[serde(rename = "t")]
    pub hash_type: u64,
}

/// Represents a Merkle array proof used in state proofs.
#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct MerkleArrayProof {
    /// Merkle proof path.
    #[serde(rename = "pth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "vec_is_empty")]
    #[serde_as(as = "Vec<Bytes>")]
    pub path: Vec<Vec<u8>>,

    /// Hash factory.
    #[serde(rename = "hsh")]
    pub hash_factory: HashFactory,

    /// Tree depth for the proof.
    #[serde(rename = "td")]
    pub tree_depth: u64,
}

/// Represents a Merkle signature verifier used for participants.
#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct MerkleSignatureVerifier {
    /// Commitment used in the verifier.
    #[serde(rename = "cmt")]
    #[serde_as(as = "Bytes")]
    pub commitment: [u8; 64],

    /// Key lifetime.
    #[serde(rename = "lf")]
    pub key_lifetime: u64,
}

/// Represents a participant in the state proof.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Participant {
    /// Merkle signature verifier information.
    #[serde(rename = "p")]
    pub verifier: MerkleSignatureVerifier,

    /// Participant weight in microalgos.
    #[serde(rename = "w")]
    pub weight: u64,
}

/// Represents a Falcon verifier containing a public key.
#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct FalconVerifier {
    /// Falcon public key (expected to be 0x701 bytes).
    #[serde(rename = "k")]
    #[serde_as(as = "Bytes")]
    pub public_key: Vec<u8>,
}

/// Represents a Falcon signature structure within the state proof.
#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct FalconSignatureStruct {
    /// Falcon signature bytes.
    #[serde(rename = "sig")]
    #[serde_as(as = "Bytes")]
    pub signature: Vec<u8>,

    /// Index within the vector commitment.
    #[serde(rename = "idx")]
    pub vector_commitment_index: u64,

    /// Merkle proof associated with the signature.
    #[serde(rename = "prf")]
    pub proof: MerkleArrayProof,

    /// Falcon verifying key.
    #[serde(rename = "vkey")]
    pub verifying_key: FalconVerifier,
}

/// Represents a signature slot commitment in the state proof.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct SigslotCommit {
    /// Signature slot information.
    #[serde(rename = "s")]
    pub sig: FalconSignatureStruct,

    /// Total weight of signatures in lower-numbered slots.
    #[serde(rename = "l")]
    pub lower_sig_weight: u64,
}

/// Represents a reveal entry in the state proof.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Reveal {
    /// Signature slot commitment.
    #[serde(rename = "s")]
    pub sigslot: SigslotCommit,

    /// Participant information.
    #[serde(rename = "p")]
    pub participant: Participant,
}

/// Represents the core state proof payload included in a transaction.
#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct StateProof {
    /// Signature commitment.
    #[serde(rename = "c")]
    #[serde_as(as = "Bytes")]
    pub sig_commit: Vec<u8>,

    /// Signed weight.
    #[serde(rename = "w")]
    pub signed_weight: u64,

    /// Signature Merkle proofs.
    #[serde(rename = "S")]
    pub sig_proofs: MerkleArrayProof,

    /// Participant Merkle proofs.
    #[serde(rename = "P")]
    pub part_proofs: MerkleArrayProof,

    /// Merkle signature salt version.
    #[serde(rename = "v")]
    pub merkle_signature_salt_version: u64,

    /// Revealed positions mapping.
    #[serde(rename = "r")]
    #[serde(default)]
    #[serde(skip_serializing_if = "map_is_empty")]
    pub reveals: BTreeMap<u64, Reveal>,

    /// Positions to reveal.
    #[serde(rename = "pr")]
    #[serde(default)]
    #[serde(skip_serializing_if = "vec_is_empty")]
    pub positions_to_reveal: Vec<u64>,
}

/// Represents the state proof message included in the transaction.
#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct StateProofMessage {
    /// Block headers commitment.
    #[serde(rename = "b")]
    #[serde_as(as = "Bytes")]
    pub block_headers_commitment: Vec<u8>,

    /// Voters commitment.
    #[serde(rename = "v")]
    #[serde_as(as = "Bytes")]
    pub voters_commitment: Vec<u8>,

    /// Natural logarithm of the proven weight.
    #[serde(rename = "P")]
    pub ln_proven_weight: u64,

    /// First attested round.
    #[serde(rename = "f")]
    pub first_attested_round: u64,

    /// Last attested round.
    #[serde(rename = "l")]
    pub last_attested_round: u64,
}

/// Represents the state proof transaction fields.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Builder)]
#[builder(
    name = "StateProofTransactionBuilder",
    setter(strip_option),
    build_fn(name = "build_fields")
)]
pub struct StateProofTransactionFields {
    /// Common transaction header fields.
    #[serde(flatten)]
    pub header: TransactionHeader,

    /// Type of the state proof.
    #[serde(rename = "sptype")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[builder(default)]
    pub state_proof_type: Option<u64>,

    /// State proof payload.
    #[serde(rename = "sp")]
    pub state_proof: StateProof,

    /// State proof message.
    #[serde(rename = "msg")]
    pub message: StateProofMessage,
}

impl StateProofTransactionBuilder {
    /// Builds the transaction fields and wraps them in a [`Transaction::StateProof`].
    pub fn build(&self) -> Result<Transaction, StateProofTransactionBuilderError> {
        self.build_fields().map(Transaction::StateProof)
    }
}
