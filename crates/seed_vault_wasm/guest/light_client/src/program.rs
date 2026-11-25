//! The light client's commands, as `run` input, and its responses.
//!
//! Each run takes one command and answers `[status: u8][payload]`: status 0
//! is success, otherwise an [`Error`] code. Integers are little-endian.
//!
//! | Command            | Input after the opcode | OK payload |
//! |--------------------|------------------------|------------|
//! | `0x01` INIT        | `[first_attested_round: u64][interval_size: u64][capacity: u64][ln_proven_weight: u64][voters_len: u8][voters_commitment]` | (none) |
//! | `0x02` ADVANCE     | `[bhc_len: u8][block_headers_commitment][voters_len: u8][voters_commitment][ln_proven_weight: u64][first_attested_round: u64][last_attested_round: u64][msgpack state proof...]` | `[next_interval: u64]` |
//! | `0x03` VERIFY_TXN  | `[txid: 32][round: u64][genesis_hash: 32][seed: 32][hash_type_len: u8][hash_type][stibhash: 32][txn_index: u64][txn_depth: u64][txn_proof_len: u16][txn_proof][header_index: u64][header_depth: u64][header_proof_len: u16][header_proof]` | (none: verified) |
//! | `0x04` STATUS      | (none) | `[first_attested_round: u64][interval_size: u64][capacity: u64][earliest_interval: u64][next_interval: u64][ln_proven_weight: u64][voters_len: u8][voters_commitment]` |
//!
//! INIT only works once: the genesis data is the root of trust, so letting
//! anyone re-initialize would let them install fake voters. To start over,
//! unload the program (which deletes its state).

use alloc::vec;
use alloc::vec::Vec;

use crate::oracle::{Message, Oracle};
use crate::stateproof::StateProof;
use crate::sumhash::Matrix;
use crate::txverify::{verify_transaction, LightBlockHeaderProof, TransactionProof};
use crate::Error;

pub const INIT: u8 = 0x01;
pub const ADVANCE: u8 = 0x02;
pub const VERIFY_TXN: u8 = 0x03;
pub const STATUS: u8 = 0x04;

/// Result of one run: the response, and the new state to save (if it
/// changed).
pub struct Outcome {
    pub response: Vec<u8>,
    pub new_state: Option<Vec<u8>>,
}

/// Runs one command against the saved `state` (if any).
pub fn run(state: Option<&[u8]>, input: &[u8]) -> Outcome {
    match execute(state, input) {
        Ok((payload, new_state)) => {
            let mut response = vec![0u8];
            response.extend_from_slice(&payload);
            Outcome {
                response,
                new_state,
            }
        }
        Err(error) => Outcome {
            response: vec![error as u8],
            new_state: None,
        },
    }
}

fn execute(state: Option<&[u8]>, input: &[u8]) -> Result<(Vec<u8>, Option<Vec<u8>>), Error> {
    let mut r = Reader(input);
    let opcode = r.u8()?;
    let oracle = state.map(Oracle::decode).transpose()?;
    match opcode {
        INIT => {
            if oracle.is_some() {
                return Err(Error::AlreadyInitialized);
            }
            let first = r.u64()?;
            let interval = r.u64()?;
            let capacity = r.u64()?;
            let ln_proven_weight = r.u64()?;
            let voters = r.bytes_u8()?.to_vec();
            r.end()?;
            let oracle = Oracle::new(first, interval, voters, ln_proven_weight, capacity)?;
            Ok((Vec::new(), Some(oracle.encode())))
        }
        ADVANCE => {
            let mut oracle = oracle.ok_or(Error::NotInitialized)?;
            let message = Message {
                block_headers_commitment: r.bytes_u8()?.to_vec(),
                voters_commitment: r.bytes_u8()?.to_vec(),
                ln_proven_weight: r.u64()?,
                first_attested_round: r.u64()?,
                last_attested_round: r.u64()?,
            };
            let proof = StateProof::decode(r.rest())?;
            oracle.advance(&Matrix::new(), &proof, &message)?;
            Ok((
                oracle.next_interval.to_le_bytes().to_vec(),
                Some(oracle.encode()),
            ))
        }
        VERIFY_TXN => {
            let oracle = oracle.ok_or(Error::NotInitialized)?;
            let txid = r.array32()?;
            let round = r.u64()?;
            let genesis_hash = r.array32()?;
            let seed = r.array32()?;
            let transaction_proof = TransactionProof {
                hash_type: r.bytes_u8()?.to_vec(),
                stib_hash: r.array32()?,
                index: r.u64()?,
                tree_depth: r.u64()?,
                proof: r.bytes_u16()?.to_vec(),
            };
            let header_proof = LightBlockHeaderProof {
                index: r.u64()?,
                tree_depth: r.u64()?,
                proof: r.bytes_u16()?.to_vec(),
            };
            r.end()?;
            let commitment = oracle.commitment_for_round(round)?;
            verify_transaction(
                &txid,
                &transaction_proof,
                &header_proof,
                round,
                &genesis_hash,
                &seed,
                &commitment,
            )?;
            Ok((Vec::new(), None))
        }
        STATUS => {
            let oracle = oracle.ok_or(Error::NotInitialized)?;
            r.end()?;
            let mut out = Vec::new();
            for v in [
                oracle.first_attested_round,
                oracle.interval_size,
                oracle.capacity,
                oracle.earliest_interval,
                oracle.next_interval,
                oracle.ln_proven_weight,
            ] {
                out.extend_from_slice(&v.to_le_bytes());
            }
            out.push(oracle.voters_commitment.len() as u8);
            out.extend_from_slice(&oracle.voters_commitment);
            Ok((out, None))
        }
        _ => Err(Error::MalformedCommand),
    }
}

/// A bounds-checked little-endian reader.
pub(crate) struct Reader<'a>(pub(crate) &'a [u8]);

impl<'a> Reader<'a> {
    pub(crate) fn take(&mut self, n: usize) -> Result<&'a [u8], Error> {
        if self.0.len() < n {
            return Err(Error::MalformedCommand);
        }
        let (head, tail) = self.0.split_at(n);
        self.0 = tail;
        Ok(head)
    }
    pub(crate) fn u8(&mut self) -> Result<u8, Error> {
        Ok(self.take(1)?[0])
    }
    pub(crate) fn u64(&mut self) -> Result<u64, Error> {
        Ok(u64::from_le_bytes(self.take(8)?.try_into().unwrap()))
    }
    fn array32(&mut self) -> Result<[u8; 32], Error> {
        Ok(self.take(32)?.try_into().unwrap())
    }
    fn bytes_u8(&mut self) -> Result<&'a [u8], Error> {
        let n = usize::from(self.u8()?);
        self.take(n)
    }
    fn bytes_u16(&mut self) -> Result<&'a [u8], Error> {
        let n = usize::from(u16::from_le_bytes(self.take(2)?.try_into().unwrap()));
        self.take(n)
    }
    pub(crate) fn rest(&mut self) -> &'a [u8] {
        core::mem::take(&mut self.0)
    }
    fn end(&self) -> Result<(), Error> {
        if self.0.is_empty() {
            Ok(())
        } else {
            Err(Error::MalformedCommand)
        }
    }
}
