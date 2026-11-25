//! State proof verification, ported from go-stateproof-verification
//! (`stateproof`, `merklesignature`, `stateproofcrypto`).

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::cmp::Ordering;

use algorand_falcon::det1024::{CompressedSignature, PublicKey, PUBLIC_KEY_SIZE};
use sha3::digest::{ExtendableOutput, Update, XofReader};
use sha3::Shake256;

use crate::merkle::{verify_vector_commitment, Proof};
use crate::msgpack::{self, array_field, bytes_field, u64_field, DecodeError, Value};
use crate::sumhash::{Matrix, DIGEST_LEN as SUMHASH_LEN};
use crate::Error;

const STATE_PROOF_SIG: &[u8] = b"sps";
const STATE_PROOF_PART: &[u8] = b"spp";
const STATE_PROOF_COIN: &[u8] = b"spc";
const KEYS_IN_MSS: &[u8] = b"KP";

const STRENGTH_TARGET: u64 = 256;
const PRECISION_BITS: u32 = 16;
/// `ceil(2^16 * ln(2))`.
const LN2_INT_APPROXIMATION: u64 = 45427;
pub const MAX_REVEALS: u64 = 640;
const VERSION_FOR_COIN_GENERATOR: u8 = 0;
pub const MAX_TREE_DEPTH: u8 = 20;
/// `merklesignature.CryptoPrimitivesID`.
const CRYPTO_PRIMITIVES_ID: u16 = 0;

/// `merklesignature.Signature`.
#[derive(Debug, Clone)]
pub struct MerkleSignature {
    /// Compressed Falcon-1024 signature.
    pub signature: Vec<u8>,
    pub vector_commitment_index: u64,
    pub proof: Proof,
    pub verifying_key: [u8; PUBLIC_KEY_SIZE],
}

/// `merklesignature.Verifier`.
#[derive(Debug, Clone)]
pub struct MerkleVerifier {
    pub commitment: [u8; SUMHASH_LEN],
    pub key_lifetime: u64,
}

/// `stateproof.Participant`.
#[derive(Debug, Clone)]
pub struct Participant {
    pub pk: MerkleVerifier,
    pub weight: u64,
}

/// `stateproof.Reveal`.
#[derive(Debug, Clone)]
pub struct Reveal {
    pub sig: MerkleSignature,
    /// `sigslotCommit.L`.
    pub l: u64,
    pub part: Participant,
}

/// `stateproof.StateProof`.
#[derive(Debug, Clone)]
pub struct StateProof {
    pub sig_commit: Vec<u8>,
    pub signed_weight: u64,
    pub sig_proofs: Proof,
    pub part_proofs: Proof,
    pub merkle_signature_salt_version: u8,
    pub reveals: BTreeMap<u64, Reveal>,
    pub positions_to_reveal: Vec<u64>,
}

fn decode_proof(v: &Value, key: &str) -> Result<Proof, DecodeError> {
    match v.field(key)? {
        Some(p) => Proof::decode(p),
        None => Ok(Proof {
            path: Vec::new(),
            hash_type: 0,
            tree_depth: 0,
        }),
    }
}

fn struct_field<'a>(v: &'a Value, key: &str) -> Result<&'a Value, DecodeError> {
    static EMPTY: Value = Value::Map(Vec::new());
    Ok(v.field(key)?.unwrap_or(&EMPTY))
}

impl MerkleSignature {
    fn decode(v: &Value) -> Result<Self, DecodeError> {
        Ok(MerkleSignature {
            signature: bytes_field(v, "sig")?,
            vector_commitment_index: u64_field(v, "idx")?,
            proof: decode_proof(v, "prf")?,
            verifying_key: array_field(struct_field(v, "vkey")?, "k")?,
        })
    }

    /// `Signature.MsgIsZero`: an empty slot.
    fn is_zero(&self) -> bool {
        self.signature.is_empty()
            && self.vector_commitment_index == 0
            && self.proof.is_zero()
            && self.verifying_key.iter().all(|b| *b == 0)
    }

    fn salt_version(&self) -> u8 {
        CompressedSignature(self.signature.clone()).salt_version()
    }

    /// `Signature.GetFixedLengthHashableRepresentation`.
    fn fixed_length_representation(&self) -> Result<Vec<u8>, Error> {
        let ct = CompressedSignature(self.signature.clone())
            .convert_to_ct()
            .map_err(|_| Error::BadSignature)?;
        let proof = self.proof.fixed_length_representation()?;
        let mut out = Vec::with_capacity(2 + ct.0.len() + PUBLIC_KEY_SIZE + 8 + proof.len());
        out.extend_from_slice(&CRYPTO_PRIMITIVES_ID.to_le_bytes());
        out.extend_from_slice(&ct.0);
        out.extend_from_slice(&self.verifying_key);
        out.extend_from_slice(&self.vector_commitment_index.to_le_bytes());
        out.extend_from_slice(&proof);
        Ok(out)
    }
}

