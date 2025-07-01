//! Algorand multisignature account representation and manipulation.
//!
//! This module provides the [`MultisigAccount`] type, which encapsulates an Algorand multisignature
//! account's version, threshold, and participating addresses. The corresponding [`Address`] is derived
//! from the domain separator, version, threshold, and the concatenated addresses, hashed to produce
//! the 32-byte digest used as the address.
//!
//! Contrary to the single signature account, it's not possible to derive a multisignature account
//! from its address, as the "public information" of a multisig account is derived with
//! a cryptographic hash function.

use crate::address::Address;
use crate::utils::hash;
use crate::{
    ALGORAND_PUBLIC_KEY_BYTE_LENGTH, ALGORAND_SIGNATURE_BYTE_LENGTH, MULTISIG_DOMAIN_SEPARATOR,
};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, Bytes};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Represents an Algorand multisignature account.
///
/// A multisignature account is defined by a version, a threshold, and a list of participating addresses.
/// The version indicates the multisig protocol version, while the threshold specifies the minimum
/// number of signatures required to authorize a transaction.
/// While technically this accepts [`Address`] types, it is expected that these will be
/// the addresses of [`Account`]s, which are 32-byte Ed25519 public keys.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct MultisigAccount {
    /// Multisig version.
    #[serde(rename = "v")]
    pub version: u8,
    /// Minimum number of signatures required.
    #[serde(rename = "thr")]
    pub threshold: u8,
    /// List of participating addresses.
    #[serde(rename = "subsig")]
    pub addrs: Vec<Address>,
}

/// Represents a single subsignature in a multisignature transaction.
#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct MultisigSubsig {
    /// The signature bytes.
    #[serde(rename = "s")]
    #[serde_as(as = "Bytes")]
    pub sig: [u8; ALGORAND_SIGNATURE_BYTE_LENGTH],
}

/// Represents a multisignature for a transaction, including the account and all subsignatures.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct MultisigSignature {
    /// The associated multisig account.
    #[serde(rename = "msig")]
    pub multisig: MultisigAccount,
    /// The list of subsignatures.
    #[serde(rename = "subsig")]
    pub subsigs: Vec<MultisigSubsig>,
}

impl From<MultisigAccount> for Address {
    /// Converts a [`MultisigAccount`] into an [`Address`] by hashing the domain separator,
    /// version, threshold, and all participating addresses.
    fn from(msig: MultisigAccount) -> Address {
        let mut buffer = Vec::with_capacity(
            MULTISIG_DOMAIN_SEPARATOR.len()
                + 2
                + msig.addrs.len() * ALGORAND_PUBLIC_KEY_BYTE_LENGTH,
        );
        buffer.extend_from_slice(MULTISIG_DOMAIN_SEPARATOR.as_bytes());
        buffer.push(msig.version);
        buffer.push(msig.threshold);
        for addr in &msig.addrs {
            buffer.extend_from_slice(addr.as_bytes());
        }
        let digest = hash(&buffer);

        Address { 0: digest }
    }
}

impl Display for MultisigAccount {
    /// Formats the [`MultisigAccount`] as a base32-encoded Algorand address string.
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", Address::from(self.clone()).as_str())
    }
}
