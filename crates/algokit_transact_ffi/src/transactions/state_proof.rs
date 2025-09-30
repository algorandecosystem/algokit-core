use crate::*;

/// Represents the hash factory used within a Merkle array proof.
#[ffi_record]
pub struct HashFactory {
    /// Hash type.
    hash_type: u64,
}

impl From<algokit_transact::HashFactory> for HashFactory {
    fn from(hf: algokit_transact::HashFactory) -> Self {
        Self {
            hash_type: hf.hash_type,
        }
    }
}

/// Represents a Merkle array proof used in state proofs.
#[ffi_record]
pub struct MerkleArrayProof {
    /// Merkle proof path.
    path: Vec<Vec<u8>>,

    /// Hash factory.
    hash_factory: HashFactory,

    /// Tree depth for the proof.
    tree_depth: u64,
}

impl From<algokit_transact::MerkleArrayProof> for MerkleArrayProof {
    fn from(proof: algokit_transact::MerkleArrayProof) -> Self {
        Self {
            path: proof.path,
            hash_factory: proof.hash_factory.into(),
            tree_depth: proof.tree_depth,
        }
    }
}

/// Represents a Merkle signature verifier used for participants.
#[ffi_record]
pub struct MerkleSignatureVerifier {
    /// Commitment used in the verifier (64 bytes).
    commitment: Vec<u8>,

    /// Key lifetime.
    key_lifetime: u64,
}

impl From<algokit_transact::MerkleSignatureVerifier> for MerkleSignatureVerifier {
    fn from(verifier: algokit_transact::MerkleSignatureVerifier) -> Self {
        Self {
            commitment: verifier.commitment.to_vec(),
            key_lifetime: verifier.key_lifetime,
        }
    }
}

/// Represents a participant in the state proof.
#[ffi_record]
pub struct Participant {
    /// Merkle signature verifier information.
    verifier: MerkleSignatureVerifier,

    /// Participant weight in microalgos.
    weight: u64,
}

impl From<algokit_transact::Participant> for Participant {
    fn from(participant: algokit_transact::Participant) -> Self {
        Self {
            verifier: participant.verifier.into(),
            weight: participant.weight,
        }
    }
}

/// Represents a Falcon verifier containing a public key.
#[ffi_record]
pub struct FalconVerifier {
    /// Falcon public key.
    public_key: Vec<u8>,
}

impl From<algokit_transact::FalconVerifier> for FalconVerifier {
    fn from(verifier: algokit_transact::FalconVerifier) -> Self {
        Self {
            public_key: verifier.public_key,
        }
    }
}

/// Represents a Falcon signature structure within the state proof.
#[ffi_record]
pub struct FalconSignatureStruct {
    /// Falcon signature bytes.
    signature: Vec<u8>,

    /// Index within the vector commitment.
    vector_commitment_index: u64,

    /// Merkle proof associated with the signature.
    proof: MerkleArrayProof,

    /// Falcon verifying key.
    verifying_key: FalconVerifier,
}

impl From<algokit_transact::FalconSignatureStruct> for FalconSignatureStruct {
    fn from(sig: algokit_transact::FalconSignatureStruct) -> Self {
        Self {
            signature: sig.signature,
            vector_commitment_index: sig.vector_commitment_index,
            proof: sig.proof.into(),
            verifying_key: sig.verifying_key.into(),
        }
    }
}

/// Represents a signature slot commitment in the state proof.
#[ffi_record]
pub struct SigslotCommit {
    /// Signature slot information.
    sig: FalconSignatureStruct,

    /// Total weight of signatures in lower-numbered slots.
    lower_sig_weight: u64,
}

impl From<algokit_transact::SigslotCommit> for SigslotCommit {
    fn from(commit: algokit_transact::SigslotCommit) -> Self {
        Self {
            sig: commit.sig.into(),
            lower_sig_weight: commit.lower_sig_weight,
        }
    }
}

