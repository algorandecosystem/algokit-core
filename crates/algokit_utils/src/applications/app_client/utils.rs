use algokit_abi::ABIValue;

use super::AppClient;
use crate::AppClientError;
use crate::clients::app_manager::AppState;
use crate::transactions::TransactionSenderError;
use crate::transactions::composer::ComposerError;

use std::collections::HashMap;
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
) -> TransactionSenderError {
    match &err {
        TransactionSenderError::ComposerError {
            source: ComposerError::PoolError { message },
        } => {
            let tx_err = crate::transactions::TransactionResultError::ParsingError {
                message: message.clone(),
            };
            let logic = client.expose_logic_error(&tx_err, is_clear);
            let msg = format_logic_error_message(&logic);
            TransactionSenderError::ValidationError { message: msg }
        }
        _ => err,
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

pub async fn get_abi_decoded_value(
    key: &[u8],
    state: &HashMap<Vec<u8>, AppState>,
    abi_type_str: &str,
    default_value_type: Option<&str>,
) -> Result<ABIValue, AppClientError> {
    let app_state = state
        .get(key)
        .ok_or_else(|| AppClientError::ValidationError {
            message: format!("State key not found: {:?}", key),
        })?;
    let effective_type = default_value_type.unwrap_or(abi_type_str);
    super::state_accessor::decode_app_state_value(effective_type, app_state)
}
