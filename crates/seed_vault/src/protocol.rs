//! Wire format for the `seed_vault_ta` IPC protocol.
//!
//! A tiny hand-rolled binary format (no serde/protobuf), driven identically
//! by the real `tipc` transport and this crate's own host-side tests.
//!
//! ## Request
//!
//! | Byte 0                     | Remaining bytes                                                    |
//! |-----------------------------|---------------------------------------------------------------------|
//! | `0x01` = `CREATE_ACCOUNT`  | `[algorithm: u8][address_index: u8]`                                |
//! | `0x02` = `LIST_ACCOUNTS`   | (none), or `[start: u32 LE]` (position of the first account to return; none = 0) |
//! | `0x03` = `REVEAL_MNEMONIC` | `[addr_len: u8][addr_len bytes of ASCII address]`                   |
//! | `0x04` = `IMPORT_ACCOUNT`  | `[algorithm: u8][address_index: u8][mnemonic_len: u8][mnemonic_len bytes of ASCII]` |
//! | `0x05` = `DELETE_ACCOUNT`  | `[addr_len: u8][addr_len bytes of ASCII address]`                   |
//! | `0x06` = `SIGN_TRANSACTION`| `[address_index: u8][addr_len: u8][addr_len bytes of ASCII address][txn_len: u16 LE][txn_len bytes of msgpack txn]` |
//!
//! `REVEAL_MNEMONIC` takes the same Algorand address a caller already got
//! back from `CREATE_ACCOUNT`/`LIST_ACCOUNTS` -- not an internal index. This
//! is deliberately asymmetric with `IMPORT_ACCOUNT`, which takes a
//! mnemonic rather than an address: `REVEAL_MNEMONIC` looks up an
//! *existing* record, while `IMPORT_ACCOUNT` has no record to look up yet
//! -- the mnemonic itself is what gets decoded into a seed and persisted.
//!
//! ## Response
//!
//! | Byte 0                | Remaining bytes            |
//! |------------------------|----------------------------|
//! | `0x00` = `OK`          | opcode-specific, see below |
//! | `0x01` = `ERR`         | 1 byte: [`ErrorCode`]      |
//!
//! `CREATE_ACCOUNT`/`IMPORT_ACCOUNT`/`DELETE_ACCOUNT` success payload
//! (for delete: the account that was removed):
//! `[algorithm: u8][address_index: u32 LE][addr_len: u8][addr_len bytes of ASCII Algorand address]`.
//!
//! `LIST_ACCOUNTS` success payload: `[total: u32 LE][count: u32 LE]`,
//! followed by `count` entries of
//! `[algorithm: u8][address_index: u32 LE][addr_len: u8][addr bytes][pk_len: u16 LE][pk_len bytes of public key]`
//! -- the accounts at positions `start..start + count` of `total`. The
//! public key is 32 bytes for Ed25519 and 1793 bytes for Falcon-1024, so
//! only a couple of Falcon accounts fit in one message: the TA packs as
//! many entries as fit in [`MAX_RESPONSE_LEN`], and the caller asks again
//! with `start + count` until it has all `total`. Positions shift if an
//! account is created/deleted between pages.
//!
//! `address_index` says which of its seed's accounts an address is (see
//! [`crate::store::AccountSummary::address_index`]): `0` = human signing
//! (the standard account other wallets/SDKs restore from the 25 words; the
//! only one an Ed25519 seed has), `1` = autosign (Falcon-1024 seeds only:
//! the same key at its next PQ-compliant salt, so both entries carry the
//! same public key).
//! `CREATE_ACCOUNT`/`IMPORT_ACCOUNT` take the index of the account to
//! return (the seed derives all of its keys regardless; `LIST_ACCOUNTS`
//! shows them all). `SIGN_TRANSACTION` takes the index the caller intends
//! to sign with, and fails with [`ErrorCode::UnknownAddressIndex`] if the
//! address isn't that account -- so the TA always knows whether a signature
//! is meant to be human-approved or automatic. An index the algorithm
//! doesn't have fails the same way. The transaction's sender must also be
//! the address, else [`ErrorCode::SenderMismatch`] (see
//! [`crate::store::AUTOSIGN_INDEX`] for why).
//!
//! `REVEAL_MNEMONIC` success payload: `[mnemonic_len: u8][mnemonic_len bytes of ASCII, space-separated words]`.
//!
//! `SIGN_TRANSACTION`'s `txn` is a msgpack-encoded transaction *without*
//! the `"TX"` prefix (`algosdk`'s `txn.encode()`). Success payload:
//! `[algorithm: u8][prefix_len: u16 LE][prefix_len bytes]`, where the
//! network-ready msgpack `SignedTxn` is `prefix || txn` (see
//! [`crate::signed_txn`]). Only the prefix is returned because a Trusty IPC
//! message is capped at about one page on the Linux side (the `trusty-ipc`
//! driver's per-message buffer), and a Falcon-1024 prefix alone (public key
//! + signature) is up to ~3.3 KB.
//!
//! `CREATE_ACCOUNT`/`LIST_ACCOUNTS` never carry private key material --
//! only addresses. `REVEAL_MNEMONIC`/`IMPORT_ACCOUNT` are the two
//! deliberate exceptions (a mnemonic leaves the secure world for one,
//! enters it for the other); see `store::reveal_mnemonic`/
//! `store::import_account`'s doc comments for the caller-side handling
//! this implies.

