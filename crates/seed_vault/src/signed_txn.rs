//! Minimal, hand-rolled canonical msgpack encoding of an Algorand
//! `SignedTxn`, so the TA can return network-ready bytes without pulling a
//! msgpack/serde stack into Trusty.
//!
//! The caller supplies the transaction already msgpack-encoded (exactly
//! what `algosdk`'s `txn.encode()` / `algokit_transact`'s `encode_raw()`
//! produce, *without* the `"TX"` prefix). It's embedded verbatim, so the
//! TA never has to understand transaction fields.
//!
//! Layouts match go-algorand's generated `msgp` code (keys sorted, empty
//! fields omitted):
//!
//! ```text
//! Ed25519:     { "sig": bin(64), "txn": <txn> }
//! Falcon-1024: { "pqsig": { "pk": bin(1793), "sch": bin("f1"),
//!                           "sig": bin(compressed sig), "slt": uint (omitted if 0) },
//!                "txn": <txn> }
//! ```
//!
//! Both signatures cover `"TX" || txn` (go-algorand `crypto.HashRep`); for
//! Falcon, the compressed deterministic signature (`FalconSigner.SignBytes`).
//!
//! Because `"txn"` always sorts last, everything *before* the transaction
//! bytes depends only on the signature. The encoders here return just that
//! prefix (ending in the `"txn"` key); the full `SignedTxn` is
//! `prefix || txn`. That keeps the TA's response size independent of the
//! transaction's size -- a Falcon prefix alone is up to ~3.3 KB, and a Trusty
//! IPC message is capped at about one page (see `seed_vault::protocol`).

use alloc::vec::Vec;

/// Domain-separation prefix Algorand signs transactions under.
pub const TX_PREFIX: &[u8] = b"TX";

/// `"TX" || txn`: the exact bytes both signature schemes sign.
pub fn bytes_to_sign(txn: &[u8]) -> Vec<u8> {
    let mut msg = Vec::with_capacity(TX_PREFIX.len() + txn.len());
    msg.extend_from_slice(TX_PREFIX);
    msg.extend_from_slice(txn);
    msg
}

/// Cheap sanity check that `txn` is a msgpack map (an encoded Algorand
/// transaction always is), and not e.g. already-prefixed `"TX..."` bytes or
/// an already-signed transaction (which is also a map, but whose first key
/// is `lsig`/`msig`/`pqsig`/`sgnr`/`sig`, never a transaction field --
/// the caller is responsible for not sending those).
pub fn looks_like_encoded_transaction(txn: &[u8]) -> bool {
    matches!(txn.first(), Some(0x81..=0x8f | 0xde))
}

/// The transaction's sender (`snd`), if `txn` is a well-formed msgpack map
/// with a 32-byte `snd`. A zero sender is omitted by canonical encoding, so
/// a missing `snd` yields `None` too.
///
/// Walks the top-level map, skipping every other value, so it doesn't need
/// to understand any transaction fields beyond `snd`.
pub fn transaction_sender(txn: &[u8]) -> Option<[u8; 32]> {
    let mut reader = Reader { bytes: txn, pos: 0 };
    let entries = reader.map_len()?;
    let mut sender = None;
    for _ in 0..entries {
        let key = reader.str()?;
        if key == b"snd" {
            sender = Some(reader.bin()?.try_into().ok()?);
        } else {
            reader.skip(0)?;
        }
    }
    // Trailing bytes mean this isn't a single transaction map.
    (reader.pos == txn.len()).then_some(sender).flatten()
}