impl MerkleVerifier {
    /// `Verifier.VerifyBytes`: the ephemeral key is committed to under
    /// `commitment` for the key's round, and it signed `msg`.
    fn verify_bytes(
        &self,
        matrix: &Matrix,
        round: u64,
        msg: &[u8],
        sig: &MerkleSignature,
    ) -> Result<(), Error> {
        if self.key_lifetime == 0 {
            return Err(Error::ZeroKeyLifetime);
        }
        let valid_key_round = round - (round % self.key_lifetime);
        let mut ephemeral = Vec::with_capacity(KEYS_IN_MSS.len() + 2 + 8 + PUBLIC_KEY_SIZE);
        ephemeral.extend_from_slice(KEYS_IN_MSS);
        ephemeral.extend_from_slice(&CRYPTO_PRIMITIVES_ID.to_le_bytes());
        ephemeral.extend_from_slice(&valid_key_round.to_le_bytes());
        ephemeral.extend_from_slice(&sig.verifying_key);
        let elems = BTreeMap::from([(sig.vector_commitment_index, ephemeral)]);
        verify_vector_commitment(matrix, &self.commitment, &elems, &sig.proof)?;
        let mut key = PublicKey::default();
        key.0.copy_from_slice(&sig.verifying_key);
        key.verify(&sig.signature, msg)
            .map_err(|_| Error::BadSignature)
    }
}

impl Participant {
    fn decode(v: &Value) -> Result<Self, DecodeError> {
        let pk = struct_field(v, "p")?;
        Ok(Participant {
            pk: MerkleVerifier {
                commitment: array_field(pk, "cmt")?,
                key_lifetime: u64_field(pk, "lf")?,
            },
            weight: u64_field(v, "w")?,
        })
    }

    /// `Participant.ToBeHashed`.
    fn hash_rep(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(STATE_PROOF_PART.len() + 16 + SUMHASH_LEN);
        out.extend_from_slice(STATE_PROOF_PART);
        out.extend_from_slice(&self.weight.to_le_bytes());
        out.extend_from_slice(&self.pk.key_lifetime.to_le_bytes());
        out.extend_from_slice(&self.pk.commitment);
        out
    }
}

impl StateProof {
    /// Decodes a msgpack-encoded state proof (as from algod's API).
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        let v = msgpack::decode(bytes)?;
        let mut reveals = BTreeMap::new();
        if let Some(r) = v.field("r")? {
            for (pos, reveal) in r.as_map()? {
                let sig_slot = struct_field(reveal, "s")?;
                let revealed = Reveal {
                    sig: MerkleSignature::decode(struct_field(sig_slot, "s")?)?,
                    l: u64_field(sig_slot, "l")?,
                    part: Participant::decode(struct_field(reveal, "p")?)?,
                };
                if reveals.insert(pos.as_u64()?, revealed).is_some() {
                    return Err(DecodeError); // duplicate key
                }
            }
        }
        let positions_to_reveal = match v.field("pr")? {
            Some(p) => p
                .as_array()?
                .iter()
                .map(Value::as_u64)
                .collect::<Result<_, _>>()?,
            None => Vec::new(),
        };
        Ok(StateProof {
            sig_commit: bytes_field(&v, "c")?,
            signed_weight: u64_field(&v, "w")?,
            sig_proofs: decode_proof(&v, "S")?,
            part_proofs: decode_proof(&v, "P")?,
            merkle_signature_salt_version: u8::try_from(u64_field(&v, "v")?)
                .map_err(|_| DecodeError)?,
            reveals,
            positions_to_reveal,
        })
    }
}

/// `stateproof.Verifier`: checks a state proof against the previous
/// interval's trusted voters commitment and ln(proven weight).
pub struct Verifier<'a> {
    pub participants_commitment: &'a [u8],
    pub ln_proven_weight: u64,
}

