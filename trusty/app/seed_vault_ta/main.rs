//! seed_vault_ta -- a Trusty Trusted Application (TA) that lets an AOSP
//! Android service create, enumerate, import, and (deliberately,
//! explicitly) back up Algorand accounts, without the private key ever
//! leaving the secure world *except* for those two sanctioned paths.
//!
//! ## What this TA exposes
//!
//! A single IPC port, `algorandecosystem.seed_vault_ta`, open to
//! non-secure (Android) callers via `PortCfg::allow_ns_connect()`. Four
//! operations, all defined byte-for-byte in `seed_vault::protocol`:
//!
//!   - `CREATE_ACCOUNT`: generates a brand-new account (a 32-byte seed from
//!     the platform CSPRNG, backed by a 25-word mnemonic), persists it via
//!     `trusty_store::TrustySecureStore`, and replies with just its
//!     Algorand address.
//!   - `LIST_ACCOUNTS`: replies with the algorithm + address of every
//!     account ever created/imported by this TA, in creation order.
//!   - `REVEAL_MNEMONIC`: replies with the literal 25-word backup phrase
//!     for the seed a given Algorand address was derived from. **Exception
//!     #1** to "keys never leave the secure world" -- see the security
//!     note below.
//!   - `IMPORT_ACCOUNT`: the mirror image of `REVEAL_MNEMONIC` -- accepts a
//!     25-word mnemonic and persists the account it recovers to, rejecting
//!     it if that seed is already stored. **Exception #2**: a secret
//!     enters the secure world instead of leaving it.
//!
//! `CREATE_ACCOUNT` and `LIST_ACCOUNTS` never serialize a seed/private key
//! into a response -- see `seed_vault::store::AccountStore`'s doc comment
//! for why that's a hard invariant.
//!
//! ## Security note: hardening `REVEAL_MNEMONIC`/`IMPORT_ACCOUNT` before shipping
//!
//! Both opcodes mirror the real "show/enter recovery phrase" flows
//! secure-enclave wallets (e.g. Solana Mobile's Seed Vault on the Seeker
//! phone) expose in Settings -- the mnemonic genuinely crosses the IPC
//! boundary, but only because the *user* asked for it right now. See
//! `seed_vault::store::reveal_mnemonic`/`import_account`'s doc comments:
//! **this file must not forward either request to the store until it can
//! prove the request carries evidence of a fresh, explicit user
//! authentication** (e.g. a recent Gatekeeper password/biometric auth
//! token). That check is **not implemented below** -- wiring it in is
//! flagged again at the matching arms in `on_message`.
//!
//! ## Division of labor / what's actually proven vs. best-effort
//!
//! - `seed_vault` (a plain `no_std` + `alloc` crate at
//!   `crates/seed_vault`) owns the wire protocol and the
//!   create/list/reveal/import logic, fully unit-tested with
//!   `cargo test -p seed_vault` -- no Trusty/AOSP checkout required.
//! - `trusty_store.rs` implements that crate's `AccountStore` trait
//!   against Trusty's real secure storage service, modeled on
//!   `trusty/app/secretkeeper/store.rs`.
//! - This file wires a `tipc::Service` event loop to
//!   `seed_vault::service::handle_request`. The `Service`/`Deserialize`/
//!   `Serialize`/`Manager`/`SingleDispatcher` shapes below are cross-checked
//!   against `rkutipc` (a published, pure-Rust reimplementation of Trusty's
//!   `tipc` wire protocol) rather than guessed, since Trusty's own `tipc`
//!   crate isn't published to crates.io/docs.rs. The official crate at
//!   `trusty/user/base/lib/tipc/rust` in your synced tree is very likely
//!   API-compatible, but treat this as a strong reference, not a guarantee
//!   -- diff against it if the build still complains.
//! - This file also supplies `getrandom`'s *custom backend* entry point
//!   (`__getrandom_v03_custom`, in the `trusty_rng` module below), sourcing
//!   entropy for `CREATE_ACCOUNT` from Trusty's own `trusty_rng_secure_rand`
//!   (a BoringSSL CSPRNG reseeded from the platform HWRNG) instead of
//!   `getrandom`'s upstream backends, none of which support
//!   `*-unknown-trusty`. See that module's doc comment for the full
//!   rationale and `trusty/vendor/getrandom` for the vendored crate this
//!   backs.
//!
//! ## Calling this TA from AOSP
//!
//! ```ignore
//! use trusty::{DEFAULT_DEVICE, TipcChannel};
//!
//! let mut channel = TipcChannel::connect(DEFAULT_DEVICE, "algorandecosystem.seed_vault_ta")?;
//!
//! // CREATE_ACCOUNT (algorithm tag 0x01 = Algo25Ed25519)
//! channel.send(&[0x01, 0x01])?;
//! let response = channel.recv()?;
//! // response = [0x00, algorithm, address_index: u32 LE, addr_len, ...address
//! // bytes] on success -- see `seed_vault::protocol` for the full wire
//! // format and reference decoders.
//!
//! // LIST_ACCOUNTS
//! channel.send(&[0x02])?;
//! let response = channel.recv()?;
//!
//! // REVEAL_MNEMONIC for a specific address (the same address returned by
//! // CREATE_ACCOUNT/LIST_ACCOUNTS above) -- only after the caller has
//! // itself gated this behind a fresh user auth prompt (see security note
//! // above). Wire shape: [0x03][addr_len: u8][addr_len bytes of ASCII
//! // address].
//! let address = "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567ABCDEFGHIJKLMNOPQRSTUVWXY"; // 58 chars
//! let mut request = alloc::vec![0x03u8, address.len() as u8];
//! request.extend_from_slice(address.as_bytes());
//! channel.send(&request)?;
//! let response = channel.recv()?;
//!
//! // IMPORT_ACCOUNT: [0x04][algorithm: u8][mnemonic_len: u8][mnemonic bytes],
//! // gated behind the same fresh-auth requirement as REVEAL_MNEMONIC.
//! let mnemonic = "abandon ability able ..."; // 25 words
//! let mut request = alloc::vec![0x04u8, 0x01u8, mnemonic.len() as u8];
//! request.extend_from_slice(mnemonic.as_bytes());
//! channel.send(&request)?;
//! let response = channel.recv()?;
//! ```

