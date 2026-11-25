//! `SeedVaultClient::emulated`, used the way an Android app without the
//! Trusty applet would: its own seed vault behind `SeedProvider`, programs in
//! `file_wasm_storage`, the same `wasm_*`/`light_client_*` calls as on a TA.
#![cfg(feature = "emulator")]

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use base64::Engine;
use seed_vault::memory_store::MemoryStore;
use seed_vault::{HUMAN_SIGN_INDEX, KeyAlgorithm, import_account, list_accounts};
use seed_vault_ffi::{
    LightBlockHeaderProof, LightClientErrorCode, SeedProvider, SeedVaultClient, SeedVaultError,
    StateProofMessage, TransactionProof, WasmErrorCode, WasmStorage, dev_publisher_public_key,
    file_wasm_storage,
};
use seed_vault_wasm::ProgramId;
use seed_vault_wasm::manifest::{Capabilities, Manifest, Signed};
use seed_vault_wasm::policy::{DEV_KEY_ID, dev_publisher_seed, publisher_keypair};
use serde_json::Value;

const FIXTURES: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../seed_vault_wasm/tests/fixtures/"
);

fn fixture(name: &str) -> Vec<u8> {
    std::fs::read(format!("{FIXTURES}{name}")).unwrap()
}

/// Stands in for the app's Room/StrongBox vault: seeds by index-0 address,
/// and can be "locked" (as if Keystore wanted a fresh unlock).
#[derive(Default)]
struct AppVault {
    store: Mutex<MemoryStore>,
    locked: Mutex<bool>,
}

impl AppVault {
    fn add(&self, algorithm: KeyAlgorithm, entropy: [u8; 32]) -> String {
        let mnemonic = words(&entropy);
        import_account(
            &mut *self.store.lock().unwrap(),
            algorithm,
            HUMAN_SIGN_INDEX,
            &mnemonic,
        )
        .unwrap()
        .address
    }
}

/// The 25 words for `entropy`, via seed_vault (an Ed25519 record's key
/// material *is* the entropy).
fn words(entropy: &[u8; 32]) -> String {
    let mut store = MemoryStore::default();
    use seed_vault::AccountStore;
    store
        .persist_record(
            0,
            &seed_vault::AccountRecord {
                algorithm: KeyAlgorithm::Algo25Ed25519,
                key_material: entropy.to_vec(),
            },
        )
        .unwrap();
    let address = list_accounts(&mut store).unwrap()[0].address.clone();
    seed_vault::reveal_mnemonic(&mut store, &address).unwrap()
}

impl SeedProvider for AppVault {
    fn seed_entropy(&self, address: String) -> Result<Option<Vec<u8>>, SeedVaultError> {
        if *self.locked.lock().unwrap() {
            return Err(SeedVaultError::PermissionDenied {
                reason: "unlock required".into(),
            });
        }
        match seed_vault::seed_entropy(&mut *self.store.lock().unwrap(), &address) {
            Ok(entropy) => Ok(Some(entropy.to_vec())),
            Err(seed_vault::StoreError::NotFound) => Ok(None),
            Err(e) => Err(SeedVaultError::InvalidArgument {
                reason: format!("{e:?}"),
            }),
        }
    }

    // Addresses are public: works even while the vault is locked.
    fn seed_addresses(&self) -> Result<Vec<String>, SeedVaultError> {
        Ok(list_accounts(&mut *self.store.lock().unwrap())
            .unwrap()
            .into_iter()
            .filter(|a| a.address_index == 0)
            .map(|a| a.address)
            .collect())
    }
}

/// In-memory `WasmStorage`, implemented app-side.
#[derive(Default)]
struct MemoryStorage(Mutex<HashMap<String, Vec<u8>>>);

impl WasmStorage for MemoryStorage {
    fn list(&self, prefix: String) -> Result<Vec<String>, SeedVaultError> {
        Ok(self
            .0
            .lock()
            .unwrap()
            .keys()
            .filter(|k| k.starts_with(&prefix))
            .cloned()
            .collect())
    }
    fn read(&self, name: String) -> Result<Option<Vec<u8>>, SeedVaultError> {
        Ok(self.0.lock().unwrap().get(&name).cloned())
    }
    fn write(&self, name: String, data: Vec<u8>) -> Result<(), SeedVaultError> {
        self.0.lock().unwrap().insert(name, data);
        Ok(())
    }
    fn delete(&self, name: String) -> Result<(), SeedVaultError> {
        self.0.lock().unwrap().remove(&name);
        Ok(())
    }
}

