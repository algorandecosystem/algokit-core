//! The guest ABI: what a WASM program must export, and the host functions
//! it may import.
//!
//! # Exports
//!
//! | Export       | Type          | Does                                                     |
//! |--------------|---------------|----------------------------------------------------------|
//! | `memory`     | memory        | The program's linear memory.                             |
//! | `public_key` | `() -> i32`   | Calls `output` with the public key for `key`.            |
//! | `sign`       | `() -> i32`   | Calls `output` with a signature over `input` with `key`. |
//! | `run`        | `() -> i32`   | General computation: calls `output` with a result for `input`. No key. |
//!
//! A module exports `memory` plus `public_key` and `sign` (a signing
//! algorithm), `run` (anything else, e.g. `guest/light_client`), or all
//! three. Each returns `0` on success. Anything else, a trap, running out of
//! fuel, or returning without calling `output` is a failure.
//!
//! # Imports (module `seed_vault`)
//!
//! | Import                               | Does                                                             |
//! |--------------------------------------|------------------------------------------------------------------|
//! | `key(ptr: i32, len: i32) -> i32`     | Writes this call's 32-byte key to `ptr`; returns 32, or -1 if `len < 32`, out of bounds, or there's no key (`run`). |
//! | `input_len() -> i32`                 | Length of this call's input (the message for `sign`, empty for `public_key`). |
//! | `input(ptr: i32, len: i32) -> i32`   | Copies up to `len` input bytes to `ptr`; returns the count, or -1 if out of bounds. |
//! | `output(ptr: i32, len: i32) -> i32`  | Sets the result to `len` bytes at `ptr` (a later call replaces it); returns 0, or -1 if too long ([`crate::vault::MAX_OUTPUT_LEN`]) or out of bounds. |
//! | `cache_get(ptr: i32, len: i32) -> i32` | Copies what this program cached for this key to `ptr`; returns its length, 0 if nothing is cached, or -1 if it doesn't fit in `len` or is out of bounds. |
//! | `address_index() -> i32`             | The request's `address_index` (0-255). |
//! | `account_entropy(ptr: i32, len: i32) -> i32` | **Needs [`Capabilities::account_entropy`]** (see below): writes the native key material of the requested account to `ptr`; returns 32, or -1 if `len < 32` or out of bounds. |
//! | `cache_put(ptr: i32, len: i32) -> i32` | Caches `len` bytes at `ptr` for this key (a later call replaces it); returns 0, or -1 if too long ([`crate::vault::MAX_CACHE_LEN`]), out of bounds, or there's no key (`run`). |
//! | `state_len() -> i32`                 | Length of the program's saved state (0 if none). |
//! | `state_get(ptr: i32, len: i32) -> i32` | Copies the saved state to `ptr`; returns its length, 0 if none, or -1 if it doesn't fit in `len` or is out of bounds. |
//! | `state_put(ptr: i32, len: i32) -> i32` | Replaces the saved state with `len` bytes at `ptr`; returns 0, or -1 if too long ([`crate::vault::MAX_STATE_LEN`]) or out of bounds. |
//!
//! # State
//!
//! Each program has one persistent state blob, in secure storage, for any
//! export: e.g. the light client's oracle, or a signing program's counters.
//! A `state_put` is saved only if the call succeeds, so a failed call never
//! leaves half-updated state. Signed versions of one algorithm share their
//! state (it's keyed like their keys, by algorithm name), so an upgrade keeps
//! it; unloading the last version deletes it.
//!
//! # Account entropy
//!
//! By default a program only gets `key`, its own derived key, so its
//! accounts are separate from the native ones and it can't leak a seed. A
//! module that imports `account_entropy` instead gets the 32 bytes the
//! vault's *native* account for (seed, `address_index`) is derived from:
//! `seed_vault::falcon_key_entropy`, i.e. the seed entropy itself for index
//! 0 and `SHA512/256("SeedVaultAutosign" || index || entropy)` for 1-255.
//! That lets it reproduce the native accounts exactly (see the `native`
//! builds of `guest/ed25519` and `guest/falcon1024`).
//!
//! Never the root seed beyond that: a leak through an autosign index
//! exposes only that one account, not the 25 words. Index 0's entropy *is*
//! the seed, which is why index 0 needs [`Capabilities::human_sign`] too.
//! A module importing `account_entropy` can only be installed with the
//! [`Capabilities::account_entropy`] capability (a signed manifest granting
//! it, or dev mode).
//!
//! # Key cache
//!
//! Deriving a key pair can be slow (Falcon-1024 keygen is ~0.5 s in the
//! interpreter), so a program can cache what it derived, e.g. its expanded
//! key pair, and skip keygen on later calls. The cache is per (program,
//! seed, `address_index`): a program only ever reads back what *it* stored
//! for *this* key. It's saved to secure storage only if the call succeeds,
//! so it survives a TA restart, and it's deleted when the program is
//! unloaded or the seed is deleted. A program must still work when nothing
//! is cached (e.g. after a new program version or a cache-full refusal).
//!
//! The key is the program's own key for the request's seed and
//! `address_index` ([`crate::derive_program_key`]), never the seed. How the
//! program turns it into a key pair is up to the program.
//!
//! Importing anything else makes the module fail to load.
//! `guest/ed25519` in this crate is a complete example in Rust.

