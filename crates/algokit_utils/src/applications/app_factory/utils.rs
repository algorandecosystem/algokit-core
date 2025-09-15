use super::{AppFactory, AppFactoryError};
use std::str::FromStr;

// (kept intentionally empty for future schema inference needs)

/// Merge user-provided create-time method args with ARC-56 literal defaults.
/// Only 'literal' default values are supported; others will be ignored and treated as missing.
pub(crate) fn merge_create_args_with_defaults(
    factory: &AppFactory,
    method_name_or_signature: &str,
    user_args: &Option<Vec<crate::transactions::AppMethodCallArg>>,
) -> Result<Vec<crate::transactions::AppMethodCallArg>, AppFactoryError> {
    use algokit_abi::abi_type::ABIType;
    use algokit_abi::arc56_contract::DefaultValueSource;
    use base64::Engine;
    use base64::engine::general_purpose::STANDARD as Base64;

    let contract = factory.app_spec();
    let method = contract
        .get_arc56_method(method_name_or_signature)
        .map_err(|e| AppFactoryError::ValidationError(e.to_string()))?;

    let mut result: Vec<crate::transactions::AppMethodCallArg> =
        Vec::with_capacity(method.args.len());
    let provided = user_args.as_ref().map(|v| v.as_slice()).unwrap_or(&[]);

    for (i, arg_def) in method.args.iter().enumerate() {
        if i < provided.len() {
            // Use provided argument as-is
            result.push(provided[i].clone());
            continue;
        }

        // Otherwise try literal default
        if let Some(default) = &arg_def.default_value {
            if matches!(default.source, DefaultValueSource::Literal) {
                // Determine ABI type to decode to: prefer the argument type
                let abi_type = ABIType::from_str(&arg_def.arg_type)
                    .map_err(|e| AppFactoryError::ValidationError(e.to_string()))?;

                let bytes = Base64.decode(&default.data).map_err(|e| {
                    AppFactoryError::ValidationError(format!(
                        "Failed to base64-decode default literal: {}",
                        e
                    ))
                })?;

                let abi_value = abi_type
                    .decode(&bytes)
                    .map_err(|e| AppFactoryError::ValidationError(e.to_string()))?;

                result.push(crate::transactions::AppMethodCallArg::ABIValue(abi_value));
                continue;
            }
        }

        // No provided arg and no supported default -> error like Python implementation
        let name = arg_def
            .name
            .as_ref()
            .cloned()
            .unwrap_or_else(|| format!("arg{}", i + 1));
        let method_name = &method.name;
        return Err(AppFactoryError::ValidationError(format!(
            "No value provided for required argument {} in call to method {}",
            name, method_name
        )));
    }

    Ok(result)
}

/// Transform a transaction error using AppClient logic error exposure for factory flows.
pub(crate) fn transform_transaction_error_for_factory(
    factory: &AppFactory,
    err: crate::transactions::TransactionSenderError,
    is_clear: bool,
) -> crate::transactions::TransactionSenderError {
    let client = crate::applications::app_client::AppClient::new(
        crate::applications::app_client::AppClientParams {
            app_id: None,
            app_spec: factory.app_spec().clone(),
            algorand: factory.algorand().clone(),
            app_name: Some(factory.app_name().to_string()),
            default_sender: factory.default_sender.clone(),
            source_maps: None,
        },
    );
    crate::applications::app_client::transform_transaction_error(&client, err, is_clear)
}
