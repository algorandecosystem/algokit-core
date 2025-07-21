use crate::abi_type::ABIType;
use crate::error::ABIError;
use sha2::{Digest, Sha512_256};
use std::fmt::Display;
use std::str::FromStr;

/// Constant for void return type in method signatures.
const VOID_RETURN_TYPE: &str = "void";

/// Transaction type in an ABI method argument.
/// These represent transactions that must be placed immediately before
/// the application call in the transaction group.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ABITransactionType {
    /// Any transaction type
    Txn,
    /// Payment transaction (algo transfer)
    Pay,
    /// Key registration transaction (configure consensus participation)
    KeyReg,
    /// Asset configuration transaction (create, configure, or destroy ASAs)
    AssetConfig,
    /// Asset transfer transaction (ASA transfer)
    AssetTransfer,
    /// Asset freeze transaction (freeze or unfreeze ASAs)
    AssetFreeze,
    /// Application call transaction (create/invoke an application)
    AppCall,
}

impl FromStr for ABITransactionType {
    type Err = ABIError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "txn" => Ok(ABITransactionType::Txn),
            "pay" => Ok(ABITransactionType::Pay),
            "keyreg" => Ok(ABITransactionType::KeyReg),
            "acfg" => Ok(ABITransactionType::AssetConfig),
            "axfer" => Ok(ABITransactionType::AssetTransfer),
            "afrz" => Ok(ABITransactionType::AssetFreeze),
            "appl" => Ok(ABITransactionType::AppCall),
            _ => Err(ABIError::ValidationError(format!(
                "Invalid transaction type: {}",
                s
            ))),
        }
    }
}

impl Display for ABITransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ABITransactionType::Txn => "txn",
            ABITransactionType::Pay => "pay",
            ABITransactionType::KeyReg => "keyreg",
            ABITransactionType::AssetConfig => "acfg",
            ABITransactionType::AssetTransfer => "axfer",
            ABITransactionType::AssetFreeze => "afrz",
            ABITransactionType::AppCall => "appl",
        };
        write!(f, "{}", s)
    }
}

/// Reference type in an ABI method argument.
/// These are encoded as uint8 indices into the transaction's foreign arrays.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ABIReferenceType {
    /// Reference to an account in the Accounts (apat) array
    Account,
    /// Reference to an application in the Foreign Apps (apfa) array
    Application,
    /// Reference to an asset in the Foreign Assets (apas) array
    Asset,
}

impl FromStr for ABIReferenceType {
    type Err = ABIError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "account" => Ok(ABIReferenceType::Account),
            "application" => Ok(ABIReferenceType::Application),
            "asset" => Ok(ABIReferenceType::Asset),
            _ => Err(ABIError::ValidationError(format!(
                "Invalid reference type: {}",
                s
            ))),
        }
    }
}

impl Display for ABIReferenceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ABIReferenceType::Account => "account",
            ABIReferenceType::Application => "application",
            ABIReferenceType::Asset => "asset",
        };
        write!(f, "{}", s)
    }
}

/// Category of an ABI method argument.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ABIMethodArgType {
    /// Value types (uint64, string, tuples, arrays, etc.) that are directly encoded in ApplicationArgs
    /// These represent actual data values passed to the method
    Value(ABIType),
    /// Transaction types that are placed as preceding transactions in the group
    /// These don't occupy space in ApplicationArgs
    Transaction(ABITransactionType),
    /// Reference types encoded as uint8 indices into foreign arrays
    /// These are encoded in ApplicationArgs as indices
    Reference(ABIReferenceType),
}

impl ABIMethodArgType {
    /// Parse an argument type from a string.
    ///
    /// This is a convenience method that delegates to the `FromStr` trait implementation.
    /// Use `ABIMethodArgType::from_str(s)` or `s.parse::<ABIMethodArgType>()` for idiomatic Rust.
    pub fn parse(s: &str) -> Result<Self, ABIError> {
        Self::from_str(s)
    }

    /// Check if this is a transaction argument.
    pub(crate) fn is_transaction(&self) -> bool {
        matches!(self, ABIMethodArgType::Transaction(_))
    }

    /// Check if this is a reference argument.
    pub(crate) fn is_reference(&self) -> bool {
        matches!(self, ABIMethodArgType::Reference(_))
    }

    /// Check if this is a value type argument (directly encoded in ApplicationArgs).
    pub(crate) fn is_value_type(&self) -> bool {
        matches!(self, ABIMethodArgType::Value(_))
    }
}

