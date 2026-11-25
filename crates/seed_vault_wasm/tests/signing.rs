//! Publisher-signed modules: manifests, capabilities, upgrades, rollback
//! protection and revocation.
// These drive the TA side, which needs the default `vault` feature.
#![cfg(feature = "vault")]

mod common;

use common::*;
use seed_vault::KeyAlgorithm;
use seed_vault_wasm::manifest::{Capabilities, Manifest, PublisherKey, Signed};
use seed_vault_wasm::policy::{publisher_keypair, DEV_KEY_ID};
use seed_vault_wasm::protocol::{request, ErrorCode};
use seed_vault_wasm::{
    derive_program_key, KeyNamespace, ProgramId, PublisherPolicy, VaultConfig, WasmStore,
};

/// "Public key" = its raw key, so tests can see which key a module got.
/// `tag` makes distinct modules (versions).
fn module(tag: &str) -> Vec<u8> {
    wat::parse_str(format!(
        r#"(module
            (import "seed_vault" "key" (func $key (param i32 i32) (result i32)))
            (import "seed_vault" "output" (func $output (param i32 i32) (result i32)))
            (memory (export "memory") 1)
            (data (i32.const 60000) "{tag}")
            (func (export "public_key") (result i32)
                (drop (call $key (i32.const 0) (i32.const 32)))
                (call $output (i32.const 0) (i32.const 32)))
            (func (export "sign") (result i32) i32.const 1))"#
    ))
    .unwrap()
}

/// A module that reads `account_entropy`.
fn entropy_module(tag: &str) -> Vec<u8> {
    wat::parse_str(format!(
        r#"(module
            (import "seed_vault" "account_entropy" (func $e (param i32 i32) (result i32)))
            (import "seed_vault" "output" (func $output (param i32 i32) (result i32)))
            (memory (export "memory") 1)
            (data (i32.const 60000) "{tag}")
            (func (export "public_key") (result i32)
                (drop (call $e (i32.const 0) (i32.const 32)))
                (call $output (i32.const 0) (i32.const 32)))
            (func (export "sign") (result i32) i32.const 1))"#
    ))
    .unwrap()
}

const NONE: Capabilities = Capabilities {
    account_entropy: false,
    human_sign: false,
};

fn signed_ta() -> (Ta, String) {
    let mut ta = Ta::signed_only();
    let address = ta.seed(KeyAlgorithm::Falcon1024, 9);
    (ta, address)
}

fn restart(ta: Ta) -> Ta {
    ta.restart(
        VaultConfig::default(),
        PublisherPolicy::new(vec![seed_vault_wasm::dev_publisher_key()]),
    )
}

#[test]
fn signed_only_mode_refuses_unsigned_modules() {
    let (mut ta, _) = signed_ta();
    assert_eq!(ta.upload(&module("a")), Err(ErrorCode::InstallDenied));
    let m = module("a");
    ta.upload_signed(&m, &manifest_for(&m, "algo", 1, NONE))
        .unwrap();
}

#[test]
fn forged_or_mismatched_manifests_are_refused() {
    let (mut ta, _) = signed_ta();
    let m = module("a");

    // For a different module.
    let other = manifest_for(&module("b"), "algo", 1, NONE);
    assert_eq!(ta.upload_signed(&m, &other), Err(ErrorCode::BadSignature));

    // Tampered after signing: e.g. a module granting itself a capability.
    let mut tampered = manifest_for(&m, "algo", 1, NONE);
    let caps_at = tampered.body.len() - 1 - "algo".len() - 1;
    tampered.body[caps_at] = 0b11;
    assert_eq!(
        ta.upload_signed(&m, &tampered),
        Err(ErrorCode::BadSignature)
    );

    // Signed by a key the TA doesn't trust, claiming the dev key's ID.
    let (_, impostor) = publisher_keypair(&[7; 32]);
    let body = Manifest {
        key_id: DEV_KEY_ID,
        program_id: ProgramId::of(&m),
        name: "algo".into(),
        version: 1,
        capabilities: NONE,
    }
    .encode();
    let forged = Signed::sign(body, &impostor);
    assert_eq!(ta.upload_signed(&m, &forged), Err(ErrorCode::BadSignature));

    // An unknown key ID.
    let body = Manifest {
        key_id: 1,
        program_id: ProgramId::of(&m),
        name: "algo".into(),
        version: 1,
        capabilities: NONE,
    }
    .encode();
    assert_eq!(
        ta.upload_signed(&m, &dev_sign(body)),
        Err(ErrorCode::BadSignature)
    );

    // Garbage.
    let id = ta.call(&request::load_begin(m.len() as u32)).unwrap()[0];
    ta.call(&request::load_chunk(id, 0, &m)).unwrap();
    assert_eq!(
        ta.call(&[&[0x22, id][..], &[1, 2, 3]].concat()),
        Err(ErrorCode::MalformedRequest)
    );

    assert!(ta.programs().is_empty());
    assert!(ta.stored("").iter().all(|n| !n.starts_with("svw1_prog_")));
}

