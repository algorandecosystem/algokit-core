use crate::error::ABIError;
use sha2::{Digest, Sha512_256};
use std::str::FromStr;

/// Transaction type in an ABI method.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ABITransactionType {
    Txn,
    Pay,
    KeyReg,
    AssetConfig,
    AssetTransfer,
    AssetFreeze,
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

impl ABITransactionType {
    /// Convert to string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            ABITransactionType::Txn => "txn",
            ABITransactionType::Pay => "pay",
            ABITransactionType::KeyReg => "keyreg",
            ABITransactionType::AssetConfig => "acfg",
            ABITransactionType::AssetTransfer => "axfer",
            ABITransactionType::AssetFreeze => "afrz",
            ABITransactionType::AppCall => "appl",
        }
    }
}

/// Reference type in an ABI method.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ABIReferenceType {
    Account,
    Application,
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

impl ABIReferenceType {
    /// Convert to string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            ABIReferenceType::Account => "account",
            ABIReferenceType::Application => "application",
            ABIReferenceType::Asset => "asset",
        }
    }
}

/// Category of an ABI method argument.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ABIMethodArgType {
    ABIType(String),
    Transaction(ABITransactionType),
    Reference(ABIReferenceType),
}

impl ABIMethodArgType {
    /// Parse an argument category from a string.
    pub fn parse(s: &str) -> Result<Self, ABIError> {
        if let Some(tx_type_str) = s.strip_prefix("txn[") {
            if let Some(tx_type_str) = tx_type_str.strip_suffix(']') {
                let tx_type = ABITransactionType::from_str(tx_type_str)?;
                return Ok(ABIMethodArgType::Transaction(tx_type));
            }
        }

        if let Ok(ref_type) = ABIReferenceType::from_str(s) {
            return Ok(ABIMethodArgType::Reference(ref_type));
        }

        Ok(ABIMethodArgType::ABIType(s.to_string()))
    }

    /// Check if this is a transaction argument.
    pub fn is_transaction(&self) -> bool {
        matches!(self, ABIMethodArgType::Transaction(_))
    }

    /// Check if this is a reference argument.
    pub fn is_reference(&self) -> bool {
        matches!(self, ABIMethodArgType::Reference(_))
    }

    /// Check if this is an ABI type argument.
    pub fn is_abi_type(&self) -> bool {
        matches!(self, ABIMethodArgType::ABIType(_))
    }
}

/// Parsed ABI method.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ABIMethod {
    pub name: String,
    pub args: Vec<ABIMethodArg>,
    pub returns: Option<String>,
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
    pub fn new(name: String) -> Self {
        Self {
            name,
            args: Vec::new(),
            returns: None,
            desc: None,
        }
    }

    /// Add an argument to the method.
    pub fn add_arg(&mut self, arg: ABIMethodArg) {
        self.args.push(arg);
    }

    /// Set the return type.
    pub fn set_returns(&mut self, returns: String) {
        self.returns = Some(returns);
    }

    /// Set the description.
    pub fn set_desc(&mut self, desc: String) {
        self.desc = Some(desc);
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

    /// Get the number of ABI type arguments.
    pub fn abi_arg_count(&self) -> usize {
        self.args
            .iter()
            .filter(|arg| arg.r#type.is_abi_type())
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
                ABIMethodArgType::ABIType(type_str) => type_str.clone(),
                ABIMethodArgType::Transaction(tx_type) => tx_type.as_str().to_string(),
                ABIMethodArgType::Reference(ref_type) => ref_type.as_str().to_string(),
            })
            .collect();

        let arg_refs: Vec<&str> = arg_types.iter().map(|s| s.as_str()).collect();
        let return_type = self.returns.as_deref().unwrap_or("void");

        build_method_signature(&self.name, &arg_refs, return_type)
    }

    /// Get the total transaction count (1 + number of transaction arguments).
    pub fn txn_count(&self) -> usize {
        1 + self.transaction_arg_count()
    }

    /// Create an ABIMethod from a method signature string.
    pub fn from_signature(signature: &str) -> Result<Self, ABIError> {
        get_method_signature(signature)
    }
}

