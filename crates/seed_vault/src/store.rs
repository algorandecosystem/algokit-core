//! Account creation/listing logic, and the persistence trait it runs on.

use alloc::string::String;
use alloc::vec::Vec;

use algokit_crypto::Keypair;
use algokit_crypto::address::address_from_public_key;
use algokit_crypto::ed25519::CryptoxideEd25519Keypair;
use zeroize::Zeroize;

/// Byte length of a standard Algorand account's private key material: a
/// 25-word BIP mnemonic seed (see `algokit_crypto::algo25`).
pub const ALGO25_SEED_LEN: usize = 32;

/// Which key-generation scheme an account's stored key material belongs to.
///
/// Persisted as a tag byte alongside each account's key material (see
/// [`AccountRecord`]) and sent over the wire (see `protocol`), so adding a
/// second algorithm (e.g. post-quantum Falcon-1024, whose keys are far
/// larger than a 32-byte ed25519 seed) is additive rather than a breaking
/// rewrite -- which is also why [`AccountRecord::key_material`] is a
/// `Vec<u8>` rather than a fixed-size array.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum KeyAlgorithm {
    /// Ed25519 keypair derived from a 32-byte seed, encoded/recovered via
    /// the standard Algorand 25-word mnemonic (`algokit_crypto::algo25`).
    /// The only algorithm this vault actually knows how to generate today.
    Algo25Ed25519 = 0x01,
}

impl KeyAlgorithm {
    /// Parses a persisted/wire-format algorithm tag byte.
    pub fn from_tag(tag: u8) -> Option<Self> {
        match tag {
            0x01 => Some(Self::Algo25Ed25519),
            _ => None,
        }
    }
}

/// One persisted *seed*: which algorithm created it, and its raw private
/// key material.
///
/// # A record is a seed, not necessarily a single address
///
/// For [`KeyAlgorithm::Algo25Ed25519`] the two happen to coincide: the seed
/// *is* the ed25519 private key, so it derives exactly one Algorand
/// address. That's not a general rule -- a future HD algorithm (e.g.
/// BIP32-Ed25519 via `algokit_crypto::xhd`) would persist one root seed per
/// record but derive *many* addresses from it. That's why address lookups
/// (see [`addresses_for_record`]) always return a `Vec<String>` rather than
/// a single `String`.
///
/// # `key_material` must be recoverable back into the exact backup phrase
///
/// `key_material` must always be the value `reveal_mnemonic` needs to
/// reconstruct the exact words the user originally wrote down. This is
/// easy to get wrong for a BIP39-style algorithm: `xhd::seed_from_mnemonic`
/// runs the mnemonic through PBKDF2-HMAC-SHA512, a one-way KDF with no
/// inverse. For such an algorithm, `key_material` must hold the mnemonic's
/// **entropy** (the bytes the words directly encode), never the derived
/// seed/root key -- otherwise `reveal_mnemonic` becomes impossible for that
/// record, permanently. `algo25`, by contrast, is a reversible encoding:
/// the seed and the mnemonic are true inverses of each other.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountRecord {
    pub algorithm: KeyAlgorithm,
    pub key_material: Vec<u8>,
}

/// An account's public identity, safe to send over IPC. Never carries key
/// material.
///
/// `address_index` is the position of this address *within its parent
/// seed* -- always `0` for [`KeyAlgorithm::Algo25Ed25519`] today (one
/// address per seed), but carried on the wire/storage now so a future HD
/// algorithm (see [`AccountRecord`]'s doc comment) doesn't need a format
/// version bump to say "this is address N under that seed".
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountSummary {
    pub algorithm: KeyAlgorithm,
    pub address_index: u32,
    pub address: String,
}

