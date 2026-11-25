//! Wire format for the WASM opcodes, carried on the same `seed_vault_ta`
//! port and in the same style as `seed_vault::protocol` (byte 0 = opcode;
//! responses `[0x00][payload]` or `[0x01][error code]`). The WASM opcodes
//! use `0x20..=0x2F`, so the TA can route on the first byte
//! ([`is_wasm_request`]) and a TA built without WASM answers them with
//! `seed_vault`'s `MalformedRequest`.
//!
//! ## Requests
//!
//! | Byte 0                  | Remaining bytes                                                           | `OK` payload                    |
//! |-------------------------|---------------------------------------------------------------------------|---------------------------------|
//! | `0x20` `LOAD_BEGIN`     | `[module_len: u32 LE]`                                                    | `[upload_id: u8]`               |
//! | `0x21` `LOAD_CHUNK`     | `[upload_id: u8][offset: u32 LE][chunk bytes]`                            | (none)                          |
//! | `0x22` `LOAD_FINISH`    | `[upload_id: u8]`, or `[upload_id: u8][signed manifest]`                  | `[program_id: 32]`              |
//! | `0x23` `UNLOAD`         | `[program_id: 32]`                                                        | (none)                          |
//! | `0x24` `LIST_PROGRAMS`  | (none)                                                                    | `[count: u8][program]*`, see below |
//! | `0x25` `PUBLIC_KEY`     | `[program_id: 32][address_index: u8][addr_len: u8][addr]`                 | `[len: u16 LE][public key]`     |
//! | `0x26` `SIGN`           | `[program_id: 32][address_index: u8][addr_len: u8][addr][len: u16 LE][message]` | `[len: u16 LE][signature]` |
//! | `0x27` `GET_VERSION`    | (none)                                                                    | `[major: u16 LE][minor: u16 LE]` |
//! | `0x28` `REVOKE`         | `[signed revocation list]`                                                | `[sequence: u32 LE]`            |
//! | `0x29` `RUN`            | `[program_id: 32][input]`                                                 | `[len: u16 LE][output]`         |
//! | `0x2A` `INPUT_BEGIN`    | `[input_len: u32 LE]`                                                     | `[input_id: u8]`                |
//! | `0x2B` `INPUT_CHUNK`    | `[input_id: u8][offset: u32 LE][chunk bytes]`                             | (none)                          |
//! | `0x2C` `RUN_INPUT`      | `[program_id: 32][input_id: u8]`                                          | `[len: u16 LE][output]`         |
//!
//! `RUN` calls a program's `run` export (general computation, not tied to a
//! seed; see [`crate::abi`]). An input too big for one message is uploaded
//! like a module -- `INPUT_BEGIN`, `INPUT_CHUNK`s in order -- then run with
//! `RUN_INPUT`.
//!
//! `[signed ...]` is [`crate::manifest::Signed`]'s encoding:
//! `[body_len: u16 LE][body][signature_len: u16 LE][signature]`, with the
//! body a [`crate::manifest::Manifest`] or [`crate::manifest::Revocations`].
//!
//! Each `LIST_PROGRAMS` entry:
//! `[program_id: 32][signed: u8][version: u32 LE][capabilities: u8][kinds: u8][name_len: u8][name]`
//! (`signed` = 1 for signed programs; unsigned ones report version 0 and
//! an empty name; capabilities bit 0 = account entropy, bit 1 = human-sign;
//! kinds bit 0 = signing (`PUBLIC_KEY`/`SIGN`), bit 1 = `RUN`). A signed
//! program's name says what it is (e.g. `light-client`), so a client can
//! find the programs it knows how to talk to.
//!
//! ## Signing
//!
//! A signed module sends its manifest with `LOAD_FINISH`; the TA checks it
//! against the publisher keys built into it (see [`crate::PublisherPolicy`])
//! and it must name this exact module. Signed versions of one algorithm
//! share keys, a new version replaces the old one, and a lower version than
//! one already installed is refused ([`ErrorCode::Rollback`]). `REVOKE`
//! applies a newer revocation list, removing the modules and keys it names.
//!
//! A module is uploaded once: `LOAD_BEGIN`, then `LOAD_CHUNK`s in order
//! (each starting where the last ended, as many bytes as fit in a message),
//! then `LOAD_FINISH`, which validates it and returns its `program_id` (the
//! SHA-256 of the module). After that, `PUBLIC_KEY`/`SIGN` can be called any
//! number of times with that `program_id`. Programs are saved to secure
//! storage, so they survive a TA restart: upload once, `UNLOAD` to remove.
//!
//! `addr` is the human-sign (index 0) address of the vault seed to use.
//! `address_index` picks which of the program's keys for that seed to use
//! (0 = human-sign, 1..=255 = autosign; see [`crate::WasmPolicy`]).

