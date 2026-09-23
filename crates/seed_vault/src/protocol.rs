//! Wire format for the `seed_vault_ta` IPC protocol.
//!
//! A tiny hand-rolled binary format (no serde/protobuf), driven identically
//! by the real `tipc` transport and this crate's own host-side tests.
//!
//! ## Request
//!
//! | Byte 0                     | Remaining bytes                                                    |
//! |-----------------------------|---------------------------------------------------------------------|
//! | `0x01` = `CREATE_ACCOUNT`  | 1 byte: [`KeyAlgorithm`] tag                                        |
//! | `0x02` = `LIST_ACCOUNTS`   | (none)                                                              |
//! | `0x03` = `REVEAL_MNEMONIC` | `[addr_len: u8][addr_len bytes of ASCII address]`                   |
//! | `0x04` = `IMPORT_ACCOUNT`  | `[algorithm: u8][mnemonic_len: u8][mnemonic_len bytes of ASCII]`    |
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
//! `CREATE_ACCOUNT`/`IMPORT_ACCOUNT` success payload:
//! `[algorithm: u8][address_index: u32 LE][addr_len: u8][addr_len bytes of ASCII Algorand address]`.
//!
//! `LIST_ACCOUNTS` success payload: `[count: u32 LE]`, followed by `count`
//! repetitions of the same shape above.
//!
//! `address_index` is the position of that address *within its parent
//! seed* (see [`crate::store::AccountSummary::address_index`]) -- always
//! `0` today, carried on the wire from day one so a future
//! multi-address-per-seed algorithm doesn't need a protocol version bump.
//!
//! `REVEAL_MNEMONIC` success payload: `[mnemonic_len: u8][mnemonic_len bytes of ASCII, space-separated words]`.
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

const STATUS_OK: u8 = 0x00;
const STATUS_ERR: u8 = 0x01;

/// One decoded [`Request::CreateAccount`]/[`Request::ListAccounts`] entry's
/// public identity, as carried in a response. Mirrors
/// [`crate::store::AccountSummary`] byte-for-byte.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WireAccount {
    pub algorithm: KeyAlgorithm,
    pub address_index: u32,
    pub address: String,
}

/// A decoded incoming request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Request {
    CreateAccount {
        algorithm: KeyAlgorithm,
    },
    ListAccounts,
    RevealMnemonic {
        address: String,
    },
    ImportAccount {
        algorithm: KeyAlgorithm,
        mnemonic: String,
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
    /// `REVEAL_MNEMONIC` targeted an address with no corresponding account.
    AccountNotFound = 0x05,
    /// `IMPORT_ACCOUNT`'s mnemonic failed checksum/wordlist validation.
    InvalidMnemonic = 0x06,
    /// `IMPORT_ACCOUNT`'s mnemonic matches an account that already exists.
    AccountAlreadyExists = 0x07,
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
        [OP_CREATE_ACCOUNT, tag] => {
            let algorithm = KeyAlgorithm::from_tag(*tag).ok_or(ErrorCode::UnsupportedAlgorithm)?;
            Ok(Request::CreateAccount { algorithm })
        }
        [OP_LIST_ACCOUNTS] => Ok(Request::ListAccounts),
        [OP_REVEAL_MNEMONIC, rest @ ..] => {
            let (addr_bytes, rest) =
                decode_length_prefixed(rest).map_err(|_| ErrorCode::MalformedRequest)?;
            if !rest.is_empty() {
                return Err(ErrorCode::MalformedRequest);
            }
            let address = core::str::from_utf8(addr_bytes)
                .map_err(|_| ErrorCode::MalformedRequest)?
                .into();
            Ok(Request::RevealMnemonic { address })
        }
        [OP_IMPORT_ACCOUNT, tag, rest @ ..] => {
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
                mnemonic,
            })
        }
        _ => Err(ErrorCode::MalformedRequest),
    }
}