use alloc::string::String;
use alloc::vec::Vec;

use crate::store::KeyAlgorithm;

const OP_CREATE_ACCOUNT: u8 = 0x01;
const OP_LIST_ACCOUNTS: u8 = 0x02;
const OP_REVEAL_MNEMONIC: u8 = 0x03;
const OP_IMPORT_ACCOUNT: u8 = 0x04;
const OP_DELETE_ACCOUNT: u8 = 0x05;
const OP_SIGN_TRANSACTION: u8 = 0x06;

/// Largest response the TA will build. The TA's port allows 4096-byte
/// messages; this leaves headroom under that for the Linux `trusty-ipc`
/// driver, whose one-page buffer also holds its own message header.
pub const MAX_RESPONSE_LEN: usize = 4000;

const STATUS_OK: u8 = 0x00;
const STATUS_ERR: u8 = 0x01;

/// An account's public identity as carried in a `CREATE_ACCOUNT`/
/// `IMPORT_ACCOUNT`/`DELETE_ACCOUNT` response. Mirrors
/// [`crate::store::AccountSummary`] minus the public key (see
/// [`ListedAccount`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WireAccount {
    pub algorithm: KeyAlgorithm,
    pub address_index: u32,
    pub address: String,
}

/// One `LIST_ACCOUNTS` entry: the account plus its public key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListedAccount {
    pub account: WireAccount,
    pub public_key: Vec<u8>,
}

/// One decoded page of a `LIST_ACCOUNTS` response.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListPage {
    /// How many accounts the TA holds in total.
    pub total: u32,
    /// The accounts at positions `start..start + accounts.len()`.
    pub accounts: Vec<ListedAccount>,
}

/// A decoded incoming request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Request {
    CreateAccount {
        algorithm: KeyAlgorithm,
        address_index: u8,
    },
    /// Lists accounts from position `start` on (see the module docs on
    /// paging).
    ListAccounts {
        start: u32,
    },
    RevealMnemonic {
        address: String,
    },
    ImportAccount {
        algorithm: KeyAlgorithm,
        address_index: u8,
        mnemonic: String,
    },
    /// Permanently removes the seed backing `address` (see
    /// `store::delete_account`).
    DeleteAccount {
        address: String,
    },
    /// Signs `txn` with the account for `address` (see
    /// `store::sign_transaction`).
    SignTransaction {
        address_index: u8,
        address: String,
        txn: Vec<u8>,
    },
}

