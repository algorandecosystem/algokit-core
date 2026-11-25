//! Example `seed_vault_wasm` guest: deterministic Falcon-1024, run entirely
//! inside the WASM sandbox.
//!
//! The 32-byte key from `seed_vault.key` (this program's own key for the
//! requested seed and `address_index`, never the seed) is treated like
//! 25-word mnemonic entropy: `pq_seed = SHA512/256("PQK" || "f1" || key)`,
//! then Falcon-1024 deterministic keygen -- the same derivation as
//! `algokit_crypto::falcon1024::account_from_entropy` / go-algorand
//! `algokey pq`.
//!
//! - `public_key` outputs the 1793-byte public key. (The address is
//!   `algokit_crypto::falcon1024::canonical_pq_address` of it.)
//! - `sign` outputs the compressed deterministic signature over the input.
//!
//! Built with `--features native`, it instead reads the native account's key
//! material (`seed_vault.account_entropy`, which needs the
//! `account_entropy` capability; for index 0, also `human_sign`): what
//! `seed_vault::falcon_key_entropy` gives for the requested index. Keygen
//! from that is exactly the vault's own, so it reproduces the vault's
//! native Falcon-1024 accounts. It never sees the root seed except for
//! index 0, whose key material is the seed.
//!
//! Keygen is slow in the interpreter (~0.5 s), so the key pair is cached
//! with `cache_put` after the first keygen for a key (the host saves it to
//! secure storage) and read back with `cache_get` from then on, so later
//! calls only pay for signing. A missing or malformed cache entry just means
//! running keygen again.
#![no_std]

extern crate alloc;

use algorand_falcon::det1024;
use cryptoxide::hashing::sha2::Sha512Trunc256;
use alloc::vec::Vec;

#[link(wasm_import_module = "seed_vault")]
extern "C" {
    #[cfg(not(feature = "native"))]
    fn key(ptr: *mut u8, len: i32) -> i32;
    fn input_len() -> i32;
    fn input(ptr: *mut u8, len: i32) -> i32;
    fn output(ptr: *const u8, len: i32) -> i32;
    fn cache_get(ptr: *mut u8, len: i32) -> i32;
    fn cache_put(ptr: *const u8, len: i32) -> i32;
    #[cfg(feature = "native")]
    fn account_entropy(ptr: *mut u8, len: i32) -> i32;
}

const OK: i32 = 0;
const ERR: i32 = 1;

/// Cached form of a key pair: `public key || private key`.
const CACHE_LEN: usize = det1024::PUBLIC_KEY_SIZE + det1024::PRIVATE_KEY_SIZE;

/// This call's key pair: from the cache if possible, else from keygen
/// (then cached).
fn keypair() -> Option<(det1024::PublicKey, det1024::PrivateKey)> {
    let mut cached = [0u8; CACHE_LEN];
    if unsafe { cache_get(cached.as_mut_ptr(), CACHE_LEN as i32) } == CACHE_LEN as i32 {
        let mut public_key = det1024::PublicKey([0; det1024::PUBLIC_KEY_SIZE]);
        let mut private_key = det1024::PrivateKey([0; det1024::PRIVATE_KEY_SIZE]);
        public_key.0.copy_from_slice(&cached[..det1024::PUBLIC_KEY_SIZE]);
        private_key.0.copy_from_slice(&cached[det1024::PUBLIC_KEY_SIZE..]);
        cached.fill(0);
        return Some((public_key, private_key));
    }
    let (public_key, private_key) = generate()?;
    cached[..det1024::PUBLIC_KEY_SIZE].copy_from_slice(&public_key.0);
    cached[det1024::PUBLIC_KEY_SIZE..].copy_from_slice(&private_key.0);
    // Best effort: a refused cache just means keygen next time too.
    unsafe { cache_put(cached.as_ptr(), CACHE_LEN as i32) };
    cached.fill(0);
    Some((public_key, private_key))
}

/// The 32 bytes of mnemonic-style entropy this call's key is derived from.
#[cfg(not(feature = "native"))]
fn key_entropy(out: &mut [u8; 32]) -> bool {
    unsafe { key(out.as_mut_ptr(), out.len() as i32) == 32 }
}

/// `seed_vault::falcon_key_entropy` for this call's `address_index`,
/// computed by the host.
#[cfg(feature = "native")]
fn key_entropy(out: &mut [u8; 32]) -> bool {
    unsafe { account_entropy(out.as_mut_ptr(), out.len() as i32) == 32 }
}

fn generate() -> Option<(det1024::PublicKey, det1024::PrivateKey)> {
    let mut entropy = [0u8; 32];
    if !key_entropy(&mut entropy) {
        entropy.fill(0);
        return None;
    }
    let mut pq_seed: [u8; 32] = Sha512Trunc256::new()
        .update(b"PQK")
        .update(b"f1")
        .update(&entropy)
        .finalize();
    entropy.fill(0);
    let pair = det1024::generate_key(&pq_seed).ok();
    pq_seed.fill(0);
    // `PrivateKey` zeroizes itself on drop.
    pair
}

fn send(bytes: &[u8]) -> i32 {
    match unsafe { output(bytes.as_ptr(), bytes.len() as i32) } {
        0 => OK,
        _ => ERR,
    }
}

#[no_mangle]
pub extern "C" fn public_key() -> i32 {
    match keypair() {
        Some((public_key, _)) => send(&public_key.0),
        None => ERR,
    }
}

#[no_mangle]
pub extern "C" fn sign() -> i32 {
    let Ok(len) = usize::try_from(unsafe { input_len() }) else {
        return ERR;
    };
    let mut message = Vec::new();
    message.resize(len, 0u8);
    if unsafe { input(message.as_mut_ptr(), len as i32) } != len as i32 {
        return ERR;
    }
    let Some((_, private_key)) = keypair() else {
        return ERR;
    };
    match private_key.sign_compressed(&message) {
        Ok(signature) => send(&signature.0),
        Err(_) => ERR,
    }
}

/// Falcon and `Vec` need an allocator. Each call is a fresh instance, so a
/// bump allocator that never frees is enough.
mod bump {
    use core::alloc::{GlobalAlloc, Layout};
    use core::cell::UnsafeCell;

    const ARENA_SIZE: usize = 128 * 1024;

    struct Bump {
        arena: UnsafeCell<[u8; ARENA_SIZE]>,
        next: UnsafeCell<usize>,
    }

    // wasm32 guests are single-threaded.
    unsafe impl Sync for Bump {}

    unsafe impl GlobalAlloc for Bump {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            let next = &mut *self.next.get();
            let start = (*next + layout.align() - 1) & !(layout.align() - 1);
            match start.checked_add(layout.size()) {
                Some(end) if end <= ARENA_SIZE => {
                    *next = end;
                    (self.arena.get() as *mut u8).add(start)
                }
                _ => core::ptr::null_mut(),
            }
        }

        unsafe fn dealloc(&self, _: *mut u8, _: Layout) {}
    }

    #[global_allocator]
    static ALLOCATOR: Bump = Bump {
        arena: UnsafeCell::new([0; ARENA_SIZE]),
        next: UnsafeCell::new(0),
    };
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}