/// Encodes a request. Provided so tests (and any future non-Trusty client)
/// don't have to hand-roll the opcode bytes.
pub fn encode_request(request: Request) -> Vec<u8> {
    match request {
        Request::CreateAccount { algorithm } => {
            alloc::vec![OP_CREATE_ACCOUNT, algorithm as u8]
        }
        Request::ListAccounts => alloc::vec![OP_LIST_ACCOUNTS],
        Request::RevealMnemonic { address } => {
            let mut out = alloc::vec![OP_REVEAL_MNEMONIC];
            let _ = encode_length_prefixed(&mut out, address.as_bytes());
            out
        }
        Request::ImportAccount {
            algorithm,
            mnemonic,
        } => {
            let mut out = alloc::vec![OP_IMPORT_ACCOUNT, algorithm as u8];
            let _ = encode_length_prefixed(&mut out, mnemonic.as_bytes());
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

fn encode_wire_account(out: &mut Vec<u8>, account: &WireAccount) -> Result<(), ErrorCode> {
    out.push(account.algorithm as u8);
    out.extend_from_slice(&account.address_index.to_le_bytes());
    encode_length_prefixed(out, account.address.as_bytes())
}

/// Encodes a successful `CREATE_ACCOUNT`/`IMPORT_ACCOUNT` response -- both
/// share the same `WireAccount` payload shape.
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

/// Encodes a successful `LIST_ACCOUNTS` response.
pub fn encode_list_accounts_response(accounts: &[WireAccount]) -> Vec<u8> {
    let mut out = alloc::vec![STATUS_OK];
    out.extend_from_slice(&(accounts.len() as u32).to_le_bytes());
    for account in accounts {
        if let Err(code) = encode_wire_account(&mut out, account) {
            return encode_error_response(code);
        }
    }
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

/// Decodes a successful `CREATE_ACCOUNT`/`IMPORT_ACCOUNT` response.
pub fn decode_create_account_response(bytes: &[u8]) -> Result<WireAccount, DecodeError> {
    let rest = decode_ok_payload(bytes)?;
    let (account, rest) = decode_one_account(rest)?;
    if !rest.is_empty() {
        return Err(DecodeError::Truncated);
    }
    Ok(account)
}

/// Decodes a successful `LIST_ACCOUNTS` response.
pub fn decode_list_accounts_response(bytes: &[u8]) -> Result<Vec<WireAccount>, DecodeError> {
    let rest = decode_ok_payload(bytes)?;
    let (count_bytes, mut rest) = rest.split_at_checked(4).ok_or(DecodeError::Truncated)?;
    let count = u32::from_le_bytes(count_bytes.try_into().unwrap());

    let mut accounts = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let (account, remaining) = decode_one_account(rest)?;
        accounts.push(account);
        rest = remaining;
    }
    if !rest.is_empty() {
        return Err(DecodeError::Truncated);
    }
    Ok(accounts)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_round_trips() {
        for req in [
            Request::CreateAccount {
                algorithm: KeyAlgorithm::Algo25Ed25519,
            },
            Request::ListAccounts,
            Request::RevealMnemonic {
                address: "A".repeat(58),
            },
            Request::ImportAccount {
                algorithm: KeyAlgorithm::Algo25Ed25519,
                mnemonic: "abandon ability able".into(),
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
            decode_request(&[OP_CREATE_ACCOUNT, 0xFF]),
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
        let accounts: Vec<WireAccount> = alloc::vec![];
        let bytes = encode_list_accounts_response(&accounts);
        assert_eq!(decode_list_accounts_response(&bytes), Ok(accounts));

        let accounts = alloc::vec![
            WireAccount {
                algorithm: KeyAlgorithm::Algo25Ed25519,
                address_index: 0,
                address: "A".repeat(58),
            },
            WireAccount {
                algorithm: KeyAlgorithm::Algo25Ed25519,
                // Distinct address_index values must round-trip too, not
                // just distinct addresses.
                address_index: 1,
                address: "B".repeat(58),
            },
        ];
        let bytes = encode_list_accounts_response(&accounts);
        assert_eq!(decode_list_accounts_response(&bytes), Ok(accounts));
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
            decode_list_accounts_response(&[STATUS_OK, 0, 0]),
            Err(DecodeError::Truncated)
        );
        assert_eq!(
            decode_reveal_mnemonic_response(&[STATUS_OK]),
            Err(DecodeError::Truncated)
        );
    }
}