/// Machine-readable error codes carried in an error response.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ErrorCode {
    /// The request bytes didn't parse (unknown opcode, wrong length, an
    /// unrecognized algorithm tag, etc).
    MalformedRequest = 0x01,
    /// The account store (secure storage on-device) failed.
    StoreFailure = 0x02,
    /// A response would not fit the wire format (e.g. an address longer
    /// than 255 bytes -- should never happen for a real Algorand address,
    /// but checked rather than assumed).
    ResponseTooLarge = 0x03,
    /// `CREATE_ACCOUNT` asked for an algorithm this build can't generate
    /// yet, or `REVEAL_MNEMONIC` targeted an account with no mnemonic
    /// representation.
    UnsupportedAlgorithm = 0x04,
    /// `REVEAL_MNEMONIC`/`DELETE_ACCOUNT` targeted an address with no
    /// corresponding account.
    AccountNotFound = 0x05,
    /// `IMPORT_ACCOUNT`'s mnemonic failed checksum/wordlist validation.
    InvalidMnemonic = 0x06,
    /// `IMPORT_ACCOUNT`'s mnemonic matches an account that already exists.
    AccountAlreadyExists = 0x07,
    /// `SIGN_TRANSACTION`'s `txn` isn't a msgpack-encoded transaction.
    InvalidTransaction = 0x08,
    /// Signing failed, or the signature didn't verify against the account's
    /// own public key.
    SigningFailed = 0x09,
    /// The requested `address_index` isn't one the account's algorithm has
    /// (e.g. `1` for Ed25519), or `SIGN_TRANSACTION`'s address isn't the key
    /// at that index.
    UnknownAddressIndex = 0x0A,
    /// `SIGN_TRANSACTION`'s transaction isn't sent by (`snd` isn't) the
    /// address it asked to sign with.
    SenderMismatch = 0x0B,
}

/// Errors decoding a *response*. Only used by clients/tests, since a real
/// Trusty TA never needs to parse its own responses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    /// Not enough bytes to decode the expected shape.
    Truncated,
    /// The status byte wasn't `0x00`/`0x01`, or the error code byte
    /// following a `0x01` status wasn't a recognized [`ErrorCode`].
    UnknownStatus,
    /// A payload byte wasn't valid UTF-8, or an algorithm tag wasn't
    /// recognized.
    Malformed,
    /// The message was a well-formed *error* response.
    ErrorResponse(ErrorCode),
}

/// Decodes a raw request payload received over IPC.
pub fn decode_request(bytes: &[u8]) -> Result<Request, ErrorCode> {
    match bytes {
        [OP_CREATE_ACCOUNT, tag, address_index] => {
            let algorithm = KeyAlgorithm::from_tag(*tag).ok_or(ErrorCode::UnsupportedAlgorithm)?;
            Ok(Request::CreateAccount {
                algorithm,
                address_index: *address_index,
            })
        }
        [OP_LIST_ACCOUNTS] => Ok(Request::ListAccounts { start: 0 }),
        [OP_LIST_ACCOUNTS, a, b, c, d] => Ok(Request::ListAccounts {
            start: u32::from_le_bytes([*a, *b, *c, *d]),
        }),
        [OP_REVEAL_MNEMONIC, rest @ ..] => Ok(Request::RevealMnemonic {
            address: decode_address_payload(rest)?,
        }),
        [OP_DELETE_ACCOUNT, rest @ ..] => Ok(Request::DeleteAccount {
            address: decode_address_payload(rest)?,
        }),
        [OP_SIGN_TRANSACTION, address_index, rest @ ..] => {
            let (addr_bytes, rest) =
                decode_length_prefixed(rest).map_err(|_| ErrorCode::MalformedRequest)?;
            let address = core::str::from_utf8(addr_bytes)
                .map_err(|_| ErrorCode::MalformedRequest)?
                .into();
            let (txn, rest) =
                decode_u16_length_prefixed(rest).ok_or(ErrorCode::MalformedRequest)?;
            if !rest.is_empty() {
                return Err(ErrorCode::MalformedRequest);
            }
            Ok(Request::SignTransaction {
                address_index: *address_index,
                address,
                txn: txn.to_vec(),
            })
        }
        [OP_IMPORT_ACCOUNT, tag, address_index, rest @ ..] => {
            let algorithm = KeyAlgorithm::from_tag(*tag).ok_or(ErrorCode::UnsupportedAlgorithm)?;
            let (mnemonic_bytes, rest) =
                decode_length_prefixed(rest).map_err(|_| ErrorCode::MalformedRequest)?;
            if !rest.is_empty() {
                return Err(ErrorCode::MalformedRequest);
            }
            let mnemonic = core::str::from_utf8(mnemonic_bytes)
                .map_err(|_| ErrorCode::MalformedRequest)?
                .into();
            Ok(Request::ImportAccount {
                algorithm,
                address_index: *address_index,
                mnemonic,
            })
        }
        _ => Err(ErrorCode::MalformedRequest),
    }
}