/// Errors an [`AccountStore`] implementation can report.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreError {
    /// The persistence backend (e.g. Trusty secure storage) failed.
    Backend,
    /// A stored record was the wrong size/shape, or tagged with an
    /// unrecognized [`KeyAlgorithm`].
    CorruptRecord,
    /// The platform CSPRNG was unavailable.
    RandomnessUnavailable,
    /// A derived keypair/address could not be recomputed from stored key
    /// material.
    KeyDerivationFailed,
    /// `create_account`/`import_account` was asked for a [`KeyAlgorithm`]
    /// this build doesn't know how to generate/import yet, or
    /// `reveal_mnemonic` targeted an account whose algorithm has no
    /// mnemonic representation.
    UnsupportedAlgorithm,
    /// `reveal_mnemonic` was asked for an address with no corresponding
    /// account.
    NotFound,
    /// `import_account` was given a mnemonic that failed checksum/wordlist
    /// validation.
    InvalidMnemonic,
    /// `import_account` was given a mnemonic for a seed that's already
    /// stored under an existing account.
    AlreadyExists,
}

/// Persistence abstraction for seed vault accounts.
///
/// # Security invariant
///
/// Implementors only ever hand full [`AccountRecord`]s (raw private key
/// material) to/from this trait. [`create_account`] and [`list_accounts`]
/// never route that material back out over IPC -- only an
/// [`AccountSummary`] (address + algorithm tag). [`reveal_mnemonic`] is the
/// one deliberate, explicitly-named exception (see its own doc comment).
///
/// Records are identified purely by creation order (`0, 1, 2, ...`); a
/// record's public identity/identities are always re-derived on demand
/// from its key material, so there's exactly one source of truth per
/// record.
pub trait AccountStore {
    /// Returns the index the next created record should use (i.e. one past
    /// the highest index currently persisted, or `0` if the store is empty).
    fn next_index(&mut self) -> Result<u32, StoreError>;

    /// Durably persists `record` at `index`. Must fail rather than silently
    /// overwrite if `index` is already occupied.
    fn persist_record(&mut self, index: u32, record: &AccountRecord) -> Result<(), StoreError>;

    /// Returns every persisted record, ordered by ascending index (i.e.
    /// creation order).
    fn all_records(&mut self) -> Result<Vec<AccountRecord>, StoreError>;
}

/// Creates a brand-new account using `algorithm`, persists its key material
/// via `store`, and returns its public identity. Key material is never
/// returned to the caller -- only the [`AccountSummary`] is.
pub fn create_account(
    store: &mut impl AccountStore,
    algorithm: KeyAlgorithm,
) -> Result<AccountSummary, StoreError> {
    match algorithm {
        KeyAlgorithm::Algo25Ed25519 => create_algo25_account(store),
    }
}

/// Imports an existing account from its backup mnemonic -- the mirror image
/// of [`reveal_mnemonic`]: instead of a seed leaving the secure world, one
/// arrives from it (typed in by the user, or scanned from a paper backup).
/// `mnemonic` is validated (checksum + wordlist) before anything is
/// persisted, and the derived seed is rejected as [`StoreError::AlreadyExists`]
/// if it matches an account already in `store` -- re-importing the same
/// phrase should not silently create a duplicate.
///
/// # This carries the same exposure as `reveal_mnemonic`, just in reverse
///
/// Whoever wires this crate's `IMPORT_ACCOUNT` opcode up to a real
/// `tipc::Service` should treat the request payload with the same care as
/// a `REVEAL_MNEMONIC` *response*: the mnemonic sat in non-secure (Android)
/// memory just before this call, same as it would right after a reveal.
/// That's an inherent, unavoidable cost of "let the user type their phrase
/// back in" -- not something this function can mitigate -- but it's why
/// import is not a lower-stakes operation than reveal, and callers should
/// treat both as equally sensitive user actions (e.g. worth similar
/// confirmation/auth UX), not just gate the reveal side.
pub fn import_account(
    store: &mut impl AccountStore,
    algorithm: KeyAlgorithm,
    mnemonic: &str,
) -> Result<AccountSummary, StoreError> {
    match algorithm {
        KeyAlgorithm::Algo25Ed25519 => import_algo25_account(store, mnemonic),
    }
}