use alloc::vec::Vec;

use wasm_runtime::wasmi::{self, Caller, Linker};
use wasm_runtime::{guest_memory, HostState};

#[allow(unused_imports)] // doc links
use crate::manifest::Capabilities;
use zeroize::Zeroize;

/// Host import module name.
pub const IMPORT_MODULE: &str = "seed_vault";
/// Export returning the public key.
pub const EXPORT_PUBLIC_KEY: &str = "public_key";
/// Export returning a signature.
pub const EXPORT_SIGN: &str = "sign";
/// Export for general computation.
pub const EXPORT_RUN: &str = "run";

/// The import that needs [`Capabilities::account_entropy`].
pub const IMPORT_ACCOUNT_ENTROPY: &str = "account_entropy";

/// Byte length of the key handed to a program.
pub const KEY_LEN: usize = 32;

/// What one call can see: its key, its input, and the output it set.
pub struct CallContext {
    /// `None` for `run`, which isn't tied to a key.
    key: Option<[u8; KEY_LEN]>,
    input: Vec<u8>,
    max_output_len: usize,
    output: Option<Vec<u8>>,
    /// What's cached for this key, loaded before the call.
    cache: Option<Vec<u8>>,
    /// What `cache_put` asked to store, saved after a successful call.
    cache_put: Option<Vec<u8>>,
    max_cache_len: usize,
    address_index: u8,
    /// The native key material for this account: only set for programs with
    /// [`Capabilities::account_entropy`].
    account_entropy: Option<[u8; KEY_LEN]>,
    /// The program's saved state, loaded before the call.
    state: Option<Vec<u8>>,
    /// What `state_put` asked to save, saved after a successful call.
    state_put: Option<Vec<u8>>,
    max_state_len: usize,
}

impl CallContext {
    pub(crate) fn new(
        key: Option<[u8; KEY_LEN]>,
        input: Vec<u8>,
        max_output_len: usize,
        cache: Option<Vec<u8>>,
        max_cache_len: usize,
    ) -> Self {
        CallContext {
            key,
            input,
            max_output_len,
            output: None,
            cache,
            cache_put: None,
            max_cache_len,
            address_index: 0,
            account_entropy: None,
            state: None,
            state_put: None,
            max_state_len: 0,
        }
    }

    pub(crate) fn with_state(mut self, state: Option<Vec<u8>>, max_state_len: usize) -> Self {
        self.state = state;
        self.max_state_len = max_state_len;
        self
    }

    pub(crate) fn take_state_put(&mut self) -> Option<Vec<u8>> {
        self.state_put.take()
    }

    pub(crate) fn with_key_info(
        mut self,
        address_index: u8,
        account_entropy: Option<[u8; KEY_LEN]>,
    ) -> Self {
        self.address_index = address_index;
        self.account_entropy = account_entropy;
        self
    }

    pub(crate) fn take_output(&mut self) -> Option<Vec<u8>> {
        self.output.take()
    }

    pub(crate) fn take_cache_put(&mut self) -> Option<Vec<u8>> {
        self.cache_put.take()
    }
}

impl Drop for CallContext {
    fn drop(&mut self) {
        self.key.zeroize();
        self.account_entropy.zeroize();
        self.input.zeroize();
        // Cached data is typically private key material.
        for blob in [&mut self.cache, &mut self.cache_put].into_iter().flatten() {
            blob.zeroize();
        }
    }
}

type Ctx<'a> = Caller<'a, HostState<CallContext>>;