impl ABIMethodArg {
    /// Create a new ABIMethodArg.
    pub fn new(r#type: ABIMethodArgType, name: Option<String>, desc: Option<String>) -> Self {
        Self { r#type, name, desc }
    }

    /// Set the description.
    pub fn set_desc(&mut self, desc: String) {
        self.desc = Some(desc);
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
        "void".to_string()
    };

    let mut parsed_method = ABIMethod::new(method_name);

    // Parse each argument and add it to the method
    for (i, arg_type) in arguments.iter().enumerate() {
        let category = validate_abi_argument_type(arg_type)?;
        let arg_name = Some(format!("arg{}", i));
        let arg = ABIMethodArg::new(category, arg_name, None);
        parsed_method.add_arg(arg);
    }

    if return_type != "void" {
        parsed_method.set_returns(return_type);
    }

    Ok(parsed_method)
}

/// Validate an ABI argument type and categorize it.
fn validate_abi_argument_type(arg_type: &str) -> Result<ABIMethodArgType, ABIError> {
    if let Ok(tx_type) = ABITransactionType::from_str(arg_type) {
        return Ok(ABIMethodArgType::Transaction(tx_type));
    }

    if let Some(tx_type_str) = arg_type.strip_prefix("txn[") {
        if let Some(tx_type_str) = tx_type_str.strip_suffix(']') {
            if let Ok(tx_type) = ABITransactionType::from_str(tx_type_str) {
                return Ok(ABIMethodArgType::Transaction(tx_type));
            }
        }
    }

    if let Ok(ref_type) = ABIReferenceType::from_str(arg_type) {
        return Ok(ABIMethodArgType::Reference(ref_type));
    }

    Ok(ABIMethodArgType::ABIType(arg_type.to_string()))
}

/// Find the matching closing parenthesis for an opening parenthesis.
fn find_matching_closing_paren(s: &str, open_pos: usize) -> Result<usize, ABIError> {
    let chars: Vec<char> = s.chars().collect();
    let mut depth = 0;

    for i in open_pos..chars.len() {
        match chars[i] {
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
        validate_abi_argument_type(arg_type)?;
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
        match ABIMethodArgType::parse("txn[pay]").unwrap() {
            ABIMethodArgType::Transaction(tx_type) => {
                assert_eq!(tx_type, ABITransactionType::Pay)
            }
            _ => panic!("Expected Transaction category"),
        }

        match ABIMethodArgType::parse("account").unwrap() {
            ABIMethodArgType::Reference(ref_type) => {
                assert_eq!(ref_type, ABIReferenceType::Account)
            }
            _ => panic!("Expected Reference category"),
        }

        match ABIMethodArgType::parse("uint64").unwrap() {
            ABIMethodArgType::ABIType(type_str) => assert_eq!(type_str, "uint64"),
            _ => panic!("Expected ABIType category"),
        }
    }

    #[test]
    fn test_parsed_abi_method() {
        let mut method = ABIMethod::new("transfer".to_string());

        method.add_arg(ABIMethodArg::new(
            ABIMethodArgType::Reference(ABIReferenceType::Account),
            Some("receiver".to_string()),
            None,
        ));
        method.add_arg(ABIMethodArg::new(
            ABIMethodArgType::ABIType("uint64".to_string()),
            Some("amount".to_string()),
            None,
        ));
        method.add_arg(ABIMethodArg::new(
            ABIMethodArgType::Transaction(ABITransactionType::Pay),
            Some("payment".to_string()),
            None,
        ));

        method.set_returns("bool".to_string());
        method.set_desc("Transfer tokens to receiver".to_string());

        assert_eq!(method.transaction_arg_count(), 1);
        assert_eq!(method.reference_arg_count(), 1);
        assert_eq!(method.abi_arg_count(), 1);
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
            assert_eq!(result.returns, expected_return.map(|s| s.to_string()));
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
        assert_eq!(method.returns, Some("bool".to_string()));
        assert_eq!(method.args.len(), 3);

        // Check first argument (address - ABI type)
        assert_eq!(method.args[0].name, Some("arg0".to_string()));
        assert!(
            matches!(method.args[0].r#type, ABIMethodArgType::ABIType(ref s) if s == "address")
        );

        // Check second argument (uint64 - ABI type)
        assert_eq!(method.args[1].name, Some("arg1".to_string()));
        assert!(matches!(method.args[1].r#type, ABIMethodArgType::ABIType(ref s) if s == "uint64"));

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
            matches!(method3.args[0].r#type, ABIMethodArgType::ABIType(ref s) if s == "(uint64,address)")
        );
        assert!(
            matches!(method3.args[1].r#type, ABIMethodArgType::ABIType(ref s) if s == "string[]")
        );
    }

    #[test]
    fn test_validate_abi_argument_type() {
        let transaction_types = vec!["pay", "keyreg", "acfg", "axfer", "afrz", "appl", "txn[pay]"];
        for txn_type in transaction_types {
            let result = validate_abi_argument_type(txn_type).unwrap();
            assert!(matches!(result, ABIMethodArgType::Transaction(_)));
        }

        let reference_types = vec!["account", "application", "asset"];
        for ref_type in reference_types {
            let result = validate_abi_argument_type(ref_type).unwrap();
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
            let result = validate_abi_argument_type(abi_type).unwrap();
            assert!(matches!(result, ABIMethodArgType::ABIType(_)));
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
        assert_eq!(method.txn_count(), 1); // 1 (no transaction args)

        // Test method with transaction args
        let method_with_txn = get_method_signature("deposit(pay,account,uint64)void").unwrap();
        assert_eq!(method_with_txn.txn_count(), 2); // 1 + 1 transaction arg
    }

    #[test]
    fn test_abi_method_from_signature() {
        let signature = "swap(uint64,(address,uint64))uint64";
        let method = ABIMethod::from_signature(signature).unwrap();

        assert_eq!(method.name, "swap");
        assert_eq!(method.returns, Some("uint64".to_string()));
        assert_eq!(method.args.len(), 2);

        // Verify it's the same as using get_method_signature directly
        let method2 = get_method_signature(signature).unwrap();
        assert_eq!(method, method2);
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
