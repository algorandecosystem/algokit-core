//! Bounds-checked access to a guest's exported `memory`, for host functions.
//!
//! Guest pointers and lengths arrive as `i32`s the guest fully controls, so
//! every access here is checked: a negative value, an overflowing
//! `ptr + len`, or a range past the end of memory is `None`, never a panic.

use alloc::vec::Vec;

use wasmi::{Caller, Extern, Memory};

use crate::HostState;

fn memory<T>(caller: &Caller<'_, HostState<T>>) -> Option<Memory> {
    caller.get_export("memory").and_then(Extern::into_memory)
}

fn range(ptr: i32, len: i32, memory_len: usize) -> Option<core::ops::Range<usize>> {
    let start = usize::try_from(ptr).ok()?;
    let len = usize::try_from(len).ok()?;
    let end = start.checked_add(len)?;
    (end <= memory_len).then_some(start..end)
}

/// Copies `len` bytes at `ptr` out of guest memory.
pub fn read<T>(caller: &Caller<'_, HostState<T>>, ptr: i32, len: i32) -> Option<Vec<u8>> {
    let memory = memory(caller)?;
    let data = memory.data(caller);
    Some(data[range(ptr, len, data.len())?].to_vec())
}

/// Copies `bytes` into guest memory at `ptr`. Returns `None` (writing
/// nothing) if they don't fit.
pub fn write<T>(caller: &mut Caller<'_, HostState<T>>, ptr: i32, bytes: &[u8]) -> Option<()> {
    let memory = memory(caller)?;
    let data = memory.data_mut(caller);
    let len = i32::try_from(bytes.len()).ok()?;
    let range = range(ptr, len, data.len())?;
    data[range].copy_from_slice(bytes);
    Some(())
}
