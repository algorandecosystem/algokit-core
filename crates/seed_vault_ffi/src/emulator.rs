//! Emulated mode (feature `emulator`): the WASM runtime in the app's own
//! process, for devices without the `seed_vault_ta` Trusty applet.
//!
//! [`SeedVaultClient::emulated`] gives a client whose `wasm_*` and
//! `light_client_*` methods run `seed_vault_wasm`'s vault in-process --
//! the same request handling, policy, publisher signatures, capabilities and
//! limits as the TA -- so programs behave identically on both, and someone
//! with only the app installed can develop and test WASM programs.
//!
//! What differs is the trust boundary: like the app's own Room/StrongBox
//! vault, signing happens in the app process, so it's protected by the
//! Android sandbox and Keystore, not a TEE. [`SeedVaultClient::is_emulated`]
//! says which you have.
//!
//! The app supplies:
//! - [`SeedProvider`]: seeds, from its own vault (with its own user-auth
//!   gating, e.g. a Keystore key requiring a recent unlock). The emulator
//!   asks for a seed's entropy only while running a call that needs it, and
//!   wipes its copy afterwards. Errors the app throws reach the caller as
//!   is (e.g. "unlock required").
//! - [`WasmStorage`]: where programs, their state and cached keys go.
//!   [`file_wasm_storage`] is a ready-made one in an app-private directory;
//!   cached keys are private key material, so an app may prefer its own,
//!   encrypting with a Keystore key.
//!
//! Deleting a seed needs nothing extra: before each WASM call the emulator
//! compares [`SeedProvider::seed_addresses`] with what it saw last, and drops
//! the keys cached for seeds that are gone (also on the first call after a
//! restart, for seeds deleted meanwhile) -- as the TA does on
//! `DELETE_ACCOUNT`.
//!
//! Native-account calls (`create_account`, `sign_transaction`, ...) aren't
//! emulated: the app's own vault already does those. They fail with
//! [`SeedVaultError::NotSupported`] on an emulated client.

use std::collections::BTreeSet;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use seed_vault::StoreError;
use seed_vault_wasm::service::handle_request;
use seed_vault_wasm::{
    PublisherKey, PublisherPolicy, SeedSource, VaultConfig, WasmStore, WasmVault,
};
use zeroize::Zeroize;

use crate::{SeedVaultClient, SeedVaultError};

/// Seeds for the emulated runtime, from the app's own vault.
#[uniffi::export(with_foreign)]
pub trait SeedProvider: Send + Sync {
    /// The 32-byte mnemonic entropy of the seed whose human-sign (index 0)
    /// address is `address`, or `None` if there's no such seed. Throw to
    /// refuse (e.g. the user must unlock first); the error reaches the
    /// caller.
    fn seed_entropy(&self, address: String) -> Result<Option<Vec<u8>>, SeedVaultError>;

    /// The human-sign (index 0) address of every seed the app has. Public
    /// data, so it should never need an unlock: the emulator calls it before
    /// each WASM call to notice deleted seeds and drop the keys it cached
    /// for them.
    fn seed_addresses(&self) -> Result<Vec<String>, SeedVaultError>;
}

/// Named-blob storage for the emulated runtime: programs, their manifests
/// and state, cached keys, rollback versions, revocations. Names are short
/// ASCII (`[a-z0-9_]`).
#[uniffi::export(with_foreign)]
pub trait WasmStorage: Send + Sync {
    /// Names of the stored blobs starting with `prefix`.
    fn list(&self, prefix: String) -> Result<Vec<String>, SeedVaultError>;
    /// The blob `name`, or `None` if there's none.
    fn read(&self, name: String) -> Result<Option<Vec<u8>>, SeedVaultError>;
    /// Creates or replaces the blob `name`.
    fn write(&self, name: String, data: Vec<u8>) -> Result<(), SeedVaultError>;
    /// Deletes the blob `name`; deleting a missing blob isn't an error.
    fn delete(&self, name: String) -> Result<(), SeedVaultError>;
}

/// A publisher public key (raw 1793-byte Falcon-1024 key, as
/// `seed_vault_wasm_sign pubkey` writes it) whose signed modules the
/// emulated runtime trusts.
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct PublisherPublicKey {
    pub id: u8,
    pub public_key: Vec<u8>,
}

/// The development publisher key. Its seed is public, so anyone can sign
/// with it: for testing only, like the TA's `dev_ff.pub`.
#[uniffi::export]
pub fn dev_publisher_public_key() -> PublisherPublicKey {
    let key = seed_vault_wasm::dev_publisher_key();
    PublisherPublicKey {
        id: key.id,
        public_key: key.public_key.0.to_vec(),
    }
}

