use crate::{Address, ALGORAND_SIGNATURE_BYTE_LENGTH};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, Bytes};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct MultisigMetadata {
    #[serde(rename = "v")]
    pub version: u8,
    #[serde(rename = "thr")]
    pub threshold: u8,
    #[serde(rename = "subsig")]
    pub addrs: Vec<Address>,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct MultisigSubsig {
    #[serde(rename = "s")]
    #[serde_as(as = "Bytes")]
    pub sig: [u8; ALGORAND_SIGNATURE_BYTE_LENGTH],
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct MultisigSignature {
    #[serde(rename = "msig")]
    pub multisig: MultisigMetadata,
    #[serde(rename = "subsig")]
    pub subsigs: Vec<MultisigSubsig>,
}