/// Represents a reveal entry in the state proof.
#[ffi_record]
pub struct Reveal {
    /// Signature slot commitment.
    sigslot: SigslotCommit,

    /// Participant information.
    participant: Participant,
}

impl From<algokit_transact::Reveal> for Reveal {
    fn from(reveal: algokit_transact::Reveal) -> Self {
        Self {
            sigslot: reveal.sigslot.into(),
            participant: reveal.participant.into(),
        }
    }
}

/// Represents the core state proof payload included in a transaction.
#[ffi_record]
pub struct StateProof {
    /// Signature commitment.
    sig_commit: Vec<u8>,

    /// Signed weight.
    signed_weight: u64,

    /// Signature Merkle proofs.
    sig_proofs: MerkleArrayProof,

    /// Participant Merkle proofs.
    part_proofs: MerkleArrayProof,

    /// Merkle signature salt version.
    merkle_signature_salt_version: u64,

    /// Revealed positions mapping (position -> Reveal).
    reveals: Vec<RevealEntry>,

    /// Positions to reveal.
    positions_to_reveal: Vec<u64>,
}

/// Helper struct for reveals map entries since FFI doesn't support maps directly.
#[ffi_record]
pub struct RevealEntry {
    /// Position key.
    position: u64,

    /// Reveal value.
    reveal: Reveal,
}

impl From<algokit_transact::StateProof> for StateProof {
    fn from(proof: algokit_transact::StateProof) -> Self {
        let reveals = proof
            .reveals
            .into_iter()
            .map(|(position, reveal)| RevealEntry {
                position,
                reveal: reveal.into(),
            })
            .collect();

        Self {
            sig_commit: proof.sig_commit,
            signed_weight: proof.signed_weight,
            sig_proofs: proof.sig_proofs.into(),
            part_proofs: proof.part_proofs.into(),
            merkle_signature_salt_version: proof.merkle_signature_salt_version,
            reveals,
            positions_to_reveal: proof.positions_to_reveal,
        }
    }
}

/// Represents the state proof message included in the transaction.
#[ffi_record]
pub struct StateProofMessage {
    /// Block headers commitment.
    block_headers_commitment: Vec<u8>,

    /// Voters commitment.
    voters_commitment: Vec<u8>,

    /// Natural logarithm of the proven weight.
    ln_proven_weight: u64,

    /// First attested round.
    first_attested_round: u64,

    /// Last attested round.
    last_attested_round: u64,
}

impl From<algokit_transact::StateProofMessage> for StateProofMessage {
    fn from(msg: algokit_transact::StateProofMessage) -> Self {
        Self {
            block_headers_commitment: msg.block_headers_commitment,
            voters_commitment: msg.voters_commitment,
            ln_proven_weight: msg.ln_proven_weight,
            first_attested_round: msg.first_attested_round,
            last_attested_round: msg.last_attested_round,
        }
    }
}

/// Parameters to define a state proof transaction.
///
/// Used to submit Algorand state proofs on-chain.
#[ffi_record]
pub struct StateProofTransactionFields {
    /// Type of the state proof.
    state_proof_type: Option<u64>,

    /// State proof payload.
    state_proof: Option<StateProof>,

    /// State proof message.
    message: Option<StateProofMessage>,
}

impl From<algokit_transact::StateProofTransactionFields> for StateProofTransactionFields {
    fn from(tx: algokit_transact::StateProofTransactionFields) -> Self {
        Self {
            state_proof_type: tx.state_proof_type,
            state_proof: tx.state_proof.map(|sp| sp.into()),
            message: tx.message.map(|msg| msg.into()),
        }
    }
}

impl TryFrom<Transaction> for algokit_transact::StateProofTransactionFields {
    type Error = AlgoKitTransactError;