/// A [`WasmStorage`] keeping each blob as a file in `directory` (created if
/// missing): use an app-private directory, e.g. under `Context.filesDir`.
/// Files are written atomically. Not encrypted: cached keys are private key
/// material, so production apps may want their own Keystore-encrypted
/// storage instead.
#[uniffi::export]
pub fn file_wasm_storage(directory: String) -> Result<Arc<dyn WasmStorage>, SeedVaultError> {
    let dir = PathBuf::from(directory);
    std::fs::create_dir_all(&dir).map_err(io_error)?;
    Ok(Arc::new(FileWasmStorage { dir }))
}

struct FileWasmStorage {
    dir: PathBuf,
}

fn io_error(e: std::io::Error) -> SeedVaultError {
    SeedVaultError::Transport {
        reason: format!("emulated storage: {e}"),
    }
}

impl FileWasmStorage {
    fn path(&self, name: &str) -> Result<PathBuf, SeedVaultError> {
        let valid = !name.is_empty()
            && name.len() <= 128
            && name
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_');
        if !valid {
            return Err(SeedVaultError::InvalidArgument {
                reason: format!("invalid storage name {name:?}"),
            });
        }
        Ok(self.dir.join(name))
    }
}

impl WasmStorage for FileWasmStorage {
    fn list(&self, prefix: String) -> Result<Vec<String>, SeedVaultError> {
        let mut names = Vec::new();
        for entry in std::fs::read_dir(&self.dir).map_err(io_error)? {
            let name = entry.map_err(io_error)?.file_name();
            if let Some(name) = name.to_str() {
                // Skips in-flight `.tmp` files, which `path` never names.
                if name.starts_with(&prefix) && !name.ends_with(".tmp") {
                    names.push(name.to_string());
                }
            }
        }
        names.sort();
        Ok(names)
    }

    fn read(&self, name: String) -> Result<Option<Vec<u8>>, SeedVaultError> {
        match std::fs::read(self.path(&name)?) {
            Ok(bytes) => Ok(Some(bytes)),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(io_error(e)),
        }
    }

    fn write(&self, name: String, mut data: Vec<u8>) -> Result<(), SeedVaultError> {
        let path = self.path(&name)?;
        let tmp = self.dir.join(format!("{name}.tmp"));
        let result = std::fs::write(&tmp, &data).and_then(|()| std::fs::rename(&tmp, &path));
        data.zeroize();
        result.map_err(io_error)
    }

    fn delete(&self, name: String) -> Result<(), SeedVaultError> {
        match std::fs::remove_file(self.path(&name)?) {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(io_error(e)),
        }
    }
}

// ---- The in-process runtime -----------------------------------------------------

/// The emulated TA: a vault plus the app's storage and seeds.
pub(crate) struct Emulator {
    inner: Mutex<Inner>,
}

struct Inner {
    vault: WasmVault<PublisherPolicy>,
    storage: Arc<dyn WasmStorage>,
    seeds: Arc<dyn SeedProvider>,
    /// Seed addresses as of the last cleanup (`None` before the first call).
    known_addresses: Option<BTreeSet<String>>,
}

impl std::fmt::Debug for Emulator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Emulator")
    }
}

/// [`SeedSource`] over the app's [`SeedProvider`], remembering the error it
/// threw (the vault only sees "backend failed").
struct Seeds<'a> {
    provider: &'a dyn SeedProvider,
    error: Option<SeedVaultError>,
}

impl SeedSource for Seeds<'_> {
    fn seed_entropy(&mut self, address: &str) -> Result<[u8; 32], StoreError> {
        match self.provider.seed_entropy(address.to_string()) {
            Ok(Some(mut bytes)) => {
                let entropy =
                    <[u8; 32]>::try_from(bytes.as_slice()).map_err(|_| StoreError::CorruptRecord);
                bytes.zeroize();
                entropy
            }
            Ok(None) => Err(StoreError::NotFound),
            Err(e) => {
                self.error = Some(e);
                Err(StoreError::Backend)
            }
        }
    }

    fn seed_addresses(&mut self) -> Result<Vec<String>, StoreError> {
        self.provider.seed_addresses().map_err(|e| {
            self.error = Some(e);
            StoreError::Backend
        })
    }
}

/// [`WasmStore`] over the app's [`WasmStorage`], remembering its errors.
struct Storage<'a> {
    storage: &'a dyn WasmStorage,
    error: Option<SeedVaultError>,
}

impl Storage<'_> {
    fn check<T>(&mut self, r: Result<T, SeedVaultError>) -> Result<T, StoreError> {
        r.map_err(|e| {
            self.error = Some(e);
            StoreError::Backend
        })
    }
}