#[test]
fn capabilities_come_from_the_manifest() {
    let (mut ta, address) = signed_ta();

    // Importing account_entropy needs the capability.
    let e = entropy_module("e");
    assert_eq!(
        ta.upload_signed(&e, &manifest_for(&e, "native", 1, NONE)),
        Err(ErrorCode::InstallDenied)
    );
    let entropy_only = Capabilities {
        account_entropy: true,
        human_sign: false,
    };
    let program = ta
        .upload_signed(&e, &manifest_for(&e, "native", 1, entropy_only))
        .unwrap();
    let expected = seed_vault::store::falcon_key_entropy(&[9; 32], 1).unwrap();
    assert_eq!(ta.public_key(&program, &address, 1), Ok(expected.to_vec()));

    // Index 0 (the seed itself) needs `human_sign`.
    assert_eq!(
        ta.public_key(&program, &address, 0),
        Err(ErrorCode::KeyUseDenied)
    );
    let m = module("m");
    let plain = ta
        .upload_signed(&m, &manifest_for(&m, "plain", 1, NONE))
        .unwrap();
    assert_eq!(
        ta.public_key(&plain, &address, 0),
        Err(ErrorCode::KeyUseDenied)
    );

    let listed = ta.listed();
    let native = listed.iter().find(|p| p.id == program).unwrap();
    assert!(native.signed && native.account_entropy && !native.human_sign);
    assert_eq!((native.name.as_str(), native.version), ("native", 1));
}

#[test]
fn a_new_version_replaces_the_old_and_keeps_its_accounts() {
    let (mut ta, address) = signed_ta();
    let (v1, v2) = (module("v1"), module("v2"));
    let id1 = ta
        .upload_signed(&v1, &manifest_for(&v1, "algo", 1, NONE))
        .unwrap();
    let key1 = ta.public_key(&id1, &address, 1).unwrap();
    // Signed keys are bound to the algorithm name, not the module bytes.
    assert_eq!(
        key1,
        derive_program_key(&[9; 32], &KeyNamespace::algorithm("algo"), 1)
    );

    let id2 = ta
        .upload_signed(&v2, &manifest_for(&v2, "algo", 2, NONE))
        .unwrap();
    assert_eq!(ta.programs(), vec![id2]);
    assert_eq!(ta.public_key(&id2, &address, 1).unwrap(), key1);
    assert_eq!(
        ta.public_key(&id1, &address, 1),
        Err(ErrorCode::UnknownProgram)
    );
    assert!(!ta
        .stored("svw1_prog_")
        .iter()
        .any(|n| n.contains(&hex(&id1))));

    // A different algorithm name: different keys, and both stay installed.
    let other = module("other");
    let id3 = ta
        .upload_signed(&other, &manifest_for(&other, "other", 1, NONE))
        .unwrap();
    assert_ne!(ta.public_key(&id3, &address, 1).unwrap(), key1);
    assert_eq!(ta.programs().len(), 2);
}

#[test]
fn rollback_to_a_lower_version_is_refused_even_after_a_restart() {
    let (mut ta, _) = signed_ta();
    let (v1, v2) = (module("v1"), module("v2"));
    ta.upload_signed(&v2, &manifest_for(&v2, "algo", 2, NONE))
        .unwrap();
    assert_eq!(
        ta.upload_signed(&v1, &manifest_for(&v1, "algo", 1, NONE)),
        Err(ErrorCode::Rollback)
    );

    // Even after unloading v2 and restarting.
    let id2 = ProgramId::of(&v2);
    ta.call(&request::unload(&id2)).unwrap();
    let mut ta = restart(ta);
    assert_eq!(
        ta.upload_signed(&v1, &manifest_for(&v1, "algo", 1, NONE)),
        Err(ErrorCode::Rollback)
    );
    // The same version again is fine.
    ta.upload_signed(&v2, &manifest_for(&v2, "algo", 2, NONE))
        .unwrap();
}