/// Just enough of a msgpack reader for [`transaction_sender`].
struct Reader<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    /// Nesting limit for [`Self::skip`], so hostile input can't exhaust the
    /// TA's stack. Real transactions nest a handful of levels at most.
    const MAX_DEPTH: usize = 16;

    fn take(&mut self, n: usize) -> Option<&'a [u8]> {
        let end = self.pos.checked_add(n)?;
        let out = self.bytes.get(self.pos..end)?;
        self.pos = end;
        Some(out)
    }

    fn byte(&mut self) -> Option<u8> {
        Some(self.take(1)?[0])
    }

    fn be(&mut self, n: usize) -> Option<usize> {
        let mut v = 0usize;
        for &b in self.take(n)? {
            v = v.checked_mul(256)? | b as usize;
        }
        Some(v)
    }

    fn map_len(&mut self) -> Option<usize> {
        match self.byte()? {
            b @ 0x80..=0x8f => Some((b & 0x0f) as usize),
            0xde => self.be(2),
            0xdf => self.be(4),
            _ => None,
        }
    }

    fn str(&mut self) -> Option<&'a [u8]> {
        let len = match self.byte()? {
            b @ 0xa0..=0xbf => (b & 0x1f) as usize,
            0xd9 => self.be(1)?,
            0xda => self.be(2)?,
            0xdb => self.be(4)?,
            _ => return None,
        };
        self.take(len)
    }

    fn bin(&mut self) -> Option<&'a [u8]> {
        let len = match self.byte()? {
            0xc4 => self.be(1)?,
            0xc5 => self.be(2)?,
            0xc6 => self.be(4)?,
            _ => return None,
        };
        self.take(len)
    }

    /// Skips one complete value of any type.
    fn skip(&mut self, depth: usize) -> Option<()> {
        if depth > Self::MAX_DEPTH {
            return None;
        }
        let (len, items) = match self.byte()? {
            0x00..=0x7f | 0xe0..=0xff | 0xc0 | 0xc2 | 0xc3 => (0, 0),
            b @ 0x80..=0x8f => (0, 2 * (b & 0x0f) as usize),
            b @ 0x90..=0x9f => (0, (b & 0x0f) as usize),
            b @ 0xa0..=0xbf => ((b & 0x1f) as usize, 0),
            0xc4 | 0xd9 => (self.be(1)?, 0),
            0xc5 | 0xda => (self.be(2)?, 0),
            0xc6 | 0xdb => (self.be(4)?, 0),
            0xc7 => (self.be(1)?.checked_add(1)?, 0),
            0xc8 => (self.be(2)?.checked_add(1)?, 0),
            0xc9 => (self.be(4)?.checked_add(1)?, 0),
            0xcc | 0xd0 => (1, 0),
            0xcd | 0xd1 => (2, 0),
            0xca | 0xce | 0xd2 => (4, 0),
            0xcb | 0xcf | 0xd3 => (8, 0),
            0xd4 => (2, 0),
            0xd5 => (3, 0),
            0xd6 => (5, 0),
            0xd7 => (9, 0),
            0xd8 => (17, 0),
            0xdc => (0, self.be(2)?),
            0xdd => (0, self.be(4)?),
            0xde => (0, self.be(2)?.checked_mul(2)?),
            0xdf => (0, self.be(4)?.checked_mul(2)?),
            0xc1 => return None, // never used
        };
        self.take(len)?;
        for _ in 0..items {
            self.skip(depth + 1)?;
        }
        Some(())
    }
}

/// Upper bound on any prefix returned by this module: the Falcon-1024 one
/// with the largest public key, signature, and `slt` field.
pub const MAX_PREFIX_LEN: usize = 64
    + algokit_crypto::falcon1024::PUBLIC_KEY_SIZE
    + algokit_crypto::falcon1024::SIGNATURE_MAX_SIZE;

/// `SignedTxn` prefix for an Ed25519 signature; append the txn bytes.
pub fn ed25519_prefix(signature: &[u8; 64]) -> Vec<u8> {
    let mut out = Vec::with_capacity(16 + signature.len());
    out.push(0x82);
    push_str(&mut out, "sig");
    push_bin(&mut out, signature);
    push_str(&mut out, "txn");
    out
}

/// `SignedTxn` prefix for a Falcon-1024 signature; append the txn bytes.
pub fn falcon1024_prefix(public_key: &[u8], salt: u8, signature: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(48 + public_key.len() + signature.len());
    out.push(0x82);
    push_str(&mut out, "pqsig");
    out.push(if salt == 0 { 0x83 } else { 0x84 });
    push_str(&mut out, "pk");
    push_bin(&mut out, public_key);
    push_str(&mut out, "sch");
    push_bin(&mut out, &algokit_crypto::falcon1024::SCHEME_FALCON1024);
    push_str(&mut out, "sig");
    push_bin(&mut out, signature);
    if salt != 0 {
        push_str(&mut out, "slt");
        push_uint8(&mut out, salt);
    }
    push_str(&mut out, "txn");
    out
}

/// Short keys only (all of ours are < 32 bytes): msgpack fixstr.
fn push_str(out: &mut Vec<u8>, s: &str) {
    debug_assert!(s.len() < 32);
    out.push(0xa0 | s.len() as u8);
    out.extend_from_slice(s.as_bytes());
}

/// msgpack bin8/bin16/bin32, smallest that fits (as `msgp.AppendBytes`).
fn push_bin(out: &mut Vec<u8>, bytes: &[u8]) {
    let len = bytes.len();
    if len <= u8::MAX as usize {
        out.push(0xc4);
        out.push(len as u8);
    } else if len <= u16::MAX as usize {
        out.push(0xc5);
        out.extend_from_slice(&(len as u16).to_be_bytes());
    } else {
        out.push(0xc6);
        out.extend_from_slice(&(len as u32).to_be_bytes());
    }
    out.extend_from_slice(bytes);
}

