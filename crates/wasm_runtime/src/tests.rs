extern crate std;

use alloc::vec::Vec;

use super::*;
use crate::guest_memory;
use wasmi::Caller;

fn runtime<T>() -> WasmRuntime<T> {
    WasmRuntime::new(&RuntimeConfig::default(), |_| Ok(())).expect("runtime")
}

fn compile<T>(runtime: &WasmRuntime<T>, wat: &str) -> CompiledProgram {
    runtime
        .compile(&wat::parse_str(wat).expect("parse WAT"))
        .expect("compile")
}

#[test]
fn calls_an_export() {
    let runtime = runtime::<()>();
    let program = compile(
        &runtime,
        r#"(module (func (export "run") (result i32) i32.const 1337))"#,
    );
    let mut instance = runtime
        .instantiate(&program, (), &Limits::default())
        .unwrap();
    assert_eq!(instance.call("run"), Ok(1337));
}

#[test]
fn program_id_is_the_sha256_of_the_bytes() {
    let runtime = runtime::<()>();
    let bytes = wat::parse_str("(module)").unwrap();
    let program = runtime.compile(&bytes).unwrap();
    assert_eq!(program.id(), ProgramId::of(&bytes));
    assert_eq!(program.id().0, cryptoxide::hashing::sha256(&bytes));
    assert_ne!(
        ProgramId::of(&bytes),
        ProgramId::of(&[bytes.as_slice(), &[0]].concat())
    );
}

#[test]
fn rejects_invalid_modules() {
    let runtime = runtime::<()>();
    assert!(matches!(
        runtime.compile(b"not wasm"),
        Err(Error::InvalidModule(_))
    ));
}

#[test]
fn missing_or_mistyped_export_is_an_error() {
    let runtime = runtime::<()>();
    let program = compile(
        &runtime,
        r#"(module (func (export "run") (result i64) i64.const 1))"#,
    );
    let mut instance = runtime
        .instantiate(&program, (), &Limits::default())
        .unwrap();
    assert!(matches!(instance.call("run"), Err(Error::MissingExport(_))));
    assert!(matches!(
        instance.call("nope"),
        Err(Error::MissingExport(_))
    ));
}

#[test]
fn infinite_loop_runs_out_of_fuel() {
    let runtime = runtime::<()>();
    let program = compile(
        &runtime,
        r#"(module (func (export "run") (result i32) (loop $l br $l) i32.const 0))"#,
    );
    let limits = Limits {
        fuel: 10_000,
        ..Limits::default()
    };
    let mut instance = runtime.instantiate(&program, (), &limits).unwrap();
    assert_eq!(instance.call("run"), Err(Error::OutOfFuel));
    // Fuel is refilled per call, so the instance is still usable.
    assert_eq!(instance.call("run"), Err(Error::OutOfFuel));
}

#[test]
fn memory_cannot_grow_past_the_limit() {
    let runtime = runtime::<()>();
    // Asks for 32 pages (2 MiB) beyond the 1 page it starts with.
    let program = compile(
        &runtime,
        r#"(module (memory (export "memory") 1)
            (func (export "run") (result i32) i32.const 32 memory.grow))"#,
    );
    let limits = Limits {
        memory_limit_bytes: 1 << 20,
        ..Limits::default()
    };
    let mut instance = runtime.instantiate(&program, (), &limits).unwrap();
    assert_eq!(instance.call("run"), Ok(-1));
}

#[test]
fn oversized_initial_memory_fails_to_instantiate() {
    let runtime = runtime::<()>();
    let program = compile(&runtime, r#"(module (memory 64))"#);
    let limits = Limits {
        memory_limit_bytes: 1 << 20,
        ..Limits::default()
    };
    assert!(runtime.instantiate(&program, (), &limits).is_err());
}

#[test]
fn unknown_import_fails_to_instantiate() {
    let runtime = runtime::<()>();
    let program = compile(&runtime, r#"(module (import "env" "secret" (func)))"#);
    assert!(matches!(
        runtime.instantiate(&program, (), &Limits::default()),
        Err(Error::Trap(_))
    ));
}

/// Host state for the host-function tests: what the guest last sent.
#[derive(Default)]
struct Captured(Vec<u8>);

fn echo_runtime() -> WasmRuntime<Captured> {
    WasmRuntime::new(&RuntimeConfig::default(), |linker| {
        // Copies guest bytes out into host state.
        linker.func_wrap(
            "env",
            "send",
            |mut caller: Caller<'_, HostState<Captured>>, ptr: i32, len: i32| -> i32 {
                match guest_memory::read(&caller, ptr, len) {
                    Some(bytes) => {
                        caller.data_mut().data_mut().0 = bytes;
                        0
                    }
                    None => -1,
                }
            },
        )?;
        // Writes fixed bytes into guest memory.
        linker.func_wrap(
            "env",
            "fill",
            |mut caller: Caller<'_, HostState<Captured>>, ptr: i32| -> i32 {
                match guest_memory::write(&mut caller, ptr, b"host") {
                    Some(()) => 0,
                    None => -1,
                }
            },
        )?;
        Ok(())
    })
    .expect("runtime")
}