/// Registers the host imports above.
pub(crate) fn define_host_functions(
    linker: &mut Linker<HostState<CallContext>>,
) -> Result<(), wasmi::Error> {
    linker.func_wrap(
        IMPORT_MODULE,
        "key",
        |mut caller: Ctx<'_>, ptr: i32, len: i32| -> i32 {
            let Some(mut key) = caller.data().data().key else {
                return -1;
            };
            if len < KEY_LEN as i32 {
                key.zeroize();
                return -1;
            }
            let written = guest_memory::write(&mut caller, ptr, &key);
            key.zeroize();
            match written {
                Some(()) => KEY_LEN as i32,
                None => -1,
            }
        },
    )?;
    linker.func_wrap(IMPORT_MODULE, "input_len", |caller: Ctx<'_>| -> i32 {
        caller.data().data().input.len() as i32
    })?;
    linker.func_wrap(
        IMPORT_MODULE,
        "input",
        |mut caller: Ctx<'_>, ptr: i32, len: i32| -> i32 {
            let Ok(len) = usize::try_from(len) else {
                return -1;
            };
            let input = &caller.data().data().input;
            let count = len.min(input.len());
            let bytes = input[..count].to_vec();
            match guest_memory::write(&mut caller, ptr, &bytes) {
                Some(()) => count as i32,
                None => -1,
            }
        },
    )?;
    linker.func_wrap(
        IMPORT_MODULE,
        "output",
        |mut caller: Ctx<'_>, ptr: i32, len: i32| -> i32 {
            let max = caller.data().data().max_output_len;
            if usize::try_from(len).map_or(true, |len| len > max) {
                return -1;
            }
            match guest_memory::read(&caller, ptr, len) {
                Some(bytes) => {
                    caller.data_mut().data_mut().output = Some(bytes);
                    0
                }
                None => -1,
            }
        },
    )?;
    linker.func_wrap(IMPORT_MODULE, "address_index", |caller: Ctx<'_>| -> i32 {
        i32::from(caller.data().data().address_index)
    })?;
    linker.func_wrap(
        IMPORT_MODULE,
        IMPORT_ACCOUNT_ENTROPY,
        |mut caller: Ctx<'_>, ptr: i32, len: i32| -> i32 {
            // The vault only sets it for programs with the capability.
            let Some(mut entropy) = caller.data().data().account_entropy else {
                return -1;
            };
            let written = if len < KEY_LEN as i32 {
                None
            } else {
                guest_memory::write(&mut caller, ptr, &entropy)
            };
            entropy.zeroize();
            match written {
                Some(()) => KEY_LEN as i32,
                None => -1,
            }
        },
    )?;
    linker.func_wrap(
        IMPORT_MODULE,
        "cache_get",
        |mut caller: Ctx<'_>, ptr: i32, len: i32| -> i32 {
            let Some(cache) = caller.data().data().cache.as_ref() else {
                return 0;
            };
            if usize::try_from(len).map_or(true, |len| len < cache.len()) {
                return -1;
            }
            let mut bytes = cache.clone();
            let written = guest_memory::write(&mut caller, ptr, &bytes);
            let count = bytes.len() as i32;
            bytes.zeroize();
            match written {
                Some(()) => count,
                None => -1,
            }
        },
    )?;
    linker.func_wrap(
        IMPORT_MODULE,
        "cache_put",
        |mut caller: Ctx<'_>, ptr: i32, len: i32| -> i32 {
            let max = caller.data().data().max_cache_len;
            if usize::try_from(len).map_or(true, |len| len > max) {
                return -1;
            }
            match guest_memory::read(&caller, ptr, len) {
                Some(bytes) => {
                    let context = caller.data_mut().data_mut();
                    if let Some(mut old) = context.cache_put.replace(bytes) {
                        old.zeroize();
                    }
                    0
                }
                None => -1,
            }
        },
    )?;
    linker.func_wrap(IMPORT_MODULE, "state_len", |caller: Ctx<'_>| -> i32 {
        caller
            .data()
            .data()
            .state
            .as_ref()
            .map_or(0, |s| s.len() as i32)
    })?;
    linker.func_wrap(
        IMPORT_MODULE,
        "state_get",
        |mut caller: Ctx<'_>, ptr: i32, len: i32| -> i32 {
            let Some(state) = caller.data().data().state.clone() else {
                return 0;
            };
            if usize::try_from(len).map_or(true, |len| len < state.len()) {
                return -1;
            }
            match guest_memory::write(&mut caller, ptr, &state) {
                Some(()) => state.len() as i32,
                None => -1,
            }
        },
    )?;
    linker.func_wrap(
        IMPORT_MODULE,
        "state_put",
        |mut caller: Ctx<'_>, ptr: i32, len: i32| -> i32 {
            let max = caller.data().data().max_state_len;
            if usize::try_from(len).map_or(true, |len| len > max) {
                return -1;
            }
            match guest_memory::read(&caller, ptr, len) {
                Some(bytes) => {
                    caller.data_mut().data_mut().state_put = Some(bytes);
                    0
                }
                None => -1,
            }
        },
    )?;
    Ok(())
}
