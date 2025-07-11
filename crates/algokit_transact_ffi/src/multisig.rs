use crate::{bytebuf_to_bytes, AlgoKitTransactError};

use ffi_macros::{ffi_func, ffi_record};
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
#[ffi_record]
pub struct MultisigSignature {
    address: String,
    version: u8,
    threshold: u8,
    subsignatures: Vec<MultisigSubsignature>,
}

#[ffi_record]
pub struct MultisigSubsignature {
    address: String,
    signature: Option<ByteBuf>,
}

impl From<algokit_transact::MultisigSignature> for MultisigSignature {
    fn from(value: algokit_transact::MultisigSignature) -> Self {
        Self {
            address: value.to_string(),
            version: value.version,
            threshold: value.threshold,
            subsignatures: value.subsignatures.into_iter().map(Into::into).collect(),
        }
    }
}

impl TryFrom<MultisigSignature> for algokit_transact::MultisigSignature {
    type Error = AlgoKitTransactError;

    fn try_from(value: MultisigSignature) -> Result<Self, Self::Error> {
        Ok(Self::new(
            value.version,
            value.threshold,
            value
                .subsignatures
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
        )?)
    }
}

impl From<algokit_transact::MultisigSubsignature> for MultisigSubsignature {
    fn from(value: algokit_transact::MultisigSubsignature) -> Self {
        Self {
            address: value.address.as_str(),
            signature: value.signature.map(|sig| sig.to_vec().into()),
        }
    }
}

impl TryFrom<MultisigSubsignature> for algokit_transact::MultisigSubsignature {
    type Error = AlgoKitTransactError;

    fn try_from(value: MultisigSubsignature) -> Result<Self, Self::Error> {
        let address = value.address.parse()?;

        Ok(Self {
            address,
            signature: value
                .signature
                .map(|sig| bytebuf_to_bytes(&sig))
                .transpose()
                .map_err(|e| {
                    AlgoKitTransactError::DecodingError(format!(
                        "Error while decoding a subsignature: {}",
                        e.to_string()
                    ))
                })?,
        })
    }
}

#[ffi_func]
pub fn empty_multisig_signature(
    version: u8,
    threshold: u8,
    participants: Vec<String>,
) -> Result<MultisigSignature, AlgoKitTransactError> {
    Ok(algokit_transact::MultisigSignature::from_participants(
        version,
        threshold,
        participants
            .into_iter()
            .map(|addr| addr.parse())
            .collect::<Result<Vec<_>, _>>()
            .map_err(AlgoKitTransactError::from)?,
    )
    .map(Into::into)?)
}

#[ffi_func]
pub fn participants_from_multisig_signature(
    multisig_signature: MultisigSignature,
) -> Result<Vec<String>, AlgoKitTransactError> {
    let multisig: algokit_transact::MultisigSignature = multisig_signature.try_into()?;
    Ok(multisig
        .participants()
        .into_iter()
        .map(|addr| addr.to_string())
        .collect())
}

#[ffi_func]
pub fn apply_multisig_subsignature(
    multisig_signature: MultisigSignature,
    participant: String,
    signature: ByteBuf,
) -> Result<MultisigSignature, AlgoKitTransactError> {
    let multisignature: algokit_transact::MultisigSignature = multisig_signature.try_into()?;
    let partially_signed_multisignature = multisignature.apply_subsignature(
        participant.parse()?,
        bytebuf_to_bytes(&signature).map_err(|e| {
            AlgoKitTransactError::DecodingError(format!(
                "Error while decoding a subsignature: {}",
                e.to_string()
            ))
        })?,
    )?;
    Ok(partially_signed_multisignature.into())
}

#[ffi_func]
pub fn merge_multignatures(
    multisig_signature_a: MultisigSignature,
    multisig_signature_b: MultisigSignature,
) -> Result<MultisigSignature, AlgoKitTransactError> {
    let multisig_a: algokit_transact::MultisigSignature = multisig_signature_a.try_into()?;
    let multisig_b: algokit_transact::MultisigSignature = multisig_signature_b.try_into()?;
    let merged_multisig = multisig_a.merge(&multisig_b)?;
    Ok(merged_multisig.into())
}
