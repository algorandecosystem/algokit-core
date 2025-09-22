//! Heartbeat transaction module for AlgoKit Core.
//!
//! This module provides functionality for creating and managing heartbeat transactions,
//! which are used to maintain participation in Algorand consensus.

use crate::Address;
use crate::Transaction;
use crate::traits::Validate;
use crate::transactions::common::TransactionHeader;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::{Bytes, serde_as};

/// Represents proof information for a heartbeat transaction.
#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Builder)]
#[builder(name = "HeartbeatProofBuilder", build_fn(name = "build_fields"))]
pub struct HeartbeatProof {
    /// Signature (64 bytes).
    #[serde(rename = "s")]
    #[serde_as(as = "Bytes")]
    pub sig: [u8; 64],

    /// Public key (32 bytes).
    #[serde(rename = "p")]
    #[serde_as(as = "Bytes")]
    pub pk: [u8; 32],

    /// Public key 2 (32 bytes).
    #[serde(rename = "p2")]
    #[serde_as(as = "Bytes")]
    pub pk2: [u8; 32],

    /// Public key 1 signature (64 bytes).
    #[serde(rename = "p1s")]
    #[serde_as(as = "Bytes")]
    pub pk1_sig: [u8; 64],

    /// Public key 2 signature (64 bytes).
    #[serde(rename = "p2s")]
    #[serde_as(as = "Bytes")]
    pub pk2_sig: [u8; 64],
}

impl HeartbeatProofBuilder {
    pub fn build(&self) -> Result<HeartbeatProof, HeartbeatProofBuilderError> {
        self.build_fields()
    }
}

/// Represents a heartbeat transaction that maintains participation in Algorand consensus.
#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Builder)]
#[builder(name = "HeartbeatTransactionBuilder", build_fn(name = "build_fields"))]
pub struct HeartbeatTransactionFields {
    /// Common transaction header fields.
    #[serde(flatten)]
    pub header: TransactionHeader,

    /// Heartbeat address.
    #[serde(rename = "a")]
    pub address: Address,

    /// Heartbeat proof.
    #[serde(rename = "prf")]
    pub proof: HeartbeatProof,

    /// Heartbeat seed.
    #[serde(rename = "sd")]
    #[serde_as(as = "Bytes")]
    pub seed: Vec<u8>,

    /// Heartbeat vote ID (32 bytes).
    #[serde(rename = "vid")]
    #[serde_as(as = "Bytes")]
    pub vote_id: [u8; 32],

    /// Heartbeat key dilution.
    #[serde(rename = "kd")]
    pub key_dilution: u64,
}

impl HeartbeatTransactionBuilder {
    pub fn build(&self) -> Result<Transaction, HeartbeatTransactionBuilderError> {
        let d = self.build_fields()?;
        d.validate().map_err(|errors| {
            HeartbeatTransactionBuilderError::ValidationError(format!(
                "Heartbeat validation failed: {}",
                errors.join("\n")
            ))
        })?;
        Ok(Transaction::Heartbeat(d))
    }
}

impl Validate for HeartbeatTransactionFields {
    fn validate(&self) -> Result<(), Vec<String>> {
        // For now, no validation is required as per the request
        Ok(())
    }
}

// https://lora.algokit.io/testnet/transaction/GCVW7GJTD5OALIXPQ3RGMYKTTYCWUJY3E4RPJTX7WHIWZK4V6NYA
