use crate::*;

/// Representation of an Algorand multisignature signature.
///
/// This struct mirrors the Rust [`algokit_transact::MultisigSignature`] type, but uses FFI-safe types
/// for cross-language interoperability.
#[ffi_record]
pub struct MultisigSignature {
    /// The derived address for the multisignature group.
    address: String,
    /// Multisig version.
    version: u8,
    /// Minimum number of signatures required.
    threshold: u8,
    /// List of subsignatures for each participant.
    subsignatures: Vec<MultisigSubsignature>,
}

/// Representation of a single subsignature in a multisignature transaction.
///
/// Each subsignature contains the participant's address and an optional signature.
#[ffi_record]
pub struct MultisigSubsignature {
    /// Address of the participant.
    address: String,
    /// Optional signature bytes for the participant.
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
                        e
                    ))
                })?,
        })
    }
}

/// Creates an empty multisignature signature from a list of participant addresses.
///
/// # Errors
///
/// Returns [`AlgoKitTransactError`] if any address is invalid or the multisignature parameters are invalid.
#[ffi_func]
pub fn new_multisig_signature(
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
            .collect::<Result<Vec<_>, _>>()?,
    )
    .map(Into::into)?)
}

/// Returns the list of participant addresses from a multisignature signature.
///
/// # Errors
///
/// Returns [`AlgoKitTransactError`] if the multisignature is invalid.
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

/// Applies a subsignature for a participant to a multisignature signature, replacing any existing signature.
///
/// # Errors
///
/// Returns [`AlgoKitTransactError`] if the participant address is invalid or not found, or if the signature bytes are invalid.
#[ffi_func]
pub fn apply_multisig_subsignature(
    multisig_signature: MultisigSignature,
    participant: String,
    subsignature: &[u8],
) -> Result<MultisigSignature, AlgoKitTransactError> {
    let multisignature: algokit_transact::MultisigSignature = multisig_signature.try_into()?;
    let partially_signed_multisignature = multisignature.apply_subsignature(
        participant.parse()?,
        subsignature.try_into().map_err(|_| {
            AlgoKitTransactError::EncodingError(format!(
                "signature should be {} bytes",
                ALGORAND_SIGNATURE_BYTE_LENGTH
            ))
        })?,
    )?;
    Ok(partially_signed_multisignature.into())
}

/// Merges two multisignature signatures, replacing signatures in the first with those from the second where present.
///
/// # Errors
///
/// Returns [`AlgoKitTransactError`] if the multisignature parameters or participants do not match.
#[ffi_func]
pub fn merge_multisignatures(
    multisig_signature_a: MultisigSignature,
    multisig_signature_b: MultisigSignature,
) -> Result<MultisigSignature, AlgoKitTransactError> {
    let multisig_a: algokit_transact::MultisigSignature = multisig_signature_a.try_into()?;
    let multisig_b: algokit_transact::MultisigSignature = multisig_signature_b.try_into()?;
    let merged_multisig = multisig_a.merge(&multisig_b)?;
    Ok(merged_multisig.into())
}
