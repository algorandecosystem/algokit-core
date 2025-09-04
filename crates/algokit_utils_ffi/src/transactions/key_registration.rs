use crate::transactions::common::UtilsError;

use super::common::CommonParams;
use algokit_utils::transactions::{
    NonParticipationKeyRegistrationParams as RustNonParticipationKeyRegistrationParams,
    OfflineKeyRegistrationParams as RustOfflineKeyRegistrationParams,
    OnlineKeyRegistrationParams as RustOnlineKeyRegistrationParams,
};

#[derive(uniffi::Record)]
pub struct OnlineKeyRegistrationParams {
    /// Common transaction parameters.
    pub common_params: CommonParams,

    /// The root participation public key.
    pub vote_key: Vec<u8>,

    /// The VRF public key.
    pub selection_key: Vec<u8>,

    /// The first round that the participation key is valid.
    pub vote_first: u64,

    /// The last round that the participation key is valid.
    pub vote_last: u64,

    /// This is the dilution for the 2-level participation key.
    pub vote_key_dilution: u64,

    /// The 64 byte state proof public key commitment.
    pub state_proof_key: Option<Vec<u8>>,
}

#[derive(uniffi::Record)]
pub struct OfflineKeyRegistrationParams {
    /// Common transaction parameters.
    pub common_params: CommonParams,

    /// Mark account as non-reward earning.
    pub non_participation: Option<bool>,
}

#[derive(uniffi::Record)]
pub struct NonParticipationKeyRegistrationParams {
    /// Common transaction parameters.
    pub common_params: CommonParams,
}

impl TryFrom<OnlineKeyRegistrationParams> for RustOnlineKeyRegistrationParams {
    type Error = UtilsError;

    fn try_from(params: OnlineKeyRegistrationParams) -> Result<Self, Self::Error> {
        let common_params = params.common_params.try_into()?;

        // Convert Vec<u8> to [u8; 32] for vote_key
        let vote_key: [u8; 32] =
            params
                .vote_key
                .try_into()
                .map_err(|_| UtilsError::UtilsError {
                    message: "Vote key must be exactly 32 bytes".to_string(),
                })?;

        // Convert Vec<u8> to [u8; 32] for selection_key
        let selection_key: [u8; 32] =
            params
                .selection_key
                .try_into()
                .map_err(|_| UtilsError::UtilsError {
                    message: "Selection key must be exactly 32 bytes".to_string(),
                })?;

        // Convert Option<Vec<u8>> to Option<[u8; 64]> for state_proof_key
        let state_proof_key = match params.state_proof_key {
            Some(key) => {
                let key_array: [u8; 64] = key.try_into().map_err(|_| UtilsError::UtilsError {
                    message: "State proof key must be exactly 64 bytes".to_string(),
                })?;
                Some(key_array)
            }
            None => None,
        };

        Ok(RustOnlineKeyRegistrationParams {
            common_params,
            vote_key,
            selection_key,
            vote_first: params.vote_first,
            vote_last: params.vote_last,
            vote_key_dilution: params.vote_key_dilution,
            state_proof_key,
        })
    }
}

impl TryFrom<OfflineKeyRegistrationParams> for RustOfflineKeyRegistrationParams {
    type Error = UtilsError;

    fn try_from(params: OfflineKeyRegistrationParams) -> Result<Self, Self::Error> {
        let common_params = params.common_params.try_into()?;
        Ok(RustOfflineKeyRegistrationParams {
            common_params,
            non_participation: params.non_participation,
        })
    }
}

impl TryFrom<NonParticipationKeyRegistrationParams> for RustNonParticipationKeyRegistrationParams {
    type Error = UtilsError;

    fn try_from(params: NonParticipationKeyRegistrationParams) -> Result<Self, Self::Error> {
        let common_params = params.common_params.try_into()?;
        Ok(RustNonParticipationKeyRegistrationParams { common_params })
    }
}
