//! Client side of the WASM opcodes ([`crate::protocol`]): builds requests,
//! splits uploads into messages, decodes responses.
//!
//! Transport-agnostic: [`Client`] takes a function that sends one request
//! and returns the raw response. The Android FFI (`seed_vault_ffi`) passes
//! one over a Trusty IPC connection; tests pass one over
//! [`crate::service::handle_request`].

use alloc::vec::Vec;

use wasm_runtime::ProgramId;

use crate::manifest::Signed;
use crate::protocol::{
    self, decode_bytes_payload, decode_response, request, ErrorCode, ListedProgram, ResponseError,
};

/// Largest request `seed_vault_ta` accepts (one Trusty IPC message).
pub const MAX_MESSAGE_LEN: usize = 4096;

/// Why a [`Client`] call failed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClientError<E> {
    /// Sending or receiving failed.
    Transport(E),
    /// The TA answered with an error.
    Applet(ErrorCode),
    /// The TA's response didn't parse.
    MalformedResponse,
    /// The request can't be sent as given (e.g. too large).
    InvalidArgument(&'static str),
}

impl<E> From<ResponseError> for ClientError<E> {
    fn from(error: ResponseError) -> Self {
        match error {
            ResponseError::Code(code) => ClientError::Applet(code),
            ResponseError::Malformed => ClientError::MalformedResponse,
        }
    }
}

/// Talks to `seed_vault_ta`'s WASM opcodes through `exchange`.
pub struct Client<F> {
    exchange: F,
}

impl<F, E> Client<F>
where
    F: FnMut(&[u8]) -> Result<Vec<u8>, E>,
{
    pub fn new(exchange: F) -> Self {
        Client { exchange }
    }

    /// One request; returns the `OK` payload.
    fn call(&mut self, request: &[u8]) -> Result<Vec<u8>, ClientError<E>> {
        if request.len() > MAX_MESSAGE_LEN {
            return Err(ClientError::InvalidArgument(
                "request too large for one message",
            ));
        }
        let response = (self.exchange)(request).map_err(ClientError::Transport)?;
        Ok(decode_response(&response)?.to_vec())
    }

    /// The TA's WASM protocol version.
    pub fn version(&mut self) -> Result<(u16, u16), ClientError<E>> {
        let payload = self.call(&request::get_version())?;
        match payload.as_slice() {
            [a, b, c, d] => Ok((u16::from_le_bytes([*a, *b]), u16::from_le_bytes([*c, *d]))),
            _ => Err(ClientError::MalformedResponse),
        }
    }

    /// Uploads and installs `module` (split into as many messages as it
    /// takes), with its signed manifest if it has one. Returns its ID.
    pub fn install(
        &mut self,
        module: &[u8],
        manifest: Option<&Signed>,
    ) -> Result<ProgramId, ClientError<E>> {
        let len = u32::try_from(module.len())
            .map_err(|_| ClientError::InvalidArgument("module too large"))?;
        let upload_id = self.upload_id(&request::load_begin(len))?;
        let mut offset = 0u32;
        for chunk in module.chunks(request::max_chunk_len(MAX_MESSAGE_LEN)) {
            self.call(&request::load_chunk(upload_id, offset, chunk))?;
            offset += chunk.len() as u32;
        }
        let finish = match manifest {
            Some(manifest) => request::load_finish_signed(upload_id, manifest),
            None => request::load_finish(upload_id),
        };
        program_id(&self.call(&finish)?)
    }

    /// Removes an installed program (with its cached keys and state).
    pub fn unload(&mut self, program: &ProgramId) -> Result<(), ClientError<E>> {
        self.call(&request::unload(program)).map(drop)
    }

    /// The installed programs.
    pub fn list_programs(&mut self) -> Result<Vec<ListedProgram>, ClientError<E>> {
        let payload = self.call(&request::list_programs())?;
        protocol::decode_list_programs(&payload).ok_or(ClientError::MalformedResponse)
    }

    /// A signing program's public key for (seed, `address_index`); `address`
    /// is the seed's index-0 address.
    pub fn public_key(
        &mut self,
        program: &ProgramId,
        address: &str,
        address_index: u8,
    ) -> Result<Vec<u8>, ClientError<E>> {
        check_address(address)?;
        let payload = self.call(&request::public_key(program, address_index, address))?;
        Ok(decode_bytes_payload(&payload)?)
    }

    /// A signing program's signature over `message`.
    pub fn sign(
        &mut self,
        program: &ProgramId,
        address: &str,
        address_index: u8,
        message: &[u8],
    ) -> Result<Vec<u8>, ClientError<E>> {
        check_address(address)?;
        if message.len() > usize::from(u16::MAX) {
            return Err(ClientError::InvalidArgument("message too large"));
        }
        let payload = self.call(&request::sign(program, address_index, address, message))?;
        Ok(decode_bytes_payload(&payload)?)
    }

    /// Runs a program's `run` export on `input`: in one message if it fits,
    /// else uploaded in chunks first.
    pub fn run(&mut self, program: &ProgramId, input: &[u8]) -> Result<Vec<u8>, ClientError<E>> {
        let single = request::run(program, input);
        let payload = if single.len() <= MAX_MESSAGE_LEN {
            self.call(&single)?
        } else {
            let len = u32::try_from(input.len())
                .map_err(|_| ClientError::InvalidArgument("input too large"))?;
            let input_id = self.upload_id(&request::input_begin(len))?;
            let mut offset = 0u32;
            for chunk in input.chunks(request::max_chunk_len(MAX_MESSAGE_LEN)) {
                self.call(&request::input_chunk(input_id, offset, chunk))?;
                offset += chunk.len() as u32;
            }
            self.call(&request::run_input(program, input_id))?
        };
        Ok(decode_bytes_payload(&payload)?)
    }

    /// Applies a signed revocation list; returns its sequence number.
    pub fn revoke(&mut self, revocations: &Signed) -> Result<u32, ClientError<E>> {
        let payload = self.call(&request::revoke(revocations))?;
        let bytes: [u8; 4] = payload
            .as_slice()
            .try_into()
            .map_err(|_| ClientError::MalformedResponse)?;
        Ok(u32::from_le_bytes(bytes))
    }

    fn upload_id(&mut self, begin: &[u8]) -> Result<u8, ClientError<E>> {
        match self.call(begin)?.as_slice() {
            [id] => Ok(*id),
            _ => Err(ClientError::MalformedResponse),
        }
    }
}

fn program_id<E>(payload: &[u8]) -> Result<ProgramId, ClientError<E>> {
    Ok(ProgramId(
        payload
            .try_into()
            .map_err(|_| ClientError::MalformedResponse)?,
    ))
}

fn check_address<E>(address: &str) -> Result<(), ClientError<E>> {
    if address.len() > usize::from(u8::MAX) {
        return Err(ClientError::InvalidArgument("address too long"));
    }
    Ok(())
}
