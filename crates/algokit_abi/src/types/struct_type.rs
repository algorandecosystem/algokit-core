use crate::{ABIError, ABIType, ABIValue};
use std::collections::HashMap;

/// Represents an ABI struct type with named fields
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructType {
    /// The name of the struct type
    pub name: String,
    /// The fields of the struct in order
    pub fields: Vec<StructField>,
}

/// Represents a field in a struct
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructField {
    /// The name of the field
    pub name: String,
    /// The ABI type of the field
    pub abi_type: Box<ABIType>,
}

impl StructType {
    /// Create a new struct type
    pub fn new(name: impl Into<String>, fields: Vec<StructField>) -> Self {
        Self {
            name: name.into(),
            fields,
        }
    }

    /// Convert this struct type to an equivalent tuple type
    pub fn to_tuple_type(&self) -> ABIType {
        let tuple_types: Vec<ABIType> = self
            .fields
            .iter()
            .map(|field| (*field.abi_type).clone())
            .collect();
        ABIType::Tuple(tuple_types)
    }

    /// Convert a struct value (HashMap) to a tuple value (Vec) for encoding
    pub fn struct_to_tuple(
        &self,
        struct_map: &HashMap<String, ABIValue>,
    ) -> Result<Vec<ABIValue>, ABIError> {
        let mut tuple_values = Vec::with_capacity(self.fields.len());

        for field in &self.fields {
            let value = struct_map
                .get(&field.name)
                .ok_or_else(|| ABIError::ValidationError {
                    message: format!("Missing field '{}' in struct '{}'", field.name, self.name),
                })?;

            // If the field is itself a struct, it should already be in the correct ABIValue form
            tuple_values.push(value.clone());
        }

        Ok(tuple_values)
    }

    /// Convert a tuple value (Vec) to a struct value (HashMap) after decoding
    pub fn tuple_to_struct(
        &self,
        tuple_values: Vec<ABIValue>,
    ) -> Result<HashMap<String, ABIValue>, ABIError> {
        if tuple_values.len() != self.fields.len() {
            return Err(ABIError::ValidationError {
                message: format!(
                    "Tuple length {} doesn't match struct '{}' field count {}",
                    tuple_values.len(),
                    self.name,
                    self.fields.len()
                ),
            });
        }

        let mut struct_map = HashMap::with_capacity(self.fields.len());

        for (field, value) in self.fields.iter().zip(tuple_values.into_iter()) {
            struct_map.insert(field.name.clone(), value);
        }

        Ok(struct_map)
    }
}

impl StructField {
    /// Create a new struct field
    pub fn new(name: impl Into<String>, abi_type: ABIType) -> Self {
        Self {
            name: name.into(),
            abi_type: Box::new(abi_type),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abi_type::BitSize;
    use num_bigint::BigUint;

    #[test]
    fn test_struct_to_tuple_type() {
        let struct_type = StructType::new(
            "Person",
            vec![
                StructField::new("name", ABIType::String),
                StructField::new("age", ABIType::Uint(BitSize::new(64).unwrap())),
                StructField::new("active", ABIType::Bool),
            ],
        );

        let tuple_type = struct_type.to_tuple_type();
        match tuple_type {
            ABIType::Tuple(types) => {
                assert_eq!(types.len(), 3);
                assert_eq!(types[0], ABIType::String);
                assert_eq!(types[1], ABIType::Uint(BitSize::new(64).unwrap()));
                assert_eq!(types[2], ABIType::Bool);
            }
            _ => panic!("Expected tuple type"),
        }
    }

    #[test]
    fn test_struct_to_tuple_conversion() {
        let struct_type = StructType::new(
            "Point",
            vec![
                StructField::new("x", ABIType::Uint(BitSize::new(32).unwrap())),
                StructField::new("y", ABIType::Uint(BitSize::new(32).unwrap())),
            ],
        );

        let mut struct_map = HashMap::new();
        struct_map.insert("x".to_string(), ABIValue::Uint(BigUint::from(10u32)));
        struct_map.insert("y".to_string(), ABIValue::Uint(BigUint::from(20u32)));

        let tuple_values = struct_type.struct_to_tuple(&struct_map).unwrap();
        assert_eq!(tuple_values.len(), 2);
        assert_eq!(tuple_values[0], ABIValue::Uint(BigUint::from(10u32)));
        assert_eq!(tuple_values[1], ABIValue::Uint(BigUint::from(20u32)));
    }

    #[test]
    fn test_tuple_to_struct_conversion() {
        let struct_type = StructType::new(
            "Point",
            vec![
                StructField::new("x", ABIType::Uint(BitSize::new(32).unwrap())),
                StructField::new("y", ABIType::Uint(BitSize::new(32).unwrap())),
            ],
        );

        let tuple_values = vec![
            ABIValue::Uint(BigUint::from(10u32)),
            ABIValue::Uint(BigUint::from(20u32)),
        ];

        let struct_map = struct_type.tuple_to_struct(tuple_values).unwrap();
        assert_eq!(struct_map.len(), 2);
        assert_eq!(
            struct_map.get("x"),
            Some(&ABIValue::Uint(BigUint::from(10u32)))
        );
        assert_eq!(
            struct_map.get("y"),
            Some(&ABIValue::Uint(BigUint::from(20u32)))
        );
    }

    #[test]
    fn test_missing_field_error() {
        let struct_type = StructType::new(
            "Point",
            vec![
                StructField::new("x", ABIType::Uint(BitSize::new(32).unwrap())),
                StructField::new("y", ABIType::Uint(BitSize::new(32).unwrap())),
            ],
        );

        let mut struct_map = HashMap::new();
        struct_map.insert("x".to_string(), ABIValue::Uint(BigUint::from(10u32)));
        // Missing "y" field

        let result = struct_type.struct_to_tuple(&struct_map);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Missing field 'y'")
        );
    }

    #[test]
    fn test_nested_struct() {
        // Create inner struct type
        let inner_struct = StructType::new(
            "Address",
            vec![
                StructField::new("street", ABIType::String),
                StructField::new("city", ABIType::String),
            ],
        );

        // Create outer struct with nested struct
        let outer_struct = StructType::new(
            "Person",
            vec![
                StructField::new("name", ABIType::String),
                StructField::new("address", ABIType::Struct(inner_struct.clone())),
            ],
        );

        // Create nested struct value
        let mut address_map = HashMap::new();
        address_map.insert(
            "street".to_string(),
            ABIValue::String("123 Main St".to_string()),
        );
        address_map.insert(
            "city".to_string(),
            ABIValue::String("Springfield".to_string()),
        );

        let mut person_map = HashMap::new();
        person_map.insert("name".to_string(), ABIValue::String("Alice".to_string()));
        person_map.insert("address".to_string(), ABIValue::Struct(address_map));

        // Convert to tuple
        let tuple_values = outer_struct.struct_to_tuple(&person_map).unwrap();
        assert_eq!(tuple_values.len(), 2);
        assert_eq!(tuple_values[0], ABIValue::String("Alice".to_string()));

        // The nested struct should remain as a struct in the tuple
        match &tuple_values[1] {
            ABIValue::Struct(map) => {
                assert_eq!(
                    map.get("street"),
                    Some(&ABIValue::String("123 Main St".to_string()))
                );
                assert_eq!(
                    map.get("city"),
                    Some(&ABIValue::String("Springfield".to_string()))
                );
            }
            _ => panic!("Expected nested struct"),
        }
    }
}