mod trusty_rng;
mod trusty_store;

extern crate alloc;

use alloc::vec::Vec;

use seed_vault::service::handle_request;
use tipc::{
    ConnectResult, Deserialize, Handle, Manager, MessageResult, PortCfg, Serialize, Serializer,
    Service, TipcError, Uuid,
};
use trusty_store::TrustySecureStore;
use zeroize::Zeroize;

const PORT: &str = "algorandecosystem.seed_vault_ta";

/// Maximum request/response size. `seed_vault::protocol` messages are tiny,
/// so this comfortably covers a realistic number of accounts per device
/// without needing message fragmentation.
const MAX_MSG_SIZE: u32 = 4096;

/// Max simultaneous client connections this TA will service at once.
const MAX_CONNECTIONS: usize = 4;

/// Thin newtype so a plain byte buffer can be used as a `tipc` message,
/// without pulling in a serialization framework for a protocol this simple.
struct RawMessage(Vec<u8>);

impl Deserialize for RawMessage {
    type Error = TipcError;
    const MAX_SERIALIZED_SIZE: usize = MAX_MSG_SIZE as usize;

    fn deserialize(bytes: &[u8], _handles: &mut [Option<Handle>]) -> Result<Self, Self::Error> {
        Ok(RawMessage(bytes.to_vec()))
    }
}

impl<'s> Serialize<'s> for RawMessage {
    fn serialize<'a: 's, S: Serializer<'s>>(
        &'a self,
        serializer: &mut S,
    ) -> Result<S::Ok, S::Error> {
        serializer.serialize_bytes(&self.0)
    }
}

/// Ties the (transport-agnostic) `seed_vault` request handler to a real
/// `tipc::Service`. Holds no per-connection state: every request is
/// self-contained, and `TrustySecureStore` opens its own secure-storage
/// session per operation (matching the pattern in
/// `trusty/app/secretkeeper/store.rs`), so `Connection = ()` is sufficient.
struct SeedVaultService;

impl Service for SeedVaultService {
    type Connection = ();
    type Message = RawMessage;

    fn on_connect(
        &self,
        _port: &PortCfg,
        _handle: &Handle,
        _peer: &Uuid,
    ) -> tipc::Result<ConnectResult<Self::Connection>> {
        // Every caller is accepted -- access control happens at the port
        // level via `PortCfg::allow_ns_connect()` below, not per-connection.
        // `_peer` is where caller identity would come from if this TA later
        // needs to scope accounts per-caller or enforce an auth-token check.
        Ok(ConnectResult::Accept(()))
    }

    fn on_message(
        &self,
        _connection: &Self::Connection,
        handle: &Handle,
        mut msg: Self::Message,
    ) -> tipc::Result<MessageResult> {
        // TODO(security): before this reaches production, REVEAL_MNEMONIC
        // (0x03) and IMPORT_ACCOUNT (0x04) requests must be rejected here
        // unless they carry proof of a fresh, explicit user authentication
        // (e.g. a Gatekeeper auth token verified the same way
        // `trusty/app/gatekeeper` does). `handle_request` is purely
        // protocol-and-store logic and has no way to enforce that -- this
        // call site is where the check belongs.
        let mut store = TrustySecureStore;
        let response = handle_request(&mut store, &msg.0);
        // The request buffer may have carried a mnemonic (IMPORT_ACCOUNT);
        // wipe it now that it's no longer needed.
        msg.0.zeroize();
        handle.send(&RawMessage(response))?;
        Ok(MessageResult::MaintainConnection)
    }
}

fn main() {
    let cfg = PortCfg::new(PORT)
        .expect("invalid port name")
        .msg_max_size(MAX_MSG_SIZE)
        .allow_ns_connect();

    let buffer = [0u8; MAX_MSG_SIZE as usize];
    // `tipc`'s `service` module is private (`mod service;`, not `pub mod
    // service;` -- see the upstream lib.rs), so `SingleDispatcher` cannot be
    // named from this crate at all. `Manager::new` is only implemented for
    // `Manager<SingleDispatcher<S>, ..>`, so leaving the dispatcher type
    // parameter as `_` still resolves correctly through inference without
    // ever spelling out that private type.
    let manager = Manager::<_, _, 1, MAX_CONNECTIONS>::new(SeedVaultService, cfg, buffer)
        .expect("failed to create tipc Manager");
    manager.run_event_loop().expect("seed_vault_ta event loop exited unexpectedly");
}