/// `[addr_len: u8][addr_len bytes of ASCII]`, with nothing trailing.
fn decode_address_payload(bytes: &[u8]) -> Result<String, ErrorCode> {
    let (addr_bytes, rest) =
        decode_length_prefixed(bytes).map_err(|_| ErrorCode::MalformedRequest)?;
    if !rest.is_empty() {
        return Err(ErrorCode::MalformedRequest);
    }
    core::str::from_utf8(addr_bytes)
        .map(Into::into)
        .map_err(|_| ErrorCode::MalformedRequest)
}

/// Encodes a request. Provided so tests (and any future non-Trusty client)
/// don't have to hand-roll the opcode bytes.
pub fn encode_request(request: Request) -> Vec<u8> {
    match request {
        Request::CreateAccount {
            algorithm,
            address_index,
        } => alloc::vec![OP_CREATE_ACCOUNT, algorithm as u8, address_index],
        Request::ListAccounts { start } => {
            let mut out = alloc::vec![OP_LIST_ACCOUNTS];
            out.extend_from_slice(&start.to_le_bytes());
            out
        }
        Request::RevealMnemonic { address } => {
            let mut out = alloc::vec![OP_REVEAL_MNEMONIC];
            let _ = encode_length_prefixed(&mut out, address.as_bytes());
            out
        }
        Request::ImportAccount {
            algorithm,
            address_index,
            mnemonic,
        } => {
            let mut out = alloc::vec![OP_IMPORT_ACCOUNT, algorithm as u8, address_index];
            let _ = encode_length_prefixed(&mut out, mnemonic.as_bytes());
            out
        }
        Request::DeleteAccount { address } => {
            let mut out = alloc::vec![OP_DELETE_ACCOUNT];
            let _ = encode_length_prefixed(&mut out, address.as_bytes());
            out
        }
        Request::SignTransaction {
            address_index,
            address,
            txn,
        } => {
            let mut out = alloc::vec![OP_SIGN_TRANSACTION, address_index];
            let _ = encode_length_prefixed(&mut out, address.as_bytes());
            let _ = encode_u16_length_prefixed(&mut out, &txn);
            out
        }
    }
}

fn encode_length_prefixed(out: &mut Vec<u8>, bytes: &[u8]) -> Result<(), ErrorCode> {
    let len: u8 = bytes
        .len()
        .try_into()
        .map_err(|_| ErrorCode::ResponseTooLarge)?;
    out.push(len);
    out.extend_from_slice(bytes);
    Ok(())
}

fn encode_u16_length_prefixed(out: &mut Vec<u8>, bytes: &[u8]) -> Result<(), ErrorCode> {
    let len: u16 = bytes
        .len()
        .try_into()
        .map_err(|_| ErrorCode::ResponseTooLarge)?;
    out.extend_from_slice(&len.to_le_bytes());
    out.extend_from_slice(bytes);
    Ok(())
}

/// `[len: u16 LE][len bytes]` -> `(bytes, rest)`.
fn decode_u16_length_prefixed(bytes: &[u8]) -> Option<(&[u8], &[u8])> {
    let (len_bytes, rest) = bytes.split_at_checked(2)?;
    let len = u16::from_le_bytes([len_bytes[0], len_bytes[1]]) as usize;
    rest.split_at_checked(len)
}

fn encode_wire_account(out: &mut Vec<u8>, account: &WireAccount) -> Result<(), ErrorCode> {
    out.push(account.algorithm as u8);
    out.extend_from_slice(&account.address_index.to_le_bytes());
    encode_length_prefixed(out, account.address.as_bytes())
}

/// Encodes a successful `CREATE_ACCOUNT`/`IMPORT_ACCOUNT`/`DELETE_ACCOUNT`
/// response -- all share the same `WireAccount` payload shape.
pub fn encode_create_account_response(
    algorithm: KeyAlgorithm,
    address_index: u32,
    address: &str,
) -> Vec<u8> {
    let mut out = alloc::vec![STATUS_OK];
    let account = WireAccount {
        algorithm,
        address_index,
        address: address.into(),
    };
    match encode_wire_account(&mut out, &account) {
        Ok(()) => out,
        Err(code) => encode_error_response(code),
    }
}