impl Verifier<'_> {
    /// `Verifier.Verify`: `proof` proves that enough weight signed
    /// `message_hash` at `round`.
    pub fn verify(
        &self,
        matrix: &Matrix,
        round: u64,
        message_hash: &[u8; 32],
        proof: &StateProof,
    ) -> Result<(), Error> {
        if proof.sig_proofs.tree_depth > MAX_TREE_DEPTH
            || proof.part_proofs.tree_depth > MAX_TREE_DEPTH
        {
            return Err(Error::TreeTooDeep);
        }
        let reveals_count = proof.positions_to_reveal.len() as u64;
        verify_weights(
            proof.signed_weight,
            self.ln_proven_weight,
            reveals_count,
            STRENGTH_TARGET,
        )?;
        for reveal in proof.reveals.values() {
            if reveal.sig.salt_version() != proof.merkle_signature_salt_version {
                return Err(Error::SaltVersionMismatch);
            }
        }

        let mut sigs = BTreeMap::new();
        let mut parts = BTreeMap::new();
        for (&pos, reveal) in &proof.reveals {
            sigs.insert(pos, committable_signature_slot(reveal)?);
            parts.insert(pos, reveal.part.hash_rep());
            reveal
                .part
                .pk
                .verify_bytes(matrix, round, message_hash, &reveal.sig)?;
        }
        verify_vector_commitment(matrix, &proof.sig_commit, &sigs, &proof.sig_proofs)?;
        verify_vector_commitment(
            matrix,
            self.participants_commitment,
            &parts,
            &proof.part_proofs,
        )?;

        let mut coins = CoinGenerator::new(&CoinChoiceSeed {
            part_commitment: self.participants_commitment,
            ln_proven_weight: self.ln_proven_weight,
            sig_commitment: &proof.sig_commit,
            signed_weight: proof.signed_weight,
            data: message_hash,
        });
        for pos in &proof.positions_to_reveal {
            let reveal = proof.reveals.get(pos).ok_or(Error::NoRevealInPosition)?;
            let coin = coins.next_coin();
            // Go's uint64 `L + Weight` wraps; mirror it exactly.
            let in_range = reveal.l <= coin && coin < reveal.l.wrapping_add(reveal.part.weight);
            if !in_range {
                return Err(Error::CoinNotInRange);
            }
        }
        Ok(())
    }
}

/// `committableSignatureSlot.ToBeHashed`.
fn committable_signature_slot(reveal: &Reveal) -> Result<Vec<u8>, Error> {
    let mut out = Vec::from(STATE_PROOF_SIG);
    if reveal.sig.is_zero() {
        return Ok(out);
    }
    if reveal.sig.signature.is_empty() {
        return Err(Error::BadSignature);
    }
    out.extend_from_slice(&reveal.l.to_le_bytes());
    out.extend_from_slice(&reveal.sig.fixed_length_representation()?);
    Ok(out)
}

// --- Weights (`weights.go`) ----------------------------------------------

/// Unsigned 256-bit integer, enough for `verifyWeights` (whose values stay
/// below ~2^210 for any u64 inputs). Little-endian u64 limbs.
#[derive(Clone, Copy, PartialEq, Eq)]
struct U256([u64; 4]);

impl U256 {
    fn from_u128(v: u128) -> Self {
        U256([v as u64, (v >> 64) as u64, 0, 0])
    }

    fn pow2(n: u32) -> Self {
        let mut out = [0u64; 4];
        out[(n / 64) as usize] = 1 << (n % 64);
        U256(out)
    }

    fn add(self, other: Self) -> Self {
        let mut out = [0u64; 4];
        let mut carry = 0u128;
        for (i, limb) in out.iter_mut().enumerate() {
            let sum = u128::from(self.0[i]) + u128::from(other.0[i]) + carry;
            *limb = sum as u64;
            carry = sum >> 64;
        }
        debug_assert_eq!(carry, 0);
        U256(out)
    }

    /// `self - other`, requiring `self >= other`.
    fn sub(self, other: Self) -> Self {
        let mut out = [0u64; 4];
        let mut borrow = 0i128;
        for (i, limb) in out.iter_mut().enumerate() {
            let diff = i128::from(self.0[i]) - i128::from(other.0[i]) - borrow;
            borrow = i128::from(diff < 0);
            *limb = (diff + (borrow << 64)) as u64;
        }
        debug_assert_eq!(borrow, 0);
        U256(out)
    }

    fn mul(self, other: Self) -> Self {
        let mut out = [0u64; 4];
        for i in 0..4 {
            let mut carry = 0u128;
            for j in 0..4 - i {
                let cur =
                    u128::from(out[i + j]) + u128::from(self.0[i]) * u128::from(other.0[j]) + carry;
                out[i + j] = cur as u64;
                carry = cur >> 64;
            }
        }
        U256(out)
    }
}