#[test]
fn host_functions_read_and_write_guest_memory() {
    let runtime = echo_runtime();
    let program = compile(
        &runtime,
        r#"(module
            (import "env" "send" (func $send (param i32 i32) (result i32)))
            (import "env" "fill" (func $fill (param i32) (result i32)))
            (memory (export "memory") 1)
            (func (export "run") (result i32)
                (drop (call $fill (i32.const 100)))
                (call $send (i32.const 100) (i32.const 4))))"#,
    );
    let mut instance = runtime
        .instantiate(&program, Captured::default(), &Limits::default())
        .unwrap();
    assert_eq!(instance.call("run"), Ok(0));
    assert_eq!(instance.data().0, b"host");
}

#[test]
fn out_of_bounds_guest_pointers_are_refused() {
    let runtime = echo_runtime();
    let program = compile(
        &runtime,
        r#"(module
            (import "env" "send" (func $send (param i32 i32) (result i32)))
            (import "env" "fill" (func $fill (param i32) (result i32)))
            (memory (export "memory") 1)
            (func (export "past_end") (result i32) (call $send (i32.const 65530) (i32.const 10)))
            (func (export "negative") (result i32) (call $send (i32.const -1) (i32.const 1)))
            (func (export "overflow") (result i32) (call $send (i32.const 2147483647) (i32.const 2147483647)))
            (func (export "write_past_end") (result i32) (call $fill (i32.const 65534))))"#,
    );
    let mut instance = runtime
        .instantiate(&program, Captured::default(), &Limits::default())
        .unwrap();
    for export in ["past_end", "negative", "overflow", "write_past_end"] {
        assert_eq!(instance.call(export), Ok(-1), "{export}");
    }
    assert!(instance.data().0.is_empty());
}

#[test]
fn host_functions_without_guest_memory_are_refused() {
    let runtime = echo_runtime();
    let program = compile(
        &runtime,
        r#"(module
            (import "env" "send" (func $send (param i32 i32) (result i32)))
            (func (export "run") (result i32) (call $send (i32.const 0) (i32.const 0))))"#,
    );
    let mut instance = runtime
        .instantiate(&program, Captured::default(), &Limits::default())
        .unwrap();
    assert_eq!(instance.call("run"), Ok(-1));
}

#[test]
fn wipe_memory_zeroes_guest_memory() {
    let runtime = runtime::<()>();
    let program = compile(
        &runtime,
        r#"(module (memory (export "memory") 1) (data (i32.const 0) "secret")
            (func (export "first") (result i32) (i32.load8_u (i32.const 0))))"#,
    );
    let mut instance = runtime
        .instantiate(&program, (), &Limits::default())
        .unwrap();
    assert_eq!(instance.call("first"), Ok(i32::from(b's')));
    instance.wipe_memory();
    assert_eq!(instance.call("first"), Ok(0));
}

#[test]
fn each_instance_starts_from_fresh_memory() {
    let runtime = runtime::<()>();
    let program = compile(
        &runtime,
        r#"(module (memory (export "memory") 1)
            (func (export "bump") (result i32)
                (i32.store (i32.const 0) (i32.add (i32.load (i32.const 0)) (i32.const 1)))
                (i32.load (i32.const 0))))"#,
    );
    let mut first = runtime
        .instantiate(&program, (), &Limits::default())
        .unwrap();
    assert_eq!(first.call("bump"), Ok(1));
    assert_eq!(first.call("bump"), Ok(2));
    let mut second = runtime
        .instantiate(&program, (), &Limits::default())
        .unwrap();
    assert_eq!(second.call("bump"), Ok(1));
}

#[test]
fn reports_callable_exports() {
    let runtime = runtime::<()>();
    let program = compile(
        &runtime,
        r#"(module (memory (export "memory") 1)
            (func (export "good") (result i32) i32.const 0)
            (func (export "takes_arg") (param i32) (result i32) i32.const 0)
            (func (export "returns_i64") (result i64) i64.const 0))"#,
    );
    assert!(program.exports_call("good"));
    assert!(!program.exports_call("takes_arg"));
    assert!(!program.exports_call("returns_i64"));
    assert!(!program.exports_call("memory"));
    assert!(!program.exports_call("missing"));
    assert!(program.exports_memory());
    assert!(!compile(&runtime, "(module)").exports_memory());
}

#[test]
fn reports_imports() {
    let runtime = echo_runtime();
    let program = compile(
        &runtime,
        r#"(module (import "env" "send" (func (param i32 i32) (result i32))))"#,
    );
    assert!(program.imports("env", "send"));
    assert!(!program.imports("env", "fill"));
    assert!(!program.imports("other", "send"));
}