impl FromStr for ABIMethodArgType {
    type Err = ABIError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Check for direct transaction types first (e.g., "pay", "keyreg", etc.)
        if let Ok(tx_type) = ABITransactionType::from_str(s) {
            return Ok(ABIMethodArgType::Transaction(tx_type));
        }

        // Check for reference types
        if let Ok(ref_type) = ABIReferenceType::from_str(s) {
            return Ok(ABIMethodArgType::Reference(ref_type));
        }

        // Default to ABI value type
        let abi_type = ABIType::from_str(s)?;
        Ok(ABIMethodArgType::Value(abi_type))
    }
}

/// Parsed ABI method.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ABIMethod {
    pub name: String,
    pub args: Vec<ABIMethodArg>,
    pub returns: Option<ABIType>,
    pub desc: Option<String>,
}

/// Argument in an ABI method.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ABIMethodArg {
    pub r#type: ABIMethodArgType,
    pub name: Option<String>,
    pub desc: Option<String>,
}

impl ABIMethod {
    /// Create a new ABIMethod.
    pub fn new(
        name: String,
        args: Vec<ABIMethodArg>,
        returns: Option<ABIType>,
        desc: Option<String>,
    ) -> Self {
        Self {
            name,
            args,
            returns,
            desc,
        }
    }

    /// Get the number of transaction arguments.
    pub fn transaction_arg_count(&self) -> usize {
        self.args
            .iter()
            .filter(|arg| arg.r#type.is_transaction())
            .count()
    }

    /// Get the number of reference arguments.
    pub fn reference_arg_count(&self) -> usize {
        self.args
            .iter()
            .filter(|arg| arg.r#type.is_reference())
            .count()
    }

    /// Get the number of value type arguments (directly encoded in ApplicationArgs).
    pub fn value_arg_count(&self) -> usize {
        self.args
            .iter()
            .filter(|arg| arg.r#type.is_value_type())
            .count()
    }

    /// Get the method selector (4-byte hash).
    pub fn selector(&self) -> Result<Vec<u8>, ABIError> {
        let signature = self.signature()?;
        get_method_selector(&signature)
    }

    /// Get the method signature string.
    pub fn signature(&self) -> Result<String, ABIError> {
        let arg_types: Vec<String> = self
            .args
            .iter()
            .map(|arg| match &arg.r#type {
                ABIMethodArgType::Value(abi_type) => abi_type.to_string(),
                ABIMethodArgType::Transaction(tx_type) => tx_type.to_string(),
                ABIMethodArgType::Reference(ref_type) => ref_type.to_string(),
            })
            .collect();

        let arg_refs: Vec<&str> = arg_types.iter().map(|s| s.as_str()).collect();
        let return_type = self
            .returns
            .as_ref()
            .map(|r| r.to_string())
            .unwrap_or_else(|| VOID_RETURN_TYPE.to_string());

        build_method_signature(&self.name, &arg_refs, &return_type)
    }
}

impl FromStr for ABIMethod {
    type Err = ABIError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        get_method_signature(s)
    }
}

impl ABIMethodArg {
    /// Create a new ABIMethodArg.
    pub fn new(r#type: ABIMethodArgType, name: Option<String>, desc: Option<String>) -> Self {
        Self { r#type, name, desc }
    }
}

/// Parse an ABI method signature into its components.
pub fn get_method_signature(signature: &str) -> Result<ABIMethod, ABIError> {
    if signature.chars().any(|c| c.is_whitespace()) {
        return Err(ABIError::ValidationError(
            "Method signature cannot contain whitespace".to_string(),
        ));
    }

    let open_paren_pos = signature.find('(').ok_or_else(|| {
        ABIError::ValidationError("Method signature must contain opening parenthesis".to_string())
    })?;

    if open_paren_pos == 0 {
        return Err(ABIError::ValidationError(
            "Method name cannot be empty".to_string(),
        ));
    }
    let method_name = signature[..open_paren_pos].to_string();

    let close_paren_pos = find_matching_closing_paren(signature, open_paren_pos)?;

    let args_str = &signature[open_paren_pos + 1..close_paren_pos];

    let arguments = if args_str.is_empty() {
        Vec::new()
    } else {
        split_arguments_by_comma(args_str)?
    };

    let return_type = if close_paren_pos + 1 < signature.len() {
        signature[close_paren_pos + 1..].to_string()
    } else {
        VOID_RETURN_TYPE.to_string()
    };

    // Parse each argument
    let mut args = Vec::new();
    for (i, arg_type) in arguments.iter().enumerate() {
        let _type = ABIMethodArgType::from_str(arg_type)?;
        let arg_name = Some(format!("arg{}", i));
        let arg = ABIMethodArg::new(_type, arg_name, None);
        args.push(arg);
    }

    // Parse return type
    let returns = if return_type != VOID_RETURN_TYPE {
        let abi_return_type = ABIType::from_str(&return_type)?;
        Some(abi_return_type)
    } else {
        None
    };

    let parsed_method = ABIMethod::new(method_name, args, returns, None);

    Ok(parsed_method)
}

/// Find the matching closing parenthesis for an opening parenthesis.
fn find_matching_closing_paren(s: &str, open_pos: usize) -> Result<usize, ABIError> {
    let chars: Vec<char> = s.chars().collect();
    let mut depth = 0;

    for (i, &ch) in chars.iter().enumerate().skip(open_pos) {
        match ch {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Ok(i);
                }
            }
            _ => {}
        }
    }

