//! The light client's oracle (light-client-poc `oracle/oracle.go` and
//! `oracle/commitmentHistory.go`): ingests state proofs in order, keeping a
//! sliding window of verified block interval commitments.

use alloc::vec::Vec;

use crate::hashes::sha256;
use crate::msgpack::{encode_struct, Field};
use crate::stateproof::{StateProof, Verifier};
use crate::sumhash::Matrix;
use crate::Error;

/// A state proof message (`types.Message` in the Go SDK).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub block_headers_commitment: Vec<u8>,
    pub voters_commitment: Vec<u8>,
    pub ln_proven_weight: u64,
    pub first_attested_round: u64,
    pub last_attested_round: u64,
}

impl Message {
    /// `crypto.HashStateProofMessage`: `SHA-256("spm" || msgpack(message))`.
    pub fn hash(&self) -> [u8; 32] {
        let encoded = encode_struct(&[
            ("P", Field::Uint(self.ln_proven_weight)),
            ("b", Field::Bin(&self.block_headers_commitment)),
            ("f", Field::Uint(self.first_attested_round)),
            ("l", Field::Uint(self.last_attested_round)),
            ("v", Field::Bin(&self.voters_commitment)),
        ]);
        let mut data = Vec::from(&b"spm"[..]);
        data.extend_from_slice(&encoded);
        sha256(&data)
    }
}

/// The oracle's whole state: what persists between runs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Oracle {
    /// `VotersCommitment`: who signs the next state proof.
    pub voters_commitment: Vec<u8>,
    /// `LnProvenWeight`, for verifying the next state proof.
    pub ln_proven_weight: u64,
    /// `CommitmentHistory`.
    pub first_attested_round: u64,
    pub interval_size: u64,
    pub capacity: u64,
    pub earliest_interval: u64,
    pub next_interval: u64,
    /// Commitments for intervals `earliest_interval..next_interval`, in order.
    pub commitments: Vec<[u8; 32]>,
}

impl Oracle {
    /// `InitializeOracle`, from trusted genesis data.
    pub fn new(
        first_attested_round: u64,
        interval_size: u64,
        voters_commitment: Vec<u8>,
        ln_proven_weight: u64,
        capacity: u64,
    ) -> Result<Self, Error> {
        // Go would divide by zero / never hold anything; refuse instead.
        if interval_size == 0 || capacity == 0 {
            return Err(Error::InvalidArgument);
        }
        Ok(Oracle {
            voters_commitment,
            ln_proven_weight,
            first_attested_round,
            interval_size,
            capacity,
            earliest_interval: 0,
            next_interval: 0,
            commitments: Vec::new(),
        })
    }

    /// `AdvanceState`: verifies `proof` attests to `message` (signed by the
    /// voters the oracle currently trusts) and, if so, moves to the next
    /// interval: trusts the message's voters and records its block headers
    /// commitment.
    pub fn advance(
        &mut self,
        matrix: &Matrix,
        proof: &StateProof,
        message: &Message,
    ) -> Result<(), Error> {
        let verifier = Verifier {
            participants_commitment: &self.voters_commitment,
            ln_proven_weight: self.ln_proven_weight,
        };
        verifier.verify(matrix, message.last_attested_round, &message.hash(), proof)?;
        self.voters_commitment = message.voters_commitment.clone();
        self.ln_proven_weight = message.ln_proven_weight;
        let mut commitment = [0u8; 32];
        let n = message.block_headers_commitment.len().min(32);
        commitment[..n].copy_from_slice(&message.block_headers_commitment[..n]);
        self.insert_commitment(commitment);
        Ok(())
    }

    /// `CommitmentHistory.InsertCommitment`.
    fn insert_commitment(&mut self, commitment: [u8; 32]) {
        self.commitments.push(commitment);
        self.next_interval += 1;
        if self.commitments.len() as u64 > self.capacity {
            self.commitments.remove(0);
            self.earliest_interval += 1;
        }
    }

    /// `GetStateProofCommitment` / `CommitmentHistory.GetCommitment`: the
    /// verified block interval commitment covering `round`.
    pub fn commitment_for_round(&self, round: u64) -> Result<[u8; 32], Error> {
        if round < self.first_attested_round {
            return Err(Error::RoundTooEarly);
        }
        // Interval i covers rounds first + i*size ..= first + (i+1)*size - 1
        // (e.g. 9..=16, 17..=24 for first 9, size 8: a state proof message's
        // FirstAttestedRound..=LastAttestedRound).
        //
        // DEPARTS FROM GO: light-client-poc also subtracts 1 when
        // `round % size == 0`, which is wrong here -- it rejects the last
        // round of every interval (round 16 wraps to an invalid interval;
        // round 24 maps to interval 0 and fails to verify). It fails safe,
        // but genuine transactions in those rounds couldn't be verified.
        let interval = (round - self.first_attested_round) / self.interval_size;
        if interval >= self.next_interval || interval < self.earliest_interval {
            return Err(Error::NoStateProofForRound);
        }
        Ok(self.commitments[(interval - self.earliest_interval) as usize])
    }

    /// Serialized state: `[version: 1][first_attested_round: u64]
    /// [interval_size: u64][capacity: u64][earliest_interval: u64]
    /// [next_interval: u64][ln_proven_weight: u64][voters_len: u8][voters]
    /// [commitment: 32]*`, integers little-endian.
    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.push(STATE_VERSION);
        for v in [
            self.first_attested_round,
            self.interval_size,
            self.capacity,
            self.earliest_interval,
            self.next_interval,
            self.ln_proven_weight,
        ] {
            out.extend_from_slice(&v.to_le_bytes());
        }
        out.push(self.voters_commitment.len() as u8);
        out.extend_from_slice(&self.voters_commitment);
        for c in &self.commitments {
            out.extend_from_slice(c);
        }
        out
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, Error> {
        let mut r = crate::program::Reader(bytes);
        let corrupt = |_| Error::CorruptState;
        if r.u8().map_err(corrupt)? != STATE_VERSION {
            return Err(Error::CorruptState);
        }
        let mut fields = [0u64; 6];
        for f in &mut fields {
            *f = r.u64().map_err(corrupt)?;
        }
        let [first_attested_round, interval_size, capacity, earliest_interval, next_interval, ln_proven_weight] =
            fields;
        let voters_len = usize::from(r.u8().map_err(corrupt)?);
        let voters_commitment = r.take(voters_len).map_err(corrupt)?.to_vec();
        let rest = r.rest();
        if rest.len() % 32 != 0
            || interval_size == 0
            || next_interval.checked_sub(earliest_interval) != Some((rest.len() / 32) as u64)
        {
            return Err(Error::CorruptState);
        }
        Ok(Oracle {
            voters_commitment,
            ln_proven_weight,
            first_attested_round,
            interval_size,
            capacity,
            earliest_interval,
            next_interval,
            commitments: rest.chunks(32).map(|c| c.try_into().unwrap()).collect(),
        })
    }
}

const STATE_VERSION: u8 = 1;