use alloc::string::String;
use alloc::vec::Vec;

use wasm_runtime::ProgramId;

use crate::manifest::Signed;
#[cfg(feature = "vault")]
use crate::vault::{ProgramInfo, WasmError};

const OP_LOAD_BEGIN: u8 = 0x20;
const OP_LOAD_CHUNK: u8 = 0x21;
const OP_LOAD_FINISH: u8 = 0x22;
const OP_UNLOAD: u8 = 0x23;
const OP_LIST_PROGRAMS: u8 = 0x24;
const OP_PUBLIC_KEY: u8 = 0x25;
const OP_SIGN: u8 = 0x26;
const OP_GET_VERSION: u8 = 0x27;
const OP_REVOKE: u8 = 0x28;
const OP_RUN: u8 = 0x29;
const OP_INPUT_BEGIN: u8 = 0x2A;
const OP_INPUT_CHUNK: u8 = 0x2B;
const OP_RUN_INPUT: u8 = 0x2C;

/// WASM wire protocol major version (bumped on breaking changes).
pub const PROTOCOL_VERSION_MAJOR: u16 = 0;
/// WASM wire protocol minor version (bumped on additive changes).
pub const PROTOCOL_VERSION_MINOR: u16 = 4;

const STATUS_OK: u8 = 0x00;
const STATUS_ERR: u8 = 0x01;

/// Whether `request` is one of this module's opcodes.
pub fn is_wasm_request(request: &[u8]) -> bool {
    matches!(request.first(), Some(0x20..=0x2F))
}

/// Error codes for the WASM opcodes (their own space, separate from
/// `seed_vault::protocol::ErrorCode`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ErrorCode {
    MalformedRequest = 0x01,
    ProgramTooLarge = 0x02,
    TooManyPrograms = 0x03,
    UnknownUpload = 0x04,
    BadChunk = 0x05,
    InvalidModule = 0x06,
    InstallDenied = 0x07,
    UnknownProgram = 0x08,
    /// The seed address isn't in the vault, or isn't a seed's index-0
    /// address.
    AccountNotFound = 0x09,
    KeyUseDenied = 0x0A,
    ProgramFailed = 0x0B,
    OutOfFuel = 0x0C,
    StoreFailure = 0x0D,
    /// Bad or unknown-key signature, or a manifest for another module.
    BadSignature = 0x0E,
    /// Older than what's already installed or applied.
    Rollback = 0x0F,
    /// The module or its signing key was revoked.
    Revoked = 0x10,
    /// A `run` input bigger than the vault allows.
    InputTooLarge = 0x11,
}

impl ErrorCode {
    pub fn from_byte(byte: u8) -> Option<Self> {
        use ErrorCode::*;
        [
            MalformedRequest,
            ProgramTooLarge,
            TooManyPrograms,
            UnknownUpload,
            BadChunk,
            InvalidModule,
            InstallDenied,
            UnknownProgram,
            AccountNotFound,
            KeyUseDenied,
            ProgramFailed,
            OutOfFuel,
            StoreFailure,
            BadSignature,
            Rollback,
            Revoked,
            InputTooLarge,
        ]
        .into_iter()
        .find(|code| *code as u8 == byte)
    }
}

#[cfg(feature = "vault")]
impl From<WasmError> for ErrorCode {
    fn from(error: WasmError) -> Self {
        match error {
            WasmError::ProgramTooLarge => ErrorCode::ProgramTooLarge,
            WasmError::TooManyPrograms => ErrorCode::TooManyPrograms,
            WasmError::UnknownUpload => ErrorCode::UnknownUpload,
            WasmError::BadChunk => ErrorCode::BadChunk,
            WasmError::InvalidModule => ErrorCode::InvalidModule,
            WasmError::InstallDenied => ErrorCode::InstallDenied,
            WasmError::UnknownProgram => ErrorCode::UnknownProgram,
            WasmError::KeyUseDenied => ErrorCode::KeyUseDenied,
            WasmError::ProgramFailed => ErrorCode::ProgramFailed,
            WasmError::OutOfFuel => ErrorCode::OutOfFuel,
            WasmError::BadSignature => ErrorCode::BadSignature,
            WasmError::Rollback => ErrorCode::Rollback,
            WasmError::Revoked => ErrorCode::Revoked,
            WasmError::InputTooLarge => ErrorCode::InputTooLarge,
            WasmError::Store(
                seed_vault::StoreError::NotFound | seed_vault::StoreError::UnknownKeyIndex,
            ) => ErrorCode::AccountNotFound,
            WasmError::Store(_) => ErrorCode::StoreFailure,
        }
    }
}