/// Encodes a successful `LIST_ACCOUNTS` response: `accounts` is the full
/// list, and the page holds as many entries from `start` on as fit in
/// [`MAX_RESPONSE_LEN`] (none if `start` is past the end).
pub fn encode_list_accounts_response(accounts: &[ListedAccount], start: u32) -> Vec<u8> {
    let mut out = alloc::vec![STATUS_OK];
    out.extend_from_slice(&(accounts.len() as u32).to_le_bytes());
    let count_at = out.len();
    out.extend_from_slice(&0u32.to_le_bytes());

    let mut count = 0u32;
    for listed in accounts.iter().skip(start as usize) {
        let mut entry = Vec::new();
        if let Err(code) = encode_wire_account(&mut entry, &listed.account)
            .and_then(|()| encode_u16_length_prefixed(&mut entry, &listed.public_key))
        {
            return encode_error_response(code);
        }
        if out.len() + entry.len() > MAX_RESPONSE_LEN {
            if count == 0 {
                // A single entry can never be this large in practice, but
                // never answer with an empty page the caller would loop on.
                return encode_error_response(ErrorCode::ResponseTooLarge);
            }
            break;
        }
        out.extend_from_slice(&entry);
        count += 1;
    }
    out[count_at..count_at + 4].copy_from_slice(&count.to_le_bytes());
    out
}

/// Encodes a successful `REVEAL_MNEMONIC` response.
pub fn encode_reveal_mnemonic_response(mnemonic: &str) -> Vec<u8> {
    let mut out = alloc::vec![STATUS_OK];
    match encode_length_prefixed(&mut out, mnemonic.as_bytes()) {
        Ok(()) => out,
        Err(code) => encode_error_response(code),
    }
}

/// Encodes a successful `SIGN_TRANSACTION` response.
pub fn encode_sign_transaction_response(algorithm: KeyAlgorithm, prefix: &[u8]) -> Vec<u8> {
    let mut out = alloc::vec![STATUS_OK, algorithm as u8];
    match encode_u16_length_prefixed(&mut out, prefix) {
        Ok(()) => out,
        Err(code) => encode_error_response(code),
    }
}

/// Encodes an error response.
pub fn encode_error_response(code: ErrorCode) -> Vec<u8> {
    alloc::vec![STATUS_ERR, code as u8]
}

fn parse_error_code(byte: Option<&u8>) -> Result<ErrorCode, DecodeError> {
    match byte {
        Some(0x01) => Ok(ErrorCode::MalformedRequest),
        Some(0x02) => Ok(ErrorCode::StoreFailure),
        Some(0x03) => Ok(ErrorCode::ResponseTooLarge),
        Some(0x04) => Ok(ErrorCode::UnsupportedAlgorithm),
        Some(0x05) => Ok(ErrorCode::AccountNotFound),
        Some(0x06) => Ok(ErrorCode::InvalidMnemonic),
        Some(0x07) => Ok(ErrorCode::AccountAlreadyExists),
        Some(0x08) => Ok(ErrorCode::InvalidTransaction),
        Some(0x09) => Ok(ErrorCode::SigningFailed),
        Some(0x0A) => Ok(ErrorCode::UnknownAddressIndex),
        Some(0x0B) => Ok(ErrorCode::SenderMismatch),
        Some(_) => Err(DecodeError::UnknownStatus),
        None => Err(DecodeError::Truncated),
    }
}

/// Decodes an error response's [`ErrorCode`], if `bytes` is one.
pub fn decode_error_code(bytes: &[u8]) -> Result<ErrorCode, DecodeError> {
    match bytes.split_first() {
        Some((&STATUS_ERR, rest)) => parse_error_code(rest.first()),
        Some((&STATUS_OK, _)) => Err(DecodeError::UnknownStatus),
        _ => Err(DecodeError::Truncated),
    }
}

/// Strips the leading status byte from a response expected to be `OK`.
/// Well-formed error responses surface as
/// `Err(DecodeError::ErrorResponse(code))` rather than being silently
/// treated as malformed.
fn decode_ok_payload(bytes: &[u8]) -> Result<&[u8], DecodeError> {
    match bytes.split_first() {
        Some((&STATUS_OK, rest)) => Ok(rest),
        Some((&STATUS_ERR, rest)) => {
            Err(DecodeError::ErrorResponse(parse_error_code(rest.first())?))
        }
        _ => Err(DecodeError::Truncated),
    }
}

