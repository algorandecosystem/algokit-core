//! Merkle array proofs (go-stateproof-verification `merklearray`).

use alloc::collections::BTreeMap;
use alloc::vec;
use alloc::vec::Vec;

use crate::hashes::{hash_obj, HashType};
use crate::msgpack::{u64_field, DecodeError, Value};
use crate::sumhash::Matrix;
use crate::Error;

/// Internal node domain separator.
const MERKLE_ARRAY_NODE: &[u8] = b"MA";
/// Deepest tree a [`SingleLeafProof`] can encode.
pub const MAX_ENCODED_TREE_DEPTH: u8 = 16;

/// `merklearray.Proof`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Proof {
    pub path: Vec<Vec<u8>>,
    /// Raw `HashFactory.HashType`; validated when used.
    pub hash_type: u64,
    pub tree_depth: u8,
}

impl Proof {
    pub fn decode(v: &Value) -> Result<Self, DecodeError> {
        let path = match v.field("pth")? {
            Some(p) => p
                .as_array()?
                .iter()
                .map(|d| d.as_bytes().map(<[u8]>::to_vec))
                .collect::<Result<_, _>>()?,
            None => Vec::new(),
        };
        let hash_type = match v.field("hsh")? {
            Some(h) => u64_field(h, "t")?,
            None => 0,
        };
        let tree_depth = u8::try_from(u64_field(v, "td")?).map_err(|_| DecodeError)?;
        Ok(Proof {
            path,
            hash_type,
            tree_depth,
        })
    }

    pub fn hash(&self) -> Result<HashType, Error> {
        HashType::from_u64(self.hash_type).ok_or(Error::UnknownHash)
    }

    pub fn is_zero(&self) -> bool {
        self.path.is_empty() && self.hash_type == 0 && self.tree_depth == 0
    }

    /// `SingleLeafProof.GetFixedLengthHashableRepresentation`: the depth,
    /// then zero digests padding the path to the maximum depth, then the path.
    pub fn fixed_length_representation(&self) -> Result<Vec<u8>, Error> {
        let size = self.hash()?.size();
        if self.tree_depth > MAX_ENCODED_TREE_DEPTH {
            return Err(Error::TreeTooDeep);
        }
        let depth = usize::from(self.tree_depth);
        let mut out = Vec::with_capacity(1 + usize::from(MAX_ENCODED_TREE_DEPTH) * size);
        out.push(self.tree_depth);
        out.resize(1 + (usize::from(MAX_ENCODED_TREE_DEPTH) - depth) * size, 0);
        for i in 0..depth {
            match self.path.get(i) {
                Some(node) if !node.is_empty() => out.extend_from_slice(node),
                _ => out.resize(out.len() + size, 0),
            }
        }
        Ok(out)
    }
}

/// `1 << depth`, or 0 when Go's shift would overflow to 0.
fn leaf_count(depth: u8) -> u64 {
    1u64.checked_shl(u32::from(depth)).unwrap_or(0)
}

/// `merkleTreeToVectorCommitmentIndex`.
fn to_vector_commitment_index(msb_index: u64, depth: u8) -> Result<u64, Error> {
    if msb_index >= leaf_count(depth) {
        return Err(Error::PositionOutOfBounds);
    }
    Ok(msb_index
        .reverse_bits()
        .checked_shr(64 - u32::from(depth))
        .unwrap_or(0))
}

/// `merklearray.Verify`: checks `elems` (position -> hash representation,
/// i.e. `hash_id || data`) are leaves of the tree with `root`.
pub fn verify(
    matrix: &Matrix,
    root: &[u8],
    elems: &BTreeMap<u64, Vec<u8>>,
    proof: &Proof,
) -> Result<(), Error> {
    if elems.is_empty() {
        return if proof.path.is_empty() {
            Ok(())
        } else {
            Err(Error::NonEmptyProofForEmptySet)
        };
    }
    let hash = proof.hash()?;
    let mut layer = Vec::with_capacity(elems.len());
    for (&pos, rep) in elems {
        if pos >= leaf_count(proof.tree_depth) {
            return Err(Error::PositionOutOfBounds);
        }
        layer.push((pos, hash.hash(matrix, rep)));
    }
    // BTreeMap iteration is already sorted by position.
    let mut hints = proof.path.iter();
    while hints.len() > 0 || layer.len() > 1 {
        layer = up(matrix, hash, &layer, &mut hints)?;
    }
    match layer.first() {
        Some((0, computed)) if computed.as_slice() == root => Ok(()),
        _ => Err(Error::RootMismatch),
    }
}

/// `merklearray.VerifyVectorCommitment`: as [`verify`], with positions in
/// vector commitment (LSB-first) order.
pub fn verify_vector_commitment(
    matrix: &Matrix,
    root: &[u8],
    elems: &BTreeMap<u64, Vec<u8>>,
    proof: &Proof,
) -> Result<(), Error> {
    let mut converted = BTreeMap::new();
    for (&pos, rep) in elems {
        converted.insert(
            to_vector_commitment_index(pos, proof.tree_depth)?,
            rep.clone(),
        );
    }
    verify(matrix, root, &converted, proof)
}

/// `partialLayer.up`.
fn up<'a>(
    matrix: &Matrix,
    hash: HashType,
    layer: &[(u64, Vec<u8>)],
    hints: &mut impl Iterator<Item = &'a Vec<u8>>,
) -> Result<Vec<(u64, Vec<u8>)>, Error> {
    let mut out = Vec::with_capacity(layer.len() / 2 + 1);
    let mut i = 0;
    while i < layer.len() {
        let (pos, node) = &layer[i];
        let sibling_pos = pos ^ 1;
        let sibling = match layer.get(i + 1) {
            Some((p, h)) if *p == sibling_pos => {
                i += 1;
                h.clone()
            }
            _ => hints.next().ok_or(Error::MissingSibling)?.clone(),
        };
        let mut pair = vec![0u8; 2 * hash.size()];
        let (left, right) = if pos & 1 == 0 {
            (node, &sibling)
        } else {
            (&sibling, node)
        };
        // Exactly Go's `pair.ToBeHashed`: copy(buf, l); copy(buf[len(l):], r)
        // into a 2*size buffer (where an oversized `l` would panic).
        if left.len() > pair.len() {
            return Err(Error::MalformedDigest);
        }
        pair[..left.len()].copy_from_slice(left);
        let r = right.len().min(pair.len() - left.len());
        pair[left.len()..left.len() + r].copy_from_slice(&right[..r]);
        out.push((pos / 2, hash_obj(hash, matrix, MERKLE_ARRAY_NODE, &pair)));
        i += 1;
    }
    Ok(out)
}
