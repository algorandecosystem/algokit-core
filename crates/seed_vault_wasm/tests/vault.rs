//! Behavior of the vault and protocol, with small hand-written WAT programs.
// These drive the TA side, which needs the default `vault` feature.
#![cfg(feature = "vault")]

mod common;

use common::*;
use seed_vault::{create_account, delete_account, KeyAlgorithm, AUTOSIGN_INDEX};
use seed_vault_wasm::protocol::{request, ErrorCode};
use seed_vault_wasm::{
    derive_program_key, Admission, Capabilities, KeyNamespace, ProgramId, PublisherPolicy,
    Revocations, Signed, VaultConfig, WasmError, WasmPolicy, WasmStore,
};
use wasm_runtime::Limits;

const IMPORTS: &str = r#"
    (import "seed_vault" "key" (func $key (param i32 i32) (result i32)))
    (import "seed_vault" "input_len" (func $input_len (result i32)))
    (import "seed_vault" "input" (func $input (param i32 i32) (result i32)))
    (import "seed_vault" "output" (func $output (param i32 i32) (result i32)))
    (import "seed_vault" "cache_get" (func $cache_get (param i32 i32) (result i32)))
    (import "seed_vault" "cache_put" (func $cache_put (param i32 i32) (result i32)))
    (import "seed_vault" "address_index" (func $address_index (result i32)))
    (memory (export "memory") 1)
    (global $calls (mut i32) (i32.const 0))"#;

/// A module with all imports whose exports have bodies `public_key`/`sign`.
/// `tag` makes otherwise-identical modules distinct programs.
fn module(tag: &str, public_key: &str, sign: &str) -> Vec<u8> {
    wat::parse_str(format!(
        r#"(module {IMPORTS}
            (data (i32.const 60000) "{tag}")
            (func (export "public_key") (result i32) {public_key})
            (func (export "sign") (result i32) {sign}))"#
    ))
    .unwrap()
}

/// "Public key" = its raw key; "signature" = `key || input`. Leaking the key
/// is the worst a program can do, so this shows what a malicious one gets.
fn leaky(tag: &str) -> Vec<u8> {
    module(
        tag,
        "(drop (call $key (i32.const 0) (i32.const 32)))
         (call $output (i32.const 0) (i32.const 32))",
        "(drop (call $key (i32.const 0) (i32.const 32)))
         (drop (call $input (i32.const 32) (call $input_len)))
         (call $output (i32.const 0) (i32.add (i32.const 32) (call $input_len)))",
    )
}

/// Outputs 1 if its key was already cached (a "hit"), else caches its key
/// and outputs 0. `sign` caches but then fails.
fn caching(tag: &str) -> Vec<u8> {
    module(
        tag,
        "(i32.store8 (i32.const 0)
            (i32.eq (call $cache_get (i32.const 100) (i32.const 32)) (i32.const 32)))
         (if (i32.eqz (i32.load8_u (i32.const 0)))
            (then
                (drop (call $key (i32.const 100) (i32.const 32)))
                (drop (call $cache_put (i32.const 100) (i32.const 32)))))
         (call $output (i32.const 0) (i32.const 1))",
        "(drop (call $key (i32.const 100) (i32.const 32)))
         (drop (call $cache_put (i32.const 100) (i32.const 32)))
         (drop (call $output (i32.const 0) (i32.const 1)))
         i32.const 1",
    )
}

fn setup() -> (Ta, String) {
    let mut ta = Ta::new();
    let address = ta.seed(KeyAlgorithm::Falcon1024, 9);
    (ta, address)
}