/// A decoded request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Request<'a> {
    LoadBegin {
        module_len: u32,
    },
    LoadChunk {
        upload_id: u8,
        offset: u32,
        chunk: &'a [u8],
    },
    LoadFinish {
        upload_id: u8,
        manifest: Option<Signed>,
    },
    Unload {
        program: ProgramId,
    },
    ListPrograms,
    PublicKey {
        program: ProgramId,
        address_index: u8,
        address: String,
    },
    Sign {
        program: ProgramId,
        address_index: u8,
        address: String,
        message: &'a [u8],
    },
    GetVersion,
    Revoke {
        revocations: Signed,
    },
    Run {
        program: ProgramId,
        input: &'a [u8],
    },
    InputBegin {
        input_len: u32,
    },
    InputChunk {
        input_id: u8,
        offset: u32,
        chunk: &'a [u8],
    },
    RunInput {
        program: ProgramId,
        input_id: u8,
    },
}

/// Decodes a request whose first byte is a WASM opcode.
pub fn decode_request(bytes: &[u8]) -> Result<Request<'_>, ErrorCode> {
    let (&op, rest) = bytes.split_first().ok_or(ErrorCode::MalformedRequest)?;
    let mut r = Reader(rest);
    let request = match op {
        OP_LOAD_BEGIN => Request::LoadBegin {
            module_len: r.u32()?,
        },
        OP_LOAD_CHUNK => Request::LoadChunk {
            upload_id: r.u8()?,
            offset: r.u32()?,
            chunk: r.rest(),
        },
        OP_LOAD_FINISH => {
            let upload_id = r.u8()?;
            let rest = r.rest();
            let manifest = if rest.is_empty() {
                None
            } else {
                Some(Signed::decode(rest).ok_or(ErrorCode::MalformedRequest)?)
            };
            Request::LoadFinish {
                upload_id,
                manifest,
            }
        }
        OP_RUN => Request::Run {
            program: r.program()?,
            input: r.rest(),
        },
        OP_INPUT_BEGIN => Request::InputBegin {
            input_len: r.u32()?,
        },
        OP_INPUT_CHUNK => Request::InputChunk {
            input_id: r.u8()?,
            offset: r.u32()?,
            chunk: r.rest(),
        },
        OP_RUN_INPUT => Request::RunInput {
            program: r.program()?,
            input_id: r.u8()?,
        },
        OP_REVOKE => Request::Revoke {
            revocations: Signed::decode(r.rest()).ok_or(ErrorCode::MalformedRequest)?,
        },
        OP_UNLOAD => Request::Unload {
            program: r.program()?,
        },
        OP_LIST_PROGRAMS => Request::ListPrograms,
        OP_PUBLIC_KEY => Request::PublicKey {
            program: r.program()?,
            address_index: r.u8()?,
            address: r.address()?,
        },
        OP_SIGN => {
            let program = r.program()?;
            let address_index = r.u8()?;
            let address = r.address()?;
            let len = r.u16()? as usize;
            Request::Sign {
                program,
                address_index,
                address,
                message: r.take(len)?,
            }
        }
        OP_GET_VERSION => Request::GetVersion,
        _ => return Err(ErrorCode::MalformedRequest),
    };
    r.end()?;
    Ok(request)
}

struct Reader<'a>(&'a [u8]);

