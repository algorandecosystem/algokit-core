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

/// Byte length of the mnemonic entropy a Falcon-1024 account is derived
/// from -- the same 32 bytes as [`ALGO25_SEED_LEN`], so the same 25 words
/// back it up.
pub const FALCON1024_ENTROPY_LEN: usize = algokit_crypto::falcon1024::ENTROPY_LEN;

/// [`AccountSummary::address_index`] of a seed's human-signing account: the
/// standard derivation, i.e. the same account any Algorand wallet (or SDK)
/// restores from the 25 words. For Falcon-1024 that's the key's
/// *canonical* (lowest PQ-compliant) salt. Every seed has one.
pub const HUMAN_SIGN_INDEX: u32 = 0;

/// [`AccountSummary::address_index`] of a [`KeyAlgorithm::Falcon1024`]
/// seed's autosign account: the *same* Falcon key at its next-lowest
/// PQ-compliant salt (`algokit_crypto::falcon1024::nth_pq_address(pk, 1)`).
/// go-algorand accepts any compliant salt, and go-algorand-sdk can sign for
/// it with `SaltedPQSigner`, so the 25 words recover it anywhere that knows
/// to look at the second salt. [`KeyAlgorithm::Algo25Ed25519`] seeds don't
/// have one.
///
/// Both accounts share one private key, and a Falcon signature doesn't
/// cover the salt, so an autosign signature would also authorize the
/// human-sign account's transactions if the salt were swapped. That's why
/// [`sign_transaction`] only signs transactions whose sender is the
/// requested address.
pub const AUTOSIGN_INDEX: u32 = 1;

/// The accounts a [`KeyAlgorithm::Falcon1024`] seed has, in order: the
/// `n`th entry is the key's `n`th-lowest PQ-compliant salt.
const FALCON1024_KEY_INDICES: [u32; 2] = [HUMAN_SIGN_INDEX, AUTOSIGN_INDEX];

/// Persisted layout of a [`KeyAlgorithm::Falcon1024`] record's
/// `key_material`:
///
/// ```text
/// [entropy: 32][salt₀: 1][address₀: 32][public_key: 1793][private_key: 2305][salt₁: 1][address₁: 32]
/// ```
///
/// The entropy comes first and is the source of truth (it's what
/// `reveal_mnemonic` turns back into the 25 words). The rest caches what
/// Falcon-1024 keygen derives from it, plus the salt/address of each of
/// [`HUMAN_SIGN_INDEX`] and [`AUTOSIGN_INDEX`], so `LIST_ACCOUNTS`, duplicate
/// checks, and signing never have to re-run keygen. Records written by
/// earlier builds stop after the private key ([`falcon_record::KEYPAIR_LEN`]) or hold
/// only the entropy; whatever is missing is re-derived on demand.
pub mod falcon_record {
    use algokit_crypto::falcon1024::{PRIVATE_KEY_SIZE, PUBLIC_KEY_SIZE};

    pub const ENTROPY: core::ops::Range<usize> = 0..32;
    pub const PUBLIC_KEY: core::ops::Range<usize> = 65..65 + PUBLIC_KEY_SIZE;
    pub const PRIVATE_KEY: core::ops::Range<usize> =
        PUBLIC_KEY.end..PUBLIC_KEY.end + PRIVATE_KEY_SIZE;
    /// Length of a record from an earlier build, caching only the keypair
    /// and index 0's salt/address: 4163 bytes.
    pub const KEYPAIR_LEN: usize = PRIVATE_KEY.end;
    /// Total length of a current record: 4196 bytes.
    pub const LEN: usize = KEYPAIR_LEN + 1 + 32;

    /// Offset of key index `index`'s salt.
    pub const fn salt(index: u32) -> usize {
        if index == 0 { ENTROPY.end } else { KEYPAIR_LEN }
    }

    /// Byte range of key index `index`'s 32-byte address.
    pub const fn address(index: u32) -> core::ops::Range<usize> {
        salt(index) + 1..salt(index) + 33
    }
}

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
    Algo25Ed25519 = 0x01,
    /// Native Algorand post-quantum account: deterministic Falcon-1024
    /// keypair derived from the *same* 32-byte entropy a 25-word Algorand
    /// mnemonic encodes, via `SHA512/256("PQK" || "f1" || entropy)`, with a
    /// PQ address `SHA512/256("PQA" || "f1" || salt || pk)` (see
    /// `algokit_crypto::falcon1024`). Matches go-algorand's `algokey pq` and
    /// go-algorand-sdk's `mnemonic.ToPQSeed`/`Falcon1024AccountFromPQSeed`.
    ///
    /// The same 25 words therefore back *different* addresses under
    /// [`Self::Algo25Ed25519`] vs this algorithm.
    Falcon1024 = 0x02,
}

impl KeyAlgorithm {
    /// Parses a persisted/wire-format algorithm tag byte.
    pub fn from_tag(tag: u8) -> Option<Self> {
        match tag {
            0x01 => Some(Self::Algo25Ed25519),
            0x02 => Some(Self::Falcon1024),
            _ => None,
        }
    }