impl WasmStore for Storage<'_> {
    fn list(&mut self, prefix: &str) -> Result<Vec<String>, StoreError> {
        let r = self.storage.list(prefix.to_string());
        self.check(r)
    }
    fn read(&mut self, name: &str) -> Result<Option<Vec<u8>>, StoreError> {
        let r = self.storage.read(name.to_string());
        self.check(r)
    }
    fn write(&mut self, name: &str, data: &[u8]) -> Result<(), StoreError> {
        let r = self.storage.write(name.to_string(), data.to_vec());
        self.check(r)
    }
    fn delete(&mut self, name: &str) -> Result<(), StoreError> {
        let r = self.storage.delete(name.to_string());
        self.check(r)
    }
}

impl Emulator {
    fn new(
        storage: Arc<dyn WasmStorage>,
        seeds: Arc<dyn SeedProvider>,
        publisher_keys: Vec<PublisherPublicKey>,
        allow_unsigned: bool,
    ) -> Result<Self, SeedVaultError> {
        let keys = publisher_keys
            .into_iter()
            .map(|k| {
                let mut public_key = seed_vault_wasm::manifest::FalconPublicKey::default();
                if k.public_key.len() != public_key.0.len() {
                    return Err(SeedVaultError::InvalidArgument {
                        reason: format!(
                            "publisher key {} must be {} bytes",
                            k.id,
                            public_key.0.len()
                        ),
                    });
                }
                public_key.0.copy_from_slice(&k.public_key);
                Ok(PublisherKey {
                    id: k.id,
                    public_key,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let mut policy = PublisherPolicy::new(keys);
        if allow_unsigned {
            policy = policy.allowing_unsigned();
        }
        Ok(Emulator {
            inner: Mutex::new(Inner {
                vault: WasmVault::new(VaultConfig::default(), policy),
                storage,
                seeds,
                known_addresses: None,
            }),
        })
    }

    /// One request through the in-process vault, exactly as the TA would
    /// handle it. If the app's storage or seed callback threw and the
    /// request failed, that error is returned instead of the vault's
    /// generic one.
    pub(crate) fn exchange(&self, request: &[u8]) -> Result<Vec<u8>, SeedVaultError> {
        let mut inner = self
            .inner
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let Inner {
            vault,
            storage,
            seeds,
            known_addresses,
        } = &mut *inner;
        forget_deleted_seeds(vault, storage.as_ref(), seeds.as_ref(), known_addresses);
        let mut storage = Storage {
            storage: storage.as_ref(),
            error: None,
        };
        let mut seeds = Seeds {
            provider: seeds.as_ref(),
            error: None,
        };
        let response = handle_request(vault, &mut seeds, &mut storage, request);
        let failed = response.first() != Some(&0);
        match seeds.error.or(storage.error) {
            Some(app_error) if failed => Err(app_error),
            _ => Ok(response),
        }
    }
}

/// Drops the keys cached for seeds the app no longer has, when its seed list
/// has lost any since last time (always on the first call). Best effort: a
/// failure here never fails the caller's request -- a deleted seed's cached
/// keys are unusable anyway (every call looks its seed up first), and the
/// next call retries.
fn forget_deleted_seeds(
    vault: &mut WasmVault<PublisherPolicy>,
    storage: &dyn WasmStorage,
    provider: &dyn SeedProvider,
    known: &mut Option<BTreeSet<String>>,
) {
    let Ok(addresses) = provider.seed_addresses() else {
        return;
    };
    let current: BTreeSet<String> = addresses.into_iter().collect();
    let seeds_gone = known
        .as_ref()
        .is_none_or(|previous| !previous.is_subset(&current));
    if seeds_gone {
        let mut storage = Storage {
            storage,
            error: None,
        };
        let mut seeds = Seeds {
            provider,
            error: None,
        };
        if vault.purge_orphaned_keys(&mut seeds, &mut storage).is_err() {
            return; // retried next call
        }
    }
    *known = Some(current);
}

#[uniffi::export]
impl SeedVaultClient {
    /// A client whose WASM methods run in this process (see the module
    /// docs): for devices without the `seed_vault_ta` applet. Modules must
    /// be signed by one of `publisher_keys` unless `allow_unsigned`
    /// (development: then unsigned modules get every capability, as in a dev
    /// TA). Use [`dev_publisher_public_key`] while testing with dev-signed
    /// modules.
    #[uniffi::constructor]
    pub fn emulated(
        storage: Arc<dyn WasmStorage>,
        seeds: Arc<dyn SeedProvider>,
        publisher_keys: Vec<PublisherPublicKey>,
        allow_unsigned: bool,
    ) -> Result<Self, SeedVaultError> {
        let emulator = Emulator::new(storage, seeds, publisher_keys, allow_unsigned)?;
        Ok(SeedVaultClient {
            device_path: "emulated".into(),
            service_name: "emulated".into(),
            timeout_ms: 0,
            emulator: Some(Arc::new(emulator)),
        })
    }

    /// Whether this client's WASM runtime runs in the app's process
    /// ([`SeedVaultClient::emulated`]) rather than in the TEE.
    pub fn is_emulated(&self) -> bool {
        self.emulator.is_some()
    }
}