fn temp_dir(name: &str) -> String {
    let dir = std::env::temp_dir().join(format!(
        "seed_vault_ffi_emulated_{name}_{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    dir.to_str().unwrap().to_string()
}

fn signed_manifest(module: &[u8], name: &str, capabilities: Capabilities) -> Vec<u8> {
    let (_, private_key) = publisher_keypair(&dev_publisher_seed());
    Signed::sign(
        Manifest {
            key_id: DEV_KEY_ID,
            program_id: ProgramId::of(module),
            name: name.into(),
            version: 1,
            capabilities,
        }
        .encode(),
        &private_key,
    )
    .encode()
}

/// Production-like: signed modules only, trusting the dev publisher key.
fn emulated(storage: Arc<dyn WasmStorage>, vault: Arc<AppVault>) -> SeedVaultClient {
    SeedVaultClient::emulated(storage, vault, vec![dev_publisher_public_key()], false).unwrap()
}

#[test]
fn emulated_falcon_signing_matches_the_vaults_native_accounts() {
    let vault = Arc::new(AppVault::default());
    let address = vault.add(KeyAlgorithm::Falcon1024, [11; 32]);
    let client = emulated(Arc::new(MemoryStorage::default()), vault.clone());
    assert!(client.is_emulated());

    let module = fixture("guest_falcon1024_native.wasm");
    // Unsigned is refused when allow_unsigned is false.
    assert_eq!(
        client.wasm_install(module.clone(), None),
        Err(SeedVaultError::Wasm {
            code: WasmErrorCode::InstallDenied
        })
    );
    let program = client
        .wasm_install(
            module.clone(),
            Some(signed_manifest(&module, "falcon1024", Capabilities::ALL)),
        )
        .unwrap();
    let listed = client.wasm_list_programs().unwrap();
    assert_eq!(listed[0].name.as_deref(), Some("falcon1024"));
    assert!(listed[0].can_sign && !listed[0].can_run);

    // Same key and signature as the native Falcon account for that seed.
    let native = list_accounts(&mut *vault.store.lock().unwrap()).unwrap();
    let human_sign = native.iter().find(|a| a.address_index == 0).unwrap();
    let public_key = client
        .wasm_public_key(program.clone(), address.clone(), 0)
        .unwrap();
    assert_eq!(public_key, human_sign.public_key);
    let message = b"emulated and native agree".to_vec();
    let signature = client
        .wasm_sign(program, address.clone(), 0, message.clone())
        .unwrap();
    let native_signature = seed_vault::store::sign_arbitrary_data(
        &mut *vault.store.lock().unwrap(),
        &address,
        0,
        &message,
    )
    .unwrap()
    .signature;
    assert_eq!(signature, native_signature);
}

#[test]
fn the_apps_errors_reach_the_caller() {
    let vault = Arc::new(AppVault::default());
    let address = vault.add(KeyAlgorithm::Algo25Ed25519, [3; 32]);
    let client = SeedVaultClient::emulated(
        Arc::new(MemoryStorage::default()),
        vault.clone(),
        vec![],
        true, // dev: unsigned allowed
    )
    .unwrap();
    let program = client
        .wasm_install(fixture("guest_ed25519.wasm"), None)
        .unwrap();
    assert!(
        client
            .wasm_public_key(program.clone(), address.clone(), 1)
            .is_ok()
    );

    // The app's vault wants an unlock: its own error, not a generic one.
    *vault.locked.lock().unwrap() = true;
    assert_eq!(
        client.wasm_public_key(program.clone(), address.clone(), 1),
        Err(SeedVaultError::PermissionDenied {
            reason: "unlock required".into()
        })
    );
    *vault.locked.lock().unwrap() = false;

    // An unknown seed.
    assert_eq!(
        client.wasm_public_key(program, "NOSUCHADDRESS".into(), 1),
        Err(SeedVaultError::Wasm {
            code: WasmErrorCode::AccountNotFound
        })
    );
    // Native calls belong to the app's own vault.
    assert!(matches!(
        client.list_accounts(),
        Err(SeedVaultError::NotSupported { .. })
    ));
}

#[test]
fn deleted_seeds_lose_their_cached_wasm_keys_automatically() {
    let vault = Arc::new(AppVault::default());
    let kept = vault.add(KeyAlgorithm::Falcon1024, [12; 32]);
    let deleted = vault.add(KeyAlgorithm::Falcon1024, [13; 32]);
    let storage = Arc::new(MemoryStorage::default());
    let client = SeedVaultClient::emulated(storage.clone(), vault.clone(), vec![], true).unwrap();
    // The Falcon guest caches its key pair.
    let program = client
        .wasm_install(fixture("guest_falcon1024.wasm"), None)
        .unwrap();
    client
        .wasm_public_key(program.clone(), kept.clone(), 1)
        .unwrap();
    client
        .wasm_public_key(program.clone(), deleted.clone(), 1)
        .unwrap();
    let cached = || storage.list("svw1_key_".into()).unwrap().len();
    assert_eq!(cached(), 2);

    // The app deletes a seed from its own vault, and the vault is locked:
    // the next WASM call of any kind drops that seed's keys, no unlock needed.
    seed_vault::delete_account(&mut *vault.store.lock().unwrap(), &deleted).unwrap();
    *vault.locked.lock().unwrap() = true;
    assert_eq!(client.wasm_list_programs().unwrap().len(), 1);
    assert_eq!(cached(), 1);

    // ... including when it happened while the app wasn't running.
    *vault.locked.lock().unwrap() = false;
    let other = vault.add(KeyAlgorithm::Falcon1024, [14; 32]);
    client.wasm_public_key(program, other.clone(), 1).unwrap();
    assert_eq!(cached(), 2);
    drop(client);
    seed_vault::delete_account(&mut *vault.store.lock().unwrap(), &other).unwrap();
    let restarted = SeedVaultClient::emulated(storage.clone(), vault, vec![], true).unwrap();
    restarted.wasm_list_programs().unwrap();
    assert_eq!(cached(), 1);
}

// ---- Light client ------------------------------------------------------------------

fn json(name: &str) -> Value {
    serde_json::from_slice(&fixture(&format!("light_client/{name}"))).unwrap()
}

fn b64(v: &Value) -> Vec<u8> {
    base64::engine::general_purpose::STANDARD
        .decode(v.as_str().unwrap())
        .unwrap()
}

fn array(v: &Value) -> Vec<u8> {
    v.as_array()
        .unwrap()
        .iter()
        .map(|b| b.as_u64().unwrap() as u8)
        .collect()
}

fn verify(client: &SeedVaultClient, program: &[u8]) -> Result<(), SeedVaultError> {
    let t = json("transaction_proof_response.json");
    let h = json("light_block_header_proof_response.json");
    client.light_client_verify_transaction(
        program.to_vec(),
        array(&json("transaction_id.txt")),
        json("round.txt").as_u64().unwrap(),
        array(&json("genesis_hash.txt")),
        array(&json("seed.txt")),
        TransactionProof {
            hash_type: t["hashtype"].as_str().unwrap().into(),
            stib_hash: b64(&t["stibhash"]),
            index: t["idx"].as_u64().unwrap(),
            tree_depth: t["treedepth"].as_u64().unwrap(),
            proof: b64(&t["proof"]),
        },
        LightBlockHeaderProof {
            index: h["index"].as_u64().unwrap(),
            tree_depth: h["treedepth"].as_u64().unwrap(),
            proof: b64(&h["proof"]),
        },
    )
}

#[test]
fn emulated_light_client_with_file_storage_survives_a_new_client() {
    let dir = temp_dir("light_client");
    let vault = Arc::new(AppVault::default());
    let module = fixture("guest_light_client.wasm");
    let program = {
        let client = emulated(file_wasm_storage(dir.clone()).unwrap(), vault.clone());
        let program = client
            .wasm_install(
                module.clone(),
                Some(signed_manifest(
                    &module,
                    "light-client",
                    Capabilities::default(),
                )),
            )
            .unwrap();
        assert_eq!(client.light_client_status(program.clone()), Ok(None));
        client
            .light_client_init(
                program.clone(),
                9,
                8,
                1000,
                b64(&json("genesis_voters_commitment.txt")),
                json("genesis_voters_ln_proven_weight.txt")
                    .as_u64()
                    .unwrap(),
            )
            .unwrap();
        assert_eq!(
            verify(&client, &program),
            Err(SeedVaultError::LightClient {
                code: LightClientErrorCode::NoStateProofForRound
            })
        );
        let m = json("state_proof_message.json");
        let message = StateProofMessage {
            block_headers_commitment: b64(&m["BlockHeadersCommitment"]),
            voters_commitment: b64(&m["VotersCommitment"]),
            ln_proven_weight: m["LnProvenWeight"].as_u64().unwrap(),
            first_attested_round: m["FirstAttestedRound"].as_u64().unwrap(),
            last_attested_round: m["LastAttestedRound"].as_u64().unwrap(),
        };
        assert_eq!(
            client.light_client_advance(program.clone(), message, b64(&json("state_proof.txt"))),
            Ok(1)
        );
        program
    };

    // A new client over the same directory (e.g. the app restarted).
    let client = emulated(file_wasm_storage(dir.clone()).unwrap(), vault);
    assert_eq!(
        client
            .light_client_status(program.clone())
            .unwrap()
            .unwrap()
            .next_interval,
        1
    );
    assert_eq!(verify(&client, &program), Ok(()));
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn file_storage_rejects_names_outside_its_directory() {
    let dir = temp_dir("names");
    let storage = file_wasm_storage(dir.clone()).unwrap();
    for bad in ["../escape", "a/b", "", "UPPER", "dot.name"] {
        assert!(
            matches!(
                storage.write(bad.into(), vec![1]),
                Err(SeedVaultError::InvalidArgument { .. })
            ),
            "{bad:?}"
        );
    }
    storage.write("svw1_ok".into(), vec![1, 2]).unwrap();
    assert_eq!(storage.read("svw1_ok".into()), Ok(Some(vec![1, 2])));
    assert_eq!(
        storage.list("svw1_".into()),
        Ok(vec!["svw1_ok".to_string()])
    );
    storage.delete("svw1_ok".into()).unwrap();
    storage.delete("svw1_ok".into()).unwrap();
    assert_eq!(storage.read("svw1_ok".into()), Ok(None));
    std::fs::remove_dir_all(dir).unwrap();
}
