//! Key registration transaction module for AlgoKit Core.
//!
//! This module provides functionality for creating and managing key registration transactions,
//! which are used to register accounts online or offline for participation in Algorand consensus.

use crate::transactions::common::TransactionHeader;
use crate::utils::is_zero_opt;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none, Bytes};

/// Represents a key registration transaction that registers an account online or offline
/// for participation in Algorand consensus.
#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Builder)]
#[builder(
    name = "KeyRegistrationTransactionBuilder",
    setter(strip_option),
    build_fn(name = "build_fields")
)]
pub struct KeyRegistrationTransactionFields {
    /// Common transaction header fields.
    #[serde(flatten)]
    pub header: TransactionHeader,

    /// Root participation public key (32 bytes).
    /// Required for online key registration.
    #[serde(rename = "votekey")]
    #[serde_as(as = "Option<Bytes>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[builder(default)]
    pub vote_key: Option<[u8; 32]>,

    /// VRF public key (32 bytes).
    /// Required for online key registration.
    #[serde(rename = "selkey")]
    #[serde_as(as = "Option<Bytes>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[builder(default)]
    pub selection_key: Option<[u8; 32]>,

    /// State proof key (64 bytes).
    /// Required for online key registration.
    #[serde(rename = "sprfkey")]
    #[serde_as(as = "Option<Bytes>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[builder(default)]
    pub state_proof_key: Option<[u8; 64]>,

    /// First round for which the participation key is valid.
    /// Required for online key registration.
    #[serde(rename = "votefst")]
    #[serde(skip_serializing_if = "is_zero_opt")]
    #[serde(default)]
    #[builder(default)]
    pub vote_first: Option<u64>,

    /// Last round for which the participation key is valid.
    /// Required for online key registration.
    #[serde(rename = "votelst")]
    #[serde(skip_serializing_if = "is_zero_opt")]
    #[serde(default)]
    #[builder(default)]
    pub vote_last: Option<u64>,

    /// Key dilution for the 2-level participation key.
    /// Required for online key registration.
    #[serde(rename = "votekd")]
    #[serde(skip_serializing_if = "is_zero_opt")]
    #[serde(default)]
    #[builder(default)]
    pub vote_key_dilution: Option<u64>,

    /// Mark account as non-reward earning.
    /// Optional field that can be used with both online and offline registration.
    #[serde(rename = "nonpart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[builder(default)]
    pub non_participation: Option<bool>,
}

impl KeyRegistrationTransactionFields {
    /// Returns true if this is an online key registration transaction.
    /// Online transactions have all required participation key fields.
    pub fn is_online(&self) -> bool {
        self.vote_key.is_some()
            && self.selection_key.is_some()
            && self.state_proof_key.is_some()
            && self.vote_first.is_some()
            && self.vote_last.is_some()
            && self.vote_key_dilution.is_some()
    }

    /// Returns true if this is an offline key registration transaction.
    /// Offline transactions have no participation key fields set.
    /// NOTE: This also ensures this is NOT a non-participating transaction
    pub fn is_offline(&self) -> bool {
        self.vote_key.is_none()
            && self.selection_key.is_none()
            && self.state_proof_key.is_none()
            && self.vote_first.is_none()
            && self.vote_last.is_none()
            && self.vote_key_dilution.is_none()
            && !self.is_non_participating()
    }

    /// Returns true if this transaction registers the account as non-participating.
    /// WARNING: This means the account will NEVER be able to participate in consensus
    pub fn is_non_participating(&self) -> bool {
        self.non_participation.is_some_and(|v| v)
    }
}