/// Returns the public identity ([`AccountSummary`]) of every Algorand
/// address derived from every persisted seed, in creation order. Never
/// returns key material.
///
/// One record can surface more than one entry here (see [`AccountRecord`]'s
/// doc comment); today, with only [`KeyAlgorithm::Algo25Ed25519`]
/// implemented, every record produces exactly one entry.
pub fn list_accounts(store: &mut impl AccountStore) -> Result<Vec<AccountSummary>, StoreError> {
    let mut records = store.all_records()?;
    let summaries = records
        .iter()
        .map(|record| {
            Ok(addresses_for_record(record)?
                .into_iter()
                .map(|(address_index, address)| AccountSummary {
                    algorithm: record.algorithm,
                    address_index,
                    address,
                })
                .collect::<Vec<_>>())
        })
        .collect::<Result<Vec<Vec<_>>, StoreError>>()
        .map(|nested| nested.into_iter().flatten().collect());

    for record in &mut records {
        record.key_material.zeroize();
    }

    summaries
}

/// Returns the 25-word backup mnemonic for the *seed* that `address` was
/// derived from -- `address` is the same identifier `create_account` and
/// `list_accounts` hand back, so a caller never needs to know anything
/// about internal storage indices. Looking up by address also means this
/// keeps working unchanged once one seed can back more than one address:
/// the mnemonic returned is the seed's *one* recovery phrase, shared by
/// every address derived from it.
///
/// # This is the one deliberate exception to "seeds never leave the secure
/// world"
///
/// Every other operation in this crate only ever hands back an address --
/// never key material. This mirrors the "reveal recovery phrase" screen
/// real hardware wallets and secure-enclave-backed apps (e.g. Solana
/// Mobile's Seed Vault) expose: a rare, explicit, user-initiated export.
///
/// Callers wiring this crate up to a real `tipc::Service` (see
/// `trusty/app/seed_vault_ta/main.rs`) **must** gate the opcode that calls
/// this function behind proof of a fresh, explicit user authentication
/// (e.g. a recent Gatekeeper/biometric auth token) -- this function itself
/// has no way to enforce that, since it knows nothing about the IPC
/// transport or caller identity.
pub fn reveal_mnemonic(store: &mut impl AccountStore, address: &str) -> Result<String, StoreError> {
    let mut records = store.all_records()?;

    // Linear scan re-deriving each record's address(es), checking *all* of
    // them (not just one) so this stays correct once one seed backs
    // multiple addresses.
    let matched = records
        .iter()
        .enumerate()
        .find_map(|(i, record)| match addresses_for_record(record) {
            Ok(addrs) if addrs.iter().any(|(_, a)| a == address) => Some(Ok(i)),
            Ok(_) => None,
            Err(e) => Some(Err(e)),
        })
        .unwrap_or(Err(StoreError::NotFound));

    let result = matched.and_then(|i| match records[i].algorithm {
        KeyAlgorithm::Algo25Ed25519 => {
            algokit_crypto::algo25::mnemonic_from_seed(&records[i].key_material)
                .map_err(|_| StoreError::CorruptRecord)
        }
    });

    for record in &mut records {
        record.key_material.zeroize();
    }

    result
}

/// Generates a fresh 32-byte seed from the platform CSPRNG (`getrandom`) and
/// persists it as a [`KeyAlgorithm::Algo25Ed25519`] record.
///
/// NOTE: `getrandom` has no built-in backend for Trusty upstream -- see the
/// `getrandom` dependency comment in `algokit_crypto/Cargo.toml`. For
/// `seed_vault_ta` specifically this is already resolved: a hand-vendored
/// `getrandom` build (`trusty/vendor/getrandom`) sources entropy from
/// Trusty's own HWRNG-backed CSPRNG via `trusty_rng_secure_rand` (see
/// `trusty/app/seed_vault_ta/main.rs`'s `trusty_rng` module). Any *other*
/// Trusty consumer of this crate would need the same wiring.
fn create_algo25_account(store: &mut impl AccountStore) -> Result<AccountSummary, StoreError> {
    let mut seed = [0u8; ALGO25_SEED_LEN];
    getrandom::fill(&mut seed).map_err(|_| StoreError::RandomnessUnavailable)?;
    persist_algo25_seed(store, seed, false)
}

/// Recovers the 32-byte seed from `mnemonic` (rejecting it as
/// [`StoreError::InvalidMnemonic`] if checksum/wordlist validation fails)
/// and persists it as a [`KeyAlgorithm::Algo25Ed25519`] record, unless a
/// matching account already exists.
fn import_algo25_account(
    store: &mut impl AccountStore,
    mnemonic: &str,
) -> Result<AccountSummary, StoreError> {
    let seed = algokit_crypto::algo25::seed_from_mnemonic(mnemonic)
        .map_err(|_| StoreError::InvalidMnemonic)?;
    persist_algo25_seed(store, seed, true)
}