impl<'a> Reader<'a> {
    fn take(&mut self, n: usize) -> Result<&'a [u8], ErrorCode> {
        if self.0.len() < n {
            return Err(ErrorCode::MalformedRequest);
        }
        let (head, tail) = self.0.split_at(n);
        self.0 = tail;
        Ok(head)
    }
    fn u8(&mut self) -> Result<u8, ErrorCode> {
        Ok(self.take(1)?[0])
    }
    fn u16(&mut self) -> Result<u16, ErrorCode> {
        Ok(u16::from_le_bytes(self.take(2)?.try_into().unwrap()))
    }
    fn u32(&mut self) -> Result<u32, ErrorCode> {
        Ok(u32::from_le_bytes(self.take(4)?.try_into().unwrap()))
    }
    fn program(&mut self) -> Result<ProgramId, ErrorCode> {
        Ok(ProgramId(self.take(32)?.try_into().unwrap()))
    }
    fn address(&mut self) -> Result<String, ErrorCode> {
        let len = self.u8()? as usize;
        let bytes = self.take(len)?;
        core::str::from_utf8(bytes)
            .map(String::from)
            .map_err(|_| ErrorCode::MalformedRequest)
    }
    fn rest(&mut self) -> &'a [u8] {
        core::mem::take(&mut self.0)
    }
    fn end(&self) -> Result<(), ErrorCode> {
        if self.0.is_empty() {
            Ok(())
        } else {
            Err(ErrorCode::MalformedRequest)
        }
    }
}

pub fn encode_error_response(code: ErrorCode) -> Vec<u8> {
    alloc::vec![STATUS_ERR, code as u8]
}

pub fn encode_ok(payload: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(1 + payload.len());
    out.push(STATUS_OK);
    out.extend_from_slice(payload);
    out
}

/// `[OK][len: u16 LE][bytes]`, for public keys and signatures. `bytes` is
/// at most [`crate::vault::MAX_OUTPUT_LEN`], so `len` always fits.
pub fn encode_bytes_response(bytes: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(3 + bytes.len());
    out.push(STATUS_OK);
    out.extend_from_slice(&(bytes.len() as u16).to_le_bytes());
    out.extend_from_slice(bytes);
    out
}

#[cfg(feature = "vault")]
pub fn encode_list_programs_response(programs: &[ProgramInfo]) -> Vec<u8> {
    let mut out = alloc::vec![STATUS_OK, programs.len() as u8];
    for program in programs {
        out.extend_from_slice(&program.id.0);
        let (version, name) = match &program.manifest {
            Some(manifest) => (manifest.version, manifest.name.as_str()),
            None => (0, ""),
        };
        out.push(u8::from(program.manifest.is_some()));
        out.extend_from_slice(&version.to_le_bytes());
        out.push(
            u8::from(program.capabilities.account_entropy)
                | (u8::from(program.capabilities.human_sign) << 1),
        );
        out.push(u8::from(program.can_sign) | (u8::from(program.can_run) << 1));
        out.push(name.len() as u8);
        out.extend_from_slice(name.as_bytes());
    }
    out
}

/// Splits a response into its `OK` payload or error code (client side).
pub fn decode_response(response: &[u8]) -> Result<&[u8], ResponseError> {
    match response {
        [STATUS_OK, payload @ ..] => Ok(payload),
        [STATUS_ERR, code] => {
            Err(ErrorCode::from_byte(*code).map_or(ResponseError::Malformed, ResponseError::Code))
        }
        _ => Err(ResponseError::Malformed),
    }
}

/// Decodes a `[len: u16 LE][bytes]` payload (`PUBLIC_KEY`, `SIGN`, `RUN`).
pub fn decode_bytes_payload(payload: &[u8]) -> Result<Vec<u8>, ResponseError> {
    let (len, bytes) = payload
        .split_first_chunk::<2>()
        .ok_or(ResponseError::Malformed)?;
    if usize::from(u16::from_le_bytes(*len)) != bytes.len() {
        return Err(ResponseError::Malformed);
    }
    Ok(bytes.to_vec())
}

/// Why a response couldn't be used (client side).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponseError {
    /// The TA answered with this error.
    Code(ErrorCode),
    /// The response didn't parse.
    Malformed,
}

/// One decoded `LIST_PROGRAMS` entry (client side).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListedProgram {
    pub id: ProgramId,
    pub signed: bool,
    pub version: u32,
    pub account_entropy: bool,
    pub human_sign: bool,
    /// Usable with `PUBLIC_KEY`/`SIGN`.
    pub can_sign: bool,
    /// Usable with `RUN`.
    pub can_run: bool,
    pub name: String,
}

