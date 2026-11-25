//! The `seed_vault_wasm` guest glue: the `run` export, reading the command
//! and the oracle's saved state from the host, saving the new state.

use alloc::vec;
use alloc::vec::Vec;

use crate::program;

#[link(wasm_import_module = "seed_vault")]
extern "C" {
    fn input_len() -> i32;
    fn input(ptr: *mut u8, len: i32) -> i32;
    fn output(ptr: *const u8, len: i32) -> i32;
    fn state_len() -> i32;
    fn state_get(ptr: *mut u8, len: i32) -> i32;
    fn state_put(ptr: *const u8, len: i32) -> i32;
}

/// Status reported if the host refuses the new state (e.g. too large).
const STATE_NOT_SAVED: u8 = crate::Error::CorruptState as u8;

fn read(len: i32, get: unsafe extern "C" fn(*mut u8, i32) -> i32) -> Option<Vec<u8>> {
    let len = usize::try_from(len).ok()?;
    let mut buf = vec![0u8; len];
    (unsafe { get(buf.as_mut_ptr(), len as i32) } == len as i32).then_some(buf)
}

/// Runs one light client command (see [`crate::program`]). Always returns 0
/// with a status byte in the output, so the host saves the new state only
/// when the command succeeded (the program only puts state then).
#[no_mangle]
pub extern "C" fn run() -> i32 {
    let Some(command) = read(unsafe { input_len() }, input) else {
        return 1;
    };
    let state = match unsafe { state_len() } {
        0 => None,
        len => match read(len, state_get) {
            Some(state) => Some(state),
            None => return 1,
        },
    };
    let mut outcome = program::run(state.as_deref(), &command);
    if let Some(new_state) = &outcome.new_state {
        if unsafe { state_put(new_state.as_ptr(), new_state.len() as i32) } != 0 {
            outcome.response = vec![STATE_NOT_SAVED];
        }
    }
    match unsafe { output(outcome.response.as_ptr(), outcome.response.len() as i32) } {
        0 => 0,
        _ => 1,
    }
}

#[global_allocator]
static ALLOCATOR: dlmalloc::GlobalDlmalloc = dlmalloc::GlobalDlmalloc;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}