#[test]
fn a_program_only_ever_sees_its_own_derived_key() {
    let (mut ta, address) = setup();
    let a = ta.upload(&leaky("a")).unwrap();
    let b = ta.upload(&leaky("b")).unwrap();

    let key_a = ta.public_key(&a, &address, 1).unwrap();
    let key_b = ta.public_key(&b, &address, 1).unwrap();
    assert_eq!(
        key_a,
        derive_program_key(&[9; 32], &KeyNamespace::program(&a), 1)
    );
    assert_eq!(
        key_b,
        derive_program_key(&[9; 32], &KeyNamespace::program(&b), 1)
    );
    assert_ne!(key_a, key_b);
    assert_ne!(key_a, [9; 32]);
    assert_ne!(key_a, ta.public_key(&a, &address, 2).unwrap());

    let signed = ta.sign(&a, &address, 1, b"hello").unwrap();
    assert_eq!(signed, [key_a.as_slice(), b"hello"].concat());
}

#[test]
fn derive_program_key_matches_its_documented_construction() {
    let program = ProgramId([3; 32]);
    let mut input = b"SeedVaultWasm".to_vec();
    input.extend_from_slice(&[3; 32]);
    input.extend_from_slice(&5u32.to_le_bytes());
    input.extend_from_slice(&[4; 32]);
    assert_eq!(
        derive_program_key(&[4; 32], &KeyNamespace::program(&program), 5),
        algokit_crypto::sha512_256(&input)
    );
}

#[test]
fn seed_must_be_named_by_an_existing_index_0_address() {
    let (mut ta, _) = setup();
    let program = ta.upload(&leaky("a")).unwrap();
    let autosign =
        create_account(&mut ta.accounts, KeyAlgorithm::Falcon1024, AUTOSIGN_INDEX).unwrap();
    for address in ["UNKNOWN", autosign.address.as_str()] {
        assert_eq!(
            ta.public_key(&program, address, 0),
            Err(ErrorCode::AccountNotFound)
        );
    }
}

#[test]
fn upload_rejects_bad_sequences() {
    let (mut ta, _) = setup();
    let module = leaky("a");
    let len = module.len() as u32;

    assert_eq!(
        ta.call(&request::load_begin(0)),
        Err(ErrorCode::ProgramTooLarge)
    );
    assert_eq!(
        ta.call(&request::load_begin(128 * 1024 + 1)),
        Err(ErrorCode::ProgramTooLarge)
    );

    let id = ta.call(&request::load_begin(len)).unwrap()[0];
    // Out of order, then too long.
    assert_eq!(
        ta.call(&request::load_chunk(id, 1, &module[1..])),
        Err(ErrorCode::BadChunk)
    );
    assert_eq!(
        ta.call(&request::load_chunk(
            id,
            0,
            &[module.as_slice(), &[0]].concat()
        )),
        Err(ErrorCode::BadChunk)
    );
    ta.call(&request::load_chunk(id, 0, &module[..10])).unwrap();
    // Finished early.
    assert_eq!(ta.call(&request::load_finish(id)), Err(ErrorCode::BadChunk));

    // A new upload replaces the unfinished one, whose ID stops working.
    let id2 = ta.call(&request::load_begin(len)).unwrap()[0];
    assert_ne!(id, id2);
    assert_eq!(
        ta.call(&request::load_chunk(id, 10, &module[10..])),
        Err(ErrorCode::UnknownUpload)
    );
    ta.call(&request::load_chunk(id2, 0, &module)).unwrap();
    ta.call(&request::load_finish(id2)).unwrap();
    // A finished upload is gone.
    assert_eq!(
        ta.call(&request::load_finish(id2)),
        Err(ErrorCode::UnknownUpload)
    );
}

