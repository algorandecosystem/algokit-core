//! Client helpers for the light client program (`guest/light_client`, a
//! Rust port of algorand/light-client-poc): its commands, encoded for
//! [`Client::run`], and its responses decoded.
//!
//! The command format is defined by the guest (`src/program.rs` there);
//! `tests/guest_light_client.rs` checks these helpers against the real
//! program.

use alloc::string::String;
use alloc::vec::Vec;

use wasm_runtime::ProgramId;

use crate::client::{Client, ClientError};

const INIT: u8 = 0x01;
const ADVANCE: u8 = 0x02;
const VERIFY_TXN: u8 = 0x03;
const STATUS: u8 = 0x04;

/// An error the light client program reported (its status byte).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LightClientError {
    MalformedCommand,
    NotInitialized,
    AlreadyInitialized,
    InvalidArgument,
    CorruptState,
    MalformedStateProof,
    UnknownHash,
    TreeTooDeep,
    PositionOutOfBounds,
    NonEmptyProofForEmptySet,
    MissingSibling,
    MalformedDigest,
    RootMismatch,
    BadSignature,
    ZeroKeyLifetime,
    SaltVersionMismatch,
    TooManyReveals,
    ZeroSignedWeight,
    InsufficientSignedWeight,
    NoRevealInPosition,
    CoinNotInRange,
    ProofLengthMismatch,
    InvalidTreeDepth,
    IndexDepthMismatch,
    RoundTooEarly,
    NoStateProofForRound,
    /// A status this client doesn't know (a newer program).
    Unknown(u8),
}

impl LightClientError {
    pub fn from_status(status: u8) -> Self {
        use LightClientError::*;
        match status {
            1 => MalformedCommand,
            2 => NotInitialized,
            3 => AlreadyInitialized,
            4 => InvalidArgument,
            5 => CorruptState,
            6 => MalformedStateProof,
            10 => UnknownHash,
            11 => TreeTooDeep,
            12 => PositionOutOfBounds,
            13 => NonEmptyProofForEmptySet,
            14 => MissingSibling,
            15 => MalformedDigest,
            16 => RootMismatch,
            17 => BadSignature,
            18 => ZeroKeyLifetime,
            19 => SaltVersionMismatch,
            20 => TooManyReveals,
            21 => ZeroSignedWeight,
            22 => InsufficientSignedWeight,
            23 => NoRevealInPosition,
            24 => CoinNotInRange,
            30 => ProofLengthMismatch,
            31 => InvalidTreeDepth,
            32 => IndexDepthMismatch,
            33 => RoundTooEarly,
            34 => NoStateProofForRound,
            other => Unknown(other),
        }
    }
}

/// Why a light client call failed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LightClientCallError<E> {
    /// Reaching the program failed.
    Client(ClientError<E>),
    /// The program ran and refused (e.g. a state proof didn't verify).
    LightClient(LightClientError),
}

impl<E> From<ClientError<E>> for LightClientCallError<E> {
    fn from(error: ClientError<E>) -> Self {
        LightClientCallError::Client(error)
    }
}

/// A state proof message, as algod returns it with a state proof.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateProofMessage {
    pub block_headers_commitment: Vec<u8>,
    pub voters_commitment: Vec<u8>,
    pub ln_proven_weight: u64,
    pub first_attested_round: u64,
    pub last_attested_round: u64,
}

/// algod's `TransactionProofResponse` (`GET /v2/blocks/{round}/transactions/{txid}/proof`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransactionProof {
    /// Must be `sha256`.
    pub hash_type: String,
    pub stib_hash: [u8; 32],
    pub index: u64,
    pub tree_depth: u64,
    pub proof: Vec<u8>,
}

/// algod's `LightBlockHeaderProof` (`GET /v2/blocks/{round}/lightheader/proof`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LightBlockHeaderProof {
    pub index: u64,
    pub tree_depth: u64,
    pub proof: Vec<u8>,
}

/// The oracle's state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Status {
    pub first_attested_round: u64,
    pub interval_size: u64,
    pub capacity: u64,
    /// Earliest interval still held.
    pub earliest_interval: u64,
    /// Intervals verified so far.
    pub next_interval: u64,
    pub ln_proven_weight: u64,
    pub voters_commitment: Vec<u8>,
}

/// Errors from encoding a command (lengths that don't fit the format).
fn too_long<E>(what: &'static str) -> LightClientCallError<E> {
    LightClientCallError::Client(ClientError::InvalidArgument(what))
}

fn push_short<E>(
    c: &mut Vec<u8>,
    bytes: &[u8],
    what: &'static str,
) -> Result<(), LightClientCallError<E>> {
    let len = u8::try_from(bytes.len()).map_err(|_| too_long(what))?;
    c.push(len);
    c.extend_from_slice(bytes);
    Ok(())
}

fn push_u16_bytes<E>(
    c: &mut Vec<u8>,
    bytes: &[u8],
    what: &'static str,
) -> Result<(), LightClientCallError<E>> {
    let len = u16::try_from(bytes.len()).map_err(|_| too_long(what))?;
    c.extend_from_slice(&len.to_le_bytes());
    c.extend_from_slice(bytes);
    Ok(())
}

