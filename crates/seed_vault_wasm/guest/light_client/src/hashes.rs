//! The hash functions state proofs use (go-stateproof-verification
//! `stateproofcrypto/hashes.go`).

use alloc::vec::Vec;

use sha2::{Digest, Sha256, Sha512_256};

use crate::sumhash::{sumhash512, Matrix};

/// `stateproofcrypto.HashType`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashType {
    Sha512_256 = 0,
    Sumhash = 1,
    Sha256 = 2,
}

impl HashType {
    pub fn from_u64(v: u64) -> Option<Self> {
        match v {
            0 => Some(Self::Sha512_256),
            1 => Some(Self::Sumhash),
            2 => Some(Self::Sha256),
            _ => None,
        }
    }

    pub fn size(self) -> usize {
        match self {
            Self::Sha512_256 | Self::Sha256 => 32,
            Self::Sumhash => 64,
        }
    }

    pub fn hash(self, matrix: &Matrix, data: &[u8]) -> Vec<u8> {
        match self {
            Self::Sha512_256 => Sha512_256::digest(data).to_vec(),
            Self::Sha256 => Sha256::digest(data).to_vec(),
            Self::Sumhash => sumhash512(matrix, data).to_vec(),
        }
    }
}

/// `GenericHashObj`: `hash(hash_id || data)`.
pub fn hash_obj(hash: HashType, matrix: &Matrix, hash_id: &[u8], data: &[u8]) -> Vec<u8> {
    let mut rep = Vec::with_capacity(hash_id.len() + data.len());
    rep.extend_from_slice(hash_id);
    rep.extend_from_slice(data);
    hash.hash(matrix, &rep)
}

pub fn sha256(data: &[u8]) -> [u8; 32] {
    Sha256::digest(data).into()
}
