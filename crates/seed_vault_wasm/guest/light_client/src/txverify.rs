//! Transaction inclusion proofs (light-client-poc
//! `transactionverifier/transactionVerifier.go`): from a transaction up to
//! the block interval commitment the oracle verified.

use alloc::vec::Vec;

use crate::hashes::sha256;
use crate::msgpack::{encode_struct, Field};
use crate::Error;

const TXN_MERKLE_LEAF: &[u8] = b"TL";
const MERKLE_ARRAY_NODE: &[u8] = b"MA";

/// algod's `TransactionProofResponse` (the fields used).
#[derive(Debug, Clone)]
pub struct TransactionProof {
    pub hash_type: Vec<u8>,
    pub stib_hash: [u8; 32],
    pub index: u64,
    pub proof: Vec<u8>,
    pub tree_depth: u64,
}

/// algod's `LightBlockHeaderProof`.
#[derive(Debug, Clone)]
pub struct LightBlockHeaderProof {
    pub index: u64,
    pub proof: Vec<u8>,
    pub tree_depth: u64,
}

/// `computeTransactionLeaf`: `SHA-256("TL" || txid || stibhash)`.
fn transaction_leaf(transaction_hash: &[u8; 32], stib_hash: &[u8; 32]) -> [u8; 32] {
    let mut data = Vec::with_capacity(2 + 64);
    data.extend_from_slice(TXN_MERKLE_LEAF);
    data.extend_from_slice(transaction_hash);
    data.extend_from_slice(stib_hash);
    sha256(&data)
}

/// `computeLightBlockHeaderLeaf` / `crypto.HashLightBlockHeader`:
/// `SHA-256("B256" || msgpack(lightBlockHeader))`.
fn light_block_header_leaf(
    round: u64,
    transaction_commitment: &[u8; 32],
    genesis_hash: &[u8; 32],
    seed: &[u8; 32],
) -> [u8; 32] {
    let encoded = encode_struct(&[
        ("0", Field::Bin(seed)),
        ("gh", Field::Bin(genesis_hash)),
        ("r", Field::Uint(round)),
        ("tc", Field::Bin(transaction_commitment)),
    ]);
    let mut data = Vec::from(&b"B256"[..]);
    data.extend_from_slice(&encoded);
    sha256(&data)
}

/// `computeVectorCommitmentRoot` (SHA-256 only).
fn vector_commitment_root(
    leaf: [u8; 32],
    leaf_index: u64,
    proof: &[u8],
    tree_depth: u64,
) -> Result<[u8; 32], Error> {
    if proof.is_empty() && tree_depth == 0 {
        return Ok(leaf);
    }
    if tree_depth.checked_mul(32) != Some(proof.len() as u64) {
        return Err(Error::ProofLengthMismatch);
    }
    if tree_depth == 0 {
        return Err(Error::InvalidTreeDepth);
    }
    // `getVectorCommitmentPositions`: index must fit in `depth` bits; its
    // bits, MSB first, say whether each node on the path is a right child.
    // Go's `1 << depth` is 0 for depth >= 64, so those always fail.
    if tree_depth >= 64 || leaf_index >= 1 << tree_depth {
        return Err(Error::IndexDepthMismatch);
    }
    let mut node = leaf;
    for (distance, sibling) in proof.chunks(32).enumerate() {
        let bit = tree_depth - 1 - distance as u64;
        let is_right = (leaf_index >> bit) & 1 == 1;
        let mut data = Vec::with_capacity(2 + 64);
        data.extend_from_slice(MERKLE_ARRAY_NODE);
        if is_right {
            data.extend_from_slice(sibling);
            data.extend_from_slice(&node);
        } else {
            data.extend_from_slice(&node);
            data.extend_from_slice(sibling);
        }
        node = sha256(&data);
    }
    Ok(node)
}

/// `VerifyTransaction`: is the transaction `transaction_hash`, confirmed in
/// `round`, committed to by `block_interval_commitment` (from the oracle)?
pub fn verify_transaction(
    transaction_hash: &[u8; 32],
    transaction_proof: &TransactionProof,
    light_block_header_proof: &LightBlockHeaderProof,
    round: u64,
    genesis_hash: &[u8; 32],
    seed: &[u8; 32],
    block_interval_commitment: &[u8; 32],
) -> Result<(), Error> {
    if transaction_proof.hash_type != b"sha256" {
        return Err(Error::UnknownHash);
    }
    let leaf = transaction_leaf(transaction_hash, &transaction_proof.stib_hash);
    let transaction_root = vector_commitment_root(
        leaf,
        transaction_proof.index,
        &transaction_proof.proof,
        transaction_proof.tree_depth,
    )?;
    let header_leaf = light_block_header_leaf(round, &transaction_root, genesis_hash, seed);
    let root = vector_commitment_root(
        header_leaf,
        light_block_header_proof.index,
        &light_block_header_proof.proof,
        light_block_header_proof.tree_depth,
    )?;
    if &root != block_interval_commitment {
        return Err(Error::RootMismatch);
    }
    Ok(())
}