/// Runs `command` and returns the payload after a success status.
fn call<F, E>(
    client: &mut Client<F>,
    program: &ProgramId,
    command: &[u8],
) -> Result<Vec<u8>, LightClientCallError<E>>
where
    F: FnMut(&[u8]) -> Result<Vec<u8>, E>,
{
    let output = client.run(program, command)?;
    match output.split_first() {
        Some((0, payload)) => Ok(payload.to_vec()),
        Some((status, _)) => Err(LightClientCallError::LightClient(
            LightClientError::from_status(*status),
        )),
        None => Err(ClientError::MalformedResponse.into()),
    }
}

/// Initializes the oracle from trusted genesis data (once; see the guest's
/// docs). `capacity` is how many intervals of history it keeps.
pub fn init<F, E>(
    client: &mut Client<F>,
    program: &ProgramId,
    first_attested_round: u64,
    interval_size: u64,
    capacity: u64,
    genesis_voters_commitment: &[u8],
    genesis_ln_proven_weight: u64,
) -> Result<(), LightClientCallError<E>>
where
    F: FnMut(&[u8]) -> Result<Vec<u8>, E>,
{
    let mut c = Vec::from([INIT]);
    for v in [
        first_attested_round,
        interval_size,
        capacity,
        genesis_ln_proven_weight,
    ] {
        c.extend_from_slice(&v.to_le_bytes());
    }
    push_short(
        &mut c,
        genesis_voters_commitment,
        "voters commitment too long",
    )?;
    call(client, program, &c).map(drop)
}

/// Verifies `state_proof` (msgpack, as from algod) attests to `message`
/// and advances the oracle. Returns the number of intervals verified.
pub fn advance<F, E>(
    client: &mut Client<F>,
    program: &ProgramId,
    message: &StateProofMessage,
    state_proof: &[u8],
) -> Result<u64, LightClientCallError<E>>
where
    F: FnMut(&[u8]) -> Result<Vec<u8>, E>,
{
    let mut c = Vec::from([ADVANCE]);
    push_short(
        &mut c,
        &message.block_headers_commitment,
        "block headers commitment too long",
    )?;
    push_short(
        &mut c,
        &message.voters_commitment,
        "voters commitment too long",
    )?;
    for v in [
        message.ln_proven_weight,
        message.first_attested_round,
        message.last_attested_round,
    ] {
        c.extend_from_slice(&v.to_le_bytes());
    }
    c.extend_from_slice(state_proof);
    let payload = call(client, program, &c)?;
    let bytes: [u8; 8] = payload
        .as_slice()
        .try_into()
        .map_err(|_| LightClientCallError::Client(ClientError::MalformedResponse))?;
    Ok(u64::from_le_bytes(bytes))
}

/// Checks the transaction `txid`, confirmed in `round`, against the block
/// interval commitment the oracle verified for that round. `Ok` means it
/// happened; a [`LightClientError`] says why not (e.g. `RootMismatch`,
/// `NoStateProofForRound`).
#[allow(clippy::too_many_arguments)]
pub fn verify_transaction<F, E>(
    client: &mut Client<F>,
    program: &ProgramId,
    txid: &[u8; 32],
    round: u64,
    genesis_hash: &[u8; 32],
    seed: &[u8; 32],
    transaction_proof: &TransactionProof,
    light_block_header_proof: &LightBlockHeaderProof,
) -> Result<(), LightClientCallError<E>>
where
    F: FnMut(&[u8]) -> Result<Vec<u8>, E>,
{
    let mut c = Vec::from([VERIFY_TXN]);
    c.extend_from_slice(txid);
    c.extend_from_slice(&round.to_le_bytes());
    c.extend_from_slice(genesis_hash);
    c.extend_from_slice(seed);
    push_short(
        &mut c,
        transaction_proof.hash_type.as_bytes(),
        "hash type too long",
    )?;
    c.extend_from_slice(&transaction_proof.stib_hash);
    c.extend_from_slice(&transaction_proof.index.to_le_bytes());
    c.extend_from_slice(&transaction_proof.tree_depth.to_le_bytes());
    push_u16_bytes(
        &mut c,
        &transaction_proof.proof,
        "transaction proof too long",
    )?;
    c.extend_from_slice(&light_block_header_proof.index.to_le_bytes());
    c.extend_from_slice(&light_block_header_proof.tree_depth.to_le_bytes());
    push_u16_bytes(
        &mut c,
        &light_block_header_proof.proof,
        "header proof too long",
    )?;
    call(client, program, &c).map(drop)
}

/// The oracle's state, or `None` before [`init`].
pub fn status<F, E>(
    client: &mut Client<F>,
    program: &ProgramId,
) -> Result<Option<Status>, LightClientCallError<E>>
where
    F: FnMut(&[u8]) -> Result<Vec<u8>, E>,
{
    let payload = match call(client, program, &[STATUS]) {
        Ok(payload) => payload,
        Err(LightClientCallError::LightClient(LightClientError::NotInitialized)) => {
            return Ok(None)
        }
        Err(e) => return Err(e),
    };
    let malformed = || LightClientCallError::Client(ClientError::MalformedResponse);
    if payload.len() < 49 {
        return Err(malformed());
    }
    let u = |i: usize| u64::from_le_bytes(payload[8 * i..8 * i + 8].try_into().unwrap());
    let voters_len = usize::from(payload[48]);
    let voters_commitment = payload
        .get(49..49 + voters_len)
        .ok_or_else(malformed)?
        .to_vec();
    Ok(Some(Status {
        first_attested_round: u(0),
        interval_size: u(1),
        capacity: u(2),
        earliest_interval: u(3),
        next_interval: u(4),
        ln_proven_weight: u(5),
        voters_commitment,
    }))
}
