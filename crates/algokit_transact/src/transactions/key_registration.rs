//! Key registration transaction module for AlgoKit Core.
//!
//! This module provides functionality for creating and managing key registration transactions,
//! which are used to register accounts online or offline for participation in Algorand consensus.

use crate::traits::Validate;
use crate::utils::is_zero_opt;
use crate::{transactions::common::TransactionHeader, Transaction};
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
    #[serde(rename = "votekey")]
    #[serde_as(as = "Option<Bytes>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[builder(default)]
    pub vote_key: Option<[u8; 32]>,

    /// VRF public key (32 bytes).
    #[serde(rename = "selkey")]
    #[serde_as(as = "Option<Bytes>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[builder(default)]
    pub selection_key: Option<[u8; 32]>,

    /// State proof key (64 bytes).
    #[serde(rename = "sprfkey")]
    #[serde_as(as = "Option<Bytes>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[builder(default)]
    pub state_proof_key: Option<[u8; 64]>,

    /// First round for which the participation key is valid.
    #[serde(rename = "votefst")]
    #[serde(skip_serializing_if = "is_zero_opt")]
    #[serde(default)]
    #[builder(default)]
    pub vote_first: Option<u64>,

    /// Last round for which the participation key is valid.
    #[serde(rename = "votelst")]
    #[serde(skip_serializing_if = "is_zero_opt")]
    #[serde(default)]
    #[builder(default)]
    pub vote_last: Option<u64>,

    /// Key dilution for the 2-level participation key.
    #[serde(rename = "votekd")]
    #[serde(skip_serializing_if = "is_zero_opt")]
    #[serde(default)]
    #[builder(default)]
    pub vote_key_dilution: Option<u64>,

    /// Mark account as non-reward earning.
    #[serde(rename = "nonpart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[builder(default)]
    pub non_participation: Option<bool>,
}

impl KeyRegistrationTransactionFields {
    /// Returns true if this is an online key registration transaction.
    pub fn is_online(&self) -> bool {
        self.vote_key.is_some()
            && self.selection_key.is_some()
            && self.state_proof_key.is_some()
            && self.vote_first.is_some()
            && self.vote_last.is_some()
            && self.vote_key_dilution.is_some()
    }

    /// Returns true if this is an offline key registration transaction.
    pub fn is_offline(&self) -> bool {
        self.vote_key.is_none()
            && self.selection_key.is_none()
            && self.state_proof_key.is_none()
            && self.vote_first.is_none()
            && self.vote_last.is_none()
            && self.vote_key_dilution.is_none()
    }

    /// Returns true if this transaction registers the account as non-participating.
    /// WARNING: This means the account will NEVER be able to participate in consensus
    pub fn is_non_participating(&self) -> bool {
        self.non_participation.is_some_and(|v| v)
    }
}

impl KeyRegistrationTransactionBuilder {
    pub fn build(&self) -> Result<Transaction, KeyRegistrationTransactionBuilderError> {
        let d = self.build_fields()?;
        d.validate().map_err(|errors| {
            KeyRegistrationTransactionBuilderError::ValidationError(format!(
                "Key registration validation failed: {}",
                errors.join("\n")
            ))
        })?;
        Ok(Transaction::KeyRegistration(d))
    }
}

impl Validate for KeyRegistrationTransactionFields {
    fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        let has_any_participation_fields = self.vote_key.is_some()
            || self.selection_key.is_some()
            || self.state_proof_key.is_some()
            || self.vote_first.is_some()
            || self.vote_last.is_some()
            || self.vote_key_dilution.is_some();