#[test]
fn rejects_modules_that_dont_follow_the_abi() {
    let (mut ta, _) = setup();
    let cases = [
        b"not wasm".to_vec(),
        // No `sign`.
        wat::parse_str(r#"(module (memory (export "memory") 1) (func (export "public_key") (result i32) i32.const 0))"#).unwrap(),
        // No memory.
        wat::parse_str(r#"(module (func (export "public_key") (result i32) i32.const 0) (func (export "sign") (result i32) i32.const 0))"#).unwrap(),
        // `sign` has the wrong type.
        wat::parse_str(r#"(module (memory (export "memory") 1) (func (export "public_key") (result i32) i32.const 0) (func (export "sign") (param i32) (result i32) i32.const 0))"#).unwrap(),
    ];
    for module in cases {
        assert_eq!(ta.upload(&module), Err(ErrorCode::InvalidModule));
    }
    assert!(ta.programs().is_empty());
    assert!(ta.stored("").is_empty(), "rejected modules aren't saved");
}

#[test]
fn a_program_importing_anything_else_cannot_run() {
    let (mut ta, address) = setup();
    let module = wat::parse_str(
        r#"(module (import "seed_vault" "seed" (func (param i32) (result i32)))
            (memory (export "memory") 1)
            (func (export "public_key") (result i32) i32.const 0)
            (func (export "sign") (result i32) i32.const 0))"#,
    )
    .unwrap();
    let program = ta.upload(&module).unwrap();
    assert_eq!(
        ta.public_key(&program, &address, 1),
        Err(ErrorCode::ProgramFailed)
    );
}

#[test]
fn failing_programs_return_errors_not_output() {
    let (mut ta, address) = setup();
    let set_output_then =
        |result: &str| format!("(drop (call $output (i32.const 0) (i32.const 4))) {result}");
    let cases = [
        // Non-zero return even after setting output.
        set_output_then("i32.const 1"),
        // Success without output.
        "i32.const 0".to_string(),
        // Trap.
        set_output_then("unreachable"),
        // Output longer than one response can carry is refused.
        "(drop (call $output (i32.const 0) (i32.const 4000))) i32.const 0".to_string(),
        // Out-of-bounds key write.
        "(drop (call $output (i32.const 0) (i32.const 1)))
         (call $key (i32.const 65530) (i32.const 32))"
            .to_string(),
    ];
    for body in cases {
        let id = ta.upload(&module("", &body, "i32.const 0")).unwrap();
        assert_eq!(
            ta.public_key(&id, &address, 1),
            Err(ErrorCode::ProgramFailed),
            "{body}"
        );
        ta.call(&request::unload(&id)).unwrap();
    }
}

#[test]
fn runaway_program_runs_out_of_fuel() {
    let config = VaultConfig {
        call_limits: Limits {
            fuel: 1_000_000,
            memory_limit_bytes: 1 << 20,
        },
        ..VaultConfig::default()
    };
    let mut ta = Ta::with(config, PublisherPolicy::dev());
    let address = ta.seed(KeyAlgorithm::Falcon1024, 9);
    let id = ta
        .upload(&module("", "(loop $l br $l) i32.const 0", "i32.const 0"))
        .unwrap();
    assert_eq!(ta.public_key(&id, &address, 1), Err(ErrorCode::OutOfFuel));
}

#[test]
fn every_call_starts_from_fresh_state() {
    let (mut ta, address) = setup();
    // Counts calls in a global and outputs the count.
    let body = "(global.set $calls (i32.add (global.get $calls) (i32.const 1)))
        (i32.store (i32.const 0) (global.get $calls))
        (call $output (i32.const 0) (i32.const 4))";
    let id = ta.upload(&module("", body, "i32.const 1")).unwrap();
    for _ in 0..3 {
        assert_eq!(
            ta.public_key(&id, &address, 1),
            Ok(1u32.to_le_bytes().to_vec())
        );
    }
}

#[test]
fn install_limits_and_unload() {
    let (mut ta, address) = setup();
    let ids: Vec<ProgramId> = (0..4)
        .map(|i| ta.upload(&leaky(&i.to_string())).unwrap())
        .collect();
    // Re-uploading an installed program is fine and changes nothing.
    assert_eq!(ta.upload(&leaky("0")), Ok(ids[0]));
    assert_eq!(ta.upload(&leaky("x")), Err(ErrorCode::TooManyPrograms));

    let mut sorted = ids.clone();
    sorted.sort();
    assert_eq!(ta.programs(), sorted);

    ta.call(&request::unload(&ids[0])).unwrap();
    assert_eq!(
        ta.call(&request::unload(&ids[0])),
        Err(ErrorCode::UnknownProgram)
    );
    assert_eq!(
        ta.public_key(&ids[0], &address, 1),
        Err(ErrorCode::UnknownProgram)
    );
    ta.upload(&leaky("x")).unwrap();
}

/// Admits one unsigned module, with no capabilities.
struct StrictPolicy(ProgramId);

impl WasmPolicy for StrictPolicy {
    fn admit(&self, program: &ProgramId, _: Option<&Signed>) -> Result<Admission, WasmError> {
        if *program == self.0 {
            Ok(Admission {
                capabilities: Capabilities::default(),
                manifest: None,
            })
        } else {
            Err(WasmError::InstallDenied)
        }
    }
    fn verify_revocations(&self, _: &Signed) -> Result<Revocations, WasmError> {
        Err(WasmError::BadSignature)
    }
    fn allow_use_key(&self, _: &ProgramId, _: &str, _: u8) -> bool {
        true
    }
}

#[test]
fn policy_is_enforced() {
    let allowed = leaky("allowed");
    let mut ta = Ta::with(
        VaultConfig::default(),
        StrictPolicy(ProgramId::of(&allowed)),
    );
    let address = ta.seed(KeyAlgorithm::Falcon1024, 9);

    assert_eq!(ta.upload(&leaky("other")), Err(ErrorCode::InstallDenied));
    let program = ta.upload(&allowed).unwrap();
    assert_eq!(
        ta.public_key(&program, &address, 0),
        Err(ErrorCode::KeyUseDenied)
    );
    assert!(ta.public_key(&program, &address, 1).is_ok());
}

#[test]
fn malformed_requests_and_version() {
    let (mut ta, _) = setup();
    for bad in [
        vec![0x20],
        vec![0x20, 1, 2, 3, 4, 5],
        vec![0x23, 0],
        vec![0x24, 0],
        vec![0x2F],
        [request::public_key(&ProgramId([0; 32]), 1, "A"), vec![0]].concat(),
        // Declared message length longer than the bytes sent.
        request::sign(&ProgramId([0; 32]), 1, "A", b"abc")[..40].to_vec(),
    ] {
        assert_eq!(ta.call(&bad), Err(ErrorCode::MalformedRequest), "{bad:?}");
    }
    assert_eq!(ta.call(&request::get_version()), Ok(vec![0, 0, 4, 0]));
    assert!(seed_vault_wasm::protocol::is_wasm_request(
        &request::get_version()
    ));
    assert!(!seed_vault_wasm::protocol::is_wasm_request(&[0x07]));
}

// --- Persistence ---------------------------------------------------------

#[test]
fn programs_survive_a_restart_and_unload_removes_them_for_good() {
    let (mut ta, address) = setup();
    let a = ta.upload(&leaky("a")).unwrap();
    let b = ta.upload(&leaky("b")).unwrap();
    let key = ta.public_key(&a, &address, 1).unwrap();

    let mut ta = ta.restart(VaultConfig::default(), PublisherPolicy::dev());
    let mut expected = vec![a, b];
    expected.sort();
    assert_eq!(ta.programs(), expected);
    // Usable straight away, same key, no re-upload.
    assert_eq!(ta.public_key(&a, &address, 1).unwrap(), key);

    ta.call(&request::unload(&a)).unwrap();
    let mut ta = ta.restart(VaultConfig::default(), PublisherPolicy::dev());
    assert_eq!(ta.programs(), vec![b]);
}

#[test]
fn tampered_saved_programs_are_dropped_on_restore() {
    let (mut ta, _) = setup();
    let a = ta.upload(&leaky("a")).unwrap();
    let name = ta.stored("svw1_prog_").pop().unwrap();
    // Swap in different bytes under the same name.
    ta.wasm_store.write(&name, &leaky("evil")).unwrap();

    let mut ta = ta.restart(VaultConfig::default(), PublisherPolicy::dev());
    assert!(ta.programs().is_empty());
    assert!(ta.stored("svw1_prog_").is_empty());
    assert_eq!(
        ta.call(&request::unload(&a)),
        Err(ErrorCode::UnknownProgram)
    );
}

#[test]
fn restore_skips_programs_the_policy_no_longer_allows() {
    let allowed = leaky("allowed");
    let mut ta = Ta::new();
    ta.upload(&allowed).unwrap();
    let other = ta.upload(&leaky("other")).unwrap();

    let policy = StrictPolicy(ProgramId::of(&allowed));
    let Ta {
        accounts,
        wasm_store,
        ..
    } = ta;
    let mut ta = Ta {
        vault: seed_vault_wasm::WasmVault::new(VaultConfig::default(), policy),
        accounts,
        wasm_store,
    };
    assert_eq!(ta.programs(), vec![ProgramId::of(&allowed)]);
    // Kept in storage, in case the policy changes back.
    assert_eq!(ta.stored("svw1_prog_").len(), 2);
    let _ = other;
}

// --- Key cache -----------------------------------------------------------

#[test]
fn cached_keys_are_reused_across_calls_and_restarts() {
    let (mut ta, address) = setup();
    let program = ta.upload(&caching("c")).unwrap();

    assert_eq!(ta.public_key(&program, &address, 1), Ok(vec![0]), "miss");
    assert_eq!(ta.public_key(&program, &address, 1), Ok(vec![1]), "hit");
    // Per key: another index or seed misses.
    assert_eq!(ta.public_key(&program, &address, 2), Ok(vec![0]));
    let other_seed = ta.seed(KeyAlgorithm::Algo25Ed25519, 5);
    assert_eq!(ta.public_key(&program, &other_seed, 1), Ok(vec![0]));

    let mut ta = ta.restart(VaultConfig::default(), PublisherPolicy::dev());
    assert_eq!(
        ta.public_key(&program, &address, 1),
        Ok(vec![1]),
        "persisted"
    );
}

#[test]
fn a_program_cannot_read_another_programs_cache() {
    let (mut ta, address) = setup();
    let a = ta.upload(&caching("a")).unwrap();
    let b = ta.upload(&caching("b")).unwrap();
    assert_eq!(ta.public_key(&a, &address, 1), Ok(vec![0]));
    assert_eq!(ta.public_key(&a, &address, 1), Ok(vec![1]));
    assert_eq!(ta.public_key(&b, &address, 1), Ok(vec![0]));
}

#[test]
fn cache_is_only_saved_when_the_call_succeeds() {
    let (mut ta, address) = setup();
    let program = ta.upload(&caching("c")).unwrap();
    // `sign` caches, then fails.
    assert_eq!(
        ta.sign(&program, &address, 1, b"m"),
        Err(ErrorCode::ProgramFailed)
    );
    assert!(ta.stored("svw1_key_").is_empty());
    assert_eq!(ta.public_key(&program, &address, 1), Ok(vec![0]));
}

#[test]
fn cached_keys_are_deleted_with_their_program() {
    let (mut ta, address) = setup();
    let a = ta.upload(&caching("a")).unwrap();
    let b = ta.upload(&caching("b")).unwrap();
    ta.public_key(&a, &address, 1).unwrap();
    ta.public_key(&b, &address, 1).unwrap();
    assert_eq!(ta.stored("svw1_key_").len(), 2);

    ta.call(&request::unload(&a)).unwrap();
    let left = ta.stored("svw1_key_");
    assert_eq!(left.len(), 1);
    assert!(left[0].contains(&format!("{:?}", b)[10..74]));
}

#[test]
fn cached_keys_are_deleted_with_their_seed() {
    let (mut ta, address) = setup();
    let kept = ta.seed(KeyAlgorithm::Algo25Ed25519, 5);
    let program = ta.upload(&caching("c")).unwrap();
    ta.public_key(&program, &address, 1).unwrap();
    ta.public_key(&program, &address, 2).unwrap();
    ta.public_key(&program, &kept, 1).unwrap();
    assert_eq!(ta.stored("svw1_key_").len(), 3);

    delete_account(&mut ta.accounts, &address).unwrap();
    ta.vault
        .purge_orphaned_keys(&mut ta.accounts, &mut ta.wasm_store)
        .unwrap();
    assert_eq!(ta.stored("svw1_key_").len(), 1);
    assert_eq!(ta.public_key(&program, &kept, 1), Ok(vec![1]));
}

#[test]
fn cache_size_and_count_are_capped() {
    let config = VaultConfig {
        max_cached_keys_per_program: 2,
        ..VaultConfig::default()
    };
    let mut ta = Ta::with(config, PublisherPolicy::dev());
    let address = ta.seed(KeyAlgorithm::Falcon1024, 9);

    // Oversized `cache_put` is refused (-1, returned as the output byte).
    let oversized = module(
        "",
        "(i32.store8 (i32.const 0) (call $cache_put (i32.const 0) (i32.const 8193)))
         (call $output (i32.const 0) (i32.const 1))",
        "i32.const 1",
    );
    let id = ta.upload(&oversized).unwrap();
    assert_eq!(ta.public_key(&id, &address, 1), Ok(vec![0xFF]));
    assert!(ta.stored("svw1_key_").is_empty());

    // Past the per-program count, new keys just aren't cached.
    let program = ta.upload(&caching("c")).unwrap();
    for index in 1..=3 {
        assert_eq!(ta.public_key(&program, &address, index), Ok(vec![0]));
    }
    assert_eq!(ta.stored("svw1_key_").len(), 2);
    assert_eq!(ta.public_key(&program, &address, 1), Ok(vec![1]));
    assert_eq!(ta.public_key(&program, &address, 3), Ok(vec![0]));
}

#[test]
fn cache_get_needs_a_big_enough_buffer() {
    let (mut ta, address) = setup();
    // `public_key` caches its 32-byte key; `sign` reports `cache_get`'s
    // result with a 16-byte and a 64-byte buffer.
    let program = ta
        .upload(&module(
            "",
            "(drop (call $key (i32.const 100) (i32.const 32)))
             (drop (call $cache_put (i32.const 100) (i32.const 32)))
             (call $output (i32.const 0) (i32.const 1))",
            "(i32.store8 (i32.const 0) (call $cache_get (i32.const 100) (i32.const 16)))
             (i32.store8 (i32.const 1) (call $cache_get (i32.const 200) (i32.const 64)))
             (call $output (i32.const 0) (i32.const 2))",
        ))
        .unwrap();
    assert_eq!(ta.sign(&program, &address, 1, b""), Ok(vec![0, 0]), "empty");
    ta.public_key(&program, &address, 1).unwrap();
    assert_eq!(ta.sign(&program, &address, 1, b""), Ok(vec![0xFF, 32]));
}

// --- Account entropy -----------------------------------------------------

/// Outputs `account_entropy` for the requested index (`sign` instead
/// reports `account_entropy`'s result with too small a buffer).
fn entropy_reader() -> Vec<u8> {
    wat::parse_str(format!(
        r#"(module
            (import "seed_vault" "account_entropy" (func $entropy (param i32 i32) (result i32)))
            {IMPORTS}
            (func (export "public_key") (result i32)
                (drop (call $entropy (i32.const 0) (i32.const 32)))
                (call $output (i32.const 0) (i32.const 32)))
            (func (export "sign") (result i32)
                (i32.store8 (i32.const 0) (call $entropy (i32.const 0) (i32.const 31)))
                (call $output (i32.const 0) (i32.const 1))))"#
    ))
    .unwrap()
}

#[test]
fn account_entropy_is_the_native_key_material_never_more() {
    let (mut ta, address) = setup();
    let program = ta.upload(&entropy_reader()).unwrap();
    // Index 0's key material is the seed itself (hence `human_sign`)...
    assert_eq!(ta.public_key(&program, &address, 0), Ok(vec![9; 32]));
    // ...autosign indices get their one-way child, as native code uses.
    for index in [1u8, 2, 255] {
        let expected = seed_vault::store::falcon_key_entropy(&[9; 32], index.into()).unwrap();
        assert_eq!(
            ta.public_key(&program, &address, index),
            Ok(expected.to_vec())
        );
        assert_ne!(expected, [9; 32]);
    }
    // Too small a buffer is refused (-1).
    assert_eq!(ta.sign(&program, &address, 1, b""), Ok(vec![0xFF]));
}

#[test]
fn importing_account_entropy_needs_the_capability() {
    // Install: StrictPolicy admits the module, but without the capability.
    let module = entropy_reader();
    let mut ta = Ta::with(VaultConfig::default(), StrictPolicy(ProgramId::of(&module)));
    assert_eq!(ta.upload(&module), Err(ErrorCode::InstallDenied));
    assert!(ta.stored("svw1_prog_").is_empty());

    // Restore: installed in dev mode, then the policy no longer grants it.
    let (mut ta, address) = setup();
    let program = ta.upload(&module).unwrap();
    let Ta {
        accounts,
        wasm_store,
        ..
    } = ta;
    let mut ta = Ta {
        vault: seed_vault_wasm::WasmVault::new(
            VaultConfig::default(),
            StrictPolicy(ProgramId::of(&module)),
        ),
        accounts,
        wasm_store,
    };
    assert!(ta.programs().is_empty());
    assert_eq!(
        ta.public_key(&program, &address, 1),
        Err(ErrorCode::UnknownProgram)
    );
    // Kept in storage, in case the policy changes back.
    assert_eq!(ta.stored("svw1_prog_").len(), 1);
}

/// Allows everything, but can stop allowing key use (e.g. a user-auth check).
struct SwitchablePolicy(std::cell::Cell<bool>);

impl WasmPolicy for SwitchablePolicy {
    fn admit(&self, _: &ProgramId, _: Option<&Signed>) -> Result<Admission, WasmError> {
        Ok(Admission {
            capabilities: Capabilities::ALL,
            manifest: None,
        })
    }
    fn verify_revocations(&self, _: &Signed) -> Result<Revocations, WasmError> {
        Err(WasmError::BadSignature)
    }
    fn allow_use_key(&self, _: &ProgramId, _: &str, _: u8) -> bool {
        self.0.get()
    }
}

#[test]
fn the_policy_is_asked_on_every_key_use() {
    let mut ta = Ta::with(VaultConfig::default(), SwitchablePolicy(true.into()));
    let address = ta.seed(KeyAlgorithm::Falcon1024, 9);
    let program = ta.upload(&leaky("a")).unwrap();
    assert!(ta.public_key(&program, &address, 1).is_ok());
    ta.vault_policy().0.set(false);
    assert_eq!(
        ta.public_key(&program, &address, 1),
        Err(ErrorCode::KeyUseDenied)
    );
}

#[test]
fn programs_without_account_entropy_never_get_key_material() {
    let (mut ta, address) = setup();
    // Asks for its index; its key is still the derived one, not the seed.
    let program = ta
        .upload(&module(
            "",
            "(i32.store8 (i32.const 0) (call $address_index))
             (drop (call $key (i32.const 1) (i32.const 32)))
             (call $output (i32.const 0) (i32.const 33))",
            "i32.const 1",
        ))
        .unwrap();
    let out = ta.public_key(&program, &address, 7).unwrap();
    assert_eq!(out[0], 7);
    assert_eq!(
        out[1..],
        derive_program_key(&[9; 32], &KeyNamespace::program(&program), 7)
    );
    assert_ne!(out[1..], [9; 32]);
}

// --- run, state and chunked input ------------------------------------------

/// A `run`-only module: a counter in its state. Input byte 0: 1 = increment
/// and succeed, 2 = increment then fail, 3 = put an oversized state.
/// Outputs the counter (as stored *before* this call, plus 1 if incremented).
fn counter() -> Vec<u8> {
    wat::parse_str(
        r#"(module
            (import "seed_vault" "input" (func $input (param i32 i32) (result i32)))
            (import "seed_vault" "output" (func $output (param i32 i32) (result i32)))
            (import "seed_vault" "key" (func $key (param i32 i32) (result i32)))
            (import "seed_vault" "state_get" (func $state_get (param i32 i32) (result i32)))
            (import "seed_vault" "state_put" (func $state_put (param i32 i32) (result i32)))
            (memory (export "memory") 2)
            (func (export "run") (result i32)
                (drop (call $input (i32.const 0) (i32.const 1)))
                (drop (call $state_get (i32.const 8) (i32.const 4)))
                (i32.store (i32.const 8) (i32.add (i32.load (i32.const 8)) (i32.const 1)))
                (if (i32.eq (i32.load8_u (i32.const 0)) (i32.const 3))
                    (then (return (call $state_put (i32.const 0) (i32.const 65537)))))
                (drop (call $state_put (i32.const 8) (i32.const 4)))
                ;; A run has no key.
                (i32.store8 (i32.const 12) (call $key (i32.const 16) (i32.const 32)))
                (drop (call $output (i32.const 8) (i32.const 5)))
                (i32.ne (i32.load8_u (i32.const 0)) (i32.const 1))))"#,
    )
    .unwrap()
}

