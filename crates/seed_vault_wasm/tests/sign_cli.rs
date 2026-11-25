//! The publisher CLI (`seed_vault_wasm_sign`), end to end against the vault.
// These drive the TA side, which needs the default `vault` feature.
#![cfg(feature = "vault")]

mod common;

use std::path::{Path, PathBuf};
use std::process::Command;

use common::*;
use seed_vault::KeyAlgorithm;
use seed_vault_wasm::protocol::ErrorCode;
use seed_vault_wasm::Signed;

fn cli(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_seed_vault_wasm_sign"))
        .args(args)
        .output()
        .expect("run seed_vault_wasm_sign")
}

fn ok(args: &[&str]) -> String {
    let out = cli(args);
    assert!(
        out.status.success(),
        "{args:?}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).unwrap()
}

fn tempdir() -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "seed_vault_wasm_sign_test_{}_{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn s(path: &Path) -> &str {
    path.to_str().unwrap()
}

#[test]
fn sign_with_the_cli_install_and_revoke() {
    let dir = tempdir();
    let (seed, public) = (dir.join("dev.seed"), dir.join("dev.pub"));
    ok(&["dev-seed", s(&seed)]);
    ok(&["pubkey", s(&seed), s(&public)]);
    // The CLI's dev key is the one the TA's dev setup trusts.
    assert_eq!(
        std::fs::read(&public).unwrap(),
        seed_vault_wasm::dev_publisher_key().public_key.0.to_vec()
    );

    let module = dir.join("ed25519.wasm");
    std::fs::write(&module, include_bytes!("fixtures/guest_ed25519.wasm")).unwrap();
    let manifest = dir.join("ed25519.svwm");
    ok(&[
        "sign",
        s(&seed),
        "255",
        s(&module),
        "ed25519-demo",
        "3",
        "none",
        s(&manifest),
    ]);
    let verified = ok(&["verify", s(&public), "255", s(&manifest), s(&module)]);
    assert!(verified.contains("valid manifest") && verified.contains("matches"));

    let mut ta = Ta::signed_only();
    let address = ta.seed(KeyAlgorithm::Algo25Ed25519, 1);
    let signed = Signed::decode(&std::fs::read(&manifest).unwrap()).unwrap();
    let bytes = std::fs::read(&module).unwrap();
    let program = ta.upload_signed(&bytes, &signed).unwrap();
    let listed = ta.listed();
    assert_eq!(
        (listed[0].name.as_str(), listed[0].version),
        ("ed25519-demo", 3)
    );
    assert!(ta.public_key(&program, &address, 1).is_ok());
    // "none": no index 0.
    assert_eq!(
        ta.public_key(&program, &address, 0),
        Err(ErrorCode::KeyUseDenied)
    );

    let list = dir.join("revocations.svwr");
    ok(&[
        "revoke",
        s(&seed),
        "255",
        "1",
        s(&list),
        &format!("module:{}", s(&module)),
    ]);
    assert!(ok(&["verify", s(&public), "255", s(&list)]).contains("valid revocation list"));
    let list = Signed::decode(&std::fs::read(&list).unwrap()).unwrap();
    assert_eq!(ta.revoke(&list), Ok(1));
    assert!(ta.programs().is_empty());

    // A freshly generated key isn't trusted by the TA.
    let (seed2, public2) = (dir.join("new.seed"), dir.join("new.pub"));
    ok(&["keygen", s(&seed2), s(&public2)]);
    let other = dir.join("other.svwm");
    ok(&[
        "sign",
        s(&seed2),
        "255",
        s(&module),
        "x",
        "1",
        "none",
        s(&other),
    ]);
    assert!(!cli(&["verify", s(&public), "255", s(&other)])
        .status
        .success());

    // Bad input fails cleanly.
    assert!(!cli(&[
        "sign",
        s(&seed),
        "255",
        s(&module),
        "Bad Name",
        "1",
        "none",
        s(&other)
    ])
    .status
    .success());
    assert!(!cli(&[
        "sign",
        s(&seed),
        "255",
        s(&module),
        "x",
        "1",
        "superuser",
        s(&other)
    ])
    .status
    .success());
    std::fs::remove_dir_all(dir).unwrap();
}