    fn try_from(tx: Transaction) -> Result<Self, Self::Error> {
        if tx.transaction_type != TransactionType::StateProof || tx.state_proof.is_none() {
            return Err(Self::Error::DecodingError {
                message: "State proof transaction data missing".to_string(),
            });
        }

        let data = tx.clone().state_proof.unwrap();
        let header: algokit_transact::TransactionHeader = tx.try_into()?;

        // Convert state proof if present
        let state_proof = data
            .state_proof
            .map(|sp| {
                let reveals = sp
                    .reveals
                    .into_iter()
                    .map(|entry| {
                        let reveal = algokit_transact::Reveal {
                            sigslot: algokit_transact::SigslotCommit {
                                sig: algokit_transact::FalconSignatureStruct {
                                    signature: entry.reveal.sigslot.sig.signature,
                                    vector_commitment_index: entry
                                        .reveal
                                        .sigslot
                                        .sig
                                        .vector_commitment_index,
                                    proof: algokit_transact::MerkleArrayProof {
                                        path: entry.reveal.sigslot.sig.proof.path,
                                        hash_factory: algokit_transact::HashFactory {
                                            hash_type: entry
                                                .reveal
                                                .sigslot
                                                .sig
                                                .proof
                                                .hash_factory
                                                .hash_type,
                                        },
                                        tree_depth: entry.reveal.sigslot.sig.proof.tree_depth,
                                    },
                                    verifying_key: algokit_transact::FalconVerifier {
                                        public_key: entry
                                            .reveal
                                            .sigslot
                                            .sig
                                            .verifying_key
                                            .public_key,
                                    },
                                },
                                lower_sig_weight: entry.reveal.sigslot.lower_sig_weight,
                            },
                            participant: algokit_transact::Participant {
                                verifier: algokit_transact::MerkleSignatureVerifier {
                                    commitment: vec_to_array::<64>(
                                        &entry.reveal.participant.verifier.commitment,
                                        "participant verifier commitment",
                                    )?,
                                    key_lifetime: entry.reveal.participant.verifier.key_lifetime,
                                },
                                weight: entry.reveal.participant.weight,
                            },
                        };
                        Ok((entry.position, reveal))
                    })
                    .collect::<Result<std::collections::BTreeMap<_, _>, Self::Error>>()?;

                Ok::<_, Self::Error>(algokit_transact::StateProof {
                    sig_commit: sp.sig_commit,
                    signed_weight: sp.signed_weight,
                    sig_proofs: algokit_transact::MerkleArrayProof {
                        path: sp.sig_proofs.path,
                        hash_factory: algokit_transact::HashFactory {
                            hash_type: sp.sig_proofs.hash_factory.hash_type,
                        },
                        tree_depth: sp.sig_proofs.tree_depth,
                    },
                    part_proofs: algokit_transact::MerkleArrayProof {
                        path: sp.part_proofs.path,
                        hash_factory: algokit_transact::HashFactory {
                            hash_type: sp.part_proofs.hash_factory.hash_type,
                        },
                        tree_depth: sp.part_proofs.tree_depth,
                    },
                    merkle_signature_salt_version: sp.merkle_signature_salt_version,
                    reveals,
                    positions_to_reveal: sp.positions_to_reveal,
                })
            })
            .transpose()?;

        // Convert message if present
        let message = data.message.map(|msg| algokit_transact::StateProofMessage {
            block_headers_commitment: msg.block_headers_commitment,
            voters_commitment: msg.voters_commitment,
            ln_proven_weight: msg.ln_proven_weight,
            first_attested_round: msg.first_attested_round,
            last_attested_round: msg.last_attested_round,
        });

        let transaction_fields = algokit_transact::StateProofTransactionFields {
            header,
            state_proof_type: data.state_proof_type,
            state_proof,
            message,
        };

        Ok(transaction_fields)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use algokit_transact::test_utils::TestDataMother;

    #[test]
    fn test_encode_transaction_validation_integration() {
        // Test valid state proof transaction
        let result = encode_transaction(TestDataMother::state_proof().transaction.into());
        assert!(result.is_ok());
    }
}