#[test]
fn revocation_removes_modules_and_keys_and_is_sequenced() {
    let (mut ta, address) = signed_ta();
    let (a, b) = (module("a"), module("b"));
    let id_a = ta
        .upload_signed(&a, &manifest_for(&a, "a", 1, NONE))
        .unwrap();
    let id_b = ta
        .upload_signed(&b, &manifest_for(&b, "b", 1, NONE))
        .unwrap();
    ta.public_key(&id_a, &address, 1).unwrap();

    // Sequence 0 is "none applied"; lists must increase.
    assert_eq!(
        ta.revoke(&revocations(0, &[], &[id_a])),
        Err(ErrorCode::Rollback)
    );
    assert_eq!(ta.revoke(&revocations(1, &[], &[id_a])), Ok(1));
    assert_eq!(ta.programs(), vec![id_b]);
    assert_eq!(
        ta.public_key(&id_a, &address, 1),
        Err(ErrorCode::UnknownProgram)
    );
    assert_eq!(
        ta.upload_signed(&a, &manifest_for(&a, "a", 1, NONE)),
        Err(ErrorCode::Revoked)
    );
    assert_eq!(
        ta.revoke(&revocations(1, &[], &[])),
        Err(ErrorCode::Rollback)
    );

    // Survives a restart.
    let mut ta = restart(ta);
    assert_eq!(ta.programs(), vec![id_b]);
    assert_eq!(
        ta.upload_signed(&a, &manifest_for(&a, "a", 1, NONE)),
        Err(ErrorCode::Revoked)
    );

    // Revoking a key: everything it signed goes. (A list revoking its own
    // signing key is refused, so this uses a second trusted key.)
    let (backup_public, backup_private) = publisher_keypair(&[5; 32]);
    let keys = vec![
        seed_vault_wasm::dev_publisher_key(),
        PublisherKey {
            id: 1,
            public_key: backup_public,
        },
    ];
    let mut ta = ta.restart(VaultConfig::default(), PublisherPolicy::new(keys));
    let self_revoking = revocations(2, &[DEV_KEY_ID], &[]);
    assert_eq!(ta.revoke(&self_revoking), Err(ErrorCode::BadSignature));
    let body = seed_vault_wasm::Revocations {
        key_id: 1,
        sequence: 2,
        revoked_keys: vec![DEV_KEY_ID],
        revoked_programs: vec![],
    }
    .encode();
    assert_eq!(ta.revoke(&Signed::sign(body, &backup_private)), Ok(2));
    assert!(ta.programs().is_empty());
    let c = module("c");
    assert_eq!(
        ta.upload_signed(&c, &manifest_for(&c, "c", 1, NONE)),
        Err(ErrorCode::Revoked)
    );
}

#[test]
fn tampered_storage_is_caught_on_restore() {
    let (mut ta, _) = signed_ta();
    let a = module("a");
    let id = ta
        .upload_signed(&a, &manifest_for(&a, "a", 1, NONE))
        .unwrap();

    // A stored manifest altered to grant more: the program is dropped.
    let name = format!("svw1_mfst_{}", hex(&id));
    let mut signed = Signed::decode(&ta.wasm_store.read(&name).unwrap().unwrap()).unwrap();
    let caps_at = signed.body.len() - 1 - 1 - 1;
    signed.body[caps_at] = 0b11;
    ta.wasm_store.write(&name, &signed.encode()).unwrap();
    let mut ta = restart(ta);
    assert!(ta.programs().is_empty());
    assert!(ta.stored("svw1_prog_").is_empty());

    // A stored revocation list that no longer verifies: fail closed (no
    // WASM at all) rather than forget what was revoked.
    ta.revoke(&revocations(1, &[], &[ProgramId([1; 32])]))
        .unwrap();
    let mut list = ta.wasm_store.read("svw1_revocations").unwrap().unwrap();
    let last = list.len() - 1;
    list[last] ^= 1;
    ta.wasm_store.write("svw1_revocations", &list).unwrap();
    let mut ta = restart(ta);
    assert_eq!(
        ta.call(&request::list_programs()),
        Err(ErrorCode::StoreFailure)
    );
}

#[test]
fn dev_mode_still_takes_unsigned_and_signed_modules() {
    let mut ta = Ta::new();
    let address = ta.seed(KeyAlgorithm::Falcon1024, 9);
    let unsigned = ta.upload(&module("u")).unwrap();
    let s = module("s");
    let signed = ta
        .upload_signed(&s, &manifest_for(&s, "algo", 1, NONE))
        .unwrap();
    let listed = ta.listed();
    let u = listed.iter().find(|p| p.id == unsigned).unwrap();
    assert!(!u.signed && u.account_entropy && u.human_sign);
    assert!(listed.iter().find(|p| p.id == signed).unwrap().signed);
    // Unsigned keys stay bound to the module bytes.
    assert_eq!(
        ta.public_key(&unsigned, &address, 1).unwrap(),
        derive_program_key(&[9; 32], &KeyNamespace::program(&unsigned), 1)
    );
}

fn hex(id: &ProgramId) -> String {
    use std::fmt::Write;
    id.0.iter().fold(String::new(), |mut out, b| {
        let _ = write!(out, "{b:02x}");
        out
    })
}

#[test]
fn the_tas_committed_dev_key_is_the_dev_publisher_key() {
    let committed: &[u8] =
        include_bytes!("../../../trusty/app/seed_vault_ta/publisher_keys/dev_ff.pub");
    assert_eq!(
        committed,
        seed_vault_wasm::dev_publisher_key().public_key.0.as_slice()
    );
}
