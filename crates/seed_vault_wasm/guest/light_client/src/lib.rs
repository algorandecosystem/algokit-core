//! An Algorand light client as a `seed_vault_wasm` guest program: a Rust
//! port of https://github.com/algorand/light-client-poc and the state proof
//! verification it builds on (go-stateproof-verification, go-sumhash).
//!
//! Unlike the signing examples, it uses no keys: it runs through the `run`
//! export and keeps its oracle in the program's persistent state, so a TEE
//! can itself decide whether an Algorand transaction happened -- trusting
//! only the genesis data it was initialized with and the state proofs it
//! verified since. See [`program`] for the commands.
#![cfg_attr(target_arch = "wasm32", no_std)]

extern crate alloc;

pub mod hashes;
pub mod merkle;
pub mod msgpack;
pub mod oracle;
pub mod program;
pub mod stateproof;
pub mod sumhash;
pub mod txverify;

#[cfg(target_arch = "wasm32")]
mod wasm;

/// Everything that can go wrong. The discriminant is the status byte
/// [`program`] reports.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Error {
    MalformedCommand = 1,
    NotInitialized = 2,
    AlreadyInitialized = 3,
    InvalidArgument = 4,
    CorruptState = 5,
    MalformedStateProof = 6,
    // State proof verification.
    UnknownHash = 10,
    TreeTooDeep = 11,
    PositionOutOfBounds = 12,
    NonEmptyProofForEmptySet = 13,
    MissingSibling = 14,
    MalformedDigest = 15,
    RootMismatch = 16,
    BadSignature = 17,
    ZeroKeyLifetime = 18,
    SaltVersionMismatch = 19,
    TooManyReveals = 20,
    ZeroSignedWeight = 21,
    InsufficientSignedWeight = 22,
    NoRevealInPosition = 23,
    CoinNotInRange = 24,
    // Transaction verification.
    ProofLengthMismatch = 30,
    InvalidTreeDepth = 31,
    IndexDepthMismatch = 32,
    RoundTooEarly = 33,
    NoStateProofForRound = 34,
}

impl From<msgpack::DecodeError> for Error {
    fn from(_: msgpack::DecodeError) -> Self {
        Error::MalformedStateProof
    }
}