/// Derives `seed`'s address, optionally rejects it as a duplicate of an
/// existing account, then persists it. Wipes `seed` on every return path.
fn persist_algo25_seed(
    store: &mut impl AccountStore,
    mut seed: [u8; ALGO25_SEED_LEN],
    reject_duplicate: bool,
) -> Result<AccountSummary, StoreError> {
    let result = (|| {
        let address = address_for_algo25_seed(&seed)?;
        if reject_duplicate {
            reject_if_address_exists(store, &address)?;
        }
        let index = store.next_index()?;
        let record = AccountRecord {
            algorithm: KeyAlgorithm::Algo25Ed25519,
            key_material: seed.to_vec(),
        };
        store.persist_record(index, &record)?;
        Ok(AccountSummary {
            algorithm: KeyAlgorithm::Algo25Ed25519,
            address_index: 0,
            address,
        })
    })();

    // Defense in depth: wipe this function's own stack copy regardless of
    // whether persistence succeeded.
    seed.zeroize();

    result
}

/// Fails with [`StoreError::AlreadyExists`] if any persisted record already
/// derives `address`.
fn reject_if_address_exists(
    store: &mut impl AccountStore,
    address: &str,
) -> Result<(), StoreError> {
    let mut records = store.all_records()?;
    let exists = records.iter().any(|record| {
        addresses_for_record(record)
            .map(|addrs| addrs.iter().any(|(_, a)| a == address))
            .unwrap_or(false)
    });

    for record in &mut records {
        record.key_material.zeroize();
    }

    if exists {
        Err(StoreError::AlreadyExists)
    } else {
        Ok(())
    }
}

/// Every Algorand address derivable from `record`'s stored key material,
/// paired with its [`AccountSummary::address_index`] within that seed.
/// Always a single `(0, address)` entry for [`KeyAlgorithm::Algo25Ed25519`]
/// today; see [`AccountRecord`]'s doc comment for why this returns a `Vec`.
fn addresses_for_record(record: &AccountRecord) -> Result<Vec<(u32, String)>, StoreError> {
    match record.algorithm {
        KeyAlgorithm::Algo25Ed25519 => {
            let seed: &[u8; ALGO25_SEED_LEN] = record
                .key_material
                .as_slice()
                .try_into()
                .map_err(|_| StoreError::CorruptRecord)?;
            Ok(alloc::vec![(0, address_for_algo25_seed(seed)?)])
        }
    }
}

