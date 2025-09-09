use super::AppClient;
use crate::AppClientError;
use crate::transactions::TransactionSenderError;
use crate::transactions::composer::ComposerError;
use algokit_abi::{ABIType, ABIValue, AVM_BYTES, AVM_STRING, AVM_UINT64};
use std::str::FromStr;

/// Format a logic error message with details.
pub fn format_logic_error_message(error: &super::types::LogicError) -> String {
    let mut parts = vec![error.logic_error_str.clone()];
    if let Some(line) = error.line_no {
        parts.push(format!("at line {}", line));
    }
    if let Some(pc) = error.pc {
        parts.push(format!("(pc={})", pc));
    }
    if let Some(lines) = &error.lines {
        parts.push("\n--- program listing ---".to_string());
        parts.extend(lines.iter().cloned());
        parts.push("--- end listing ---".to_string());
    }
    parts.join(" ")
}

/// Transform a transaction error with logic error enhancement.
pub fn transform_transaction_error(
    client: &AppClient,
    err: TransactionSenderError,
    is_clear: bool,
) -> AppClientError {
    match &err {
        // TODO: confirm this?
        TransactionSenderError::ComposerError {
            source: ComposerError::PoolError { message },
        } => {
            let tx_err = crate::transactions::TransactionResultError::ParsingError {
                message: message.clone(),
            };
            let logic = client.expose_logic_error(&tx_err, is_clear);
            let msg = format_logic_error_message(&logic);
            AppClientError::ValidationError { message: msg }
        }
        _ => AppClientError::TransactionSenderError { source: err },
    }
}

/// Parse account reference strings to addresses.
pub fn parse_account_refs_strs(
    account_refs: &Option<Vec<String>>,
) -> Result<Option<Vec<algokit_transact::Address>>, AppClientError> {
    match account_refs {
        None => Ok(None),
        Some(refs) => {
            let mut result = Vec::with_capacity(refs.len());
            for s in refs {
                result.push(
                    algokit_transact::Address::from_str(s)
                        .map_err(|e| AppClientError::TransactError { source: e })?,
                );
            }
            Ok(Some(result))
        }
    }
}

pub fn get_abi_decoded_value(value: &[u8], value_type: &str) -> Result<ABIValue, AppClientError> {
    match value_type {
        AVM_STRING => {
            let s = String::from_utf8(value.to_vec()).map_err(|_| AppClientError::DecodeError {
                message: "Failed to convert bytes to utf-8 string".to_string(),
            })?;
            Ok(ABIValue::from(s))
        }
        AVM_BYTES => Ok(ABIValue::Bytes(value.to_vec())),
        AVM_UINT64 => {
            let uint64_abi_type =
                ABIType::from_str("uint64").map_err(|e| AppClientError::ABIError { source: e })?;
            Ok(uint64_abi_type
                .decode(&value)
                .map_err(|e| AppClientError::ABIError { source: e })?)
        }
        _ => {
            // TODO: struct will be handled in another PR
            let abi_type = ABIType::from_str(&value_type)
                .map_err(|e| AppClientError::ABIError { source: e })?;
            Ok(abi_type
                .decode(&value)
                .map_err(|e| AppClientError::ABIError { source: e })?)
        }
    }
}