fn count(out: &[u8]) -> u32 {
    assert_eq!(out[4], 0xFF, "run must not get a key");
    u32::from_le_bytes(out[..4].try_into().unwrap())
}

#[test]
fn run_state_is_saved_only_on_success() {
    let (mut ta, _) = setup();
    let program = ta.upload(&counter()).unwrap();
    assert_eq!(count(&ta.run(&program, &[1]).unwrap()), 1);
    assert_eq!(count(&ta.run(&program, &[1]).unwrap()), 2);
    // Increments, then fails: not saved.
    assert_eq!(ta.run(&program, &[2]), Err(ErrorCode::ProgramFailed));
    // Oversized state is refused (-1), and the call fails.
    assert_eq!(ta.run(&program, &[3]), Err(ErrorCode::ProgramFailed));
    assert_eq!(count(&ta.run(&program, &[1]).unwrap()), 3);

    // Persists, and is per program.
    let mut ta = ta.restart(VaultConfig::default(), PublisherPolicy::dev());
    assert_eq!(count(&ta.run(&program, &[1]).unwrap()), 4);
    let other = ta
        .upload(&[counter().as_slice(), &[0, 2, 1, b'x']].concat())
        .unwrap();
    assert_eq!(count(&ta.run(&other, &[1]).unwrap()), 1);

    // Unloading deletes it.
    ta.call(&request::unload(&program)).unwrap();
    let program = ta.upload(&counter()).unwrap();
    assert_eq!(count(&ta.run(&program, &[1]).unwrap()), 1);
}