fn decode_length_prefixed(bytes: &[u8]) -> Result<(&[u8], &[u8]), DecodeError> {
    let (&len, rest) = bytes.split_first().ok_or(DecodeError::Truncated)?;
    let len = len as usize;
    if rest.len() < len {
        return Err(DecodeError::Truncated);
    }
    Ok(rest.split_at(len))
}

fn decode_one_account(bytes: &[u8]) -> Result<(WireAccount, &[u8]), DecodeError> {
    let (&tag, rest) = bytes.split_first().ok_or(DecodeError::Truncated)?;
    let algorithm = KeyAlgorithm::from_tag(tag).ok_or(DecodeError::Malformed)?;
    let (index_bytes, rest) = rest.split_at_checked(4).ok_or(DecodeError::Truncated)?;
    let address_index = u32::from_le_bytes(index_bytes.try_into().unwrap());
    let (addr_bytes, rest) = decode_length_prefixed(rest)?;
    let address = core::str::from_utf8(addr_bytes)
        .map_err(|_| DecodeError::Malformed)?
        .into();
    Ok((
        WireAccount {
            algorithm,
            address_index,
            address,
        },
        rest,
    ))
}

/// Decodes a successful `CREATE_ACCOUNT`/`IMPORT_ACCOUNT`/`DELETE_ACCOUNT`
/// response.
pub fn decode_create_account_response(bytes: &[u8]) -> Result<WireAccount, DecodeError> {
    let rest = decode_ok_payload(bytes)?;
    let (account, rest) = decode_one_account(rest)?;
    if !rest.is_empty() {
        return Err(DecodeError::Truncated);
    }
    Ok(account)
}

/// Decodes one page of a successful `LIST_ACCOUNTS` response.
pub fn decode_list_accounts_response(bytes: &[u8]) -> Result<ListPage, DecodeError> {
    let rest = decode_ok_payload(bytes)?;
    let (total_bytes, rest) = rest.split_at_checked(4).ok_or(DecodeError::Truncated)?;
    let total = u32::from_le_bytes(total_bytes.try_into().unwrap());
    let (count_bytes, mut rest) = rest.split_at_checked(4).ok_or(DecodeError::Truncated)?;
    let count = u32::from_le_bytes(count_bytes.try_into().unwrap());

    // `count` is untrusted: don't pre-allocate from it.
    let mut accounts = Vec::new();
    for _ in 0..count {
        let (account, remaining) = decode_one_account(rest)?;
        let (public_key, remaining) =
            decode_u16_length_prefixed(remaining).ok_or(DecodeError::Truncated)?;
        accounts.push(ListedAccount {
            account,
            public_key: public_key.to_vec(),
        });
        rest = remaining;
    }
    if !rest.is_empty() {
        return Err(DecodeError::Truncated);
    }
    Ok(ListPage { total, accounts })
}

/// Decodes a successful `REVEAL_MNEMONIC` response into the mnemonic
/// string.
pub fn decode_reveal_mnemonic_response(bytes: &[u8]) -> Result<String, DecodeError> {
    let rest = decode_ok_payload(bytes)?;
    let (mnemonic_bytes, rest) = decode_length_prefixed(rest)?;
    if !rest.is_empty() {
        return Err(DecodeError::Truncated);
    }
    core::str::from_utf8(mnemonic_bytes)
        .map(Into::into)
        .map_err(|_| DecodeError::Malformed)
}