        if has_any_participation_fields {
            if self.vote_key.is_none() {
                errors.push("vote_key is required for online key registration".to_string());
            }
            if self.selection_key.is_none() {
                errors.push("selection_key is required for online key registration".to_string());
            }
            if self.state_proof_key.is_none() {
                errors.push("state_proof_key is required for online key registration".to_string());
            }
            if self.vote_first.is_none() {
                errors.push("vote_first is required for online key registration".to_string());
            }
            if self.vote_last.is_none() {
                errors.push("vote_last is required for online key registration".to_string());
            }
            if self.vote_key_dilution.is_none() {
                errors
                    .push("vote_key_dilution is required for online key registration".to_string());
            }

            if let (Some(first), Some(last)) = (self.vote_first, self.vote_last) {
                if first >= last {
                    errors.push("vote_first must be less than vote_last".to_string());
                }
            }

            if self.is_non_participating() {
                errors.push(
                    "Online key registration cannot have non_participation flag set".to_string(),
                );
            }
        }

        match errors.is_empty() {
            true => Ok(()),
            false => Err(errors),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::TransactionHeaderMother;

    fn create_test_header() -> TransactionHeader {
        TransactionHeaderMother::example().build().unwrap()
    }

    #[test]
    fn test_validate_valid_online_key_registration() {
        let key_reg = KeyRegistrationTransactionFields {
            header: create_test_header(),
            vote_key: Some([1u8; 32]),
            selection_key: Some([2u8; 32]),
            state_proof_key: Some([3u8; 64]),
            vote_first: Some(100),
            vote_last: Some(200),
            vote_key_dilution: Some(1000),
            non_participation: None,
        };

        let result = key_reg.validate();
        assert!(result.is_ok());
        assert!(key_reg.is_online());
        assert!(!key_reg.is_offline());
        assert!(!key_reg.is_non_participating());
    }

    #[test]
    fn test_validate_valid_offline_key_registration() {
        let key_reg = KeyRegistrationTransactionFields {
            header: create_test_header(),
            vote_key: None,
            selection_key: None,
            state_proof_key: None,
            vote_first: None,
            vote_last: None,
            vote_key_dilution: None,
            non_participation: None,
        };

        let result = key_reg.validate();
        assert!(result.is_ok());
        assert!(!key_reg.is_online());
        assert!(key_reg.is_offline());
        assert!(!key_reg.is_non_participating());
    }

    #[test]
    fn test_validate_valid_non_participating_key_registration() {
        let key_reg = KeyRegistrationTransactionFields {
            header: create_test_header(),
            vote_key: None,
            selection_key: None,
            state_proof_key: None,
            vote_first: None,
            vote_last: None,
            vote_key_dilution: None,
            non_participation: Some(true),
        };

        let result = key_reg.validate();
        assert!(result.is_ok());
        assert!(!key_reg.is_online());
        assert!(key_reg.is_offline()); // Non-participating is considered offline
        assert!(key_reg.is_non_participating());
    }

    #[test]
    fn test_validate_online_missing_vote_key() {
        let key_reg = KeyRegistrationTransactionFields {
            header: create_test_header(),
            vote_key: None, // Missing required field
            selection_key: Some([2u8; 32]),
            state_proof_key: Some([3u8; 64]),
            vote_first: Some(100),
            vote_last: Some(200),
            vote_key_dilution: Some(1000),
            non_participation: None,
        };

        let result = key_reg.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.contains("vote_key is required")));
    }

    #[test]
    fn test_validate_online_multiple_missing_fields() {
        let key_reg = KeyRegistrationTransactionFields {
            header: create_test_header(),
            vote_key: None,      // Missing
            selection_key: None, // Missing
            state_proof_key: Some([3u8; 64]),
            vote_first: None, // Missing
            vote_last: Some(200),
            vote_key_dilution: None, // Missing
            non_participation: None,
        };

        let result = key_reg.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 4); // Should have 4 missing field errors

        let error_text = errors.join("\n");
        assert!(error_text.contains("vote_key is required"));
        assert!(error_text.contains("selection_key is required"));
        assert!(error_text.contains("vote_first is required"));
        assert!(error_text.contains("vote_key_dilution is required"));
    }

    #[test]
    fn test_validate_invalid_vote_round_range() {
        let key_reg = KeyRegistrationTransactionFields {
            header: create_test_header(),
            vote_key: Some([1u8; 32]),
            selection_key: Some([2u8; 32]),
            state_proof_key: Some([3u8; 64]),
            vote_first: Some(200), // Greater than vote_last
            vote_last: Some(100),
            vote_key_dilution: Some(1000),
            non_participation: None,
        };

        let result = key_reg.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors
            .iter()
            .any(|e| e.contains("vote_first must be less than vote_last")));
    }

    #[test]
    fn test_validate_equal_vote_rounds() {
        let key_reg = KeyRegistrationTransactionFields {
            header: create_test_header(),
            vote_key: Some([1u8; 32]),
            selection_key: Some([2u8; 32]),
            state_proof_key: Some([3u8; 64]),
            vote_first: Some(100), // Equal to vote_last
            vote_last: Some(100),
            vote_key_dilution: Some(1000),
            non_participation: None,
        };

        let result = key_reg.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors
            .iter()
            .any(|e| e.contains("vote_first must be less than vote_last")));
    }

    #[test]
    fn test_validate_online_with_non_participation_flag() {
        let key_reg = KeyRegistrationTransactionFields {
            header: create_test_header(),
            vote_key: Some([1u8; 32]),
            selection_key: Some([2u8; 32]),
            state_proof_key: Some([3u8; 64]),
            vote_first: Some(100),
            vote_last: Some(200),
            vote_key_dilution: Some(1000),
            non_participation: Some(true), // Invalid for online registration
        };

        let result = key_reg.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors
            .iter()
            .any(|e| e.contains("Online key registration cannot have non_participation flag set")));
    }

    #[test]
    fn test_validate_offline_with_participation_fields() {
        let key_reg = KeyRegistrationTransactionFields {
            header: create_test_header(),
            vote_key: Some([1u8; 32]), // Should not be set for offline
            selection_key: None,
            state_proof_key: None,
            vote_first: None,
            vote_last: None,
            vote_key_dilution: None,
            non_participation: None,
        };

        let result = key_reg.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        // Since vote_key is set, it should be treated as online registration with missing required fields
        assert!(errors
            .iter()
            .any(|e| e.contains("selection_key is required")));
        assert!(errors
            .iter()
            .any(|e| e.contains("state_proof_key is required")));
        assert!(errors.iter().any(|e| e.contains("vote_first is required")));
        assert!(errors.iter().any(|e| e.contains("vote_last is required")));
        assert!(errors
            .iter()
            .any(|e| e.contains("vote_key_dilution is required")));
    }

    #[test]
    fn test_validate_partial_fields_invalid() {
        let key_reg = KeyRegistrationTransactionFields {
            header: create_test_header(),
            vote_key: Some([1u8; 32]),
            selection_key: Some([2u8; 32]),
            state_proof_key: None, // Missing some fields
            vote_first: None,
            vote_last: None,
            vote_key_dilution: None,
            non_participation: None,
        };

        let result = key_reg.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        // Should have specific missing field errors since some participation fields are set
        assert!(errors
            .iter()
            .any(|e| e.contains("state_proof_key is required")));
        assert!(errors.iter().any(|e| e.contains("vote_first is required")));
        assert!(errors
            .iter()
            .any(|e| e.contains("vote_key_dilution is required")));
    }

    #[test]
    fn test_builder_validation_integration() {
        // Test that the builder properly validates
        let result = KeyRegistrationTransactionBuilder::default()
            .header(create_test_header())
            .vote_key([1u8; 32])
            .selection_key([2u8; 32])
            // Missing state_proof_key and other required fields
            .build();

        assert!(result.is_err());

        // Test valid builder
        let result = KeyRegistrationTransactionBuilder::default()
            .header(create_test_header())
            .vote_key([1u8; 32])
            .selection_key([2u8; 32])
            .state_proof_key([3u8; 64])
            .vote_first(100)
            .vote_last(200)
            .vote_key_dilution(1000)
            .build();

        assert!(result.is_ok());
        if let Ok(Transaction::KeyRegistration(fields)) = result {
            assert!(fields.is_online());
        }
    }
}