impl PartialOrd for U256 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for U256 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.iter().rev().cmp(other.0.iter().rev())
    }
}

/// `verifyWeights`: are `reveals` enough to prove, with security
/// `strength_target`, that `signed_weight` exceeds the proven weight?
fn verify_weights(
    signed_weight: u64,
    ln_proven_weight: u64,
    reveals: u64,
    strength_target: u64,
) -> Result<(), Error> {
    if reveals > MAX_REVEALS {
        return Err(Error::TooManyReveals);
    }
    if signed_weight == 0 {
        return Err(Error::ZeroSignedWeight);
    }
    let (y, x, w) = sub_expressions(signed_weight);
    let n = U256::from_u128(u128::from(reveals));
    let lhs = n.mul(x.add(w.mul(y)));
    let rhs = U256::from_u128(
        u128::from(strength_target) * u128::from(LN2_INT_APPROXIMATION)
            + u128::from(reveals) * u128::from(ln_proven_weight),
    )
    .mul(y);
    if lhs < rhs {
        return Err(Error::InsufficientSignedWeight);
    }
    Ok(())
}

/// `getSubExpressions`.
fn sub_expressions(signed_weight: u64) -> (U256, U256, U256) {
    let d = 63 - signed_weight.leading_zeros(); // bits.Len64 - 1
    let sw = U256::from_u128(u128::from(signed_weight));
    let sw_squared = sw.mul(sw);
    let tmp = U256::pow2(d + 2).mul(sw);
    let y = U256::pow2(2 * d).add(tmp).add(sw_squared);
    let x = sw_squared
        .sub(U256::pow2(2 * d))
        .mul(U256::from_u128(3))
        .mul(U256::pow2(PRECISION_BITS));
    let w = U256::from_u128(u128::from(d) * u128::from(LN2_INT_APPROXIMATION - 1));
    (y, x, w)
}

// --- Coin generation (`coinGenerator.go`) ---------------------------------

struct CoinChoiceSeed<'a> {
    part_commitment: &'a [u8],
    ln_proven_weight: u64,
    sig_commitment: &'a [u8],
    signed_weight: u64,
    data: &'a [u8; 32],
}

struct CoinGenerator {
    xof: <Shake256 as ExtendableOutput>::Reader,
    signed_weight: u64,
    threshold: u128,
}

impl CoinGenerator {
    fn new(choice: &CoinChoiceSeed<'_>) -> Self {
        let mut shake = Shake256::default();
        shake.update(STATE_PROOF_COIN);
        shake.update(&[VERSION_FOR_COIN_GENERATOR]);
        shake.update(choice.part_commitment);
        shake.update(&choice.ln_proven_weight.to_le_bytes());
        shake.update(choice.sig_commitment);
        shake.update(&choice.signed_weight.to_le_bytes());
        shake.update(choice.data);
        // Rejection sampling: only accept draws below the largest multiple
        // of `signed_weight` that fits in 64 bits, so `% signed_weight` is
        // uniform. (signed_weight > 0 is checked by `verify_weights` first.)
        let sw = u128::from(choice.signed_weight);
        CoinGenerator {
            xof: shake.finalize_xof(),
            signed_weight: choice.signed_weight,
            threshold: ((1u128 << 64) / sw) * sw,
        }
    }

    fn next_coin(&mut self) -> u64 {
        loop {
            let mut draw = [0u8; 8];
            self.xof.read(&mut draw);
            let z = u64::from_le_bytes(draw);
            if u128::from(z) < self.threshold {
                return z % self.signed_weight;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u256_arithmetic() {
        let a = U256::from_u128(u128::MAX);
        let b = a.mul(a); // (2^128-1)^2 = 2^256 - 2^129 + 1
        assert_eq!(b.0, [1, 0, u64::MAX - 1, u64::MAX]);
        assert_eq!(b.sub(U256::from_u128(1)).0, [0, 0, u64::MAX - 1, u64::MAX]);
        assert!(U256::pow2(200) > U256::pow2(199).add(U256::pow2(198)));
        assert_eq!(U256::pow2(70).0, [0, 64, 0, 0]);
    }

    #[test]
    fn weights_reject_zero_and_too_many() {
        assert_eq!(verify_weights(0, 1, 1, 256), Err(Error::ZeroSignedWeight));
        assert_eq!(
            verify_weights(1000, 1, MAX_REVEALS + 1, 256),
            Err(Error::TooManyReveals)
        );
        // Huge inputs don't overflow.
        let _ = verify_weights(u64::MAX, u64::MAX, MAX_REVEALS, 256);
    }
}