    /// The [`AccountSummary::address_index`] values a seed of this
    /// algorithm derives: human-sign only for Ed25519, human-sign and
    /// autosign for Falcon-1024.
    pub fn key_indices(self) -> &'static [u32] {
        match self {
            Self::Algo25Ed25519 => &[HUMAN_SIGN_INDEX],
            Self::Falcon1024 => &FALCON1024_KEY_INDICES,
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
/// (see `addresses_for_record`) always return a `Vec<String>` rather than
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
/// `address_index` says which of its seed's keys this is:
/// [`HUMAN_SIGN_INDEX`] (0) or [`AUTOSIGN_INDEX`] (1, Falcon-1024 only; see
/// [`KeyAlgorithm::key_indices`]). The TA only reports the index today;
/// enforcing "index 0 needs a human to approve each signature" is the
/// pending user-authentication TODO in `seed_vault_ta`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountSummary {
    pub algorithm: KeyAlgorithm,
    pub address_index: u32,
    pub address: String,
    /// The account's public key: 32 bytes for
    /// [`KeyAlgorithm::Algo25Ed25519`] (identical to the address bytes),
    /// 1793 bytes for [`KeyAlgorithm::Falcon1024`] (the address is a hash of
    /// it, so it can't be recovered from the address).
    pub public_key: Vec<u8>,
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
    /// `sign_transaction` was given bytes that aren't a msgpack-encoded
    /// transaction.
    InvalidTransaction,
    /// Signing failed, or the fresh signature didn't verify against the
    /// account's own public key.
    SigningFailed,
    /// An `address_index` the account's algorithm doesn't derive (see
    /// [`KeyAlgorithm::key_indices`]).
    UnknownKeyIndex,
    /// `sign_transaction`'s transaction isn't sent by the requested address.
    SenderMismatch,
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
/// Records are identified by a `u32` index assigned in creation order.
/// Indices can have gaps once records are deleted (see [`delete_account`]).
pub trait AccountStore {
    /// Returns the index the next created record should use (i.e. one past
    /// the highest index currently persisted, or `0` if the store is empty).
    fn next_index(&mut self) -> Result<u32, StoreError>;

    /// Durably persists `record` at `index`. Must fail rather than silently
    /// overwrite if `index` is already occupied.
    fn persist_record(&mut self, index: u32, record: &AccountRecord) -> Result<(), StoreError>;

    /// Returns every persisted record paired with its index, ordered by
    /// ascending index (i.e. creation order).
    fn all_indexed_records(&mut self) -> Result<Vec<(u32, AccountRecord)>, StoreError>;

    /// Permanently removes the record at `index`. Fails with
    /// [`StoreError::NotFound`] if no such record exists (backends that
    /// can't distinguish "missing" from other failures may report
    /// [`StoreError::Backend`]).
    fn delete_record(&mut self, index: u32) -> Result<(), StoreError>;

    /// Every persisted record, in ascending index order, without indices.
    fn all_records(&mut self) -> Result<Vec<AccountRecord>, StoreError> {
        Ok(self
            .all_indexed_records()?
            .into_iter()
            .map(|(_, record)| record)
            .collect())
    }
}

/// Creates a brand-new seed using `algorithm`, persists its key material via
/// `store`, and returns the public identity of its key `address_index` (one
/// of [`KeyAlgorithm::key_indices`], else [`StoreError::UnknownKeyIndex`];
/// a Falcon-1024 seed has both of its accounts either way). Key material is
/// never returned to the caller -- only the [`AccountSummary`] is.
pub fn create_account(
    store: &mut impl AccountStore,
    algorithm: KeyAlgorithm,
    address_index: u32,
) -> Result<AccountSummary, StoreError> {
    check_key_index(algorithm, address_index)?;
    create_mnemonic_backed_account(store, algorithm, address_index)
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
///
/// Returns the public identity of key `address_index`, like
/// [`create_account`].
pub fn import_account(
    store: &mut impl AccountStore,
    algorithm: KeyAlgorithm,
    address_index: u32,
    mnemonic: &str,
) -> Result<AccountSummary, StoreError> {
    check_key_index(algorithm, address_index)?;
    import_mnemonic_backed_account(store, algorithm, address_index, mnemonic)
}

fn check_key_index(algorithm: KeyAlgorithm, address_index: u32) -> Result<(), StoreError> {
    if algorithm.key_indices().contains(&address_index) {
        Ok(())
    } else {
        Err(StoreError::UnknownKeyIndex)
    }
}

/// Returns the public identity ([`AccountSummary`]) of every Algorand
/// address derived from every persisted seed, in creation order. Never
/// returns key material.
///
/// One record can surface more than one entry here (see [`AccountRecord`]'s
/// doc comment); today every record produces exactly one entry.
pub fn list_accounts(store: &mut impl AccountStore) -> Result<Vec<AccountSummary>, StoreError> {
    let mut records = store.all_indexed_records()?;
    let summaries = records
        .iter()
        .map(|(_, record)| summaries_for_record(record))
        .collect::<Result<Vec<Vec<_>>, StoreError>>()
        .map(|nested| nested.into_iter().flatten().collect());

    wipe(&mut records);
    summaries
}

/// Returns the 25-word backup mnemonic for the *seed* that `address` was
/// derived from -- `address` is the same identifier `create_account` and
/// `list_accounts` hand back, so a caller never needs to know anything
/// about internal storage indices. Looking up by address also means this
/// works for every address of a seed (see [`KeyAlgorithm::key_indices`]):
/// the mnemonic returned is the seed's *one* recovery phrase, shared by all.
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
    let mut records = store.all_indexed_records()?;

    // Both algorithms persist the raw mnemonic entropy first, so the backup
    // phrase is the same `algo25` encoding either way.
    let result = find_record_by_address(&records, address).and_then(|(pos, _)| {
        let entropy = record_entropy(&records[pos].1)?;
        algokit_crypto::algo25::mnemonic_from_seed(entropy).map_err(|_| StoreError::CorruptRecord)
    });

    wipe(&mut records);
    result
}

/// Permanently deletes the persisted *seed* that `address` was derived
/// from, and returns that address's [`AccountSummary`] so the caller can
/// confirm what was removed.
///
/// Because a record is a seed (see [`AccountRecord`]), this removes *all*
/// of its addresses (for Falcon-1024, human-sign and autosign). Importing the same
/// mnemonic under the *other* [`KeyAlgorithm`] creates a separate record,
/// which this leaves untouched.
///
/// # Irreversible
///
/// The key material is gone for good unless the user still holds the
/// 25-word backup. Callers wiring this to a real `tipc::Service` should gate
/// it behind the same fresh user authentication as [`reveal_mnemonic`] --
/// otherwise any process that can reach the TA can destroy funds.
pub fn delete_account(
    store: &mut impl AccountStore,
    address: &str,
) -> Result<AccountSummary, StoreError> {
    let mut records = store.all_indexed_records()?;

    let result = find_record_by_address(&records, address).and_then(|(pos, _)| {
        let (index, record) = &records[pos];
        let summary = summaries_for_record(record)?
            .into_iter()
            .find(|s| s.address == address)
            .ok_or(StoreError::NotFound)?;
        store.delete_record(*index)?;
        Ok(summary)
    });

    wipe(&mut records);
    result
}

/// Signs `txn` -- a msgpack-encoded Algorand transaction *without* the
/// `"TX"` prefix, e.g. `algosdk`'s `txn.encode()` -- with the account whose
/// address is `address`, and returns the [`SignedTxnPrefix`] for it: the
/// network-ready msgpack `SignedTxn` (`POST /v2/transactions` body) is
/// `prefix || txn`. See [`crate::signed_txn`] for the exact layouts and why
/// only the prefix is returned.
///
/// `address_index` is the kind of key the caller intends to sign with
/// ([`HUMAN_SIGN_INDEX`] or [`AUTOSIGN_INDEX`]); if `address` isn't that
/// key this fails with [`StoreError::UnknownKeyIndex`]. The transaction's
/// sender (`snd`) must be `address` itself, else
/// [`StoreError::SenderMismatch`]: this is what keeps a Falcon seed's
/// autosign account from being used to sign for its human-sign account
/// (see [`AUTOSIGN_INDEX`]). Rekeyed senders are therefore not supported.
///
/// - **Ed25519:** `{sig, txn}`.
/// - **Falcon-1024:** `{pqsig: {pk, sch: "f1", sig, slt}, txn}`. Only
///   networks whose consensus enables `EnablePQSchemeFalcon1024` accept it
///   (only `vFuture` in go-algorand at the time of writing), and the
///   transaction fee must cover go-algorand's extra Falcon fee contribution
///   (+2x min fee, i.e. 3x min fee total).
///
/// The `"TX"` domain prefix is always added *here*, so a caller can never
/// get this TA to sign arbitrary non-transaction bytes. Beyond the sender,
/// the TA doesn't check the transaction's contents. A production wallet
/// should display the transaction and get user confirmation (and auth)
/// first for human-sign accounts.
pub fn sign_transaction(
    store: &mut impl AccountStore,
    address: &str,
    address_index: u32,
    txn: &[u8],
) -> Result<SignedTxnPrefix, StoreError> {
    if !crate::signed_txn::looks_like_encoded_transaction(txn) {
        return Err(StoreError::InvalidTransaction);
    }
    let sender =
        crate::signed_txn::transaction_sender(txn).ok_or(StoreError::InvalidTransaction)?;
    if address_from_public_key(&sender) != address {
        return Err(StoreError::SenderMismatch);
    }
    let mut records = store.all_indexed_records()?;
    let result = find_record_by_address(&records, address).and_then(|(pos, key_index)| {
        // The caller states which kind of key it means to sign with;
        // refuse rather than silently sign with a different one.
        if key_index != address_index {
            return Err(StoreError::UnknownKeyIndex);
        }
        let record = &records[pos].1;
        Ok(SignedTxnPrefix {
            algorithm: record.algorithm,
            address_index,
            prefix: sign_with_record(record, key_index, txn)?,
        })
    });
    wipe(&mut records);
    result
}

/// Result of [`sign_transaction`]. The signed transaction is
/// `prefix || txn`, where `txn` is the exact bytes that were signed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignedTxnPrefix {
    /// Which kind of signature `prefix` carries.
    pub algorithm: KeyAlgorithm,
    /// Which of the seed's keys signed ([`HUMAN_SIGN_INDEX`] or
    /// [`AUTOSIGN_INDEX`]).
    pub address_index: u32,
    /// msgpack `SignedTxn` bytes up to and including the `"txn"` key.
    pub prefix: Vec<u8>,
}

impl SignedTxnPrefix {
    /// The full msgpack `SignedTxn` for `txn`.
    pub fn signed_txn(&self, txn: &[u8]) -> Vec<u8> {
        [self.prefix.as_slice(), txn].concat()
    }
}

/// The [`SignedTxnPrefix::prefix`] for `txn` signed by key `key_index` of
/// `record`.
fn sign_with_record(
    record: &AccountRecord,
    key_index: u32,
    txn: &[u8],
) -> Result<Vec<u8>, StoreError> {
    use crate::signed_txn;
    use algokit_crypto::falcon1024::{PrivateKey, PublicKey};

    let message = signed_txn::bytes_to_sign(txn);
    match record.algorithm {
        KeyAlgorithm::Algo25Ed25519 => {
            let keypair = ed25519_keypair(record)?;
            let signature = keypair.sign(&message);
            if !algokit_crypto::ed25519::verify(&message, &keypair.verifying_key(), &signature) {
                return Err(StoreError::SigningFailed);
            }
            Ok(signed_txn::ed25519_prefix(&signature))
        }
        KeyAlgorithm::Falcon1024 => {
            let km = &record.key_material;
            let (public_key, private_key) = if km.len() >= falcon_record::KEYPAIR_LEN {
                let mut public_key = PublicKey([0u8; algokit_crypto::falcon1024::PUBLIC_KEY_SIZE]);
                public_key.0.copy_from_slice(&km[falcon_record::PUBLIC_KEY]);
                let mut private_key = PrivateKey::default();
                private_key
                    .0
                    .copy_from_slice(&km[falcon_record::PRIVATE_KEY]);
                (public_key, private_key)
            } else {
                // Entropy-only record from an earlier build: re-derive.
                let account = derive_falcon_account(record)?;
                (account.public_key, account.private_key)
            };
            let (salt, _) = falcon_salt_and_address(record, &public_key.0, key_index)?;
            let signature = private_key
                .sign_compressed(&message)
                .map_err(|_| StoreError::SigningFailed)?;
            // `private_key` zeroizes on drop. Verifying here catches a
            // miscompiled/emulated-FP Falcon build before a bad signature
            // ever leaves the TA.
            public_key
                .verify(&signature, &message)
                .map_err(|_| StoreError::SigningFailed)?;
            Ok(signed_txn::falcon1024_prefix(
                &public_key.0,
                salt,
                &signature.0,
            ))
        }
    }
}

/// Position (within `records`) of the record deriving `address`, and which
/// of its keys ([`KeyAlgorithm::key_indices`]) `address` belongs to.
fn find_record_by_address(
    records: &[(u32, AccountRecord)],
    address: &str,
) -> Result<(usize, u32), StoreError> {
    records
        .iter()
        .enumerate()
        .find_map(|(pos, (_, record))| match addresses_for_record(record) {
            Ok(addrs) => addrs
                .into_iter()
                .find(|(_, a)| a == address)
                .map(|(key_index, _)| Ok((pos, key_index))),
            Err(e) => Some(Err(e)),
        })
        .unwrap_or(Err(StoreError::NotFound))
}

fn summaries_for_record(record: &AccountRecord) -> Result<Vec<AccountSummary>, StoreError> {
    Ok(public_identities(record)?
        .into_iter()
        .map(|(address_index, address, public_key)| AccountSummary {
            algorithm: record.algorithm,
            address_index,
            address,
            public_key,
        })
        .collect())
}

fn wipe(records: &mut [(u32, AccountRecord)]) {
    for (_, record) in records {
        record.key_material.zeroize();
    }
}

/// Generates a fresh 32-byte seed/entropy from the platform CSPRNG
/// (`getrandom`) and persists it as an `algorithm` record. Both supported
/// algorithms are backed by exactly 32 bytes of 25-word-mnemonic entropy.
///
/// NOTE: `getrandom` has no built-in backend for Trusty upstream -- see the
/// `getrandom` dependency comment in `algokit_crypto/Cargo.toml`. For
/// `seed_vault_ta` specifically this is already resolved: a hand-vendored
/// `getrandom` build (`trusty/vendor/getrandom`) sources entropy from
/// Trusty's own HWRNG-backed CSPRNG via `trusty_rng_secure_rand` (see
/// `trusty/app/seed_vault_ta/main.rs`'s `trusty_rng` module). Any *other*
/// Trusty consumer of this crate would need the same wiring.
fn create_mnemonic_backed_account(
    store: &mut impl AccountStore,
    algorithm: KeyAlgorithm,
    address_index: u32,
) -> Result<AccountSummary, StoreError> {
    let mut seed = [0u8; ALGO25_SEED_LEN];
    getrandom::fill(&mut seed).map_err(|_| StoreError::RandomnessUnavailable)?;
    persist_seed(store, algorithm, address_index, seed, false)
}

/// Recovers the 32-byte seed/entropy from `mnemonic` (rejecting it as
/// [`StoreError::InvalidMnemonic`] if checksum/wordlist validation fails)
/// and persists it as an `algorithm` record, unless a matching account
/// already exists.
fn import_mnemonic_backed_account(
    store: &mut impl AccountStore,
    algorithm: KeyAlgorithm,
    address_index: u32,
    mnemonic: &str,
) -> Result<AccountSummary, StoreError> {
    let seed = algokit_crypto::algo25::seed_from_mnemonic(mnemonic)
        .map_err(|_| StoreError::InvalidMnemonic)?;
    persist_seed(store, algorithm, address_index, seed, true)
}

/// Builds `seed`'s full record under `algorithm` (for Falcon-1024 that
/// means running keygen once per key and caching it, see [`falcon_record`]),
/// optionally rejects it as a duplicate of an existing account, then
/// persists it and returns key `address_index`'s summary. Wipes `seed` and
/// the built record on every return path.
fn persist_seed(
    store: &mut impl AccountStore,
    algorithm: KeyAlgorithm,
    address_index: u32,
    mut seed: [u8; ALGO25_SEED_LEN],
    reject_duplicate: bool,
) -> Result<AccountSummary, StoreError> {
    let result = build_record(algorithm, &seed).and_then(|(mut record, address)| {
        let persisted = (|| {
            // Both keys come from the same seed, so checking the human-sign
            // address is enough.
            if reject_duplicate {
                reject_if_address_exists(store, &address)?;
            }
            let identity = public_identity(&record, address_index)?;
            let index = store.next_index()?;
            store.persist_record(index, &record)?;
            Ok(identity)
        })();
        record.key_material.zeroize();
        persisted.map(|(address, public_key)| AccountSummary {
            algorithm,
            address_index,
            address,
            public_key,
        })
    });

    // Defense in depth: wipe this function's own stack copy regardless of
    // whether persistence succeeded.
    seed.zeroize();

    result
}

/// The record to persist for `seed` under `algorithm`, plus its human-sign
/// (index 0) address.
fn build_record(
    algorithm: KeyAlgorithm,
    seed: &[u8; ALGO25_SEED_LEN],
) -> Result<(AccountRecord, String), StoreError> {
    match algorithm {
        KeyAlgorithm::Algo25Ed25519 => {
            let record = AccountRecord {
                algorithm,
                key_material: seed.to_vec(),
            };
            let (address, _) = public_identity(&record, HUMAN_SIGN_INDEX)?;
            Ok((record, address))
        }
        KeyAlgorithm::Falcon1024 => {
            let account = algokit_crypto::falcon1024::account_from_entropy(seed)
                .map_err(|_| StoreError::KeyDerivationFailed)?;
            let (autosign_salt, autosign_address) =
                algokit_crypto::falcon1024::nth_pq_address(&account.public_key.0, 1)
                    .map_err(|_| StoreError::KeyDerivationFailed)?;
            let mut key_material = Vec::with_capacity(falcon_record::LEN);
            key_material.extend_from_slice(seed);
            key_material.push(account.salt);
            key_material.extend_from_slice(&account.address);
            key_material.extend_from_slice(&account.public_key.0);
            key_material.extend_from_slice(&account.private_key.0);
            key_material.push(autosign_salt);
            key_material.extend_from_slice(&autosign_address);
            debug_assert_eq!(key_material.len(), falcon_record::LEN);
            // `account.private_key` is zeroized when `account` drops here.
            Ok((
                AccountRecord {
                    algorithm,
                    key_material,
                },
                account.address_string(),
            ))
        }
    }
}

/// Fails with [`StoreError::AlreadyExists`] if any persisted record already
/// derives `address`.
fn reject_if_address_exists(
    store: &mut impl AccountStore,
    address: &str,
) -> Result<(), StoreError> {
    let mut records = store.all_indexed_records()?;
    let exists = records.iter().any(|(_, record)| {
        addresses_for_record(record)
            .map(|addrs| addrs.iter().any(|(_, a)| a == address))
            .unwrap_or(false)
    });

    wipe(&mut records);

    if exists {
        Err(StoreError::AlreadyExists)
    } else {
        Ok(())
    }
}

/// The 32 bytes of mnemonic entropy at the start of every record.
fn record_entropy(record: &AccountRecord) -> Result<&[u8; ALGO25_SEED_LEN], StoreError> {
    let valid_len = match record.algorithm {
        KeyAlgorithm::Algo25Ed25519 => record.key_material.len() == ALGO25_SEED_LEN,
        // Current records, plus the two shapes earlier builds wrote.
        KeyAlgorithm::Falcon1024 => matches!(
            record.key_material.len(),
            FALCON1024_ENTROPY_LEN | falcon_record::KEYPAIR_LEN | falcon_record::LEN
        ),
    };
    if !valid_len {
        return Err(StoreError::CorruptRecord);
    }
    record.key_material[falcon_record::ENTROPY]
        .try_into()
        .map_err(|_| StoreError::CorruptRecord)
}

/// Every Algorand address `record` derives, paired with its
/// [`AccountSummary::address_index`] (see [`KeyAlgorithm::key_indices`]).
///
/// Current Falcon-1024 records answer from their cache (no keygen, no salt
/// search).
fn addresses_for_record(record: &AccountRecord) -> Result<Vec<(u32, String)>, StoreError> {
    Ok(public_identities(record)?
        .into_iter()
        .map(|(key_index, address, _)| (key_index, address))
        .collect())
}

/// `(address_index, address, public_key)` for every account `record`
/// derives. Both Falcon-1024 accounts carry the same public key.
fn public_identities(record: &AccountRecord) -> Result<Vec<(u32, String, Vec<u8>)>, StoreError> {
    match record.algorithm {
        KeyAlgorithm::Algo25Ed25519 => {
            let public_key = ed25519_keypair(record)?.verifying_key();
            Ok(alloc::vec![(
                HUMAN_SIGN_INDEX,
                address_from_public_key(&public_key),
                public_key.to_vec(),
            )])
        }
        KeyAlgorithm::Falcon1024 => {
            let public_key = falcon_public_key(record)?;
            FALCON1024_KEY_INDICES
                .iter()
                .map(|&key_index| {
                    let (_, address) = falcon_salt_and_address(record, &public_key, key_index)?;
                    Ok((
                        key_index,
                        address_from_public_key(&address),
                        public_key.clone(),
                    ))
                })
                .collect()
        }
    }
}

/// Address and public key of account `key_index` of `record` (see
/// [`AccountSummary::public_key`]).
fn public_identity(
    record: &AccountRecord,
    key_index: u32,
) -> Result<(String, Vec<u8>), StoreError> {
    public_identities(record)?
        .into_iter()
        .find(|(index, _, _)| *index == key_index)
        .map(|(_, address, public_key)| (address, public_key))
        .ok_or(StoreError::UnknownKeyIndex)
}

fn ed25519_keypair(record: &AccountRecord) -> Result<CryptoxideEd25519Keypair, StoreError> {
    CryptoxideEd25519Keypair::try_generate(Some(*record_entropy(record)?))
        .map_err(|_| StoreError::KeyDerivationFailed)
}

/// Runs Falcon-1024 keygen for `record`'s entropy. The private key is
/// zeroized when the returned account drops.
fn derive_falcon_account(
    record: &AccountRecord,
) -> Result<algokit_crypto::falcon1024::Falcon1024Account, StoreError> {
    algokit_crypto::falcon1024::account_from_entropy(record_entropy(record)?)
        .map_err(|_| StoreError::KeyDerivationFailed)
}

/// A Falcon-1024 record's public key: cached, or re-derived for
/// entropy-only records from earlier builds.
fn falcon_public_key(record: &AccountRecord) -> Result<Vec<u8>, StoreError> {
    record_entropy(record)?; // validates the length
    if record.key_material.len() >= falcon_record::KEYPAIR_LEN {
        Ok(record.key_material[falcon_record::PUBLIC_KEY].to_vec())
    } else {
        Ok(derive_falcon_account(record)?.public_key.0.to_vec())
    }
}

/// Salt and raw address of Falcon-1024 account `key_index`: cached when
/// the record has it, else the `key_index`th-lowest PQ-compliant salt of
/// `public_key`.
fn falcon_salt_and_address(
    record: &AccountRecord,
    public_key: &[u8],
    key_index: u32,
) -> Result<(u8, [u8; 32]), StoreError> {
    if !FALCON1024_KEY_INDICES.contains(&key_index) {
        return Err(StoreError::UnknownKeyIndex);
    }
    let km = &record.key_material;
    if let Some(address) = km.get(falcon_record::address(key_index)) {
        let address = address.try_into().map_err(|_| StoreError::CorruptRecord)?;
        return Ok((km[falcon_record::salt(key_index)], address));
    }
    algokit_crypto::falcon1024::nth_pq_address(public_key, key_index as usize)
        .map_err(|_| StoreError::KeyDerivationFailed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory_store::MemoryStore;

    fn address_for_algo25_seed(seed: &[u8; ALGO25_SEED_LEN]) -> Result<String, StoreError> {
        let record = AccountRecord {
            algorithm: KeyAlgorithm::Algo25Ed25519,
            key_material: seed.to_vec(),
        };
        Ok(public_identity(&record, HUMAN_SIGN_INDEX)?.0)
    }

    #[test]
    fn create_account_persists_and_returns_a_valid_address() {
        let mut store = MemoryStore::default();
        let summary = create_account(&mut store, KeyAlgorithm::Algo25Ed25519, HUMAN_SIGN_INDEX)
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
        let first =
            create_account(&mut store, KeyAlgorithm::Algo25Ed25519, HUMAN_SIGN_INDEX).unwrap();
        let second =
            create_account(&mut store, KeyAlgorithm::Algo25Ed25519, HUMAN_SIGN_INDEX).unwrap();
        let third =
            create_account(&mut store, KeyAlgorithm::Algo25Ed25519, HUMAN_SIGN_INDEX).unwrap();

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
        let a = create_account(&mut store, KeyAlgorithm::Algo25Ed25519, HUMAN_SIGN_INDEX).unwrap();
        let b = create_account(&mut store, KeyAlgorithm::Algo25Ed25519, HUMAN_SIGN_INDEX).unwrap();
        assert_ne!(a.address, b.address);
    }

    #[test]
    fn unrecognized_algorithm_tag_does_not_parse() {
        assert_eq!(KeyAlgorithm::from_tag(0x00), None);
        assert_eq!(KeyAlgorithm::from_tag(0x03), None);
        assert_eq!(
            KeyAlgorithm::from_tag(0x01),
            Some(KeyAlgorithm::Algo25Ed25519)
        );
        assert_eq!(KeyAlgorithm::from_tag(0x02), Some(KeyAlgorithm::Falcon1024));
    }

    /// go-algorand `cmd/algokey/pq_test.go` vector: entropy `1..=32`.
    fn falcon_known_entropy() -> [u8; 32] {
        core::array::from_fn(|i| (i + 1) as u8)
    }
    const FALCON_KNOWN_ADDRESS: &str = "ZEJ4BLG3XWAUUZQGCEDJLYIC6D2NCWHRSX5DJMDPE54PXXR7G3PCQTARXU";

    #[test]
    fn falcon_import_matches_go_algorand_known_answer() {
        let mnemonic = algokit_crypto::algo25::mnemonic_from_seed(&falcon_known_entropy()).unwrap();
        let mut store = MemoryStore::default();
        let imported = import_account(
            &mut store,
            KeyAlgorithm::Falcon1024,
            HUMAN_SIGN_INDEX,
            &mnemonic,
        )
        .unwrap();
        assert_eq!(imported.algorithm, KeyAlgorithm::Falcon1024);
        assert_eq!(imported.address, FALCON_KNOWN_ADDRESS);

        // The record caches keygen output after the entropy, and that cache
        // matches a fresh derivation exactly: index 0 is the canonical salt
        // (go-algorand/SDK compatible), index 1 the next compliant salt of
        // the same key.
        let records = store.all_records().unwrap();
        let km = &records[0].key_material;
        assert_eq!(km.len(), falcon_record::LEN);
        assert_eq!(&km[falcon_record::ENTROPY], &falcon_known_entropy());
        let fresh =
            algokit_crypto::falcon1024::account_from_entropy(&falcon_known_entropy()).unwrap();
        assert_eq!(&km[falcon_record::PUBLIC_KEY], &fresh.public_key.0[..]);
        assert_eq!(&km[falcon_record::PRIVATE_KEY], &fresh.private_key.0[..]);
        for key_index in [HUMAN_SIGN_INDEX, AUTOSIGN_INDEX] {
            let (salt, address) =
                algokit_crypto::falcon1024::nth_pq_address(&fresh.public_key.0, key_index as usize)
                    .unwrap();
            assert_eq!(km[falcon_record::salt(key_index)], salt);
            assert_eq!(&km[falcon_record::address(key_index)], &address);
        }
        assert_eq!(km[falcon_record::salt(HUMAN_SIGN_INDEX)], fresh.salt);
        assert_eq!(
            &km[falcon_record::address(HUMAN_SIGN_INDEX)],
            &fresh.address
        );
    }

    use crate::signed_txn::test_txn_from;

    #[test]
    fn falcon_seed_has_human_sign_and_autosign_keys() {
        let mut store = MemoryStore::default();
        let autosign =
            create_account(&mut store, KeyAlgorithm::Falcon1024, AUTOSIGN_INDEX).unwrap();
        assert_eq!(autosign.address_index, AUTOSIGN_INDEX);

        let listed = list_accounts(&mut store).unwrap();
        assert_eq!(
            listed.iter().map(|s| s.address_index).collect::<Vec<_>>(),
            alloc::vec![HUMAN_SIGN_INDEX, AUTOSIGN_INDEX]
        );
        assert_eq!(listed[1], autosign);
        // Same Falcon key; index 0 at the canonical salt, index 1 at the
        // next PQ-compliant one.
        assert_eq!(listed[0].public_key, listed[1].public_key);
        for summary in &listed {
            let (_, address) = algokit_crypto::falcon1024::nth_pq_address(
                &summary.public_key,
                summary.address_index as usize,
            )
            .unwrap();
            assert!(algokit_crypto::falcon1024::is_pq_compliant(&address));
            assert_eq!(address_from_public_key(&address), summary.address);
        }
        assert_ne!(listed[0].address, listed[1].address);

        // One mnemonic backs both; importing it elsewhere (asking for either
        // index) reproduces both keys exactly.
        let mnemonic = reveal_mnemonic(&mut store, &autosign.address).unwrap();
        assert_eq!(
            reveal_mnemonic(&mut store, &listed[0].address).unwrap(),
            mnemonic
        );
        let mut other = MemoryStore::default();
        let human = import_account(
            &mut other,
            KeyAlgorithm::Falcon1024,
            HUMAN_SIGN_INDEX,
            &mnemonic,
        )
        .unwrap();
        assert_eq!(human, listed[0]);
        assert_eq!(list_accounts(&mut other).unwrap(), listed);

        // Deleting via the autosign address removes the whole seed.
        delete_account(&mut store, &autosign.address).unwrap();
        assert!(list_accounts(&mut store).unwrap().is_empty());
    }

    #[test]
    fn ed25519_seed_has_only_the_human_sign_key() {
        let mut store = MemoryStore::default();
        assert_eq!(
            create_account(&mut store, KeyAlgorithm::Algo25Ed25519, AUTOSIGN_INDEX),
            Err(StoreError::UnknownKeyIndex)
        );
        let mnemonic = algokit_crypto::algo25::mnemonic_from_seed(&[5u8; 32]).unwrap();
        assert_eq!(
            import_account(
                &mut store,
                KeyAlgorithm::Algo25Ed25519,
                AUTOSIGN_INDEX,
                &mnemonic
            ),
            Err(StoreError::UnknownKeyIndex)
        );
        assert!(list_accounts(&mut store).unwrap().is_empty());
        assert_eq!(
            create_account(&mut store, KeyAlgorithm::Falcon1024, 2),
            Err(StoreError::UnknownKeyIndex)
        );

        let ed = import_account(
            &mut store,
            KeyAlgorithm::Algo25Ed25519,
            HUMAN_SIGN_INDEX,
            &mnemonic,
        )
        .unwrap();
        assert_eq!(list_accounts(&mut store).unwrap(), alloc::vec![ed.clone()]);
        let txn = test_txn_from(&ed.address);
        assert_eq!(
            sign_transaction(&mut store, &ed.address, AUTOSIGN_INDEX, &txn),
            Err(StoreError::UnknownKeyIndex)
        );
        sign_transaction(&mut store, &ed.address, HUMAN_SIGN_INDEX, &txn).unwrap();
    }

    /// Pulls `(public_key, signature, salt)` out of a Falcon-1024 prefix.
    fn parse_falcon_prefix(prefix: &[u8]) -> (&[u8], &[u8], u8) {
        // `{pqsig: {pk: bin16(1793), sch, sig[, slt]}, txn}`; pk body at byte 14.
        let has_salt = prefix[7] == 0x84;
        let pk = &prefix[14..14 + 1793];
        let rest = prefix[14 + 1793..]
            .strip_prefix(b"\xa3sch\xc4\x02f1\xa3sig")
            .unwrap();
        let (len, start) = match rest[0] {
            0xc4 => (rest[1] as usize, 2),
            _ => (u16::from_be_bytes([rest[1], rest[2]]) as usize, 3),
        };
        let sig = &rest[start..start + len];
        let rest = &rest[start + len..];
        let salt = if has_salt {
            match rest.strip_prefix(b"\xa3slt").unwrap() {
                [0xcc, v, ..] => *v,
                [v, ..] => *v,
                [] => panic!("missing salt"),
            }
        } else {
            0
        };
        (pk, sig, salt)
    }

    #[test]
    fn sign_uses_the_salt_for_the_stated_index() {
        let mut store = MemoryStore::default();
        create_account(&mut store, KeyAlgorithm::Falcon1024, HUMAN_SIGN_INDEX).unwrap();
        let listed = list_accounts(&mut store).unwrap();

        for summary in &listed {
            let txn = test_txn_from(&summary.address);
            let signed =
                sign_transaction(&mut store, &summary.address, summary.address_index, &txn)
                    .unwrap();
            assert_eq!(signed.address_index, summary.address_index);
            // The carried pk + salt derive exactly this account's address
            // (what go-algorand checks against the sender), and the
            // signature verifies under that pk.
            let (pk, sig, salt) = parse_falcon_prefix(&signed.prefix);
            assert_eq!(pk, &summary.public_key[..]);
            assert_eq!(
                address_from_public_key(&algokit_crypto::falcon1024::pq_address(
                    algokit_crypto::falcon1024::SCHEME_FALCON1024,
                    salt,
                    pk,
                )),
                summary.address
            );
            let mut public_key = algokit_crypto::falcon1024::PublicKey::default();
            public_key.0.copy_from_slice(pk);
            public_key
                .verify(sig, &crate::signed_txn::bytes_to_sign(&txn))
                .unwrap();
        }

        // Stating the other index for an address is refused, not silently
        // signed with the other salt.
        for (address, wrong_index) in [
            (&listed[1].address, HUMAN_SIGN_INDEX),
            (&listed[0].address, AUTOSIGN_INDEX),
        ] {
            assert_eq!(
                sign_transaction(&mut store, address, wrong_index, &test_txn_from(address)),
                Err(StoreError::UnknownKeyIndex)
            );
        }
    }

    #[test]
    fn autosign_cannot_sign_the_human_sign_accounts_transactions() {
        // Both accounts share one Falcon key and the salt isn't signed, so an
        // autosign signature over a human-sign transaction would become a
        // valid human-sign authorization by swapping `slt`. The sender check
        // is what stops that.
        let mut store = MemoryStore::default();
        create_account(&mut store, KeyAlgorithm::Falcon1024, AUTOSIGN_INDEX).unwrap();
        let listed = list_accounts(&mut store).unwrap();
        let (human, autosign) = (&listed[0], &listed[1]);

        assert_eq!(
            sign_transaction(
                &mut store,
                &autosign.address,
                AUTOSIGN_INDEX,
                &test_txn_from(&human.address),
            ),
            Err(StoreError::SenderMismatch)
        );
        // A transaction with no sender at all is rejected too.
        assert_eq!(
            sign_transaction(
                &mut store,
                &autosign.address,
                AUTOSIGN_INDEX,
                b"\x81\xa4type\xa3pay",
            ),
            Err(StoreError::InvalidTransaction)
        );
    }

    #[test]
    fn falcon_records_from_earlier_builds_derive_the_same_keys() {
        let entropy = [11u8; 32];
        let mnemonic = algokit_crypto::algo25::mnemonic_from_seed(&entropy).unwrap();
        let mut full = MemoryStore::default();
        import_account(
            &mut full,
            KeyAlgorithm::Falcon1024,
            HUMAN_SIGN_INDEX,
            &mnemonic,
        )
        .unwrap();
        let expected = list_accounts(&mut full).unwrap();
        let full_km = full.all_records().unwrap()[0].key_material.clone();

        // Entropy only, and entropy + keypair + index 0's salt/address only.
        for len in [32, falcon_record::KEYPAIR_LEN] {
            let mut store = MemoryStore::default();
            store
                .persist_record(
                    0,
                    &AccountRecord {
                        algorithm: KeyAlgorithm::Falcon1024,
                        key_material: full_km[..len].to_vec(),
                    },
                )
                .unwrap();
            assert_eq!(list_accounts(&mut store).unwrap(), expected, "len {len}");
            let txn = test_txn_from(&expected[1].address);
            let signed =
                sign_transaction(&mut store, &expected[1].address, AUTOSIGN_INDEX, &txn).unwrap();
            assert_eq!(
                signed,
                sign_transaction(&mut full, &expected[1].address, AUTOSIGN_INDEX, &txn).unwrap()
            );
        }
    }

    #[test]
    fn legacy_entropy_only_falcon_record_still_works() {
        let mut store = MemoryStore::default();
        store
            .persist_record(
                0,
                &AccountRecord {
                    algorithm: KeyAlgorithm::Falcon1024,
                    key_material: falcon_known_entropy().to_vec(),
                },
            )
            .unwrap();
        let listed = list_accounts(&mut store).unwrap();
        assert_eq!(listed[0].address, FALCON_KNOWN_ADDRESS);
        let mnemonic = reveal_mnemonic(&mut store, FALCON_KNOWN_ADDRESS).unwrap();
        assert_eq!(
            algokit_crypto::algo25::seed_from_mnemonic(&mnemonic).unwrap(),
            falcon_known_entropy()
        );
    }

    #[test]
    fn truncated_falcon_record_is_corrupt() {
        let mut store = MemoryStore::default();
        store
            .persist_record(
                0,
                &AccountRecord {
                    algorithm: KeyAlgorithm::Falcon1024,
                    key_material: alloc::vec![0u8; 100],
                },
            )
            .unwrap();
        assert_eq!(list_accounts(&mut store), Err(StoreError::CorruptRecord));
    }

    #[test]
    fn delete_account_removes_only_the_target() {
        let mut store = MemoryStore::default();
        let a = create_account(&mut store, KeyAlgorithm::Algo25Ed25519, HUMAN_SIGN_INDEX).unwrap();
        let b = create_account(&mut store, KeyAlgorithm::Falcon1024, HUMAN_SIGN_INDEX).unwrap();
        let c = create_account(&mut store, KeyAlgorithm::Algo25Ed25519, HUMAN_SIGN_INDEX).unwrap();

        let deleted = delete_account(&mut store, &b.address).unwrap();
        assert_eq!(deleted, b);

        let remaining: Vec<String> = list_accounts(&mut store)
            .unwrap()
            .into_iter()
            .map(|s| s.address)
            .collect();
        assert_eq!(remaining, alloc::vec![a.address.clone(), c.address]);
        assert_eq!(
            reveal_mnemonic(&mut store, &b.address),
            Err(StoreError::NotFound)
        );
        assert_eq!(
            delete_account(&mut store, &b.address),
            Err(StoreError::NotFound)
        );
        // Still fully usable afterwards.
        reveal_mnemonic(&mut store, &a.address).unwrap();
        create_account(&mut store, KeyAlgorithm::Falcon1024, HUMAN_SIGN_INDEX).unwrap();
        // a, c, and the new Falcon seed's two accounts.
        assert_eq!(list_accounts(&mut store).unwrap().len(), 4);
    }

    #[test]
    fn delete_then_reimport_restores_the_account() {
        let mut store = MemoryStore::default();
        let created =
            create_account(&mut store, KeyAlgorithm::Falcon1024, HUMAN_SIGN_INDEX).unwrap();
        let mnemonic = reveal_mnemonic(&mut store, &created.address).unwrap();
        delete_account(&mut store, &created.address).unwrap();
        assert!(list_accounts(&mut store).unwrap().is_empty());

        let reimported = import_account(
            &mut store,
            KeyAlgorithm::Falcon1024,
            HUMAN_SIGN_INDEX,
            &mnemonic,
        )
        .unwrap();
        assert_eq!(reimported.address, created.address);
    }

    #[test]
    fn deleting_one_algorithm_keeps_the_other_for_the_same_mnemonic() {
        let mnemonic = algokit_crypto::algo25::mnemonic_from_seed(&falcon_known_entropy()).unwrap();
        let mut store = MemoryStore::default();
        let ed = import_account(
            &mut store,
            KeyAlgorithm::Algo25Ed25519,
            HUMAN_SIGN_INDEX,
            &mnemonic,
        )
        .unwrap();
        let pq = import_account(
            &mut store,
            KeyAlgorithm::Falcon1024,
            HUMAN_SIGN_INDEX,
            &mnemonic,
        )
        .unwrap();

        delete_account(&mut store, &pq.address).unwrap();
        let listed = list_accounts(&mut store).unwrap();
        assert_eq!(listed, alloc::vec![ed]);
    }

    #[test]
    fn falcon_create_reveal_import_round_trip() {
        let mut store = MemoryStore::default();
        let created =
            create_account(&mut store, KeyAlgorithm::Falcon1024, HUMAN_SIGN_INDEX).unwrap();
        assert_eq!(created.address.len(), 58);

        let mnemonic = reveal_mnemonic(&mut store, &created.address).unwrap();
        assert_eq!(mnemonic.split_whitespace().count(), 25);

        let mut other_store = MemoryStore::default();
        let imported = import_account(
            &mut other_store,
            KeyAlgorithm::Falcon1024,
            HUMAN_SIGN_INDEX,
            &mnemonic,
        )
        .unwrap();
        assert_eq!(imported.address, created.address);

        assert_eq!(
            import_account(
                &mut store,
                KeyAlgorithm::Falcon1024,
                HUMAN_SIGN_INDEX,
                &mnemonic
            ),
            Err(StoreError::AlreadyExists)
        );
    }

    #[test]
    fn same_mnemonic_backs_distinct_ed25519_and_falcon_accounts() {
        let mnemonic = algokit_crypto::algo25::mnemonic_from_seed(&falcon_known_entropy()).unwrap();
        let mut store = MemoryStore::default();
        let ed = import_account(
            &mut store,
            KeyAlgorithm::Algo25Ed25519,
            HUMAN_SIGN_INDEX,
            &mnemonic,
        )
        .unwrap();
        let pq = import_account(
            &mut store,
            KeyAlgorithm::Falcon1024,
            HUMAN_SIGN_INDEX,
            &mnemonic,
        )
        .unwrap();
        assert_ne!(ed.address, pq.address);

        let listed = list_accounts(&mut store).unwrap();
        assert_eq!(listed.len(), 3);
        assert_eq!(listed[0].algorithm, KeyAlgorithm::Algo25Ed25519);
        assert_eq!(listed[1].algorithm, KeyAlgorithm::Falcon1024);
        assert_eq!(listed[1].address, FALCON_KNOWN_ADDRESS);
        assert_eq!(listed[2].algorithm, KeyAlgorithm::Falcon1024);
        assert_eq!(listed[2].address_index, AUTOSIGN_INDEX);

        // Either address reveals the same 25 words.
        assert_eq!(reveal_mnemonic(&mut store, &ed.address).unwrap(), mnemonic);
        assert_eq!(reveal_mnemonic(&mut store, &pq.address).unwrap(), mnemonic);
    }

    #[test]
    fn reveal_mnemonic_recovers_the_same_address() {
        let mut store = MemoryStore::default();
        let summary =
            create_account(&mut store, KeyAlgorithm::Algo25Ed25519, HUMAN_SIGN_INDEX).unwrap();

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
        let first =
            create_account(&mut store, KeyAlgorithm::Algo25Ed25519, HUMAN_SIGN_INDEX).unwrap();
        let second =
            create_account(&mut store, KeyAlgorithm::Algo25Ed25519, HUMAN_SIGN_INDEX).unwrap();

        let mnemonic = reveal_mnemonic(&mut store, &second.address).unwrap();
        let seed = algokit_crypto::algo25::seed_from_mnemonic(&mnemonic).unwrap();
        assert_eq!(address_for_algo25_seed(&seed).unwrap(), second.address);
        assert_ne!(address_for_algo25_seed(&seed).unwrap(), first.address);
    }

    #[test]
    fn reveal_mnemonic_unknown_address_is_not_found() {
        let mut store = MemoryStore::default();
        create_account(&mut store, KeyAlgorithm::Algo25Ed25519, HUMAN_SIGN_INDEX).unwrap();
        assert_eq!(
            reveal_mnemonic(&mut store, &"A".repeat(58)),
            Err(StoreError::NotFound)
        );
    }

    #[test]
    fn import_account_round_trips_with_create_and_reveal() {
        let mut store = MemoryStore::default();
        let created =
            create_account(&mut store, KeyAlgorithm::Algo25Ed25519, HUMAN_SIGN_INDEX).unwrap();
        let mnemonic = reveal_mnemonic(&mut store, &created.address).unwrap();

        // Importing a *different* store with that same mnemonic must
        // recover the identical address.
        let mut other_store = MemoryStore::default();
        let imported = import_account(
            &mut other_store,
            KeyAlgorithm::Algo25Ed25519,
            HUMAN_SIGN_INDEX,
            &mnemonic,
        )
        .unwrap();
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
                HUMAN_SIGN_INDEX,
                "not a real mnemonic"
            ),
            Err(StoreError::InvalidMnemonic)
        );
    }

    #[test]
    fn import_account_rejects_a_seed_already_present() {
        let mut store = MemoryStore::default();
        let created =
            create_account(&mut store, KeyAlgorithm::Algo25Ed25519, HUMAN_SIGN_INDEX).unwrap();
        let mnemonic = reveal_mnemonic(&mut store, &created.address).unwrap();

        assert_eq!(
            import_account(
                &mut store,
                KeyAlgorithm::Algo25Ed25519,
                HUMAN_SIGN_INDEX,
                &mnemonic
            ),
            Err(StoreError::AlreadyExists)
        );
        // The duplicate attempt must not have been persisted.
        assert_eq!(store.all_records().unwrap().len(), 1);
    }
}