    Err(ABIError::ValidationError(
        "Mismatched parentheses in method signature".to_string(),
    ))
}

/// Split arguments by comma, respecting nested parentheses.
fn split_arguments_by_comma(args_str: &str) -> Result<Vec<String>, ABIError> {
    let mut arguments = Vec::new();
    let mut current_arg = String::new();
    let mut depth = 0;

    for ch in args_str.chars() {
        match ch {
            '(' => {
                depth += 1;
                current_arg.push(ch);
            }
            ')' => {
                depth -= 1;
                if depth < 0 {
                    return Err(ABIError::ValidationError(
                        "Mismatched parentheses in method signature".to_string(),
                    ));
                }
                current_arg.push(ch);
            }
            ',' if depth == 0 => {
                if current_arg.is_empty() {
                    return Err(ABIError::ValidationError(
                        "Empty argument in method signature".to_string(),
                    ));
                }
                arguments.push(current_arg.trim().to_string());
                current_arg.clear();
            }
            _ => {
                current_arg.push(ch);
            }
        }
    }

    if !current_arg.is_empty() {
        arguments.push(current_arg.trim().to_string());
    }

    if depth != 0 {
        return Err(ABIError::ValidationError(
            "Mismatched parentheses in method signature".to_string(),
        ));
    }

    Ok(arguments)
}

/// Calculate the ABI method selector from a method signature.
pub fn get_method_selector(signature: &str) -> Result<Vec<u8>, ABIError> {
    if signature.chars().any(|c| c.is_whitespace()) {
        return Err(ABIError::ValidationError(
            "Method signature cannot contain whitespace".to_string(),
        ));
    }

    let mut hasher = Sha512_256::new();
    hasher.update(signature.as_bytes());
    let hash = hasher.finalize();

    Ok(hash[..4].to_vec())
}

/// Build an ABI method signature from components.
pub fn build_method_signature(
    name: &str,
    arg_types: &[&str],
    return_type: &str,
) -> Result<String, ABIError> {
    if name.is_empty() {
        return Err(ABIError::ValidationError(
            "Method name cannot be empty".to_string(),
        ));
    }

    for arg_type in arg_types {
        ABIMethodArgType::from_str(arg_type)?;
    }

    let args_str = arg_types.join(",");
    let signature = format!("{}({}){}", name, args_str, return_type);

    if signature.chars().any(|c| c.is_whitespace()) {
        return Err(ABIError::ValidationError(
            "Generated signature contains whitespace".to_string(),
        ));
    }

    Ok(signature)
}