fn address_for_algo25_seed(seed: &[u8; ALGO25_SEED_LEN]) -> Result<String, StoreError> {
    let keypair = CryptoxideEd25519Keypair::try_generate(Some(*seed))
        .map_err(|_| StoreError::KeyDerivationFailed)?;
    Ok(address_from_public_key(&keypair.verifying_key()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory_store::MemoryStore;

    #[test]
    fn create_account_persists_and_returns_a_valid_address() {
        let mut store = MemoryStore::default();
        let summary = create_account(&mut store, KeyAlgorithm::Algo25Ed25519)
            .expect("create_account should succeed");
        assert_eq!(summary.address.len(), 58);
        assert_eq!(summary.address_index, 0);

        // The address must be independently re-derivable from what got
        // persisted -- i.e. `create_account` didn't just return a random
        // string disconnected from the stored key material.
        let records = store.all_records().expect("all_records should succeed");
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].algorithm, KeyAlgorithm::Algo25Ed25519);
        assert_eq!(
            addresses_for_record(&records[0]).unwrap(),
            alloc::vec![(0, summary.address)]
        );
    }

    #[test]
    fn list_accounts_reflects_creation_order() {
        let mut store = MemoryStore::default();
        let first = create_account(&mut store, KeyAlgorithm::Algo25Ed25519).unwrap();
        let second = create_account(&mut store, KeyAlgorithm::Algo25Ed25519).unwrap();
        let third = create_account(&mut store, KeyAlgorithm::Algo25Ed25519).unwrap();

        let listed = list_accounts(&mut store).unwrap();
        let addresses: Vec<String> = listed.into_iter().map(|s| s.address).collect();
        assert_eq!(
            addresses,
            alloc::vec![first.address, second.address, third.address]
        );
    }

    #[test]
    fn accounts_are_never_identical() {
        let mut store = MemoryStore::default();
        let a = create_account(&mut store, KeyAlgorithm::Algo25Ed25519).unwrap();
        let b = create_account(&mut store, KeyAlgorithm::Algo25Ed25519).unwrap();
        assert_ne!(a.address, b.address);
    }

    #[test]
    fn unrecognized_algorithm_tag_does_not_parse() {
        assert_eq!(KeyAlgorithm::from_tag(0x02), None);
        assert_eq!(
            KeyAlgorithm::from_tag(0x01),
            Some(KeyAlgorithm::Algo25Ed25519)
        );
    }

    #[test]
    fn reveal_mnemonic_recovers_the_same_address() {
        let mut store = MemoryStore::default();
        let summary = create_account(&mut store, KeyAlgorithm::Algo25Ed25519).unwrap();

        let mnemonic =
            reveal_mnemonic(&mut store, &summary.address).expect("reveal should succeed");
        assert_eq!(mnemonic.split_whitespace().count(), 25);

        // The revealed mnemonic must actually recover the same account --
        // not just be *some* 25-word string.
        let seed = algokit_crypto::algo25::seed_from_mnemonic(&mnemonic).unwrap();
        assert_eq!(address_for_algo25_seed(&seed).unwrap(), summary.address);
    }

    #[test]
    fn reveal_mnemonic_picks_out_the_right_account_among_several() {
        let mut store = MemoryStore::default();
        let first = create_account(&mut store, KeyAlgorithm::Algo25Ed25519).unwrap();
        let second = create_account(&mut store, KeyAlgorithm::Algo25Ed25519).unwrap();

        let mnemonic = reveal_mnemonic(&mut store, &second.address).unwrap();
        let seed = algokit_crypto::algo25::seed_from_mnemonic(&mnemonic).unwrap();
        assert_eq!(address_for_algo25_seed(&seed).unwrap(), second.address);
        assert_ne!(address_for_algo25_seed(&seed).unwrap(), first.address);
    }

    #[test]
    fn reveal_mnemonic_unknown_address_is_not_found() {
        let mut store = MemoryStore::default();
        create_account(&mut store, KeyAlgorithm::Algo25Ed25519).unwrap();
        assert_eq!(
            reveal_mnemonic(&mut store, &"A".repeat(58)),
            Err(StoreError::NotFound)
        );
    }

    #[test]
    fn import_account_round_trips_with_create_and_reveal() {
        let mut store = MemoryStore::default();
        let created = create_account(&mut store, KeyAlgorithm::Algo25Ed25519).unwrap();
        let mnemonic = reveal_mnemonic(&mut store, &created.address).unwrap();

        // Importing a *different* store with that same mnemonic must
        // recover the identical address.
        let mut other_store = MemoryStore::default();
        let imported =
            import_account(&mut other_store, KeyAlgorithm::Algo25Ed25519, &mnemonic).unwrap();
        assert_eq!(imported.address, created.address);
        assert_eq!(imported.address_index, 0);
    }

    #[test]
    fn import_account_rejects_invalid_mnemonic() {
        let mut store = MemoryStore::default();
        assert_eq!(
            import_account(
                &mut store,
                KeyAlgorithm::Algo25Ed25519,
                "not a real mnemonic"
            ),
            Err(StoreError::InvalidMnemonic)
        );
    }

    #[test]
    fn import_account_rejects_a_seed_already_present() {
        let mut store = MemoryStore::default();
        let created = create_account(&mut store, KeyAlgorithm::Algo25Ed25519).unwrap();
        let mnemonic = reveal_mnemonic(&mut store, &created.address).unwrap();

        assert_eq!(
            import_account(&mut store, KeyAlgorithm::Algo25Ed25519, &mnemonic),
            Err(StoreError::AlreadyExists)
        );
        // The duplicate attempt must not have been persisted.
        assert_eq!(store.all_records().unwrap().len(), 1);
    }
}
