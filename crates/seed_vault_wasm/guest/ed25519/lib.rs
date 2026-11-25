//! Example `seed_vault_wasm` guest: Ed25519 key derivation and signing,
//! run entirely inside the WASM sandbox.
//!
//! The 32-byte key the host hands over through `seed_vault.key` is this
//! program's own key for the requested seed and `address_index` (see
//! `seed_vault_wasm::derive_program_key`), never the seed itself. It's used
//! directly as the Ed25519 secret seed.
//!
//! Built with `--features native`, it instead reads the native account's key
//! material (`seed_vault.account_entropy`, which needs the
//! `account_entropy` capability; for index 0, also `human_sign`) and uses
//! that as the Ed25519 seed, exactly as `seed_vault` does for an
//! `Algo25Ed25519` account -- so it reproduces the vault's native algo25
//! account. Ed25519 seeds only have index 0, so other indices fail.
#![no_std]

use cryptoxide::ed25519;

#[link(wasm_import_module = "seed_vault")]
extern "C" {
    #[cfg(not(feature = "native"))]
    fn key(ptr: *mut u8, len: i32) -> i32;
    fn input_len() -> i32;
    fn input(ptr: *mut u8, len: i32) -> i32;
    fn output(ptr: *const u8, len: i32) -> i32;
    #[cfg(feature = "native")]
    fn account_entropy(ptr: *mut u8, len: i32) -> i32;
    #[cfg(feature = "native")]
    fn address_index() -> i32;
}

const OK: i32 = 0;
const ERR: i32 = 1;

/// Largest message this guest will sign (the TA caps requests at 4 KB).
const MAX_INPUT: usize = 4096;

/// The 32-byte Ed25519 seed for this call.
#[cfg(not(feature = "native"))]
fn secret_seed(out: &mut [u8; 32]) -> bool {
    unsafe { key(out.as_mut_ptr(), out.len() as i32) == 32 }
}

/// The native algo25 account: index 0's key material (the seed itself).
#[cfg(feature = "native")]
fn secret_seed(out: &mut [u8; 32]) -> bool {
    unsafe { address_index() == 0 && account_entropy(out.as_mut_ptr(), out.len() as i32) == 32 }
}

fn keypair() -> Option<([u8; 64], [u8; 32])> {
    let mut seed = [0u8; 32];
    if !secret_seed(&mut seed) {
        seed.fill(0);
        return None;
    }
    let pair = ed25519::keypair(&seed);
    seed.fill(0);
    Some(pair)
}

fn send(bytes: &[u8]) -> i32 {
    match unsafe { output(bytes.as_ptr(), bytes.len() as i32) } {
        0 => OK,
        _ => ERR,
    }
}

/// Outputs the 32-byte Ed25519 public key.
#[no_mangle]
pub extern "C" fn public_key() -> i32 {
    match keypair() {
        Some((mut secret, public)) => {
            secret.fill(0);
            send(&public)
        }
        None => ERR,
    }
}

/// Outputs the 64-byte Ed25519 signature over the input bytes.
#[no_mangle]
pub extern "C" fn sign() -> i32 {
    let mut message = [0u8; MAX_INPUT];
    let len = unsafe { input_len() };
    if len < 0 || len as usize > MAX_INPUT {
        return ERR;
    }
    if unsafe { input(message.as_mut_ptr(), len) } != len {
        return ERR;
    }
    match keypair() {
        Some((mut secret, _)) => {
            let signature = ed25519::signature(&message[..len as usize], &secret);
            secret.fill(0);
            send(&signature)
        }
        None => ERR,
    }
}

/// `cryptoxide` links `alloc`, so a global allocator must exist. The host
/// creates a fresh instance for every call, so a bump allocator that never
/// frees is enough.
mod bump {
    use core::alloc::{GlobalAlloc, Layout};
    use core::cell::UnsafeCell;

    const ARENA_SIZE: usize = 64 * 1024;

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
