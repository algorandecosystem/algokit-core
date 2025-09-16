use super::AppClient;
use crate::AppClientError;
use crate::transactions::TransactionSenderError;
use std::str::FromStr;

fn contains_logic_error(s: &str) -> bool {
    s.contains("logic eval error") && s.contains("pc=")
}

/// Transform a transaction error with logic error enhancement.
pub fn transform_transaction_error(
    client: &AppClient,
    err: TransactionSenderError,
    is_clear_state_program: bool,
) -> AppClientError {
    let err_str = err.to_string();
    if contains_logic_error(&err_str) {
        let tx_err = crate::transactions::TransactionResultError::ParsingError {
            message: err_str.clone(),
        };
        let logic = client.expose_logic_error(&tx_err, is_clear_state_program);
        return AppClientError::LogicError {
            message: logic.message.clone(),
            logic: Box::new(logic),
        };
    }

    AppClientError::TransactionSenderError { source: err }
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