#[test]
fn chunked_run_input_is_checked() {
    let config = VaultConfig {
        max_input_len: 10_000,
        ..VaultConfig::default()
    };
    let mut ta = Ta::with(config, PublisherPolicy::dev());
    let program = ta.upload(&counter()).unwrap();
    assert_eq!(
        ta.call(&request::input_begin(10_001)),
        Err(ErrorCode::InputTooLarge)
    );
    let id = ta.call(&request::input_begin(5)).unwrap()[0];
    assert_eq!(
        ta.call(&request::input_chunk(id, 1, b"x")),
        Err(ErrorCode::BadChunk)
    );
    ta.call(&request::input_chunk(id, 0, &[1, 0])).unwrap();
    // Incomplete.
    assert_eq!(
        ta.call(&request::run_input(&program, id)),
        Err(ErrorCode::BadChunk)
    );
    ta.call(&request::input_chunk(id, 2, &[0, 0, 0])).unwrap();
    let out = ta.call(&request::run_input(&program, id)).unwrap();
    assert_eq!(count(&bytes_payload(&out)), 1);
    // Consumed.
    assert_eq!(
        ta.call(&request::run_input(&program, id)),
        Err(ErrorCode::UnknownUpload)
    );
    // Through the harness, an input needing several messages.
    let big = [&[1u8][..], &[0u8; 9_000]].concat();
    assert_eq!(count(&ta.run(&program, &big).unwrap()), 2);
}
