pub const HASH_BYTES_LENGTH: usize = 32;
pub const ALGORAND_CHECKSUM_BYTE_LENGTH: usize = 4;
pub const ALGORAND_ADDRESS_LENGTH: usize = 58;
pub const ALGORAND_PUBLIC_KEY_BYTE_LENGTH: usize = 32;
pub const ALGORAND_SECRET_KEY_BYTE_LENGTH: usize = 32;
pub const ALGORAND_SIGNATURE_BYTE_LENGTH: usize = 64;
pub const ALGORAND_SIGNATURE_ENCODING_INCR: usize = 75;
pub type Byte32 = [u8; 32];

pub const MULTISIG_DOMAIN_SEPARATOR: &str = "MultisigAddr";
pub const EMPTY_SIGNATURE: [u8; ALGORAND_SIGNATURE_BYTE_LENGTH] =
    [0; ALGORAND_SIGNATURE_BYTE_LENGTH];

// LogicSig domain separators
pub const PROGRAM_DOMAIN_SEPARATOR: &str = "Program";
pub const MULTISIG_PROGRAM_DOMAIN_SEPARATOR: &str = "MsigProgram";
pub const PQ_PROGRAM_DOMAIN_SEPARATOR: &str = "PQProgram";

// Post-quantum schemes and domain separators
pub const PQ_SCHEME_FALCON1024: [u8; 2] = *b"f1";
pub const PQ_ADDRESS_DOMAIN_SEPARATOR: &str = "PQA";
pub const PQ_KEY_DOMAIN_SEPARATOR: &str = "PQK";
