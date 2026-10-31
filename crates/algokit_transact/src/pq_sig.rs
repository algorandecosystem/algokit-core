//! Native post-quantum (PQ) transaction authorization -- the `pqsig` field
//! of a `SignedTxn` (go-algorand `transactions.PQSig`).
//!
//! ```text
//! SignedTxn { pqsig: { pk: bin, sch: bin(2), sig: bin, slt?: uint8 }, txn: .. }
//! ```
//!
//! For Falcon-1024 (`sch = "f1"`), `pk` is the 1793-byte public key, `sig` the
//! compressed deterministic signature over `"TX" || msgpack(txn)`, and `slt`
//! the address salt (omitted when 0). The sender (or `sgnr`, if rekeyed) must
//! equal `SHA512/256("PQA" || sch || slt || pk)` -- see
//! `algokit_crypto::falcon1024::pq_address`.

use serde::{Deserialize, Serialize};
use serde_with::{Bytes, serde_as};

/// Two-byte PQ scheme identifier for Falcon-1024 (`protocol.PQSchemeFalcon1024`).
pub const PQ_SCHEME_FALCON1024: [u8; 2] = *b"f1";

/// A native post-quantum signature authorizing a transaction.
#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct PqSignature {
    /// The signer's PQ public key (1793 bytes for Falcon-1024).
    #[serde(rename = "pk")]
    #[serde_as(as = "Bytes")]
    pub public_key: Vec<u8>,

    /// Two-byte scheme identifier, e.g. [`PQ_SCHEME_FALCON1024`].
    #[serde(rename = "sch")]
    #[serde_as(as = "Bytes")]
    pub scheme: [u8; 2],

    /// The signature over `"TX" || msgpack(txn)` (compressed format for
    /// Falcon-1024).
    #[serde(rename = "sig")]
    #[serde_as(as = "Bytes")]
    pub signature: Vec<u8>,

    /// Address salt: which PQ-compliant address of `public_key` this
    /// authorizes. Zero encodes as absent.
    #[serde(rename = "slt")]
    #[serde(default, skip_serializing_if = "is_zero_u8")]
    pub salt: u8,
}

impl PqSignature {
    /// A Falcon-1024 `pqsig`.
    pub fn falcon1024(public_key: Vec<u8>, signature: Vec<u8>, salt: u8) -> Self {
        Self {
            public_key,
            scheme: PQ_SCHEME_FALCON1024,
            signature,
            salt,
        }
    }
}

fn is_zero_u8(v: &u8) -> bool {
    *v == 0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{TransactionHeaderMother, TransactionMother};
    use crate::{Address, AlgorandMsgpack, SignedTransaction, Transaction, TransactionId};
    use algokit_crypto::falcon1024;

    /// A payment sent by the Falcon-1024 account derived from `entropy`,
    /// signed with that account's key.
    fn falcon_signed_payment(
        entropy: [u8; 32],
    ) -> (SignedTransaction, falcon1024::Falcon1024Account) {
        let account = falcon1024::account_from_entropy(&entropy).unwrap();
        let header = TransactionHeaderMother::simple_testnet()
            .sender(Address(account.address))
            .build()
            .unwrap();
        let txn: Transaction = TransactionMother::simple_payment()
            .header(header)
            .build()
            .unwrap();
        let signature = account
            .private_key
            .sign_compressed(&txn.encode().unwrap())
            .unwrap();
        let stxn = SignedTransaction {
            transaction: txn,
            signature: None,
            auth_address: None,
            multisignature: None,
            logic_signature: None,
            pq_signature: Some(PqSignature::falcon1024(
                account.public_key.0.to_vec(),
                signature.0,
                account.salt,
            )),
        };
        (stxn, account)
    }

    /// msgpack bin8/bin16, as go-algorand's `msgp.AppendBytes`.
    fn bin(bytes: &[u8]) -> Vec<u8> {
        let mut out = match bytes.len() {
            n if n <= 0xff => vec![0xc4, n as u8],
            n => [&[0xc5][..], &(n as u16).to_be_bytes()].concat(),
        };
        out.extend_from_slice(bytes);
        out
    }

    /// The exact bytes go-algorand (and the Trusty `seed_vault` TA's
    /// `signed_txn::falcon1024_prefix`) produce for a Falcon `SignedTxn`.
    fn expected_encoding(pq: &PqSignature, raw_txn: &[u8]) -> Vec<u8> {
        let mut out = vec![0x82, 0xa5];
        out.extend_from_slice(b"pqsig");
        out.push(if pq.salt == 0 { 0x83 } else { 0x84 });
        out.extend_from_slice(b"\xa2pk");
        out.extend(bin(&pq.public_key));
        out.extend_from_slice(b"\xa3sch\xc4\x02f1\xa3sig");
        out.extend(bin(&pq.signature));
        if pq.salt != 0 {
            out.extend_from_slice(b"\xa3slt");
            if pq.salt >= 0x80 {
                out.push(0xcc);
            }
            out.push(pq.salt);
        }
        out.extend_from_slice(b"\xa3txn");
        out.extend_from_slice(raw_txn);
        out
    }

    #[test]
    fn falcon1024_signed_transaction_matches_go_algorand_encoding() {
        // Salt 1 for the go-algorand known-answer entropy below.
        let entropy: [u8; 32] = core::array::from_fn(|i| (i + 1) as u8);
        let (stxn, account) = falcon_signed_payment(entropy);
        assert_ne!(account.salt, 0, "exercise the `slt` field");

        let encoded = stxn.encode().unwrap();
        let pq = stxn.pq_signature.as_ref().unwrap();
        assert_eq!(
            encoded,
            expected_encoding(pq, &stxn.transaction.encode_raw().unwrap())
        );

        // The signature verifies over exactly the bytes algod will check.
        account
            .public_key
            .verify(&pq.signature, &stxn.transaction.encode().unwrap())
            .unwrap();

        // And the sender is the PQ address the key + salt authorize.
        assert_eq!(
            falcon1024::pq_address(PQ_SCHEME_FALCON1024, pq.salt, &pq.public_key),
            *stxn.transaction.header().sender.as_bytes()
        );
    }

    #[test]
    fn falcon1024_signed_transaction_round_trips() {
        let (stxn, _) = falcon_signed_payment([7u8; 32]);
        let decoded = SignedTransaction::decode(&stxn.encode().unwrap()).unwrap();
        assert_eq!(decoded, stxn);
        assert_eq!(decoded.id().unwrap(), stxn.transaction.id().unwrap());
    }

    #[test]
    fn zero_salt_is_omitted() {
        let pq = PqSignature::falcon1024(vec![1u8; 1793], vec![2u8; 1200], 0);
        let stxn = SignedTransaction {
            transaction: TransactionMother::simple_payment().build().unwrap(),
            signature: None,
            auth_address: None,
            multisignature: None,
            logic_signature: None,
            pq_signature: Some(pq.clone()),
        };
        let encoded = stxn.encode().unwrap();
        assert_eq!(
            encoded,
            expected_encoding(&pq, &stxn.transaction.encode_raw().unwrap())
        );
        assert_eq!(SignedTransaction::decode(&encoded).unwrap(), stxn);
    }
}