/// Decodes a successful `SIGN_TRANSACTION` response into the signing
/// algorithm and the `SignedTxn` prefix (append the txn bytes to it).
pub fn decode_sign_transaction_response(
    bytes: &[u8],
) -> Result<(KeyAlgorithm, Vec<u8>), DecodeError> {
    let rest = decode_ok_payload(bytes)?;
    let (&tag, rest) = rest.split_first().ok_or(DecodeError::Truncated)?;
    let algorithm = KeyAlgorithm::from_tag(tag).ok_or(DecodeError::Malformed)?;
    let (prefix, rest) = decode_u16_length_prefixed(rest).ok_or(DecodeError::Truncated)?;
    if !rest.is_empty() {
        return Err(DecodeError::Truncated);
    }
    Ok((algorithm, prefix.to_vec()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_round_trips() {
        for req in [
            Request::CreateAccount {
                algorithm: KeyAlgorithm::Algo25Ed25519,
                address_index: 0,
            },
            Request::ListAccounts { start: 0 },
            Request::ListAccounts { start: 70_000 },
            Request::RevealMnemonic {
                address: "A".repeat(58),
            },
            Request::ImportAccount {
                algorithm: KeyAlgorithm::Algo25Ed25519,
                address_index: 1,
                mnemonic: "abandon ability able".into(),
            },
            Request::DeleteAccount {
                address: "B".repeat(58),
            },
            Request::SignTransaction {
                address_index: 1,
                address: "C".repeat(58),
                txn: alloc::vec![0x81; 3000],
            },
        ] {
            let bytes = encode_request(req.clone());
            assert_eq!(decode_request(&bytes), Ok(req));
        }
    }

    #[test]
    fn unknown_opcode_is_malformed() {
        assert_eq!(decode_request(&[0xFF]), Err(ErrorCode::MalformedRequest));
        assert_eq!(decode_request(&[]), Err(ErrorCode::MalformedRequest));
        assert_eq!(
            decode_request(&[OP_LIST_ACCOUNTS, 0x00]),
            Err(ErrorCode::MalformedRequest)
        );
    }

    #[test]
    fn unrecognized_algorithm_tag_in_request_is_unsupported() {
        assert_eq!(
            decode_request(&[OP_CREATE_ACCOUNT, 0xFF, 0]),
            Err(ErrorCode::UnsupportedAlgorithm)
        );
    }

    #[test]
    fn truncated_reveal_mnemonic_request_is_malformed() {
        assert_eq!(
            decode_request(&[OP_REVEAL_MNEMONIC]),
            Err(ErrorCode::MalformedRequest)
        );
        assert_eq!(
            decode_request(&[OP_REVEAL_MNEMONIC, 5, b'a', b'b']),
            Err(ErrorCode::MalformedRequest)
        );
    }

    #[test]
    fn truncated_delete_account_request_is_malformed() {
        assert_eq!(
            decode_request(&[OP_DELETE_ACCOUNT]),
            Err(ErrorCode::MalformedRequest)
        );
        assert_eq!(
            decode_request(&[OP_DELETE_ACCOUNT, 5, b'a', b'b']),
            Err(ErrorCode::MalformedRequest)
        );
        assert_eq!(
            decode_request(&[OP_DELETE_ACCOUNT, 1, b'a', b'b']),
            Err(ErrorCode::MalformedRequest)
        );
    }

    #[test]
    fn truncated_sign_transaction_request_is_malformed() {
        for bytes in [
            &[OP_SIGN_TRANSACTION][..],
            &[OP_SIGN_TRANSACTION, 1, b'a'],
            &[OP_SIGN_TRANSACTION, 1, b'a', 3, 0, 0x81, 0x00],
            &[OP_SIGN_TRANSACTION, 1, b'a', 1, 0, 0x81, 0x00],
        ] {
            assert_eq!(decode_request(bytes), Err(ErrorCode::MalformedRequest));
        }
    }

    #[test]
    fn sign_transaction_response_round_trips() {
        let prefix = alloc::vec![0x82; 3300];
        let bytes = encode_sign_transaction_response(KeyAlgorithm::Falcon1024, &prefix);
        assert_eq!(
            decode_sign_transaction_response(&bytes),
            Ok((KeyAlgorithm::Falcon1024, prefix))
        );
        assert_eq!(
            decode_sign_transaction_response(&[STATUS_OK, 0x02, 5, 0, 1]),
            Err(DecodeError::Truncated)
        );
    }

    #[test]
    fn create_account_response_round_trips() {
        let address = "A".repeat(58);
        let bytes = encode_create_account_response(KeyAlgorithm::Algo25Ed25519, 0, &address);
        assert_eq!(
            decode_create_account_response(&bytes),
            Ok(WireAccount {
                algorithm: KeyAlgorithm::Algo25Ed25519,
                address_index: 0,
                address,
            })
        );
    }

    #[test]
    fn list_accounts_response_round_trips_including_empty() {
        let bytes = encode_list_accounts_response(&[], 0);
        assert_eq!(
            decode_list_accounts_response(&bytes),
            Ok(ListPage {
                total: 0,
                accounts: alloc::vec![],
            })
        );

        let accounts = alloc::vec![
            ListedAccount {
                account: WireAccount {
                    algorithm: KeyAlgorithm::Algo25Ed25519,
                    address_index: 0,
                    address: "A".repeat(58),
                },
                public_key: alloc::vec![1; 32],
            },
            ListedAccount {
                account: WireAccount {
                    algorithm: KeyAlgorithm::Falcon1024,
                    // Distinct address_index values must round-trip too, not
                    // just distinct addresses.
                    address_index: 1,
                    address: "B".repeat(58),
                },
                public_key: alloc::vec![2; 1793],
            },
        ];
        let bytes = encode_list_accounts_response(&accounts, 0);
        assert_eq!(
            decode_list_accounts_response(&bytes),
            Ok(ListPage { total: 2, accounts })
        );
    }

    #[test]
    fn list_accounts_pages_stay_under_the_message_limit() {
        let falcon = |i: u8| ListedAccount {
            account: WireAccount {
                algorithm: KeyAlgorithm::Falcon1024,
                address_index: 0,
                address: String::from(char::from(b'A' + i)).repeat(58),
            },
            public_key: alloc::vec![i; 1793],
        };
        let accounts: Vec<_> = (0..5).map(falcon).collect();

        let mut start = 0u32;
        let mut collected = Vec::new();
        loop {
            let bytes = encode_list_accounts_response(&accounts, start);
            assert!(bytes.len() <= MAX_RESPONSE_LEN);
            let page = decode_list_accounts_response(&bytes).unwrap();
            assert_eq!(page.total, 5);
            if page.accounts.is_empty() {
                break;
            }
            start += page.accounts.len() as u32;
            collected.extend(page.accounts);
        }
        assert_eq!(collected, accounts);
        // Two ~1.9 KB Falcon entries per page.
        assert_eq!(
            decode_list_accounts_response(&encode_list_accounts_response(&accounts, 0))
                .unwrap()
                .accounts
                .len(),
            2
        );
    }

    #[test]
    fn reveal_mnemonic_response_round_trips() {
        let mnemonic = String::from(
            "abandon ability able about above absent absorb abstract absurd abuse \
            access accident account accuse achieve acid acoustic acquire across act action actor \
            actress actual",
        );
        let bytes = encode_reveal_mnemonic_response(&mnemonic);
        assert_eq!(decode_reveal_mnemonic_response(&bytes), Ok(mnemonic));
    }

    #[test]
    fn truncated_import_account_request_is_malformed() {
        assert_eq!(
            decode_request(&[OP_IMPORT_ACCOUNT]),
            Err(ErrorCode::MalformedRequest)
        );
        assert_eq!(
            decode_request(&[OP_IMPORT_ACCOUNT, 0x01, 5, b'a', b'b']),
            Err(ErrorCode::MalformedRequest)
        );
    }

    #[test]
    fn unrecognized_algorithm_tag_in_import_request_is_unsupported() {
        assert_eq!(
            decode_request(&[OP_IMPORT_ACCOUNT, 0xFF, 0]),
            Err(ErrorCode::UnsupportedAlgorithm)
        );
    }

    #[test]
    fn error_response_round_trips() {
        for code in [
            ErrorCode::MalformedRequest,
            ErrorCode::StoreFailure,
            ErrorCode::ResponseTooLarge,
            ErrorCode::UnsupportedAlgorithm,
            ErrorCode::AccountNotFound,
            ErrorCode::InvalidMnemonic,
            ErrorCode::AccountAlreadyExists,
            ErrorCode::InvalidTransaction,
            ErrorCode::SigningFailed,
            ErrorCode::UnknownAddressIndex,
            ErrorCode::SenderMismatch,
        ] {
            let bytes = encode_error_response(code);
            assert_eq!(decode_error_code(&bytes), Ok(code));
        }
    }

    #[test]
    fn truncated_responses_are_rejected() {
        assert_eq!(
            decode_create_account_response(&[STATUS_OK]),
            Err(DecodeError::Truncated)
        );
        assert_eq!(
            decode_list_accounts_response(&[STATUS_OK, 0, 0, 0, 0, 0, 0]),
            Err(DecodeError::Truncated)
        );
        assert_eq!(
            decode_reveal_mnemonic_response(&[STATUS_OK]),
            Err(DecodeError::Truncated)
        );
    }
}