/// msgpack unsigned int as `msgp.AppendUint8`: positive fixint or uint8.
fn push_uint8(out: &mut Vec<u8>, v: u8) {
    if v >= 0x80 {
        out.push(0xcc);
    }
    out.push(v);
}

/// Test helper: a minimal canonical transaction `{snd, type: "pay"}` sent
/// by `address` (base32 decoded; checksum not verified).
#[cfg(test)]
pub(crate) fn test_txn_from(address: &str) -> Vec<u8> {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
    let (mut buffer, mut bits, mut decoded) = (0u32, 0, Vec::new());
    for c in address.bytes() {
        let v = ALPHABET.iter().position(|&a| a == c).expect("base32") as u32;
        buffer = (buffer << 5) | v;
        bits += 5;
        if bits >= 8 {
            bits -= 8;
            decoded.push((buffer >> bits) as u8);
        }
    }
    [
        &b"\x82\xa3snd\xc4\x20"[..],
        &decoded[..32],
        b"\xa4type\xa3pay",
    ]
    .concat()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encoders_produce_canonical_headers() {
        let txn = [0x81, 0xa4, b't', b'y', b'p', b'e', 0xa3, b'p', b'a', b'y'];
        let ed = [ed25519_prefix(&[7u8; 64]), txn.to_vec()].concat();
        assert_eq!(&ed[..7], &[0x82, 0xa3, b's', b'i', b'g', 0xc4, 64]);
        assert!(ed.ends_with(&[&b"\xa3txn"[..], &txn].concat()));

        let pq = falcon1024_prefix(&[1u8; 1793], 0, &[2u8; 1200]);
        assert!(pq.ends_with(b"\xa3txn"));
        // 2-entry map, "pqsig" -> 3-entry map (salt 0 omitted), "pk" -> bin16(1793).
        assert_eq!(
            &pq[..13],
            &[
                0x82, 0xa5, b'p', b'q', b's', b'i', b'g', 0x83, 0xa2, b'p', b'k', 0xc5, 0x07
            ]
        );
        assert_eq!(pq[13], 0x01);

        let pq_salted = falcon1024_prefix(&[1u8; 1793], 200, &[2u8; 1200]);
        assert_eq!(pq_salted[7], 0x84);
        let slt = pq_salted.windows(4).position(|w| w == b"\xa3slt").unwrap();
        assert_eq!(&pq_salted[slt + 4..slt + 6], &[0xcc, 200]);

        let largest = falcon1024_prefix(&[1u8; 1793], 255, &[2u8; 1423]);
        assert!(largest.len() <= MAX_PREFIX_LEN);
    }

    #[test]
    fn transaction_sender_finds_snd_among_other_fields() {
        let snd = [7u8; 32];
        // {amt: 5, apaa: [bin, 300-byte str], fee: uint16, note: bin, snd: bin32, type: "pay"}
        let mut txn = alloc::vec![0x86];
        txn.extend_from_slice(b"\xa3amt\x05");
        txn.extend_from_slice(b"\xa4apaa\x92\xc4\x01\x00\xda\x01\x2c");
        txn.extend_from_slice(&[b'x'; 300]);
        txn.extend_from_slice(b"\xa3fee\xcd\x0b\xb8\xa4note\xc4\x02hi\xa3snd\xc4\x20");
        txn.extend_from_slice(&snd);
        txn.extend_from_slice(b"\xa4type\xa3pay");
        assert_eq!(transaction_sender(&txn), Some(snd));

        // No snd, truncated, trailing garbage, or snd of the wrong length.
        assert_eq!(transaction_sender(b"\x81\xa4type\xa3pay"), None);
        assert_eq!(transaction_sender(&txn[..txn.len() - 1]), None);
        assert_eq!(
            transaction_sender(&[txn.as_slice(), b"\x00"].concat()),
            None
        );
        assert_eq!(transaction_sender(b"\x81\xa3snd\xc4\x02ab"), None);
        // Absurd nesting is rejected rather than recursed into.
        let deep = [&[0x81, 0xa1, b'x'][..], &[0x91; 64], &[0x00]].concat();
        assert_eq!(transaction_sender(&deep), None);
    }

    #[test]
    fn transaction_shape_check() {
        assert!(looks_like_encoded_transaction(&[0x89, 0x00]));
        assert!(!looks_like_encoded_transaction(b"TX\x89"));
        assert!(!looks_like_encoded_transaction(&[]));
        assert!(!looks_like_encoded_transaction(&[0x80]));
    }
}