/// Find a method by name in a collection of methods.
pub fn get_method_by_name<'a>(
    methods: &'a [ABIMethod],
    name: &str,
) -> Result<&'a ABIMethod, ABIError> {
    let filtered_methods: Vec<&ABIMethod> = methods
        .iter()
        .filter(|method| method.name == name)
        .collect();

    match filtered_methods.len() {
        0 => Err(ABIError::ValidationError(format!(
            "found 0 methods with the name {}",
            name
        ))),
        1 => Ok(filtered_methods[0]),
        count => {
            let signatures: Vec<String> = filtered_methods
                .iter()
                .map(|method| {
                    method
                        .signature()
                        .unwrap_or_else(|_| format!("{}(?)", method.name))
                })
                .collect();
            Err(ABIError::ValidationError(format!(
                "found {} methods with the same name {}: {}",
                count,
                name,
                signatures.join(",")
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex;

    #[test]
    fn test_transaction_type_from_str() {
        let test_cases = vec![
            ("txn", Ok(ABITransactionType::Txn)),
            ("pay", Ok(ABITransactionType::Pay)),
            ("keyreg", Ok(ABITransactionType::KeyReg)),
            ("acfg", Ok(ABITransactionType::AssetConfig)),
            ("axfer", Ok(ABITransactionType::AssetTransfer)),
            ("afrz", Ok(ABITransactionType::AssetFreeze)),
            ("appl", Ok(ABITransactionType::AppCall)),
            ("invalid", Err(())),
        ];

        for (input, expected) in test_cases {
            match expected {
                Ok(expected_type) => {
                    assert_eq!(ABITransactionType::from_str(input).unwrap(), expected_type)
                }
                Err(_) => assert!(ABITransactionType::from_str(input).is_err()),
            }
        }
    }

    #[test]
    fn test_reference_type_from_str() {
        let test_cases = vec![
            ("account", Ok(ABIReferenceType::Account)),
            ("application", Ok(ABIReferenceType::Application)),
            ("asset", Ok(ABIReferenceType::Asset)),
            ("invalid", Err(())),
        ];

        for (input, expected) in test_cases {
            match expected {
                Ok(expected_type) => {
                    assert_eq!(ABIReferenceType::from_str(input).unwrap(), expected_type)
                }
                Err(_) => assert!(ABIReferenceType::from_str(input).is_err()),
            }
        }
    }

    #[test]
    fn test_argument_category_parse() {
        // Test direct transaction type
        match ABIMethodArgType::from_str("pay").unwrap() {
            ABIMethodArgType::Transaction(tx_type) => {
                assert_eq!(tx_type, ABITransactionType::Pay)
            }
            _ => panic!("Expected Transaction type"),
        }

        // Test reference type
        match ABIMethodArgType::from_str("account").unwrap() {
            ABIMethodArgType::Reference(ref_type) => {
                assert_eq!(ref_type, ABIReferenceType::Account)
            }
            _ => panic!("Expected Reference type"),
        }

        // Test ABI type
        match ABIMethodArgType::from_str("uint64").unwrap() {
            ABIMethodArgType::Value(abi_type) => {
                assert_eq!(abi_type.to_string(), "uint64");
            }
            _ => panic!("Expected Value type"),
        }
    }

    #[test]
    fn test_argument_category_from_str() {
        // Test that FromStr trait works consistently
        use std::str::FromStr;

        let test_cases = vec!["pay", "account", "uint64", "string", "bool"];

        for input in test_cases {
            let from_str_direct = ABIMethodArgType::from_str(input).unwrap();
            let from_str_trait = <ABIMethodArgType as FromStr>::from_str(input).unwrap();
            assert_eq!(
                from_str_direct, from_str_trait,
                "Both FromStr methods should produce identical results for input: {}",
                input
            );
        }

        // Test error cases
        assert!(ABIMethodArgType::from_str("invalid_type").is_err());
    }

    #[test]
    fn test_parsed_abi_method() {
        let args = vec![
            ABIMethodArg::new(
                ABIMethodArgType::Reference(ABIReferenceType::Account),
                Some("receiver".to_string()),
                None,
            ),
            ABIMethodArg::new(
                ABIMethodArgType::Value(ABIType::from_str("uint64").unwrap()),
                Some("amount".to_string()),
                None,
            ),
            ABIMethodArg::new(
                ABIMethodArgType::Transaction(ABITransactionType::Pay),
                Some("payment".to_string()),
                None,
            ),
        ];

        let method = ABIMethod::new(
            "transfer".to_string(),
            args,
            Some(ABIType::from_str("bool").unwrap()),
            Some("Transfer tokens to receiver".to_string()),
        );

        assert_eq!(method.transaction_arg_count(), 1);
        assert_eq!(method.reference_arg_count(), 1);
        assert_eq!(method.value_arg_count(), 1);
        assert_eq!(method.args.len(), 3);
    }

    #[test]
    fn test_get_method_signature() {
        let test_cases = vec![
            ("add(uint64,uint64)uint64", "add", Some("uint64"), 2),
            ("getName()string", "getName", Some("string"), 0),
            ("doSomething(uint64)", "doSomething", None, 1),
            (
                "process((uint64,string),bool)(uint64,bool)",
                "process",
                Some("(uint64,bool)"),
                2,
            ),
        ];

        for (signature, expected_name, expected_return, expected_arg_count) in test_cases {
            let result = get_method_signature(signature).unwrap();
            assert_eq!(result.name, expected_name);
            if let Some(expected_return_str) = expected_return {
                let expected_abi_type = ABIType::from_str(expected_return_str).unwrap();
                assert_eq!(result.returns, Some(expected_abi_type));
            } else {
                assert_eq!(result.returns, None);
            }
            assert_eq!(
                result.args.len(),
                expected_arg_count,
                "Wrong number of args for {}",
                signature
            );
        }

        assert!(get_method_signature("add(uint64, uint64)uint64").is_err());
        assert!(get_method_signature("(uint64)uint64").is_err());
        assert!(get_method_signature("method").is_err());
    }

    #[test]
    fn test_get_method_signature_with_args() {
        // Test that arguments are properly parsed and categorized
        let method = get_method_signature("transfer(address,uint64,pay)bool").unwrap();
        assert_eq!(method.name, "transfer");
        assert_eq!(method.returns, Some(ABIType::from_str("bool").unwrap()));
        assert_eq!(method.args.len(), 3);

        // Check first argument (address - ABI type)
        assert_eq!(method.args[0].name, Some("arg0".to_string()));
        assert!(
            matches!(method.args[0].r#type, ABIMethodArgType::Value(ref abi_type) if abi_type.to_string() == "address")
        );

        // Check second argument (uint64 - ABI type)
        assert_eq!(method.args[1].name, Some("arg1".to_string()));
        assert!(
            matches!(method.args[1].r#type, ABIMethodArgType::Value(ref abi_type) if abi_type.to_string() == "uint64")
        );

        // Check third argument (pay - Transaction type)
        assert_eq!(method.args[2].name, Some("arg2".to_string()));
        assert!(matches!(
            method.args[2].r#type,
            ABIMethodArgType::Transaction(ABITransactionType::Pay)
        ));

        // Test method with reference types
        let method2 = get_method_signature("addAsset(asset,account)void").unwrap();
        assert_eq!(method2.args.len(), 2);
        assert!(matches!(
            method2.args[0].r#type,
            ABIMethodArgType::Reference(ABIReferenceType::Asset)
        ));
        assert!(matches!(
            method2.args[1].r#type,
            ABIMethodArgType::Reference(ABIReferenceType::Account)
        ));

        // Test with complex types
        let method3 = get_method_signature("swap((uint64,address),string[])uint64").unwrap();
        assert_eq!(method3.args.len(), 2);
        assert!(
            matches!(method3.args[0].r#type, ABIMethodArgType::Value(ref abi_type) if abi_type.to_string() == "(uint64,address)")
        );
        assert!(
            matches!(method3.args[1].r#type, ABIMethodArgType::Value(ref abi_type) if abi_type.to_string() == "string[]")
        );
    }

    #[test]
    fn test_abi_method_arg_type_from_str() {
        let transaction_types = vec!["pay", "keyreg", "acfg", "axfer", "afrz", "appl"];
        for txn_type in transaction_types {
            let result = ABIMethodArgType::from_str(txn_type).unwrap();
            assert!(matches!(result, ABIMethodArgType::Transaction(_)));
        }

        let reference_types = vec!["account", "application", "asset"];
        for ref_type in reference_types {
            let result = ABIMethodArgType::from_str(ref_type).unwrap();
            assert!(matches!(result, ABIMethodArgType::Reference(_)));
        }

        let abi_types = vec![
            "uint64",
            "string",
            "bool",
            "address",
            "(uint64,string)",
            "uint64[]",
        ];
        for abi_type in abi_types {
            let result = ABIMethodArgType::from_str(abi_type).unwrap();
            assert!(matches!(result, ABIMethodArgType::Value(_)));
        }
    }

    #[test]
    fn test_get_method_selector() {
        let test_signatures = vec![
            "transfer(address,uint64)bool",
            "optIn()void",
            "swap(uint64,(address,uint64))uint64",
            "deposit(pay,address)uint64",
            "addUser(account)void",
            "noArgs()uint64",
        ];

        for signature in test_signatures {
            let selector = get_method_selector(signature).unwrap();
            assert_eq!(selector.len(), 4);
        }

        assert!(get_method_selector("add(uint64, uint64)uint64").is_err());
    }

    #[test]
    fn test_method_selector_values() {
        let test_cases = vec![
            ("add(uint64,uint64)uint64", "fe6bdf69"),
            ("transfer(uint64,address)void", "e9bb5be3"),
            ("optIn()void", "29314d95"),
            ("deposit(pay,uint64)void", "f2355b55"),
            ("addUser(account,string)void", "2156f19c"),
            ("bootstrap(pay,pay,application)void", "895c2a3b"),
        ];

        for (signature, expected_hex) in test_cases {
            let selector = get_method_selector(signature).unwrap();
            assert_eq!(hex::encode(&selector), expected_hex);
        }
    }

    #[test]
    fn test_selector_properties() {
        let sig = "transfer(address,uint64)bool";
        let selector1 = get_method_selector(sig).unwrap();
        let selector2 = get_method_selector(sig).unwrap();
        assert_eq!(selector1, selector2);

        let selector3 = get_method_selector("transfer(address,uint32)bool").unwrap();
        assert_ne!(selector1, selector3);

        let selector4 = get_method_selector("send(address,uint64)bool").unwrap();
        assert_ne!(selector1, selector4);
    }

    #[test]
    fn test_abi_method_instance_methods() {
        // Test method selector() and signature() methods
        let method = get_method_signature("transfer(address,uint64)bool").unwrap();

        // Test signature method
        let signature = method.signature().unwrap();
        assert_eq!(signature, "transfer(address,uint64)bool");

        // Test selector method
        let selector = method.selector().unwrap();
        let expected_selector = get_method_selector("transfer(address,uint64)bool").unwrap();
        assert_eq!(selector, expected_selector);

        // Test txn_count method
        assert_eq!(method.transaction_arg_count(), 0); // 0 (no transaction args)

        // Test method with transaction args
        let method_with_txn = get_method_signature("deposit(pay,account,uint64)void").unwrap();
        assert_eq!(method_with_txn.transaction_arg_count(), 1); // 1 transaction arg
    }

    #[test]
    fn test_abi_method_from_str() {
        let signature = "swap(uint64,(address,uint64))uint64";
        let method = signature.parse::<ABIMethod>().unwrap();

        assert_eq!(method.name, "swap");
        assert_eq!(method.returns, Some(ABIType::from_str("uint64").unwrap()));
        assert_eq!(method.args.len(), 2);

        // Verify it's the same as using get_method_signature directly
        let method2 = get_method_signature(signature).unwrap();
        assert_eq!(method, method2);

        // Test using FromStr::from_str directly
        let method3 = ABIMethod::from_str(signature).unwrap();
        assert_eq!(method, method3);
    }

    #[test]
    fn test_get_method_by_name() {
        let method1 = get_method_signature("transfer(address,uint64)bool").unwrap();
        let method2 = get_method_signature("deposit(pay,uint64)void").unwrap();
        let method3 = get_method_signature("getBalance()uint64").unwrap();

        let methods = vec![method1, method2, method3];

        // Test finding existing methods
        let found = get_method_by_name(&methods, "transfer").unwrap();
        assert_eq!(found.name, "transfer");
        assert_eq!(found.args.len(), 2);

        let found2 = get_method_by_name(&methods, "getBalance").unwrap();
        assert_eq!(found2.name, "getBalance");
        assert_eq!(found2.args.len(), 0);

        // Test method not found
        let not_found = get_method_by_name(&methods, "nonexistent");
        assert!(not_found.is_err());

        // Test the error message
        if let Err(ABIError::ValidationError(msg)) = not_found {
            assert!(msg.contains("found 0 methods with the name nonexistent"));
        } else {
            panic!("Expected ValidationError");
        }
    }

    #[test]
    fn test_get_method_by_name_multiple_matches() {
        // Create methods with the same name but different signatures
        let method1 = get_method_signature("transfer(address,uint64)bool").unwrap();
        let method2 = get_method_signature("transfer(address,uint64,string)void").unwrap();

        let methods = vec![method1, method2];

        // Test that multiple matches return an error
        let result = get_method_by_name(&methods, "transfer");
        assert!(result.is_err());

        if let Err(ABIError::ValidationError(msg)) = result {
            assert!(msg.contains("found 2 methods with the same name transfer"));
            assert!(msg.contains("transfer(address,uint64)bool"));
            assert!(msg.contains("transfer(address,uint64,string)void"));
        } else {
            panic!("Expected ValidationError for multiple matches");
        }
    }
}