/// Decodes a `LIST_PROGRAMS` `OK` payload (client side).
pub fn decode_list_programs(payload: &[u8]) -> Option<Vec<ListedProgram>> {
    let mut r = Reader(payload);
    let count = r.u8().ok()?;
    let programs = (0..count)
        .map(|_| {
            let id = ProgramId(r.take(32).ok()?.try_into().ok()?);
            let signed = r.u8().ok()? == 1;
            let version = r.u32().ok()?;
            let capabilities = r.u8().ok()?;
            let kinds = r.u8().ok()?;
            let name = r.address().ok()?;
            Some(ListedProgram {
                id,
                signed,
                version,
                account_entropy: capabilities & 1 != 0,
                human_sign: capabilities & 2 != 0,
                can_sign: kinds & 1 != 0,
                can_run: kinds & 2 != 0,
                name,
            })
        })
        .collect::<Option<Vec<_>>>()?;
    r.end().ok()?;
    Some(programs)
}

pub fn encode_get_version_response() -> Vec<u8> {
    let mut out = alloc::vec![STATUS_OK];
    out.extend_from_slice(&PROTOCOL_VERSION_MAJOR.to_le_bytes());
    out.extend_from_slice(&PROTOCOL_VERSION_MINOR.to_le_bytes());
    out
}

/// Client-side request builders (used by tests and as a reference for the
/// Android client).
pub mod request {
    use super::*;

    pub fn load_begin(module_len: u32) -> Vec<u8> {
        [&[OP_LOAD_BEGIN][..], &module_len.to_le_bytes()].concat()
    }

    pub fn load_chunk(upload_id: u8, offset: u32, chunk: &[u8]) -> Vec<u8> {
        [
            &[OP_LOAD_CHUNK, upload_id][..],
            &offset.to_le_bytes(),
            chunk,
        ]
        .concat()
    }

    pub fn load_finish(upload_id: u8) -> Vec<u8> {
        alloc::vec![OP_LOAD_FINISH, upload_id]
    }

    pub fn load_finish_signed(upload_id: u8, manifest: &Signed) -> Vec<u8> {
        [&[OP_LOAD_FINISH, upload_id][..], &manifest.encode()].concat()
    }

    pub fn revoke(revocations: &Signed) -> Vec<u8> {
        [&[OP_REVOKE][..], &revocations.encode()].concat()
    }

    pub fn run(program: &ProgramId, input: &[u8]) -> Vec<u8> {
        [&[OP_RUN][..], &program.0, input].concat()
    }

    pub fn input_begin(input_len: u32) -> Vec<u8> {
        [&[OP_INPUT_BEGIN][..], &input_len.to_le_bytes()].concat()
    }

    pub fn input_chunk(input_id: u8, offset: u32, chunk: &[u8]) -> Vec<u8> {
        [
            &[OP_INPUT_CHUNK, input_id][..],
            &offset.to_le_bytes(),
            chunk,
        ]
        .concat()
    }

    pub fn run_input(program: &ProgramId, input_id: u8) -> Vec<u8> {
        [&[OP_RUN_INPUT][..], &program.0, &[input_id]].concat()
    }

    pub fn unload(program: &ProgramId) -> Vec<u8> {
        [&[OP_UNLOAD][..], &program.0].concat()
    }

    pub fn list_programs() -> Vec<u8> {
        alloc::vec![OP_LIST_PROGRAMS]
    }

    fn key_request(op: u8, program: &ProgramId, address_index: u8, address: &str) -> Vec<u8> {
        [
            &[op][..],
            &program.0,
            &[address_index, address.len() as u8],
            address.as_bytes(),
        ]
        .concat()
    }

    pub fn public_key(program: &ProgramId, address_index: u8, address: &str) -> Vec<u8> {
        key_request(OP_PUBLIC_KEY, program, address_index, address)
    }

    pub fn sign(program: &ProgramId, address_index: u8, address: &str, message: &[u8]) -> Vec<u8> {
        let mut out = key_request(OP_SIGN, program, address_index, address);
        out.extend_from_slice(&(message.len() as u16).to_le_bytes());
        out.extend_from_slice(message);
        out
    }

    pub fn get_version() -> Vec<u8> {
        alloc::vec![OP_GET_VERSION]
    }

    /// Largest `LOAD_CHUNK` payload that fits in a `max_msg_len`-byte message.
    pub const fn max_chunk_len(max_msg_len: usize) -> usize {
        max_msg_len - 6
    }
}
